# Phase 3 · Fast Interpreter Concrete Data Structures
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.11 interpreter-structure draft  
Depends on: Phase 3 Baseline JIT Backend Interface v0.10  
Depends on: Phase 3 GC Root Enumeration and Safepoint Model v0.9  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: concrete fast-interpreter runtime structures, call stack, frames, slot arrays, region stack, instruction pointer, block dispatch, feedback/cache storage, helper transition, safepoint polling, interpreter/JIT shared metadata  
Out of scope: concrete Rust implementation, concrete byte encoding, concrete threaded-dispatch implementation, concrete Cranelift lowering, optimizing JIT, public bytecode

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

This document defines the concrete execution structures for the EIR fast interpreter.

The fast interpreter is the production non-JIT execution target.

It consumes:

```text
RuntimePlan
EIR
RuntimeHelperTable
SafepointTable
RootMap
FrameMap
FeedbackTable
InlineCacheTable
```

It must preserve the same execution metadata used by the baseline JIT.

The fast interpreter must not be a naive SIR-walk evaluator.

---

## 1. Design Goals

The fast interpreter must provide:

```text
low dispatch overhead
slot-based local access
preplanned field/case access
explicit region stack
explicit pending control
safepoint polling
root visibility
helper-call transition
feedback collection
inline-cache storage
JIT-compatible frame metadata
diagnostic source mapping
```

The fast interpreter must avoid:

```text
textual name lookup in hot path
HashMap locals in hot path
SIR table traversal in hot path
dynamic string field lookup for fixed records
Rc/RefCell proliferation in every hot operation
Rust panic for language control flow
hidden raw object pointers
```

---

## 2. FastInterpreter

### 2.1 FastInterpreter

```text
FastInterpreter {
  vm: VmRef
  runtime_plan: RuntimePlanRef
  eir_module: EirModuleRef
  call_stack: InterpreterCallStack
  dispatch_state: DispatchState
  feedback_store: FeedbackStore
  inline_cache_store: InlineCacheStore
  safepoint_poller: SafepointPoller
  helper_bridge: HelperBridge
  diagnostics: DiagnosticSink
}
```

### 2.2 VM Reference

`VmRef` gives controlled access to:

```text
heap
module environment
capability environment
runtime helper table
GC interface
diagnostic engine
host boundary
```

The interpreter must not bypass these VM services.

### 2.3 Interpreter State Rule

Interpreter state is VM-internal.

It must not be exposed as language ABI.

It must be inspectable by GC, diagnostics, and future debugger/profiler layers through controlled APIs.

---

## 3. DispatchState

### 3.1 DispatchState

```text
DispatchState {
  current_frame: FrameId
  current_function: EirFunctionId
  current_block: EirBlockId
  instruction_index: UInt
  pending_jump?: PendingJump
  current_source_span?: SourceSpanId
}
```

### 3.2 Instruction Pointer

The interpreter instruction pointer is:

```text
(EirFunctionId, EirBlockId, instruction_index)
```

It is not a public bytecode address.

### 3.3 EirLocation

```text
EirLocation {
  eir_function_id: EirFunctionId
  block_id: EirBlockId
  op_index?: UInt
  terminator: Bool
}
```

EirLocation is used by:

```text
diagnostics
safepoints
root maps
deopt metadata
profiling
debug mode
```

### 3.4 PendingJump

```text
PendingJump {
  target_block: EirBlockId
  arguments: List<SlotId>
}
```

PendingJump is used for block argument transfer.

---

## 4. InterpreterCallStack

### 4.1 InterpreterCallStack

```text
InterpreterCallStack {
  frames: Vec<InterpreterFrame>
  max_depth: UInt
}
```

### 4.2 Stack Depth

The VM must enforce a maximum logical call depth.

If exceeded, it raises:

```text
StackOverflowError
```

or equivalent VM runtime error category if Phase 1 has not yet named it.

### 4.3 Logical vs Host Stack

The interpreter may use Rust call stack or explicit trampoline internally.

But it must maintain a logical call stack for:

```text
diagnostics
GC roots
deoptimization
debugging
JIT transition
```

### 4.4 Stack Walking

The call stack must be walkable at safepoints.

Stack walking must expose:

```text
frame id
function id
module id
EIR location
source span
slot roots
region roots
pending control
```

---

## 5. InterpreterFrame

### 5.1 InterpreterFrame

```text
InterpreterFrame {
  frame_id: FrameId
  frame_kind: FrameKind
  module_id: ModuleId
  function_id?: FunctionId
  eir_function_id: EirFunctionId
  slots: SlotArray
  region_stack: RegionStack
  frame_map: FrameMapId
  return_target?: ReturnTarget
  caller_location?: EirLocation
  current_source_span?: SourceSpanId
  pending_control_slot?: SlotId
}
```

### 5.2 FrameKind

```text
FrameKind =
  | ModuleInit
  | Function
  | BuiltinAdapter
  | Test
  | HelperInternal
```

### 5.3 ModuleInit Frame

A module initialization frame executes synthetic module initialization EIR.

It cannot accept source-level return/break/continue.

### 5.4 Function Frame

A function frame owns:

```text
parameter slots
local slots
temporary slots
capture slots
region stack
return target
```

### 5.5 BuiltinAdapter Frame

A builtin adapter frame may be used when a builtin needs language-visible stack trace context.

Pure internal helper frames may be hidden from source stack traces.

### 5.6 Test Frame

A test frame is used by the test runner.

Test execution is not ordinary module initialization.

---

## 6. SlotArray

### 6.1 SlotArray

```text
SlotArray {
  slots: Vec<SlotState>
  layout: SlotLayoutRef
  initialized_bitmap?: BitSet
  heap_ref_bitmap?: BitSet
}
```

### 6.2 SlotState

```text
SlotState =
  | Uninitialized
  | Value(Value)
  | Cell(BindingCellRef)
  | RuntimeInternal(RuntimeValue)
```

### 6.3 Slot Access

Slot access is by `SlotId`.

No ordinary variable access uses text lookup.

### 6.4 Initialization Tracking

The interpreter may use:

```text
SlotState tags
initialized_bitmap
validation-derived initialization facts
```

Uninitialized source binding read raises `UninitializedBindingError`.

Uninitialized internal temporary read is VM bug or EIR validation failure.

### 6.5 Heap Reference Tracking

`heap_ref_bitmap` may accelerate root enumeration.

If value layout can distinguish heap refs cheaply, bitmap may be omitted.

Moving GC requires every live heap reference location to be updateable.

### 6.6 Slot Write Protocol

Slot writes must perform:

```text
mutability check where required
type contract check where attached
write barrier if heap reference mutation occurs
heap_ref tracking update
diagnostic source span preservation
```

### 6.7 Temporary Reuse

Temporaries may be reused only if liveness analysis proves no semantic effect.

Debug mode may disable aggressive reuse for better diagnostics.

---

## 7. BindingCell Runtime

### 7.1 BindingCellRef

```text
BindingCellRef {
  cell_id: CellId
}
```

### 7.2 BindingCell

```text
BindingCell {
  binding_id: BindingId
  state: BindingState
  mutability: BindingMutability
  type_contract?: TypeId
  owner: CellOwner
}
```

### 7.3 BindingState

```text
BindingState =
  | Uninitialized
  | Initialized(Value)
```

### 7.4 Cell Access

Cell read:

```text
check initialized
return value
```

Cell write:

```text
check mutability
check type contract
write value
run write barrier
```

### 7.5 Cell Root

Every live cell is traceable by GC.

Cells in closure captures or module exports may outlive frames.

---

## 8. RegionStack

### 8.1 RegionStack

```text
RegionStack {
  regions: Vec<RuntimeRegionFrame>
}
```

### 8.2 RuntimeRegionFrame

```text
RuntimeRegionFrame {
  region_id: ControlRegionId
  region_kind: ControlRegionKind
  owner_node: NodeId
  cleanup_state: CleanupState
  loop_target?: LoopTarget
  match_state?: MatchState
  try_state?: TryState
  source_span?: SourceSpanId
}
```

### 8.3 CleanupState

```text
CleanupState {
  defer_stack: Vec<DeferredCallable>
  resource_stack: Vec<ResourceCleanup>
  finally_state?: FinallyState
  cleanup_progress: CleanupProgress
}
```

### 8.4 CleanupProgress

```text
CleanupProgress =
  | NotStarted
  | RunningDefers
  | RunningResources
  | RunningFinally
  | Complete
```

CleanupProgress is required for deopt, diagnostics, and interruption safety.

### 8.5 LoopTarget

```text
LoopTarget {
  break_target: EirBlockId
  continue_target: EirBlockId
  loop_region: ControlRegionId
}
```

### 8.6 Region Stack Rule

All region stack entries must be visible to:

```text
unwind helper
GC root enumeration
deopt reconstruction
debug diagnostics
```

---

## 9. PendingControl Storage

### 9.1 PendingControlSlot

A frame may reserve a hidden slot for pending control.

```text
PendingControlSlot {
  slot_id: SlotId
}
```

### 9.2 PendingControlValue

```text
PendingControlValue =
  | Normal
  | Return(Value)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
```

### 9.3 PendingControl Rule

Pending control is not a source value.

It is VM-internal control state.

It must be traceable if it contains heap values.

### 9.4 Unwind Compatibility

Unwind helper reads and updates pending control.

Finally override writes new pending control.

Cleanup errors may replace or augment pending control according to Phase 2 semantics.

---

## 10. Block Dispatch

### 10.1 Block Execution

Block execution is:

```text
fetch current block
execute operations sequentially
execute terminator
dispatch terminator result
```

### 10.2 Operation Dispatch

Implementation options:

```text
match over EirOp
function pointer table
threaded dispatch where safe
quickened operation variants
```

The specification does not require one dispatch mechanism.

### 10.3 No Fallthrough

A block must explicitly terminate.

Interpreter must reject block fallthrough.

### 10.4 Block Arguments

Block argument transfer must be simultaneous.

Recommended implementation:

```text
copy jump argument values into temporary transfer buffer
then assign block parameter slots
```

This prevents clobbering.

### 10.5 Dispatch Diagnostics

On operation failure, current EirLocation and SourceSpan are captured.

---

## 11. Terminator Dispatch

### 11.1 TerminatorResult

```text
TerminatorResult =
  | NextBlock(EirBlockId)
  | Return(Value)
  | Raise(ErrorHandle)
  | Unwind(PendingControlValue)
  | Deopt(DeoptId)
  | Halt
```

### 11.2 NextBlock

Updates current block and resets instruction index.

### 11.3 Return

If active cleanup exists, return is converted to pending control and enters unwind.

If no active cleanup exists, frame returns to caller.

### 11.4 Raise

Raise enters unwind path.

If no handler exists, it propagates to caller.

### 11.5 Halt

Halt is valid only for VM-internal completion contexts.

Source program termination should be represented as module/test/function completion, not arbitrary halt.

---

## 12. FeedbackStore

### 12.1 FeedbackStore

```text
FeedbackStore {
  slots: Vec<FeedbackSlotRuntime>
}
```

### 12.2 FeedbackSlotRuntime

```text
FeedbackSlotRuntime {
  feedback_slot_id: FeedbackSlotId
  kind: FeedbackKind
  state: FeedbackState
  payload: FeedbackPayload
  update_count: UInt
}
```

### 12.3 FeedbackPayload

```text
FeedbackPayload =
  | Empty
  | TypeFeedbackPayload
  | ShapeFeedbackPayload
  | CallTargetFeedbackPayload
  | BranchFeedbackPayload
  | AllocationFeedbackPayload
  | ErrorFeedbackPayload
```

### 12.4 Feedback Update Rule

Feedback update must not change observable semantics.

Feedback may be disabled in deterministic testing mode if required.

### 12.5 Hotness Counters

Hotness counters may be stored as feedback payload or separate counter table.

Hotness events:

```text
function entry
loop backedge
call site execution
guard failure
allocation count
```

---

## 13. InlineCacheStore

### 13.1 InlineCacheStore

```text
InlineCacheStore {
  slots: Vec<InlineCacheRuntimeSlot>
}
```

### 13.2 InlineCacheRuntimeSlot

```text
InlineCacheRuntimeSlot {
  inline_cache_slot_id: InlineCacheSlotId
  state: InlineCacheState
  payload: InlineCacheRuntimePayload
  invalidation_epoch: UInt
}
```

### 13.3 Runtime Payloads

```text
InlineCacheRuntimePayload =
  | Empty
  | CallCacheRuntime
  | AttributeCacheRuntime
  | IndexCacheRuntime
  | TypeCheckCacheRuntime
```

### 13.4 Cache Epoch

Each cache slot has an invalidation epoch.

If global or module epoch changes, cache may become stale.

### 13.5 Cache Update Rule

Cache transitions:

```text
Uninitialized -> Monomorphic
Monomorphic -> Polymorphic
Polymorphic -> Megamorphic
any -> Disabled
```

Cache update must be atomic with respect to interpreter reentrancy if reentrancy is allowed.

---

## 14. Call Cache Runtime

### 14.1 CallCacheRuntime

```text
CallCacheRuntime {
  observed_targets: Vec<ObservedCallTarget>
  arity_shape: ArityShape
  miss_count: UInt
}
```

### 14.2 ObservedCallTarget

```text
ObservedCallTarget {
  callee_kind: CallableKind
  function_id?: FunctionId
  builtin_id?: BuiltinFunctionId
  shape_id?: ShapeId
  compiled_function_id?: CompiledFunctionId
}
```

### 14.3 Call Cache Use

A call cache may skip generic lookup only if guards pass.

Fallback helper is required.

### 14.4 Compiled Target

If compiled target exists, interpreter dispatch may call compiled function through VM call bridge.

The bridge must preserve frame/root/safepoint metadata.

---

## 15. Access Cache Runtime

### 15.1 AttributeCacheRuntime

```text
AttributeCacheRuntime {
  receiver_shape: ShapeId
  field_index?: FieldIndex
  method_index?: MethodIndex
  miss_count: UInt
}
```

### 15.2 IndexCacheRuntime

```text
IndexCacheRuntime {
  receiver_kind: RuntimeValueKind
  key_kind?: RuntimeValueKind
  strategy: IndexStrategy
  miss_count: UInt
}
```

### 15.3 IndexStrategy

```text
IndexStrategy =
  | ListIntIndex
  | MapHashLookup
  | SliceList
  | SliceString
  | GenericHelper
```

### 15.4 Access Cache Rule

Access cache must respect:

```text
readonly checks
shape checks
field mutability
type contracts
write barriers
error categories
```

---

## 16. SafepointPoller

### 16.1 SafepointPoller

```text
SafepointPoller {
  poll_epoch: UInt
  gc_requested: Bool
  interrupt_requested: Bool
  profiling_requested: Bool
  debug_trap_requested: Bool
}
```

### 16.2 Poll Sites

The interpreter polls at:

```text
loop backedge
function call
helper call
allocation
host call
module import
long-running builtin
debug-inserted poll
```

### 16.3 Poll Result

```text
PollResult =
  | Continue
  | RunGc
  | Interrupt
  | Cancel
  | DebugTrap
  | ProfileSample
```

### 16.4 Poll Rule

Polling must occur only where root maps can be constructed.

If no root map is available, poll must be delayed to next valid safepoint.

---

## 17. HelperBridge

### 17.1 HelperBridge

```text
HelperBridge {
  helper_table: RuntimeHelperTableRef
  current_helper_call?: HelperCallState
}
```

### 17.2 HelperCallState

```text
HelperCallState {
  helper_id: RuntimeHelperId
  arguments: Vec<Value>
  source_span?: SourceSpanId
  safepoint_id?: SafepointId
  roots_visible: Bool
}
```

### 17.3 Helper Transition

Before calling helper:

```text
validate helper descriptor
marshal arguments
make roots visible if required
perform capability check if required
enter helper call state
call helper
normalize result
exit helper call state
```

### 17.4 Helper Result Normalization

Helper returns are normalized to:

```text
Value
VmControl
VmError
```

Language errors become `VmControl::Raise`.

Structural VM errors become `VmError`.

---

## 18. Interpreter/JIT Bridge

### 18.1 Bridge Purpose

The interpreter may call compiled functions.

Compiled functions may deopt or call back into interpreter.

### 18.2 Interpreter to JIT

Interpreter-to-JIT call requires:

```text
argument representation conversion
frame setup or compiled frame bridge
root visibility at call boundary
safepoint metadata
return value normalization
error/control propagation
```

### 18.3 JIT to Interpreter

JIT-to-interpreter fallback requires:

```text
frame reconstruction
slot reconstruction
region stack reconstruction
pending control reconstruction
source location reconstruction
```

### 18.4 Bridge Rule

Interpreter and JIT share:

```text
RuntimePlan
EIR
FrameMap
RootMap
SafepointMap
Deopt metadata
RuntimeHelperTable
```

They must not implement divergent semantics.

---

## 19. Source Mapping Runtime

### 19.1 SourceLocationState

```text
SourceLocationState {
  current_eir_location: EirLocation
  current_node_id?: NodeId
  current_source_span?: SourceSpanId
}
```

### 19.2 Source Mapping Rule

Before executing an op that may raise or call helper, interpreter updates current source mapping.

### 19.3 Diagnostic Capture

On error, diagnostics capture:

```text
current source span
EIR location
SIR NodeId
frame stack
helper context if relevant
```

### 19.4 Helper Source Mapping

Helper calls receive source span or source mapping reference when they may raise.

---

## 20. Root Enumeration for Interpreter

### 20.1 InterpreterRootEnumerator

```text
InterpreterRootEnumerator {
  call_stack: InterpreterCallStackRef
  module_environment: ModuleEnvironmentRef
  helper_bridge: HelperBridgeRef
  host_roots: HostRootRegistryRef
}
```

### 20.2 Enumeration Steps

At safepoint:

1. enumerate frames
2. enumerate initialized live slots
3. enumerate cell/capture roots
4. enumerate region stack
5. enumerate pending control
6. enumerate helper arguments
7. enumerate module roots
8. enumerate host roots
9. emit RootSet

### 20.3 Live Slot Mode

Modes:

```text
AllInitializedSlots
LivenessDerivedSlots
DebugAllSlots
```

Bootstrap may use `AllInitializedSlots`.

Production should prefer `LivenessDerivedSlots`.

### 20.4 Root Update

If moving GC occurs, root enumerator must update locations.

Values must be written back to their slots/cells/root locations.

---

## 21. FrameMap Runtime Use

### 21.1 FrameMap Access

Each InterpreterFrame references FrameMapId.

FrameMap provides:

```text
visible bindings
slot mapping
source span mapping
region state mapping
deopt reconstruction metadata
```

### 21.2 Debug Inspection

Debug inspection uses FrameMap, not raw slot layout.

### 21.3 Stack Trace

Stack trace uses:

```text
function name
module name
source span
call site
```

Internal helper frames may be elided unless debug mode includes VM internals.

---

## 22. Operation Execution Contract

### 22.1 execute_op

Conceptual:

```text
execute_op(state, op) -> OpResult
```

### 22.2 OpResult

```text
OpResult =
  | Continue
  | Raise(ErrorHandle)
  | Deopt(DeoptId)
  | InternalError(VmError)
```

### 22.3 Contract

`execute_op` must:

```text
read only declared slots
write only declared dest slots
preserve source mapping
update feedback if needed
respect barrier hooks
respect helper descriptors
not modify instruction pointer except through defined protocol
```

---

## 23. Terminator Execution Contract

### 23.1 execute_terminator

```text
execute_terminator(state, terminator) -> TerminatorResult
```

### 23.2 Contract

`execute_terminator` must:

```text
check Bool for Branch
perform block argument transfer for Jump
mark loop backedge safepoint for LoopBackedge
enter unwind for Return/Raise/Break/Continue
reject invalid Unreachable
```

### 23.3 Terminator Source Mapping

Terminator diagnostics use terminator source span or owner node span.

---

## 24. Reentrancy and Host Calls

### 24.1 Reentrancy

The first VM may forbid reentrant execution except through explicitly supported host calls.

If reentrancy is allowed, interpreter state mutation must be protected.

### 24.2 Host Call State

Before host call:

```text
roots visible
capability checked
helper state entered
source mapping saved
```

After host call:

```text
result normalized
host errors converted
host roots released or transferred
state restored
```

### 24.3 Reentrancy Validation

If a host call reenters VM, nested call stack must be represented explicitly.

---

## 25. Interrupt and Cancellation

### 25.1 Cancellation

Cancellation may be delivered at safepoints.

Cancellation should become a VM-controlled interrupt/error category, not Rust panic.

### 25.2 Interrupt Safety

Interrupt cannot occur at arbitrary unsafe point.

It must occur only at safepoints or explicit poll points.

### 25.3 Cleanup on Cancellation

If cancellation behaves like Raise, structured unwinding must run.

If cancellation is host abort, VM must document cleanup guarantee.

---

## 26. Determinism Modes

### 26.1 Deterministic Execution Mode

A deterministic mode may disable:

```text
profile-dependent quickening
nondeterministic cache updates
adaptive JIT installation
timing-dependent optimization
```

Semantic results must remain same in all modes.

### 26.2 Test Mode

Test mode should prioritize:

```text
stable diagnostics
stable source mapping
clear stack traces
predictable assertion behavior
```

---

## 27. Validation

Fast interpreter structure validation must reject:

```text
frame without slot layout
frame without frame map
slot array size mismatch
invalid current block
instruction index out of range
block without terminator
region stack entry without RegionPlan
pending control slot missing where needed
helper call without descriptor
safepoint poll without root map
feedback slot kind mismatch
inline cache slot kind mismatch
JIT bridge without compatible metadata
source mapping missing for may-raise op where required
```

---

## 28. Compatibility

Changing any of the following invalidates RuntimePlan/EIR/JIT caches:

```text
SlotArray layout
Frame layout
RegionStack layout
InlineCache runtime payload layout
Feedback payload layout
SafepointPoller protocol
HelperBridge ABI
Interpreter/JIT bridge protocol
FrameMap runtime format
Root enumeration mode
```

These are VM-internal compatibility boundaries.

They are not public ABI.

---

## 29. Non-Goals

This document does not define:

```text
final Rust struct definitions
binary EIR encoding
computed-goto implementation
threaded-dispatch implementation
concrete GC implementation
concrete JIT backend implementation
debugger protocol
profiler format
native ABI
public bytecode
```

---

## 30. Next Work

Next Phase 3 documents should define:

```text
runtime helper implementation plan
GC implementation staging plan
JIT lowering matrix per EIR operation
fast interpreter implementation milestones
Phase 3 consistency audit

```
