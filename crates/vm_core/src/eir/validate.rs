//! EIR validation.
//!
//! Spec: `PHASE-3-EIR-SCHEMA-CLOSURE.md` §23, `PHASE-3-VALIDATION-MATRIX.md`,
//! `PHASE-3-GC-METADATA-OWNERSHIP.md`

use std::collections::{BTreeMap, BTreeSet};

use crate::digest::Digest;
use crate::eir::resolve::resolve_wire_module;
use crate::eir::schema::{
    AccessOp, CallOp, CheckOp, ConstructOp, EirBlock, EirFunction, EirModule, EirOp, EirOpKind,
    EirTerminator, GuardFailureAction, GuardOp, PatternOp, RuntimeHelperOp,
};
use crate::eir::wire::EirModuleWire;
use crate::id::{
    AccessSiteId, CallSiteId, CaseId, ConstantId, DeoptId, EirBlockId, EirFunctionId, FieldId,
    RuntimeHelperId, SafepointId, ShapeId, SlotId, TypeId,
};

/// Known helpers for EIR validation (populated from `RuntimeHelperRegistry` via `vm_runtime::helpers::eir_validation_view`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelperRegistryView {
    known: BTreeSet<RuntimeHelperId>,
    may_collect: BTreeSet<RuntimeHelperId>,
    may_raise: BTreeSet<RuntimeHelperId>,
}

impl HelperRegistryView {
    #[must_use]
    pub fn from_ids(ids: impl IntoIterator<Item = RuntimeHelperId>) -> Self {
        Self {
            known: ids.into_iter().collect(),
            may_collect: BTreeSet::new(),
            may_raise: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn with_may_collect(mut self, ids: impl IntoIterator<Item = RuntimeHelperId>) -> Self {
        self.may_collect = ids.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_may_raise(mut self, ids: impl IntoIterator<Item = RuntimeHelperId>) -> Self {
        self.may_raise = ids.into_iter().collect();
        self
    }

    #[must_use]
    pub fn contains(&self, helper_id: RuntimeHelperId) -> bool {
        self.known.contains(&helper_id)
    }

    #[must_use]
    pub fn may_collect(&self, helper_id: RuntimeHelperId) -> bool {
        self.may_collect.contains(&helper_id)
    }

    #[must_use]
    pub fn may_raise(&self, helper_id: RuntimeHelperId) -> bool {
        self.may_raise.contains(&helper_id)
    }
}

/// Validation context for an EIR module against a RuntimePlan execution binding.
#[derive(Debug, Clone)]
pub struct EirValidationContext {
    pub slot_ids: BTreeSet<SlotId>,
    pub helper_registry: HelperRegistryView,
    pub expected_runtime_plan_digest: Digest,
    pub call_site_ids: BTreeSet<CallSiteId>,
    pub access_site_ids: BTreeSet<AccessSiteId>,
    pub safepoint_ids: BTreeSet<SafepointId>,
    pub deopt_ids: BTreeSet<DeoptId>,
    pub type_ids: BTreeSet<TypeId>,
    pub shape_ids: BTreeSet<ShapeId>,
    pub field_ids: BTreeSet<FieldId>,
    pub case_ids: BTreeSet<CaseId>,
    /// GC profile requires write barriers on heap-reference mutations.
    pub requires_write_barrier: bool,
    /// Access sites registered with write-barrier policy (RuntimePlan binding).
    pub barrier_access_site_ids: BTreeSet<AccessSiteId>,
    pub gc_may_run: bool,
}

/// Input to the single EIR validation entry point.
#[derive(Debug, Clone, Copy)]
#[allow(private_interfaces)]
pub enum EirModuleInput<'a> {
    Resolved(&'a EirModule),
    Wire(&'a EirModuleWire),
}

/// EIR validation failure codes aligned with frozen rejection requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EirValidationError {
    UnknownOpKind(u8),
    UnknownTerminatorKind(u8),
    UnknownSlotId(SlotId),
    UnknownConstantId(ConstantId),
    UnknownTypeId(TypeId),
    UnknownShapeId(ShapeId),
    UnknownFieldId(FieldId),
    UnknownCaseId(CaseId),
    UnknownRuntimeHelperId(RuntimeHelperId),
    UnknownCallSiteId(CallSiteId),
    UnknownAccessSiteId(AccessSiteId),
    UnknownSafepointId(SafepointId),
    UnknownDeoptId(DeoptId),
    BlockWithoutTerminator(EirBlockId),
    InvalidBlockGraph(EirBlockId),
    InvalidBlockArgumentCount(EirBlockId),
    MayRaiseWithoutSourceMapping,
    GuardWithoutFailureAction,
    MayCollectWithoutRootMap,
    HeapWriteWithoutBarrierPolicy,
    RuntimePlanDigestMismatch,
    InvalidEntryBlock(EirFunctionId),
}

impl std::fmt::Display for EirValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for EirValidationError {}

/// Validate a complete EIR module before execution (wire or resolved).
pub fn validate_eir_module(
    input: EirModuleInput<'_>,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    match input {
        EirModuleInput::Resolved(module) => validate_eir_module_semantic(module, ctx),
        EirModuleInput::Wire(wire) => {
            if wire.source_runtime_plan_digest != ctx.expected_runtime_plan_digest {
                return Err(EirValidationError::RuntimePlanDigestMismatch);
            }
            let module = resolve_wire_module(wire)?;
            validate_eir_module_semantic(&module, ctx)
        }
    }
}

fn validate_eir_module_semantic(
    module: &EirModule,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    if module.source_runtime_plan_digest != ctx.expected_runtime_plan_digest {
        return Err(EirValidationError::RuntimePlanDigestMismatch);
    }
    let safepoints_with_root_map = validate_gc_metadata(module, ctx)?;
    for function in &module.functions {
        validate_eir_function(function, module, ctx, &safepoints_with_root_map)?;
    }
    Ok(())
}

fn validate_gc_metadata(
    module: &EirModule,
    ctx: &EirValidationContext,
) -> Result<BTreeSet<SafepointId>, EirValidationError> {
    let mut safepoints_with_root_map = BTreeSet::new();

    for root_map in module.root_maps.maps.values() {
        for slot in &root_map.roots {
            require_slot(*slot, ctx)?;
        }
    }

    for record in module.safepoints.records.values() {
        require_safepoint(record.safepoint_id, ctx)?;
        let root_map_key = record.root_map.raw();
        let root_map_valid = record.root_map.is_valid()
            && module.root_maps.maps.contains_key(&root_map_key);
        if ctx.gc_may_run {
            if !root_map_valid {
                return Err(EirValidationError::MayCollectWithoutRootMap);
            }
            safepoints_with_root_map.insert(record.safepoint_id);
        }
    }

    Ok(safepoints_with_root_map)
}

/// Validate one EIR function including block graph integrity.
pub(crate) fn validate_eir_function(
    function: &EirFunction,
    module: &EirModule,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    let block_ids: BTreeSet<EirBlockId> = function.blocks.iter().map(|b| b.block_id).collect();
    let blocks_by_id: BTreeMap<EirBlockId, &EirBlock> = function
        .blocks
        .iter()
        .map(|block| (block.block_id, block))
        .collect();

    if !block_ids.contains(&function.entry_block) {
        return Err(EirValidationError::InvalidEntryBlock(function.eir_function_id));
    }

    for block in &function.blocks {
        validate_eir_block(
            block,
            &block_ids,
            &blocks_by_id,
            module,
            ctx,
            safepoints_with_root_map,
        )?;
    }
    Ok(())
}

fn validate_eir_block(
    block: &EirBlock,
    block_ids: &BTreeSet<EirBlockId>,
    blocks_by_id: &BTreeMap<EirBlockId, &EirBlock>,
    module: &EirModule,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    for op in &block.ops {
        validate_eir_op(
            op,
            block_ids,
            blocks_by_id,
            module,
            ctx,
            safepoints_with_root_map,
        )?;
    }
    validate_terminator(
        &block.terminator,
        block_ids,
        blocks_by_id,
        ctx,
        safepoints_with_root_map,
    )?;
    Ok(())
}

fn validate_eir_op(
    op: &EirOp,
    block_ids: &BTreeSet<EirBlockId>,
    _blocks_by_id: &BTreeMap<EirBlockId, &EirBlock>,
    module: &EirModule,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    if op_requires_source_mapping(&op.kind, ctx) && !has_source_mapping(&op.metadata) {
        return Err(EirValidationError::MayRaiseWithoutSourceMapping);
    }

    match &op.kind {
        EirOpKind::RuntimeHelper(helper_op) => {
            validate_helper_op(helper_op, ctx, safepoints_with_root_map)?
        }
        EirOpKind::Call(call) => validate_call_op(call, ctx)?,
        EirOpKind::Access(access) => validate_access_op(access, ctx)?,
        EirOpKind::Guard(guard) => validate_guard_op(guard, ctx, safepoints_with_root_map)?,
        EirOpKind::Load(load) => validate_slots_in_load(load, ctx)?,
        EirOpKind::Store(store) => validate_slots_in_store(store, ctx)?,
        EirOpKind::Unary(unary) => {
            require_slot(unary.dest, ctx)?;
            require_slot(unary.operand, ctx)?;
        }
        EirOpKind::Binary(binary) => {
            require_slot(binary.dest, ctx)?;
            require_slot(binary.left, ctx)?;
            require_slot(binary.right, ctx)?;
        }
        EirOpKind::Logical(logical) => {
            require_slot(logical.dest, ctx)?;
            require_block(logical.left_block, block_ids)?;
            require_block(logical.right_block, block_ids)?;
            require_block(logical.merge_block, block_ids)?;
        }
        EirOpKind::Check(check) => validate_check_op(check, ctx)?,
        EirOpKind::Construct(construct) => {
            validate_construct_op(construct, ctx, safepoints_with_root_map)?
        }
        EirOpKind::Pattern(pattern) => validate_pattern_op(pattern, block_ids, ctx)?,
        EirOpKind::Constant(constant) => {
            require_slot(constant.dest, ctx)?;
            require_constant(constant.constant, module)?;
        }
        EirOpKind::Safepoint(safepoint) => {
            require_safepoint(safepoint.safepoint_id, ctx)?;
            if ctx.gc_may_run
                && !safepoints_with_root_map.contains(&safepoint.safepoint_id)
            {
                return Err(EirValidationError::MayCollectWithoutRootMap);
            }
        }
        EirOpKind::Debug(_) => {}
    }
    Ok(())
}

fn validate_terminator(
    terminator: &EirTerminator,
    block_ids: &BTreeSet<EirBlockId>,
    blocks_by_id: &BTreeMap<EirBlockId, &EirBlock>,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    match terminator {
        EirTerminator::Jump(jump) => {
            require_block(jump.target, block_ids)?;
            for slot in &jump.args {
                require_slot(*slot, ctx)?;
            }
            require_block_transfer_args(jump.target, &jump.args, blocks_by_id)?;
        }
        EirTerminator::Branch(branch) => {
            require_slot(branch.condition, ctx)?;
            require_block(branch.then_block, block_ids)?;
            require_block(branch.else_block, block_ids)?;
        }
        EirTerminator::Return(ret) => {
            if let Some(value) = ret.value {
                require_slot(value, ctx)?;
            }
        }
        EirTerminator::Raise(raise) => {
            require_slot(raise.error, ctx)?;
        }
        EirTerminator::LoopBackedge(edge) => {
            require_block(edge.target, block_ids)?;
            require_safepoint(edge.safepoint_id, ctx)?;
            if ctx.gc_may_run && !safepoints_with_root_map.contains(&edge.safepoint_id) {
                return Err(EirValidationError::MayCollectWithoutRootMap);
            }
            for slot in &edge.args {
                require_slot(*slot, ctx)?;
            }
            require_block_transfer_args(edge.target, &edge.args, blocks_by_id)?;
        }
        EirTerminator::Switch(sw) => {
            require_slot(sw.scrutinee, ctx)?;
            require_block(sw.default, block_ids)?;
            for case in &sw.cases {
                require_slot(case.value, ctx)?;
                require_block(case.target, block_ids)?;
            }
        }
        EirTerminator::Unwind(unwind) => {
            require_slot(unwind.pending_control_slot, ctx)?;
        }
        EirTerminator::Unreachable(_) => {}
    }
    Ok(())
}

fn validate_helper_op(
    op: &RuntimeHelperOp,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    if !op.helper_id.is_valid() || !ctx.helper_registry.contains(op.helper_id) {
        return Err(EirValidationError::UnknownRuntimeHelperId(op.helper_id));
    }
    validate_may_collect_root_map(op.helper_id, op.safepoint_id, ctx, safepoints_with_root_map)?;
    if let Some(dest) = op.dest {
        require_slot(dest, ctx)?;
    }
    for arg in &op.args {
        require_slot(*arg, ctx)?;
    }
    if let Some(call_site) = op.call_site {
        require_call_site(call_site, ctx)?;
    }
    if let Some(access_site) = op.access_site {
        require_access_site(access_site, ctx)?;
    }
    if let Some(safepoint) = op.safepoint_id {
        require_safepoint(safepoint, ctx)?;
    }
    if let Some(deopt) = op.deopt_id {
        require_deopt(deopt, ctx)?;
    }
    Ok(())
}

fn validate_may_collect_root_map(
    helper_id: RuntimeHelperId,
    safepoint_id: Option<SafepointId>,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    if !ctx.gc_may_run || !ctx.helper_registry.may_collect(helper_id) {
        return Ok(());
    }
    let Some(sp) = safepoint_id else {
        return Err(EirValidationError::MayCollectWithoutRootMap);
    };
    if !safepoints_with_root_map.contains(&sp) {
        return Err(EirValidationError::MayCollectWithoutRootMap);
    }
    Ok(())
}

fn validate_call_op(call: &CallOp, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    if let Some(dest) = call.dest {
        require_slot(dest, ctx)?;
    }
    require_slot(call.callee, ctx)?;
    for arg in &call.positional_args {
        require_slot(*arg, ctx)?;
    }
    for named in &call.named_args {
        require_slot(named.slot, ctx)?;
    }
    require_call_site(call.call_site_id, ctx)?;
    if let Some(type_id) = call.result_type_check {
        require_type(type_id, ctx)?;
    }
    Ok(())
}

fn validate_access_op(
    access: &AccessOp,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    require_access_site(access.access_site_id, ctx)?;
    require_slot(access.receiver, ctx)?;
    if let Some(dest) = access.dest {
        require_slot(dest, ctx)?;
    }
    if let Some(index) = access.index {
        require_slot(index, ctx)?;
    }
    if let Some(value) = access.value {
        require_slot(value, ctx)?;
    }
    Ok(())
}

fn validate_guard_op(
    guard: &GuardOp,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    if matches!(guard.on_failure, GuardFailureAction::Raise)
        && guard.failure_error.is_none()
        && guard.helper_id.is_none()
        && guard.deopt_id.is_none()
    {
        return Err(EirValidationError::GuardWithoutFailureAction);
    }
    for input in &guard.inputs {
        require_slot(*input, ctx)?;
    }
    if let Some(helper_id) = guard.helper_id {
        if !ctx.helper_registry.contains(helper_id) {
            return Err(EirValidationError::UnknownRuntimeHelperId(helper_id));
        }
        validate_may_collect_root_map(helper_id, None, ctx, safepoints_with_root_map)?;
    }
    if let Some(deopt) = guard.deopt_id {
        require_deopt(deopt, ctx)?;
    }
    Ok(())
}

fn validate_check_op(check: &CheckOp, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    match check {
        CheckOp::Bool(c) => require_slot(c.operand, ctx)?,
        CheckOp::Type(c) => {
            require_slot(c.operand, ctx)?;
            require_type(c.expected_type, ctx)?;
        }
        CheckOp::Callable(c) => require_slot(c.operand, ctx)?,
        CheckOp::Arity(c) => require_slot(c.operand, ctx)?,
        CheckOp::Hashable(c) => require_slot(c.operand, ctx)?,
        CheckOp::Readonly(c) => require_slot(c.operand, ctx)?,
        CheckOp::Capability(c) => require_slot(c.operand, ctx)?,
        CheckOp::Shape(c) => {
            require_slot(c.operand, ctx)?;
            require_shape(c.shape_id, ctx)?;
        }
        CheckOp::Overflow(c) => {
            require_slot(c.left, ctx)?;
            require_slot(c.right, ctx)?;
        }
        CheckOp::DivisionByZero(c) => require_slot(c.divisor, ctx)?,
    }
    Ok(())
}

fn validate_construct_op(
    construct: &ConstructOp,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    match construct {
        ConstructOp::List(list) => {
            require_slot(list.dest, ctx)?;
            for element in &list.elements {
                require_slot(*element, ctx)?;
            }
            validate_allocating_root_map(list.safepoint_id, ctx, safepoints_with_root_map)?;
        }
        ConstructOp::Map(map) => {
            require_slot(map.dest, ctx)?;
            for (key, value) in &map.entries {
                require_slot(*key, ctx)?;
                require_slot(*value, ctx)?;
            }
            validate_allocating_root_map(map.safepoint_id, ctx, safepoints_with_root_map)?;
        }
        ConstructOp::Record(record) => {
            require_slot(record.dest, ctx)?;
            require_shape(record.shape_id, ctx)?;
            for value in &record.field_values {
                require_slot(*value, ctx)?;
            }
            validate_allocating_root_map(record.safepoint_id, ctx, safepoints_with_root_map)?;
        }
        ConstructOp::EnumValue(enum_value) => {
            require_slot(enum_value.dest, ctx)?;
            require_shape(enum_value.enum_shape, ctx)?;
            require_case(enum_value.case_id, ctx)?;
            for payload in &enum_value.payload_slots {
                require_slot(*payload, ctx)?;
            }
            validate_allocating_root_map(enum_value.safepoint_id, ctx, safepoints_with_root_map)?;
        }
        ConstructOp::Function(function) => {
            require_slot(function.dest, ctx)?;
            validate_allocating_root_map(function.safepoint_id, ctx, safepoints_with_root_map)?;
        }
        ConstructOp::Error(err) => {
            require_slot(err.dest, ctx)?;
            require_slot(err.message_slot, ctx)?;
        }
    }
    Ok(())
}

fn validate_allocating_root_map(
    safepoint_id: Option<SafepointId>,
    ctx: &EirValidationContext,
    safepoints_with_root_map: &BTreeSet<SafepointId>,
) -> Result<(), EirValidationError> {
    if !ctx.gc_may_run {
        return Ok(());
    }
    let Some(sp) = safepoint_id else {
        return Err(EirValidationError::MayCollectWithoutRootMap);
    };
    if !safepoints_with_root_map.contains(&sp) {
        return Err(EirValidationError::MayCollectWithoutRootMap);
    }
    require_safepoint(sp, ctx)
}

fn validate_pattern_op(
    pattern: &PatternOp,
    block_ids: &BTreeSet<EirBlockId>,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    match pattern {
        PatternOp::CheckLiteral(p) => {
            require_slot(p.scrutinee, ctx)?;
            require_slot(p.expected, ctx)?;
        }
        PatternOp::CheckRecordShape(p) => {
            require_slot(p.scrutinee, ctx)?;
            require_shape(p.record_shape, ctx)?;
        }
        PatternOp::CheckEnumCase(p) => {
            require_slot(p.scrutinee, ctx)?;
            require_shape(p.enum_shape, ctx)?;
            require_case(p.case_id, ctx)?;
        }
        PatternOp::CheckListLength(p) => require_slot(p.scrutinee, ctx)?,
        PatternOp::CheckMapKey(p) => {
            require_slot(p.scrutinee, ctx)?;
            require_slot(p.key, ctx)?;
        }
        PatternOp::Bind(bind) => {
            require_slot(bind.dest, ctx)?;
            require_slot(bind.source, ctx)?;
        }
        PatternOp::Branch(branch) => {
            require_block(branch.success_block, block_ids)?;
            require_block(branch.failure_block, block_ids)?;
        }
        PatternOp::Commit(commit) => {
            for slot in &commit.binding_slots {
                require_slot(*slot, ctx)?;
            }
        }
        PatternOp::Rollback(rollback) => {
            for slot in &rollback.restored_slots {
                require_slot(*slot, ctx)?;
            }
        }
    }
    Ok(())
}

fn validate_slots_in_load(
    load: &crate::eir::schema::LoadOp,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    use crate::eir::schema::LoadOp;
    match load {
        LoadOp::Slot(s) => {
            require_slot(s.dest, ctx)?;
            require_slot(s.source, ctx)?;
        }
        LoadOp::Cell(s) => {
            require_slot(s.dest, ctx)?;
            require_slot(s.cell_slot, ctx)?;
        }
        LoadOp::Capture(s) => require_slot(s.dest, ctx)?,
        LoadOp::ModuleSlot(s) => {
            require_slot(s.dest, ctx)?;
            require_slot(s.module_slot, ctx)?;
        }
        LoadOp::Field(s) => {
            require_slot(s.dest, ctx)?;
            require_slot(s.receiver, ctx)?;
            require_shape(s.record_shape, ctx)?;
            require_field(s.field_id, ctx)?;
            require_access_site(s.access_site_id, ctx)?;
        }
        LoadOp::EnumPayload(s) => {
            require_slot(s.dest, ctx)?;
            require_slot(s.receiver, ctx)?;
            require_shape(s.enum_shape, ctx)?;
            require_case(s.case_id, ctx)?;
        }
    }
    Ok(())
}

fn validate_slots_in_store(
    store: &crate::eir::schema::StoreOp,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    use crate::eir::schema::StoreOp;
    match store {
        StoreOp::Slot(s) => {
            require_slot(s.dest, ctx)?;
            require_slot(s.value, ctx)?;
        }
        StoreOp::Cell(s) => {
            require_slot(s.cell_slot, ctx)?;
            require_slot(s.value, ctx)?;
            if let Some(type_id) = s.type_check {
                require_type(type_id, ctx)?;
            }
        }
        StoreOp::ModuleSlot(s) => {
            require_slot(s.module_slot, ctx)?;
            require_slot(s.value, ctx)?;
            if let Some(type_id) = s.type_check {
                require_type(type_id, ctx)?;
            }
        }
        StoreOp::Field(s) => {
            require_slot(s.receiver, ctx)?;
            require_slot(s.value, ctx)?;
            require_shape(s.record_shape, ctx)?;
            require_field(s.field_id, ctx)?;
            require_access_site(s.access_site_id, ctx)?;
            require_heap_write_barrier(s.access_site_id, ctx)?;
            if let Some(type_id) = s.type_check {
                require_type(type_id, ctx)?;
            }
        }
        StoreOp::ListIndex(s) => {
            require_slot(s.receiver, ctx)?;
            require_slot(s.index, ctx)?;
            require_slot(s.value, ctx)?;
            require_access_site(s.access_site_id, ctx)?;
            require_heap_write_barrier(s.access_site_id, ctx)?;
        }
        StoreOp::MapEntry(s) => {
            require_slot(s.receiver, ctx)?;
            require_slot(s.key, ctx)?;
            require_slot(s.value, ctx)?;
            require_access_site(s.access_site_id, ctx)?;
            require_heap_write_barrier(s.access_site_id, ctx)?;
        }
    }
    Ok(())
}

fn require_constant(
    constant: ConstantId,
    module: &EirModule,
) -> Result<(), EirValidationError> {
    if !constant.is_valid() || !module.constants.constants.contains_key(&constant.raw()) {
        return Err(EirValidationError::UnknownConstantId(constant));
    }
    Ok(())
}

fn require_type(type_id: TypeId, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    if !type_id.is_valid() || !ctx.type_ids.contains(&type_id) {
        return Err(EirValidationError::UnknownTypeId(type_id));
    }
    Ok(())
}

fn require_shape(shape_id: ShapeId, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    if !shape_id.is_valid() || !ctx.shape_ids.contains(&shape_id) {
        return Err(EirValidationError::UnknownShapeId(shape_id));
    }
    Ok(())
}

fn require_field(field_id: FieldId, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    if !field_id.is_valid() || !ctx.field_ids.contains(&field_id) {
        return Err(EirValidationError::UnknownFieldId(field_id));
    }
    Ok(())
}

fn require_case(case_id: CaseId, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    if !case_id.is_valid() || !ctx.case_ids.contains(&case_id) {
        return Err(EirValidationError::UnknownCaseId(case_id));
    }
    Ok(())
}

fn require_heap_write_barrier(
    access_site_id: AccessSiteId,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    if !ctx.requires_write_barrier {
        return Ok(());
    }
    if !ctx.barrier_access_site_ids.contains(&access_site_id) {
        return Err(EirValidationError::HeapWriteWithoutBarrierPolicy);
    }
    Ok(())
}

fn require_block_transfer_args(
    target: EirBlockId,
    args: &[SlotId],
    blocks_by_id: &BTreeMap<EirBlockId, &EirBlock>,
) -> Result<(), EirValidationError> {
    let Some(block) = blocks_by_id.get(&target) else {
        return Err(EirValidationError::InvalidBlockGraph(target));
    };
    if block.parameters.len() != args.len() {
        return Err(EirValidationError::InvalidBlockArgumentCount(target));
    }
    Ok(())
}

fn require_slot(slot: SlotId, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    if !slot.is_valid() || !ctx.slot_ids.contains(&slot) {
        return Err(EirValidationError::UnknownSlotId(slot));
    }
    Ok(())
}

fn require_block(
    block: EirBlockId,
    block_ids: &BTreeSet<EirBlockId>,
) -> Result<(), EirValidationError> {
    if !block.is_valid() || !block_ids.contains(&block) {
        return Err(EirValidationError::InvalidBlockGraph(block));
    }
    Ok(())
}

fn require_call_site(
    id: CallSiteId,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    if !id.is_valid() || !ctx.call_site_ids.contains(&id) {
        return Err(EirValidationError::UnknownCallSiteId(id));
    }
    Ok(())
}

fn require_access_site(
    id: AccessSiteId,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    if !id.is_valid() || !ctx.access_site_ids.contains(&id) {
        return Err(EirValidationError::UnknownAccessSiteId(id));
    }
    Ok(())
}

fn require_safepoint(
    id: SafepointId,
    ctx: &EirValidationContext,
) -> Result<(), EirValidationError> {
    if !id.is_valid() || !ctx.safepoint_ids.contains(&id) {
        return Err(EirValidationError::UnknownSafepointId(id));
    }
    Ok(())
}

fn require_deopt(id: DeoptId, ctx: &EirValidationContext) -> Result<(), EirValidationError> {
    if !id.is_valid() || !ctx.deopt_ids.contains(&id) {
        return Err(EirValidationError::UnknownDeoptId(id));
    }
    Ok(())
}

fn has_source_mapping(metadata: &crate::eir::schema::OpMetadata) -> bool {
    metadata.source_span.is_some() || metadata.debug_origin.is_some()
}

fn op_requires_source_mapping(kind: &EirOpKind, ctx: &EirValidationContext) -> bool {
    match kind {
        EirOpKind::RuntimeHelper(op) => ctx.helper_registry.may_raise(op.helper_id),
        EirOpKind::Check(_)
        | EirOpKind::Call(_)
        | EirOpKind::Guard(_)
        | EirOpKind::Construct(ConstructOp::Error(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eir::fixtures::{
        barrier_required_validation_context, case_bound_validation_context,
        eir_module_wire_with_missing_terminator, eir_module_wire_with_unknown_op_kind,
        eir_module_wire_with_unknown_terminator_kind, eir_module_with_digest_mismatch,
        eir_module_with_guard_without_failure_action, eir_module_with_heap_store_without_barrier,
        eir_module_with_invalid_block_argument_count, eir_module_with_invalid_block_graph,
        eir_module_with_invalid_entry_block, eir_module_with_invalid_helper,
        eir_module_with_may_collect_without_root_map, eir_module_with_unknown_access_site_id,
        eir_module_with_unknown_call_site_id, eir_module_with_unknown_case_id,
        eir_module_with_unknown_constant, eir_module_with_unknown_deopt_id,
        eir_module_with_unknown_field_id, eir_module_with_unknown_shape_id,
        eir_module_with_unknown_slot, eir_module_with_unknown_type_id,
        eir_module_with_unmapped_raise_op, field_bound_validation_context,
        may_collect_validation_context, minimal_eir_validation_context, minimal_valid_eir_module,
        shape_bound_validation_context, type_bound_validation_context,
    };

    #[test]
    fn minimal_valid_eir_module_passes_validation() {
        let module = minimal_valid_eir_module();
        let ctx = minimal_eir_validation_context();
        assert!(validate_eir_module(EirModuleInput::Resolved(&module), &ctx).is_ok());
    }

    #[test]
    fn unknown_op_kind_is_rejected() {
        let wire = eir_module_wire_with_unknown_op_kind();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Wire(&wire), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownOpKind(255)));
    }

    #[test]
    fn unknown_terminator_kind_is_rejected() {
        let wire = eir_module_wire_with_unknown_terminator_kind();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Wire(&wire), &ctx).unwrap_err();
        assert!(matches!(
            err,
            EirValidationError::UnknownTerminatorKind(200)
        ));
    }

    #[test]
    fn invalid_block_graph_is_rejected() {
        let module = eir_module_with_invalid_block_graph();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::InvalidBlockGraph(_)));
    }

    #[test]
    fn block_without_terminator_is_rejected() {
        let wire = eir_module_wire_with_missing_terminator();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Wire(&wire), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::BlockWithoutTerminator(_)));
    }

    #[test]
    fn invalid_helper_reference_is_rejected() {
        let module = eir_module_with_invalid_helper();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(
            err,
            EirValidationError::UnknownRuntimeHelperId(_)
        ));
    }

    #[test]
    fn may_raise_op_without_source_mapping_is_rejected() {
        let module = eir_module_with_unmapped_raise_op();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert_eq!(err, EirValidationError::MayRaiseWithoutSourceMapping);
    }

    #[test]
    fn may_collect_helper_without_root_map_is_rejected() {
        let module = eir_module_with_may_collect_without_root_map();
        let ctx = may_collect_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert_eq!(err, EirValidationError::MayCollectWithoutRootMap);
    }

    #[test]
    fn unknown_constant_id_is_rejected() {
        let module = eir_module_with_unknown_constant();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownConstantId(_)));
    }

    #[test]
    fn invalid_block_argument_count_is_rejected() {
        let module = eir_module_with_invalid_block_argument_count();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(
            err,
            EirValidationError::InvalidBlockArgumentCount(_)
        ));
    }

    #[test]
    fn heap_store_without_barrier_policy_is_rejected() {
        let module = eir_module_with_heap_store_without_barrier();
        let ctx = barrier_required_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert_eq!(err, EirValidationError::HeapWriteWithoutBarrierPolicy);
    }

    #[test]
    fn unknown_type_id_is_rejected() {
        let module = eir_module_with_unknown_type_id();
        let ctx = type_bound_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownTypeId(_)));
    }

    #[test]
    fn unknown_shape_id_is_rejected() {
        let module = eir_module_with_unknown_shape_id();
        let ctx = shape_bound_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownShapeId(_)));
    }

    #[test]
    fn unknown_field_id_is_rejected() {
        let module = eir_module_with_unknown_field_id();
        let ctx = field_bound_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownFieldId(_)));
    }

    #[test]
    fn unknown_case_id_is_rejected() {
        let module = eir_module_with_unknown_case_id();
        let ctx = case_bound_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownCaseId(_)));
    }

    #[test]
    fn unknown_call_site_id_is_rejected() {
        let module = eir_module_with_unknown_call_site_id();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownCallSiteId(_)));
    }

    #[test]
    fn unknown_access_site_id_is_rejected() {
        let module = eir_module_with_unknown_access_site_id();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownAccessSiteId(_)));
    }

    #[test]
    fn unknown_deopt_id_is_rejected() {
        let module = eir_module_with_unknown_deopt_id();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownDeoptId(_)));
    }

    #[test]
    fn runtime_plan_digest_mismatch_is_rejected() {
        let module = eir_module_with_digest_mismatch();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert_eq!(err, EirValidationError::RuntimePlanDigestMismatch);
    }

    #[test]
    fn unknown_slot_id_is_rejected() {
        let module = eir_module_with_unknown_slot();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownSlotId(_)));
    }

    #[test]
    fn guard_without_failure_action_is_rejected() {
        let module = eir_module_with_guard_without_failure_action();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert_eq!(err, EirValidationError::GuardWithoutFailureAction);
    }

    #[test]
    fn invalid_entry_block_is_rejected() {
        let module = eir_module_with_invalid_entry_block();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::InvalidEntryBlock(_)));
    }
}