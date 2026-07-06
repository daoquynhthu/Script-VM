//! RuntimePlan schema types.
//!
//! Spec: `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md`

use std::collections::BTreeMap;

use vm_diag::source_span::SourceSpanId;

use crate::digest::Digest;
use crate::id::{
    AccessSiteId, BindingId, CallSiteId, CapabilityId, CaseId, CaseIndex, DeoptId, EffectId,
    EnumId, EirFunctionId, FieldId, FieldIndex, FrameMapId, FunctionId, InterfaceId, ModuleId,
    NodeId, RecordId, RuntimeHelperId, SafepointId, ShapeId, SlotId, SlotLayoutId, TypeId,
};
use crate::profile::{RuntimeTargetProfile, Version};

/// Half-open ID range `[start, start + len)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdRange<T> {
    pub start: T,
    pub len: u32,
}

impl<T: Copy> IdRange<T> {
    #[must_use]
    pub const fn empty(at: T) -> Self {
        Self { start: at, len: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportKind {
    WholeModule,
    NamedValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotKind {
    Local,
    Parameter,
    Capture,
    Module,
    Temporary,
    Builtin,
    HiddenRuntime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotStorage {
    Value,
    Cell,
    Constant,
    TypeDescriptor,
    ModuleRef,
    RuntimeInternal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mutability {
    Immutable,
    Mutable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotLayoutOwner {
    Module,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessKind {
    AttributeRead,
    AttributeWrite,
    MethodRead,
    IndexRead,
    IndexWrite,
    Slice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SafepointKind {
    FunctionCall,
    LoopBackedge,
    Allocation,
    HostCall,
    HelperCall,
    RaiseBoundary,
    ImportBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeoptReason {
    TypeFeedback,
    ArityMismatch,
    GuardFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EirLocation(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportPlanEntry {
    pub node_id: NodeId,
    pub imported_module: ModuleId,
    pub import_kind: ImportKind,
    pub local_binding_slot: SlotId,
    pub required_interface: Option<InterfaceId>,
    pub source_span: SourceSpanId,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ImportPlan {
    pub imports: Vec<ImportPlanEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPlanEntry {
    pub exported_name: String,
    pub binding_id: BindingId,
    pub slot_id: SlotId,
    pub interface_type: Option<TypeId>,
    pub source_span: SourceSpanId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPlan {
    pub exports: Vec<ExportPlanEntry>,
    pub seal_after_init: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePlan {
    pub module_id: ModuleId,
    pub module_slot_layout: SlotLayoutId,
    pub initialization_function: EirFunctionId,
    pub import_plan: ImportPlan,
    pub export_plan: ExportPlan,
    pub module_state_slot: SlotId,
    pub module_object_slot: SlotId,
    pub source_order: Vec<NodeId>,
    pub source_span: Option<SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModulePlanTable {
    pub modules: BTreeMap<u32, ModulePlan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParameterLayout {
    pub parameter_slots: Vec<SlotId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CaptureLayout {
    pub capture_slots: Vec<SlotId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DefaultArgumentPlan {
    pub default_slots: Vec<SlotId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionPlan {
    pub function_id: FunctionId,
    pub module_id: ModuleId,
    pub entry_eir_function: EirFunctionId,
    pub parameter_layout: ParameterLayout,
    pub local_slot_layout: SlotLayoutId,
    pub capture_layout: CaptureLayout,
    pub default_argument_plan: DefaultArgumentPlan,
    pub return_type: Option<TypeId>,
    pub effect: Option<EffectId>,
    pub required_capabilities: Vec<CapabilityId>,
    pub call_site_range: IdRange<CallSiteId>,
    pub access_site_range: IdRange<AccessSiteId>,
    pub safepoint_range: IdRange<SafepointId>,
    pub deopt_range: IdRange<DeoptId>,
    pub source_span: Option<SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FunctionPlanTable {
    pub functions: BTreeMap<u32, FunctionPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotDescriptor {
    pub slot_id: SlotId,
    pub kind: SlotKind,
    pub binding_id: Option<BindingId>,
    pub type_hint: Option<TypeId>,
    pub mutability: Option<Mutability>,
    pub storage: SlotStorage,
    pub initialized_at_entry: bool,
    pub source_span: Option<SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiddenSlotDescriptor {
    pub slot_id: SlotId,
    pub storage: SlotStorage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotLayout {
    pub slot_layout_id: SlotLayoutId,
    pub owner: SlotLayoutOwner,
    pub slots: Vec<SlotDescriptor>,
    pub hidden_slots: Vec<HiddenSlotDescriptor>,
    pub max_slot_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SlotLayoutTable {
    pub layouts: BTreeMap<u32, SlotLayout>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTypeDesc {
    pub type_id: TypeId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeCheckStrategy {
    Exact,
    Structural,
    Erased,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypePlan {
    pub runtime_types: BTreeMap<u32, RuntimeTypeDesc>,
    pub type_check_strategies: BTreeMap<u32, TypeCheckStrategy>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFieldShape {
    pub field_id: FieldId,
    pub field_index: FieldIndex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodShape {
    pub name: String,
    pub function_id: FunctionId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordShape {
    pub record_id: RecordId,
    pub shape_id: ShapeId,
    pub fields: Vec<RecordFieldShape>,
    pub methods: Vec<MethodShape>,
    pub field_index_by_id: BTreeMap<u32, FieldIndex>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCaseShape {
    pub case_id: CaseId,
    pub case_index: CaseIndex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumShape {
    pub enum_id: EnumId,
    pub shape_id: ShapeId,
    pub cases: Vec<EnumCaseShape>,
    pub case_index_by_id: BTreeMap<u32, CaseIndex>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ShapePlan {
    pub record_shapes: BTreeMap<u32, RecordShape>,
    pub enum_shapes: BTreeMap<u32, EnumShape>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallSiteRecord {
    pub call_site_id: CallSiteId,
    pub owner_function: EirFunctionId,
    pub node_id: NodeId,
    pub arity_shape: u32,
    pub required_capabilities: Vec<CapabilityId>,
    pub source_span: SourceSpanId,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CallSiteTable {
    pub call_sites: BTreeMap<u32, CallSiteRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessSiteRecord {
    pub access_site_id: AccessSiteId,
    pub owner_function: EirFunctionId,
    pub node_id: NodeId,
    pub access_kind: AccessKind,
    pub receiver_type_hint: Option<TypeId>,
    pub shape_hint: Option<ShapeId>,
    pub field_id: Option<FieldId>,
    pub field_index: Option<FieldIndex>,
    pub source_span: SourceSpanId,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AccessSiteTable {
    pub access_sites: BTreeMap<u32, AccessSiteRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafepointSeed {
    pub safepoint_id: SafepointId,
    pub owner_function: EirFunctionId,
    pub kind: SafepointKind,
    pub source_span: Option<SourceSpanId>,
    pub requires_root_map: bool,
    pub may_trigger_gc: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SafepointSeedTable {
    pub safepoints: BTreeMap<u32, SafepointSeed>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeoptSeed {
    pub deopt_id: DeoptId,
    pub owner_function: EirFunctionId,
    pub source_eir_location: Option<EirLocation>,
    pub frame_map_id: FrameMapId,
    pub reason: DeoptReason,
    pub source_span: Option<SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeoptSeedTable {
    pub deopts: BTreeMap<u32, DeoptSeed>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HelperRequirementReason {
    EirReference,
    RuntimePlanReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHelperRequirement {
    pub helper_id: RuntimeHelperId,
    pub reason: HelperRequirementReason,
    pub required_by: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeHelperRequirementTable {
    pub required_helpers: BTreeMap<u32, RuntimeHelperRequirement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilityCheckLocation {
    ModuleInit,
    FunctionEntry,
    CallSite,
    HostBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilityEnvironmentMutability {
    Immutable,
    EpochTracked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityGate {
    pub capability_id: CapabilityId,
    pub effect: Option<EffectId>,
    pub checked_at: CapabilityCheckLocation,
    pub source_span: SourceSpanId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityGatePlan {
    pub gates: Vec<CapabilityGate>,
    pub environment_digest: Option<Digest>,
    pub mutability_policy: CapabilityEnvironmentMutability,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimePlanSourceMap {
    pub entries: Vec<(NodeId, SourceSpanId)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticTable {
    pub messages: Vec<String>,
}

/// Top-level execution plan container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePlan {
    pub plan_version: Version,
    pub source_sir_digest: Digest,
    pub phase2_schema_version: Version,
    pub vm_version: Version,
    pub target_profile: RuntimeTargetProfile,
    pub module_plans: ModulePlanTable,
    pub function_plans: FunctionPlanTable,
    pub type_plan: TypePlan,
    pub shape_plan: ShapePlan,
    pub slot_layouts: SlotLayoutTable,
    pub call_site_table: CallSiteTable,
    pub access_site_table: AccessSiteTable,
    pub safepoint_seed_table: SafepointSeedTable,
    pub deopt_seed_table: DeoptSeedTable,
    pub helper_requirements: RuntimeHelperRequirementTable,
    pub capability_gate_plan: CapabilityGatePlan,
    pub source_map: RuntimePlanSourceMap,
    pub diagnostics: Option<DiagnosticTable>,
}