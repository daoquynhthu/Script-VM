# Phase 3 · RuntimePlan Schema Closure

Document class: Normative specification  
Normative status: This document defines the canonical closed minimal RuntimePlan schema for Phase 3 VM specifications.

Created: 2026-06-29 09:21:35

---

## Normative Interpretation

This document is interpreted under `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md`.

Unmarked planning-style words such as `bootstrap`, `recommended`, `staged`, `first implementation`, `later`, `plan`, or `milestone` do not create implementation-plan status inside this normative document. They are interpreted as one of:

```text
MUST / MUST NOT / SHOULD / MAY
BOOTSTRAP allowance
RECOMMENDED implementation option
DEFERRED design area
NON-NORMATIVE NOTE
```

according to their local context and the normative keyword policy.

If this document conflicts with a later canonical repair document, the later canonical repair document owns the repaired term or schema.



## 0. Purpose

This document repairs audit item:

```text
R4: Close RuntimePlan schema.
```

It resolves blocker:

```text
B-02: RuntimePlan schema is still framework-level.
```

RuntimePlan is the internal semantic-to-execution bridge from frozen SIR to EIR.

---

## 1. RuntimePlan Boundary

RuntimePlan is:

```text
VM-internal
SIR-digest-bound
target-profile-bound
discardable
cacheable only under compatibility rules
not public bytecode
not package ABI
not native ABI
```

---

## 2. RuntimePlan Schema

```text
RuntimePlan {
  plan_version: Version
  source_sir_digest: Digest
  phase2_schema_version: Version
  vm_version: Version
  target_profile: RuntimeTargetProfile
  module_plans: ModulePlanTable
  function_plans: FunctionPlanTable
  type_plan: TypePlan
  shape_plan: ShapePlan
  slot_layouts: SlotLayoutTable
  call_site_table: CallSiteTable
  access_site_table: AccessSiteTable
  safepoint_seed_table: SafepointSeedTable
  deopt_seed_table: DeoptSeedTable
  helper_requirements: RuntimeHelperRequirementTable
  capability_gate_plan: CapabilityGatePlan
  source_map: RuntimePlanSourceMap
  diagnostics?: DiagnosticTable
}
```

Every field except diagnostics is REQUIRED.

---

## 3. RuntimeTargetProfile

```text
RuntimeTargetProfile {
  vm_version: Version
  architecture: TargetArchitecture
  pointer_width: UInt
  value_layout_profile: ValueLayoutProfileRef
  heap_profile: HeapProfileRef
  gc_profile: GcProfileRef
  interpreter_profile: InterpreterProfileRef
  jit_profile?: JitProfileRef
  capability_environment_digest?: Digest
}
```

The full profile participates in RuntimePlan cache identity.

---

## 4. ModulePlanTable

```text
ModulePlanTable {
  modules: Map<ModuleId, ModulePlan>
}
```

### 4.1 ModulePlan

```text
ModulePlan {
  module_id: ModuleId
  module_slot_layout: SlotLayoutId
  initialization_function: EirFunctionId
  import_plan: ImportPlan
  export_plan: ExportPlan
  module_state_slot: SlotId
  module_object_slot: SlotId
  source_order: List<NodeId>
  source_span?: SourceSpanId
}
```

### 4.2 ImportPlan

```text
ImportPlan {
  imports: List<ImportPlanEntry>
}
```

```text
ImportPlanEntry {
  node_id: NodeId
  imported_module: ModuleId
  import_kind: ImportKind
  local_binding_slot: SlotId
  required_interface?: InterfaceId
  source_span: SourceSpanId
}
```

### 4.3 ExportPlan

```text
ExportPlan {
  exports: List<ExportPlanEntry>
  seal_after_init: Bool
}
```

```text
ExportPlanEntry {
  exported_name: String
  binding_id: BindingId
  slot_id: SlotId
  interface_type?: TypeId
  source_span: SourceSpanId
}
```

---

## 5. FunctionPlanTable

```text
FunctionPlanTable {
  functions: Map<FunctionId, FunctionPlan>
}
```

### 5.1 FunctionPlan

```text
FunctionPlan {
  function_id: FunctionId
  module_id: ModuleId
  entry_eir_function: EirFunctionId
  parameter_layout: ParameterLayout
  local_slot_layout: SlotLayoutId
  capture_layout: CaptureLayout
  default_argument_plan: DefaultArgumentPlan
  return_type?: TypeId
  effect?: EffectId
  required_capabilities: List<CapabilityId>
  call_site_range: IdRange<CallSiteId>
  access_site_range: IdRange<AccessSiteId>
  safepoint_range: IdRange<SafepointId>
  deopt_range: IdRange<DeoptId>
  source_span?: SourceSpanId
}
```

---

## 6. SlotLayoutTable

```text
SlotLayoutTable {
  layouts: Map<SlotLayoutId, SlotLayout>
}
```

### 6.1 SlotLayout

```text
SlotLayout {
  slot_layout_id: SlotLayoutId
  owner: SlotLayoutOwner
  slots: List<SlotDescriptor>
  hidden_slots: List<HiddenSlotDescriptor>
  max_slot_count: UInt
}
```

### 6.2 SlotDescriptor

```text
SlotDescriptor {
  slot_id: SlotId
  kind: SlotKind
  binding_id?: BindingId
  type_hint?: TypeId
  mutability?: Mutability
  storage: SlotStorage
  initialized_at_entry: Bool
  source_span?: SourceSpanId
}
```

```text
SlotKind =
  | Local
  | Parameter
  | Capture
  | Module
  | Temporary
  | Builtin
  | HiddenRuntime
```

```text
SlotStorage =
  | Value
  | Cell
  | Constant
  | TypeDescriptor
  | ModuleRef
  | RuntimeInternal
```

---

## 7. TypePlan

```text
TypePlan {
  runtime_types: Map<TypeId, RuntimeTypeDesc>
  type_check_strategies: Map<TypeId, TypeCheckStrategy>
}
```

Every TypeId used by EIR or RuntimePlan MUST resolve here or in the frozen SIR TypeTable by explicit reference.

---

## 8. ShapePlan

```text
ShapePlan {
  record_shapes: Map<RecordId, RecordShape>
  enum_shapes: Map<EnumId, EnumShape>
}
```

### 8.1 RecordShape

```text
RecordShape {
  record_id: RecordId
  shape_id: ShapeId
  fields: List<RecordFieldShape>
  methods: List<MethodShape>
  field_index_by_id: Map<FieldId, FieldIndex>
}
```

### 8.2 EnumShape

```text
EnumShape {
  enum_id: EnumId
  shape_id: ShapeId
  cases: List<EnumCaseShape>
  case_index_by_id: Map<CaseId, CaseIndex>
}
```

Normal execution MUST use FieldId/CaseId-derived indices, not string lookup.

---

## 9. CallSiteTable

```text
CallSiteTable {
  call_sites: Map<CallSiteId, CallSiteRecord>
}
```

```text
CallSiteRecord {
  call_site_id: CallSiteId
  owner_function: EirFunctionId
  node_id: NodeId
  call_kind_hint?: CallKind
  arity_shape: ArityShape
  effect?: EffectId
  required_capabilities: List<CapabilityId>
  source_span: SourceSpanId
}
```

---

## 10. AccessSiteTable

```text
AccessSiteTable {
  access_sites: Map<AccessSiteId, AccessSiteRecord>
}
```

```text
AccessSiteRecord {
  access_site_id: AccessSiteId
  owner_function: EirFunctionId
  node_id: NodeId
  access_kind: AccessKind
  receiver_type_hint?: TypeId
  shape_hint?: ShapeId
  field_id?: FieldId
  field_index?: FieldIndex
  source_span: SourceSpanId
}
```

---

## 11. SafepointSeedTable

```text
SafepointSeedTable {
  safepoints: Map<SafepointId, SafepointSeed>
}
```

```text
SafepointSeed {
  safepoint_id: SafepointId
  owner_function: EirFunctionId
  kind: SafepointKind
  source_span?: SourceSpanId
  requires_root_map: Bool
  may_trigger_gc: Bool
}
```

---

## 12. DeoptSeedTable

```text
DeoptSeedTable {
  deopts: Map<DeoptId, DeoptSeed>
}
```

```text
DeoptSeed {
  deopt_id: DeoptId
  owner_function: EirFunctionId
  source_eir_location?: EirLocation
  frame_map_id: FrameMapId
  reason: DeoptReason
  source_span?: SourceSpanId
}
```

---

## 13. RuntimeHelperRequirementTable

```text
RuntimeHelperRequirementTable {
  required_helpers: Map<RuntimeHelperId, RuntimeHelperRequirement>
}
```

```text
RuntimeHelperRequirement {
  helper_id: RuntimeHelperId
  reason: HelperRequirementReason
  required_by: List<EirLocationOrPlanRef>
}
```

All helpers referenced by RuntimePlan/EIR MUST exist in the canonical helper registry.

---

## 14. CapabilityGatePlan

```text
CapabilityGatePlan {
  gates: List<CapabilityGate>
  environment_digest?: Digest
  mutability_policy: CapabilityEnvironmentMutability
}
```

```text
CapabilityGate {
  capability_id: CapabilityId
  effect?: EffectId
  checked_at: CapabilityCheckLocation
  source_span: SourceSpanId
}
```

---

## 15. Validation Requirements

RuntimePlan validation MUST reject:

```text
missing required table
unresolved ModuleId
unresolved FunctionId
unresolved SlotId
unresolved TypeId
unresolved ShapeId
unresolved CallSiteId
unresolved AccessSiteId
unresolved SafepointId
unresolved DeoptId
unresolved RuntimeHelperId
slot layout mismatch
function plan without entry EIR function
module plan without initialization function
record shape without field index map
enum shape without case index map
capability gate without capability
cache profile mismatch
source SIR digest mismatch
```

---

## 16. Cache Identity

RuntimePlan cache key MUST include:

```text
source_sir_digest
phase2_schema_version
vm_version
plan_version
target_profile
feature set
capability environment digest or epoch policy
dependency interface digests
stdlib interface digest
helper registry digest
```

---

## 17. Audit Tracking

This document completes:

```text
R4
```

It resolves:

```text
B-02
```

It partially supports:

```text
M-04
M-10
M-11
```
