//! RuntimePlan validation.
//!
//! Spec: `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md` §15, `PHASE-3-VALIDATION-MATRIX.md`

use std::collections::BTreeSet;

use crate::id::{
    AccessSiteId, CallSiteId, DeoptId, EirFunctionId, FunctionId, ModuleId, RuntimeHelperId,
    SafepointId, SlotId, SlotLayoutId, TypeId,
};
use crate::runtime_plan::schema::{
    EnumShape, FunctionPlan, ModulePlan, RecordShape, RuntimePlan, ShapePlan, SlotLayoutTable,
    TypePlan,
};

/// Validation failure codes aligned with frozen rejection requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    UnresolvedModuleId(ModuleId),
    UnresolvedFunctionId(FunctionId),
    UnresolvedSlotId(SlotId),
    UnresolvedSlotLayoutId(SlotLayoutId),
    UnresolvedTypeId(TypeId),
    UnresolvedShapeId(u32),
    UnresolvedCallSiteId(CallSiteId),
    UnresolvedAccessSiteId(AccessSiteId),
    UnresolvedSafepointId(SafepointId),
    UnresolvedDeoptId(DeoptId),
    UnresolvedRuntimeHelperId(RuntimeHelperId),
    FunctionPlanWithoutEntryEir(FunctionId),
    ModulePlanWithoutInitFunction(ModuleId),
    RecordShapeMissingFieldIndex(u32),
    EnumShapeMissingCaseIndex(u32),
    CapabilityGateWithoutCapability,
    CacheProfileMismatch,
    SourceSirDigestMismatch,
    SlotLayoutMismatch(SlotLayoutId),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ValidationError {}

/// Validate a RuntimePlan against frozen schema requirements.
pub fn validate_runtime_plan(plan: &RuntimePlan) -> Result<(), ValidationError> {
    if plan.vm_version != plan.target_profile.vm_version {
        return Err(ValidationError::CacheProfileMismatch);
    }

    if plan.source_sir_digest.0 == 0 {
        return Err(ValidationError::SourceSirDigestMismatch);
    }

    let module_ids = collect_module_ids(plan);
    let function_ids = collect_function_ids(plan);
    let slot_ids = collect_slot_ids(&plan.slot_layouts);
    let layout_ids = collect_layout_ids(&plan.slot_layouts);
    let type_ids = collect_type_ids(&plan.type_plan);
    let shape_ids = collect_shape_ids(&plan.shape_plan);
    let call_site_ids = collect_call_site_ids(plan);
    let access_site_ids = collect_access_site_ids(plan);
    let safepoint_ids = collect_safepoint_ids(plan);
    let deopt_ids = collect_deopt_ids(plan);
    let eir_functions = collect_eir_functions(plan);

    for module in plan.module_plans.modules.values() {
        validate_module_plan(module, &layout_ids, &slot_ids, &eir_functions)?;
    }

    for function in plan.function_plans.functions.values() {
        validate_function_plan(
            function,
            &module_ids,
            &layout_ids,
            &slot_ids,
            &call_site_ids,
            &access_site_ids,
            &safepoint_ids,
            &deopt_ids,
            &eir_functions,
        )?;
    }

    for record in plan.shape_plan.record_shapes.values() {
        validate_record_shape(record)?;
    }
    for enum_shape in plan.shape_plan.enum_shapes.values() {
        validate_enum_shape(enum_shape)?;
    }

    for gate in &plan.capability_gate_plan.gates {
        if !gate.capability_id.is_valid() {
            return Err(ValidationError::CapabilityGateWithoutCapability);
        }
        let _ = gate.capability_id;
    }

    for requirement in plan.helper_requirements.required_helpers.values() {
        if !requirement.helper_id.is_valid() {
            return Err(ValidationError::UnresolvedRuntimeHelperId(
                requirement.helper_id,
            ));
        }
    }

    for type_id in referenced_type_ids(plan) {
        if !type_ids.contains(&type_id) {
            return Err(ValidationError::UnresolvedTypeId(type_id));
        }
    }

    for shape_id in referenced_shape_ids(plan) {
        if !shape_ids.contains(&shape_id) {
            return Err(ValidationError::UnresolvedShapeId(shape_id));
        }
    }

    let _ = function_ids;
    Ok(())
}

fn validate_module_plan(
    module: &ModulePlan,
    layout_ids: &BTreeSet<SlotLayoutId>,
    slot_ids: &BTreeSet<SlotId>,
    eir_functions: &BTreeSet<EirFunctionId>,
) -> Result<(), ValidationError> {
    if !module.module_id.is_valid() {
        return Err(ValidationError::UnresolvedModuleId(module.module_id));
    }
    if !layout_ids.contains(&module.module_slot_layout) {
        return Err(ValidationError::UnresolvedSlotLayoutId(
            module.module_slot_layout,
        ));
    }
    if !module.initialization_function.is_valid()
        || !eir_functions.contains(&module.initialization_function)
    {
        return Err(ValidationError::ModulePlanWithoutInitFunction(
            module.module_id,
        ));
    }
    for slot in [
        module.module_state_slot,
        module.module_object_slot,
    ] {
        if !slot_ids.contains(&slot) {
            return Err(ValidationError::UnresolvedSlotId(slot));
        }
    }
    for import in &module.import_plan.imports {
        if !slot_ids.contains(&import.local_binding_slot) {
            return Err(ValidationError::UnresolvedSlotId(import.local_binding_slot));
        }
    }
    for export in &module.export_plan.exports {
        if !slot_ids.contains(&export.slot_id) {
            return Err(ValidationError::UnresolvedSlotId(export.slot_id));
        }
    }
    Ok(())
}

fn validate_function_plan(
    function: &FunctionPlan,
    module_ids: &BTreeSet<ModuleId>,
    layout_ids: &BTreeSet<SlotLayoutId>,
    slot_ids: &BTreeSet<SlotId>,
    call_site_ids: &BTreeSet<CallSiteId>,
    access_site_ids: &BTreeSet<AccessSiteId>,
    safepoint_ids: &BTreeSet<SafepointId>,
    deopt_ids: &BTreeSet<DeoptId>,
    eir_functions: &BTreeSet<EirFunctionId>,
) -> Result<(), ValidationError> {
    if !function.function_id.is_valid() {
        return Err(ValidationError::UnresolvedFunctionId(function.function_id));
    }
    if !module_ids.contains(&function.module_id) {
        return Err(ValidationError::UnresolvedModuleId(function.module_id));
    }
    if !function.entry_eir_function.is_valid()
        || !eir_functions.contains(&function.entry_eir_function)
    {
        return Err(ValidationError::FunctionPlanWithoutEntryEir(
            function.function_id,
        ));
    }
    if !layout_ids.contains(&function.local_slot_layout) {
        return Err(ValidationError::SlotLayoutMismatch(function.local_slot_layout));
    }
    for slot in function
        .parameter_layout
        .parameter_slots
        .iter()
        .chain(function.capture_layout.capture_slots.iter())
        .chain(function.default_argument_plan.default_slots.iter())
    {
        if !slot_ids.contains(slot) {
            return Err(ValidationError::UnresolvedSlotId(*slot));
        }
    }
    validate_call_site_range(
        function.call_site_range.start,
        function.call_site_range.len,
        call_site_ids,
    )?;
    validate_access_site_range(
        function.access_site_range.start,
        function.access_site_range.len,
        access_site_ids,
    )?;
    validate_safepoint_range(
        function.safepoint_range.start,
        function.safepoint_range.len,
        safepoint_ids,
    )?;
    validate_deopt_range(
        function.deopt_range.start,
        function.deopt_range.len,
        deopt_ids,
    )?;
    let _ = eir_functions;
    Ok(())
}

fn validate_call_site_range(
    start: CallSiteId,
    len: u32,
    known: &BTreeSet<CallSiteId>,
) -> Result<(), ValidationError> {
    for offset in 0..len {
        let id = CallSiteId::new(start.raw() + offset);
        if !known.contains(&id) {
            return Err(ValidationError::UnresolvedCallSiteId(id));
        }
    }
    Ok(())
}

fn validate_access_site_range(
    start: AccessSiteId,
    len: u32,
    known: &BTreeSet<AccessSiteId>,
) -> Result<(), ValidationError> {
    for offset in 0..len {
        let id = AccessSiteId::new(start.raw() + offset);
        if !known.contains(&id) {
            return Err(ValidationError::UnresolvedAccessSiteId(id));
        }
    }
    Ok(())
}

fn validate_safepoint_range(
    start: SafepointId,
    len: u32,
    known: &BTreeSet<SafepointId>,
) -> Result<(), ValidationError> {
    for offset in 0..len {
        let id = SafepointId::new(start.raw() + offset);
        if !known.contains(&id) {
            return Err(ValidationError::UnresolvedSafepointId(id));
        }
    }
    Ok(())
}

fn validate_deopt_range(
    start: DeoptId,
    len: u32,
    known: &BTreeSet<DeoptId>,
) -> Result<(), ValidationError> {
    for offset in 0..len {
        let id = DeoptId::new(start.raw() + offset);
        if !known.contains(&id) {
            return Err(ValidationError::UnresolvedDeoptId(id));
        }
    }
    Ok(())
}

fn validate_record_shape(record: &RecordShape) -> Result<(), ValidationError> {
    for field in &record.fields {
        if !record
            .field_index_by_id
            .contains_key(&field.field_id.raw())
        {
            return Err(ValidationError::RecordShapeMissingFieldIndex(
                field.field_id.raw(),
            ));
        }
    }
    Ok(())
}

fn validate_enum_shape(enum_shape: &EnumShape) -> Result<(), ValidationError> {
    for case in &enum_shape.cases {
        if !enum_shape
            .case_index_by_id
            .contains_key(&case.case_id.raw())
        {
            return Err(ValidationError::EnumShapeMissingCaseIndex(case.case_id.raw()));
        }
    }
    Ok(())
}

fn collect_module_ids(plan: &RuntimePlan) -> BTreeSet<ModuleId> {
    plan.module_plans
        .modules
        .values()
        .map(|m| m.module_id)
        .collect()
}

fn collect_function_ids(plan: &RuntimePlan) -> BTreeSet<FunctionId> {
    plan.function_plans
        .functions
        .values()
        .map(|f| f.function_id)
        .collect()
}

fn collect_slot_ids(layouts: &SlotLayoutTable) -> BTreeSet<SlotId> {
    let mut ids = BTreeSet::new();
    for layout in layouts.layouts.values() {
        for slot in &layout.slots {
            ids.insert(slot.slot_id);
        }
        for hidden in &layout.hidden_slots {
            ids.insert(hidden.slot_id);
        }
    }
    ids
}

fn collect_layout_ids(layouts: &SlotLayoutTable) -> BTreeSet<SlotLayoutId> {
    layouts
        .layouts
        .values()
        .map(|layout| layout.slot_layout_id)
        .collect()
}

fn collect_type_ids(type_plan: &TypePlan) -> BTreeSet<TypeId> {
    type_plan
        .runtime_types
        .values()
        .map(|desc| desc.type_id)
        .collect()
}

fn collect_shape_ids(shape_plan: &ShapePlan) -> BTreeSet<u32> {
    let mut ids = BTreeSet::new();
    for shape in shape_plan.record_shapes.values() {
        ids.insert(shape.shape_id.raw());
    }
    for shape in shape_plan.enum_shapes.values() {
        ids.insert(shape.shape_id.raw());
    }
    ids
}

fn collect_call_site_ids(plan: &RuntimePlan) -> BTreeSet<CallSiteId> {
    plan.call_site_table
        .call_sites
        .values()
        .map(|r| r.call_site_id)
        .collect()
}

fn collect_access_site_ids(plan: &RuntimePlan) -> BTreeSet<AccessSiteId> {
    plan.access_site_table
        .access_sites
        .values()
        .map(|r| r.access_site_id)
        .collect()
}

fn collect_safepoint_ids(plan: &RuntimePlan) -> BTreeSet<SafepointId> {
    plan.safepoint_seed_table
        .safepoints
        .values()
        .map(|r| r.safepoint_id)
        .collect()
}

fn collect_deopt_ids(plan: &RuntimePlan) -> BTreeSet<DeoptId> {
    plan.deopt_seed_table
        .deopts
        .values()
        .map(|r| r.deopt_id)
        .collect()
}

fn collect_eir_functions(plan: &RuntimePlan) -> BTreeSet<EirFunctionId> {
    let mut ids = BTreeSet::new();
    for module in plan.module_plans.modules.values() {
        ids.insert(module.initialization_function);
    }
    for function in plan.function_plans.functions.values() {
        ids.insert(function.entry_eir_function);
    }
    for record in plan.call_site_table.call_sites.values() {
        ids.insert(record.owner_function);
    }
    for record in plan.access_site_table.access_sites.values() {
        ids.insert(record.owner_function);
    }
    for seed in plan.safepoint_seed_table.safepoints.values() {
        ids.insert(seed.owner_function);
    }
    for seed in plan.deopt_seed_table.deopts.values() {
        ids.insert(seed.owner_function);
    }
    ids
}

fn referenced_type_ids(plan: &RuntimePlan) -> BTreeSet<TypeId> {
    let mut ids = BTreeSet::new();
    for function in plan.function_plans.functions.values() {
        if let Some(return_type) = function.return_type {
            ids.insert(return_type);
        }
    }
    for record in plan.access_site_table.access_sites.values() {
        if let Some(receiver_type) = record.receiver_type_hint {
            ids.insert(receiver_type);
        }
    }
    ids
}

fn referenced_shape_ids(plan: &RuntimePlan) -> BTreeSet<u32> {
    let mut ids = BTreeSet::new();
    for record in plan.access_site_table.access_sites.values() {
        if let Some(shape_hint) = record.shape_hint {
            ids.insert(shape_hint.raw());
        }
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_plan::fixtures::minimal_valid_plan;

    #[test]
    fn minimal_plan_passes_validation() {
        let plan = minimal_valid_plan();
        assert!(validate_runtime_plan(&plan).is_ok());
    }

    #[test]
    fn unresolved_module_reference_is_rejected() {
        let mut plan = minimal_valid_plan();
        if let Some(function) = plan.function_plans.functions.get_mut(&0) {
            function.module_id = ModuleId::new(99);
        }
        let err = validate_runtime_plan(&plan).unwrap_err();
        assert!(matches!(err, ValidationError::UnresolvedModuleId(_)));
    }

    #[test]
    fn function_without_entry_eir_is_rejected() {
        let mut plan = minimal_valid_plan();
        if let Some(function) = plan.function_plans.functions.get_mut(&0) {
            function.entry_eir_function = EirFunctionId::INVALID;
        }
        let err = validate_runtime_plan(&plan).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::FunctionPlanWithoutEntryEir(_)
        ));
    }

    #[test]
    fn record_shape_missing_field_index_is_rejected() {
        let mut plan = minimal_valid_plan();
        if let Some(record) = plan.shape_plan.record_shapes.get_mut(&0) {
            record.field_index_by_id.clear();
        }
        let err = validate_runtime_plan(&plan).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::RecordShapeMissingFieldIndex(_)
        ));
    }

    #[test]
    fn cache_profile_mismatch_is_rejected() {
        let mut plan = minimal_valid_plan();
        plan.target_profile.vm_version = crate::profile::Version::new(9, 9, 9);
        let err = validate_runtime_plan(&plan).unwrap_err();
        assert_eq!(err, ValidationError::CacheProfileMismatch);
    }
}