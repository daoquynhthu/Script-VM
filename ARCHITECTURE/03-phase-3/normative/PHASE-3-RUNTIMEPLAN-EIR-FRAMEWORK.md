# Phase 3 · RuntimePlan and EIR Framework
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.4 framework draft  
Depends on: Phase 3 Performance and JIT Architecture v0.3  
Depends on: Phase 3 VM Runtime Semantics Round 1 v0.2  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: RuntimePlan schema framework, EIR identity, EIR function/block/operation model, slot layout, call/access site tables, safepoint/root map/deopt metadata framework, fast interpreter boundary  
Out of scope: full EIR instruction semantics, complete node lowering rules, concrete JIT backend implementation, concrete GC implementation, public bytecode

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



## 0. Status

This document defines the bridge between frozen SIR semantics and high-performance VM execution.

The VM execution pipeline is:

```text
SIR
  -> validation
  -> RuntimePlan
  -> EIR
  -> fast interpreter
  -> baseline JIT
  -> optimizing JIT
```

RuntimePlan and EIR are internal VM structures.

They are not public bytecode.

They are not package ABI.

They are not external compiler targets.

They may be cached, but caches are discardable and invalidated by VM/runtime compatibility checks.

---

## 1. Design Position

### 1.1 Why RuntimePlan Exists

SIR is semantic and table-rich.

Directly executing SIR would repeatedly consult:

```text
SymbolTable
ScopeTable
BindingTable
TypeTable
Record descriptors
Enum descriptors
PatternTable
ControlRegionTable
ModuleInterfaceDescriptor
```

That is correct for validation and diagnostics, but unsuitable as a hot execution path.

RuntimePlan converts validated SIR into execution-ready metadata.

### 1.2 Why EIR Exists

EIR is the compact internal execution form.

It exists so that the VM can have:

```text
fast interpreter
quickening
inline caches
type feedback
shape feedback
safepoints
deopt points
baseline JIT input
optimizing JIT input
```

without making executable IR public.

### 1.3 Core Rule

RuntimePlan and EIR may optimize representation.

They must not alter semantics.

The source of semantic authority remains:

```text
Phase 1 source semantics
Phase 2 SIR semantics
```

---

## 2. RuntimePlan Overview

### 2.1 RuntimePlan

```text
RuntimePlan {
  plan_version: Version
  source_sir_digest: Digest
  target_profile: RuntimeTargetProfile
  module_plans: List<ModulePlan>
  function_plans: List<FunctionPlan>
  type_plan: TypePlan
  shape_plan: ShapePlan
  call_site_table: CallSiteTable
  access_site_table: AccessSiteTable
  safepoint_seed_table: SafepointSeedTable
  deopt_seed_table: DeoptSeedTable
  helper_table: RuntimeHelperTable
  capability_gate_plan: CapabilityGatePlan
  diagnostics?: DiagnosticTable
}
```

### 2.2 RuntimeTargetProfile

```text
RuntimeTargetProfile {
  vm_version: Version
  architecture?: TargetArchitecture
  pointer_width?: UInt
  value_layout_profile: ValueLayoutProfile
  heap_profile: HeapProfile
  interpreter_profile: InterpreterProfile
  jit_profile?: JitProfile
  gc_profile?: GcProfile
}
```

The target profile records VM execution assumptions.

It is not public ABI.

### 2.3 RuntimePlan Digest

RuntimePlan cache keys must include:

```text
SIR semantic digest
SIR interface digest
Phase 2 schema version
VM version
RuntimePlan version
target profile
feature set
capability environment digest
dependency interface digests
stdlib interface digest
```

If any input changes incompatibly, RuntimePlan must be regenerated.

---

## 3. ModulePlan

### 3.1 ModulePlan

```text
ModulePlan {
  module_id: ModuleId
  module_name: QualifiedName
  module_scope: ScopeId
  module_slots: SlotLayout
  initialization_function: EirFunctionId
  import_plan: ImportPlan
  export_plan: ExportPlan
  module_state_slot: ModuleStateSlot
  source_map: SourceMapRef
}
```

### 3.2 ImportPlan

```text
ImportPlan {
  imports: List<ImportPlanEntry>
}
```

```text
ImportPlanEntry {
  import_node: NodeId
  import_descriptor: ImportDescriptor
  required_interface_digest?: Digest
  target_module_name: QualifiedName
  local_slot?: SlotId
  import_kind: ImportKind
  source_span?: SourceSpanId
}
```

```text
ImportKind =
  | WholeModule
  | NamedValue
```

### 3.3 ExportPlan

```text
ExportPlan {
  exports: List<ExportPlanEntry>
  sealed_after_init: Bool
}
```

```text
ExportPlanEntry {
  exported_name: SymbolId
  binding_id: BindingId
  source_slot: SlotId
  stable_interface_id: InterfaceId
}
```

Exports reference slots/cells.

They do not copy values.

### 3.4 Module Initialization Function

A module body is lowered to an internal initialization function.

This function is not source-visible.

It owns top-level execution order.

It must preserve:

```text
top-level declaration order
import execution order
export binding visibility
module state transitions
circular import checks
capability gates
```

---

## 4. FunctionPlan

### 4.1 FunctionPlan

```text
FunctionPlan {
  function_id: FunctionId
  name?: SymbolId
  source_function_node?: NodeId
  module_id: ModuleId
  parameter_layout: ParameterLayout
  local_layout: SlotLayout
  capture_layout: CaptureLayout
  return_type?: TypeId
  entry_eir_function: EirFunctionId
  call_site_range: CallSiteRange
  access_site_range: AccessSiteRange
  safepoint_range: SafepointRange
  deopt_range: DeoptRange
  source_map: SourceMapRef
}
```

### 4.2 ParameterLayout

```text
ParameterLayout {
  parameters: List<ParameterSlot>
}
```

```text
ParameterSlot {
  binding_id: BindingId
  name: SymbolId
  slot_id: SlotId
  type_contract?: TypeId
  default_node?: NodeId
  default_eir?: EirBlockId
}
```

Default argument expressions are evaluated at call time.

They must not be moved to function creation.

### 4.3 CaptureLayout

```text
CaptureLayout {
  captures: List<CaptureSlot>
}
```

```text
CaptureSlot {
  binding_id: BindingId
  capture_index: UInt
  capture_kind: CaptureKind
  cell_slot: SlotId
}
```

Captured mutable bindings must use cells.

Captured immutable values may later be optimized, but semantics must remain cell-compatible.

---

## 5. Slot Layout

### 5.1 SlotId

```text
SlotId
```

Encoded internally as compact integer.

SlotId is VM-internal.

### 5.2 SlotLayout

```text
SlotLayout {
  slots: List<SlotDescriptor>
  temporary_count: UInt
  max_live_slots: UInt
}
```

### 5.3 SlotDescriptor

```text
SlotDescriptor {
  slot_id: SlotId
  slot_kind: SlotKind
  binding_id?: BindingId
  type_hint?: TypeId
  mutability?: BindingMutability
  storage: SlotStorage
  source_span?: SourceSpanId
}
```

### 5.4 SlotKind

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

### 5.5 SlotStorage

```text
SlotStorage =
  | Value
  | Cell
  | Constant
  | TypeDescriptor
  | ModuleRef
  | RuntimeInternal
```

### 5.6 Slot Rules

Hot-path local access must use slots.

SIR `BindingId` lookup is resolved during RuntimePlan generation.

Execution must not perform ordinary variable lookup by string.

### 5.7 Slot Cell Rule

A slot stores a cell when:

```text
binding is captured mutably
binding may be observed by closure after frame creation
binding is exported by module
binding is shared across module import/export boundary
```

Otherwise, a slot may store direct `Value`.

---

## 6. TypePlan and ShapePlan

### 6.1 TypePlan

```text
TypePlan {
  runtime_types: List<RuntimeTypeDesc>
}
```

```text
RuntimeTypeDesc {
  type_id: TypeId
  type_kind: RuntimeTypeKind
  check_strategy: TypeCheckStrategy
}
```

### 6.2 TypeCheckStrategy

```text
TypeCheckStrategy =
  | AlwaysAccept
  | NeverAccept
  | ImmediateKindCheck
  | ShapeCheck
  | UnionCheck
  | OptionalCheck
  | FunctionSignatureCheck
  | ExtensionCheck
```

### 6.3 ShapePlan

```text
ShapePlan {
  record_shapes: List<RecordShape>
  enum_shapes: List<EnumShape>
}
```

### 6.4 RecordShape

```text
RecordShape {
  shape_id: ShapeId
  record_id: RecordId
  type_id: TypeId
  field_count: UInt
  fields: List<RecordFieldShape>
  method_table: MethodTableRef
}
```

```text
RecordFieldShape {
  field_id: FieldId
  field_index: FieldIndex
  name: SymbolId
  mutability: FieldMutability
  type_contract?: TypeId
}
```

### 6.5 EnumShape

```text
EnumShape {
  shape_id: ShapeId
  enum_id: EnumId
  type_id: TypeId
  cases: List<EnumCaseShape>
}
```

```text
EnumCaseShape {
  case_id: CaseId
  case_index: CaseIndex
  payload_layout: PayloadLayout
}
```

### 6.6 Shape Rule

Record and enum access must lower from symbolic IDs to shape/index access.

```text
FieldId -> FieldIndex
CaseId -> CaseIndex
```

String lookup is not the normal execution path.

---

## 7. Call Site Table

### 7.1 CallSiteTable

```text
CallSiteTable {
  sites: List<CallSiteRecord>
}
```

### 7.2 CallSiteRecord

```text
CallSiteRecord {
  call_site_id: CallSiteId
  node_id: NodeId
  enclosing_function: FunctionId
  argument_layout: ArgumentLayout
  expected_callee_kind?: CallableKindSet
  inline_cache_slot: InlineCacheSlotId
  type_feedback_slot: FeedbackSlotId
  source_span?: SourceSpanId
}
```

### 7.3 ArgumentLayout

```text
ArgumentLayout {
  positional_count: UInt
  named_arguments: List<NamedArgumentLayout>
  has_defaults_possible: Bool
}
```

```text
NamedArgumentLayout {
  name: SymbolId
  argument_index: UInt
}
```

### 7.4 CallableKindSet

```text
CallableKindSet {
  user_function: Bool
  builtin_function: Bool
  record_constructor: Bool
  enum_case_constructor: Bool
  bound_method: Bool
  host_function: Bool
}
```

### 7.5 Call Site Requirement

Every call expression in EIR must reference a CallSiteId.

Even if the first implementation does not use inline caches, the identity must exist.

---

## 8. Access Site Table

### 8.1 AccessSiteTable

```text
AccessSiteTable {
  sites: List<AccessSiteRecord>
}
```

### 8.2 AccessSiteRecord

```text
AccessSiteRecord {
  access_site_id: AccessSiteId
  node_id: NodeId
  enclosing_function: FunctionId
  access_kind: AccessKind
  expected_receiver_shape?: ShapeId
  resolved_field_index?: FieldIndex
  inline_cache_slot: InlineCacheSlotId
  feedback_slot: FeedbackSlotId
  source_span?: SourceSpanId
}
```

### 8.3 AccessKind

```text
AccessKind =
  | AttributeRead
  | AttributeWrite
  | MethodRead
  | IndexRead
  | IndexWrite
  | Slice
```

### 8.4 Access Site Requirement

Attribute, method, index, and slice operations that can become hot should have AccessSiteId.

Record field access should resolve to field indices when statically known.

Generic access may remain, but it must be cacheable.

---

## 9. Inline Cache Model

### 9.1 InlineCacheSlot

```text
InlineCacheSlot {
  slot_id: InlineCacheSlotId
  state: InlineCacheState
  payload: InlineCachePayload
}
```

### 9.2 InlineCacheState

```text
InlineCacheState =
  | Uninitialized
  | Monomorphic
  | Polymorphic
  | Megamorphic
  | Disabled
```

### 9.3 InlineCachePayload

```text
InlineCachePayload =
  | Empty
  | CallCache
  | AttributeCache
  | IndexCache
  | TypeCheckCache
```

### 9.4 Cache Rule

Inline cache state is optimization state.

It must be guarded.

It must be invalidated or bypassed when assumptions fail.

It must not change observable semantics.

---

## 10. Feedback Tables

### 10.1 FeedbackTable

```text
FeedbackTable {
  slots: List<FeedbackSlot>
}
```

### 10.2 FeedbackSlot

```text
FeedbackSlot {
  feedback_slot_id: FeedbackSlotId
  kind: FeedbackKind
  state: FeedbackState
}
```

### 10.3 FeedbackKind

```text
FeedbackKind =
  | TypeFeedback
  | ShapeFeedback
  | CallTargetFeedback
  | BranchFeedback
  | AllocationFeedback
  | ErrorFeedback
```

### 10.4 FeedbackState

```text
FeedbackState =
  | Empty
  | Stable
  | Polymorphic
  | Megamorphic
  | Unstable
```

### 10.5 Feedback Rule

Feedback is speculative.

Any optimization derived from feedback must have fallback or deopt path.

---

## 11. EIR Overview

### 11.1 EirModule

```text
EirModule {
  eir_version: Version
  source_runtime_plan_digest: Digest
  functions: List<EirFunction>
  constants: ConstantPool
  source_map: EirSourceMap
  safepoints: SafepointTable
  deopt_points: DeoptPointTable
}
```

### 11.2 EirFunction

```text
EirFunction {
  eir_function_id: EirFunctionId
  function_id?: FunctionId
  name?: SymbolId
  slot_layout: SlotLayout
  blocks: List<EirBlock>
  entry_block: EirBlockId
  call_site_range: CallSiteRange
  access_site_range: AccessSiteRange
  source_span?: SourceSpanId
}
```

### 11.3 EirBlock

```text
EirBlock {
  block_id: EirBlockId
  parameters: List<EirBlockParam>
  operations: List<EirOp>
  terminator: EirTerminator
  source_span?: SourceSpanId
}
```

### 11.4 EirBlockParam

```text
EirBlockParam {
  slot_id: SlotId
  type_hint?: TypeId
}
```

### 11.5 EIR Structure Rule

EIR is block-structured.

A block contains operations and ends with exactly one terminator.

Operations produce values into slots.

Terminators control flow.

---

## 12. EIR Operation Families

### 12.1 EirOp

Round 0 EIR defines operation families, not every final opcode.

```text
EirOp =
  | LoadOp
  | StoreOp
  | ConstantOp
  | UnaryOp
  | BinaryOp
  | LogicalOp
  | CheckOp
  | CallOp
  | AccessOp
  | ConstructOp
  | PatternOp
  | RuntimeHelperOp
  | SafepointOp
  | GuardOp
  | DebugOp
```

### 12.2 LoadOp

Load operations read from:

```text
local slot
capture slot
module slot
constant pool
field index
enum payload index
```

### 12.3 StoreOp

Store operations write to:

```text
local slot
capture cell
module cell
record field
list index
map entry
temporary slot
```

Store operations must pass through write-barrier hooks where heap mutation may occur.

### 12.4 CheckOp

Check operations include:

```text
type check
Bool condition check
callable check
hashable check
readonly check
capability check
arity check
overflow check
division by zero check
```

### 12.5 CallOp

Call operations reference:

```text
CallSiteId
callee slot
argument slots
result slot
fallback helper
```

### 12.6 AccessOp

Access operations reference:

```text
AccessSiteId
receiver slot
field index or key slot
result slot
fallback helper
```

### 12.7 ConstructOp

Construct operations include:

```text
list construction
map construction
record construction
enum value construction
function object construction
error construction
```

### 12.8 PatternOp

Pattern operations include:

```text
literal match
record shape match
field pattern load
enum case match
payload pattern load
list length match
map key match
or-pattern dispatch
binding write
```

### 12.9 RuntimeHelperOp

Runtime helper operations call VM helper functions.

They must declare:

```text
may allocate
may raise
is safepoint
required capability
```

### 12.10 GuardOp

Guards verify speculative assumptions.

On failure, they branch to fallback or deopt.

---

## 13. EIR Terminators

### 13.1 EirTerminator

```text
EirTerminator =
  | Jump
  | Branch
  | Return
  | Raise
  | LoopBackedge
  | Switch
  | Unwind
  | Unreachable
```

### 13.2 Jump

Unconditional block transition.

### 13.3 Branch

Bool-based branch.

Must not use truthiness.

### 13.4 Return

Returns a value from the function.

Must preserve return contract checking.

### 13.5 Raise

Raises an error value.

Must preserve language error semantics.

### 13.6 LoopBackedge

Loop transition and safepoint candidate.

May update hotness counters.

### 13.7 Switch

Used for enum cases, pattern decisions, or dense dispatch.

### 13.8 Unwind

Transfers control into structured unwinding logic.

May target runtime helper if lowering remains high-level.

---

## 14. Safepoint Framework

### 14.1 SafepointTable

```text
SafepointTable {
  safepoints: List<SafepointRecord>
}
```

### 14.2 SafepointRecord

```text
SafepointRecord {
  safepoint_id: SafepointId
  eir_function_id: EirFunctionId
  eir_location: EirLocation
  source_span?: SourceSpanId
  live_slots: List<SlotId>
  live_objects: List<RootRef>
  frame_state: FrameStateRef
  safepoint_kind: SafepointKind
}
```

### 14.3 SafepointKind

```text
SafepointKind =
  | FunctionCall
  | LoopBackedge
  | Allocation
  | HostCall
  | RaiseBoundary
  | ImportBoundary
  | HelperCall
  | JitSideExit
```

### 14.4 RootRef

```text
RootRef =
  | SlotRoot { slot_id: SlotId }
  | CaptureRoot { slot_id: SlotId }
  | ModuleRoot { module_id: ModuleId }
  | RegionRoot { region_id: ControlRegionId }
  | PendingControlRoot
  | ErrorRoot
  | HostRoot
```

### 14.5 Safepoint Rule

Every allocation, loop backedge, host call, and function call should have or be able to synthesize a safepoint.

---

## 15. Deopt Metadata Framework

### 15.1 DeoptPointTable

```text
DeoptPointTable {
  deopt_points: List<DeoptPointRecord>
}
```

### 15.2 DeoptPointRecord

```text
DeoptPointRecord {
  deopt_id: DeoptId
  eir_function_id: EirFunctionId
  eir_location: EirLocation
  source_span?: SourceSpanId
  frame_map: FrameMap
  local_slot_map: LocalSlotMap
  value_stack_map: ValueStackMap
  region_stack_state: RegionStackState
  pending_control_state?: PendingControlState
}
```

### 15.3 FrameMap

```text
FrameMap {
  function_id?: FunctionId
  module_id: ModuleId
  call_site_id?: CallSiteId
  visible_slots: List<VisibleSlot>
}
```

### 15.4 VisibleSlot

```text
VisibleSlot {
  binding_id?: BindingId
  slot_id: SlotId
  value_kind_hint?: RuntimeValueKind
}
```

### 15.5 RegionStackState

```text
RegionStackState {
  active_regions: List<ControlRegionId>
  defer_state_refs: List<DeferStateRef>
  resource_state_refs: List<ResourceStateRef>
  finally_state_refs: List<FinallyStateRef>
}
```

### 15.6 Deopt Rule

Any EIR operation or JIT code that depends on speculative type, shape, call-target, module-state, readonly, or arithmetic assumptions must be able to fall back or deopt.

---

## 16. Runtime Helper Table

### 16.1 RuntimeHelperTable

```text
RuntimeHelperTable {
  helpers: List<RuntimeHelperDescriptor>
}
```

### 16.2 RuntimeHelperDescriptor

```text
RuntimeHelperDescriptor {
  helper_id: RuntimeHelperId
  name: String
  signature: RuntimeHelperSignature
  may_allocate: Bool
  may_raise: Bool
  is_safepoint: Bool
  required_capability?: CapabilityId
  effect?: EffectId
}
```

### 16.3 Helper Families

Required helper families:

```text
generic call
method bind
record construction
enum construction
generic field access
generic index access
map lookup
string slicing
type contract check
pattern fallback
raise construction
unwinding
defer execution
resource close
module import
capability check
allocation
write barrier
```

### 16.4 Helper Rule

Runtime helpers are internal.

They do not define public native ABI.

JIT code may call helpers only through VM-controlled helper table.

---

## 17. Fast Interpreter Framework

### 17.1 Interpreter State

```text
FastInterpreterState {
  vm
  eir_module
  current_function
  current_block
  instruction_pointer
  slots
  frame
  region_stack
  feedback_table
}
```

### 17.2 Dispatch

The interpreter dispatches over EIR operations.

It must preserve:

```text
left-to-right evaluation
slot semantics
control terminator semantics
structured unwinding
safepoint behavior
runtime helper behavior
diagnostic source mapping
```

### 17.3 Quickening

The interpreter may quicken:

```text
generic call
generic field access
generic type check
generic binary op
generic pattern op
```

Quickened ops must retain fallback or deopt metadata.

### 17.4 Interpreter and JIT Sharing

The fast interpreter and baseline JIT consume the same RuntimePlan and EIR.

They must not diverge semantically.

---

## 18. RuntimePlan Generation Validation

RuntimePlan generation must check:

```text
all BindingId values have slots
all captured bindings have capture layout
all module exports have module slots or cells
all FieldId values map to field indices
all CaseId values map to case indices
all TypeId values map to runtime type descriptors
all ControlRegionId values map to region plans
all PatternId values map to pattern plans
all call nodes have CallSiteId
all access nodes have AccessSiteId where applicable
all safepoint candidates have seed entries
all source spans are preserved where available
```

Failure is a VM planning diagnostic.

It must not be ignored.

---

## 19. EIR Validation

Before fast interpretation or JIT, EIR must validate:

```text
every block has one terminator
all slot reads dominate or are initialized before use
all block targets exist
all helper IDs exist
all call site IDs exist
all access site IDs exist
all safepoint IDs exist where referenced
all deopt IDs exist where referenced
all type checks reference valid type descriptors
all shape guards reference valid shapes
all write operations have barrier hook path
all raise/unwind paths preserve control semantics
```

---

## 20. Compatibility and Cache Rules

RuntimePlan and EIR are VM-internal.

They may change across VM versions.

Cache invalidation must occur on:

```text
SIR digest change
Phase 2 schema incompatibility
VM version change
RuntimePlan version change
EIR version change
target profile change
dependency interface digest change
capability environment change
stdlib interface change
helper table incompatibility
GC/value layout profile change
JIT backend profile change
```

No RuntimePlan or EIR cache may be treated as package ABI.

---

## 21. Non-Goals

This document does not define:

```text
public bytecode
public EIR files
external compiler target
native plugin ABI
concrete Cranelift lowering
concrete LLVM lowering
complete GC
complete JIT
complete opcode list
complete node lowering rules
debugger protocol
profiler format
```

---

## 22. Next Work

Next documents should define:

```text
EIR operation semantics round 1
SIR-to-RuntimePlan lowering rules
SIR-to-EIR lowering rules
fast interpreter execution semantics
GC root enumeration details
baseline JIT backend interface
runtime helper function contracts

```
