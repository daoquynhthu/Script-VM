# Phase 3 · GC Root Enumeration and Safepoint Model
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.9 GC/safepoint draft  
Depends on: Phase 3 Runtime Helper Contracts v0.8  
Depends on: Phase 3 Structured Control and Unwinding Lowering Round 2 v0.7  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: GC root enumeration, safepoint schema, frame maps, root maps, moving-GC compatibility, write barrier placement, allocation protocol, JIT/interpreter root visibility, GC validation  
Out of scope: concrete garbage collector algorithm, concrete moving collector, concrete generational collector, concrete Cranelift lowering, concrete LLVM lowering, public native ABI

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

This document defines the VM's GC and safepoint architecture.

It does not require a full GC implementation in the first VM.

It requires that the VM architecture does not block:

```text
tracing GC
generational GC
moving GC
compacting GC
incremental GC
JIT stack walking
deoptimization
safe interruption
profiling
```

The first VM may use a simple heap, arena, or non-moving tracing prototype.

But it must preserve the root/safepoint/write-barrier structure defined here.

---

## 1. Core Position

The VM must not be architecturally tied to CPython-style reference counting.

Allowed as bootstrap detail:

```text
Rc
Arc
RefCell
arena allocation
generational arena
slotmap-like heap
```

Forbidden as architecture:

```text
language-visible reference counts
public object header layout
external native access to raw object addresses
JIT assumptions based on non-moving raw pointers
object identity as exposed machine address
native extension ownership of VM object lifetime
```

The VM object model is handle-based.

Object movement must remain possible.

---

## 2. GC Architecture Tiers

The VM supports staged memory-management implementation.

### 2.1 Tier G0 · Bootstrap Heap

Permitted:

```text
typed arena
generational arena
slotmap-like heap
Rc/RefCell inside isolated bootstrap implementation
```

Required even in G0:

```text
ObjectId abstraction
Heap handle abstraction
allocation hook
root enumeration interface
write barrier hook position
safepoint records
```

### 2.2 Tier G1 · Non-Moving Tracing GC

Adds:

```text
mark phase
root scanning
object tracing
sweep or reclaim
cycle collection
```

### 2.3 Tier G2 · Generational GC

Adds:

```text
young/old generations
remembered sets
write barriers
minor collection
promotion
```

### 2.4 Tier G3 · Moving / Compacting GC

Adds:

```text
object relocation
handle update
slot/root update
frame map update
JIT stack map relocation
forwarding
compaction
```

### 2.5 Tier G4 · Incremental / Concurrent GC

Deferred.

Requires:

```text
precise barriers
safepoint or handshake protocol
mutator cooperation
concurrent root handling
```

Phase 3 does not implement G4.

---

## 3. Object Reference Model

### 3.1 Object Handle

Runtime values must refer to heap objects through VM-managed handles.

Conceptual:

```text
ObjRef {
  object_id: ObjectId
  generation?: Generation
  type_tag?: ObjectKind
}
```

### 3.2 ObjectId

`ObjectId` is VM-internal.

It may be:

```text
arena index
generational index
handle table index
compressed reference
tagged reference
```

It must not be:

```text
public pointer
native ABI value
stable serialization identity
module interface identity
```

### 3.3 Moving-GC Compatibility

All heap object references in roots must be discoverable and updateable.

Root locations must be represented by:

```text
slot reference
frame map entry
module cell reference
capture cell reference
region stack entry
pending control entry
host root handle
JIT stack map entry
```

not by hidden raw pointers.

### 3.4 Identity Semantics

Language identity is based on VM object identity, not exposed address.

If a moving GC changes physical address, language `is` result must not change.

---

## 4. Root Set

### 4.1 Root Set Definition

The root set is every live reference that may reach a heap object at a safepoint.

Root categories:

```text
slot roots
capture roots
module roots
constant roots
region stack roots
pending control roots
error roots
suppressed error roots
defer roots
resource roots
iterator roots
temporary roots
helper argument roots
host roots
JIT frame roots
```

### 4.2 RootSet

```text
RootSet {
  roots: List<RootRef>
}
```

### 4.3 RootRef

```text
RootRef =
  | SlotRoot
  | CellRoot
  | ModuleRoot
  | ConstantRoot
  | RegionRoot
  | PendingControlRoot
  | ErrorRoot
  | DeferRoot
  | ResourceRoot
  | IteratorRoot
  | HelperArgRoot
  | HostRoot
  | JitFrameRoot
```

### 4.4 Root Precision

The VM should prefer precise roots.

Conservative scanning may be used only as bootstrap implementation detail and must not become JIT/GC architecture.

Precise root maps are required for moving GC.

---

## 5. Slot Roots

### 5.1 SlotRoot

```text
SlotRoot {
  frame_id: FrameId
  slot_id: SlotId
  value_kind_hint?: RuntimeValueKind
}
```

### 5.2 Slot Root Rule

At safepoint, all live slots containing heap references must be reported.

Slots containing only immediates may be omitted if the value layout makes that distinction reliable.

### 5.3 Slot Liveness

Slot liveness may come from:

```text
EIR liveness analysis
FrameMap
interpreter slot state
debug mode all-slots strategy
```

The bootstrap VM may conservatively report all initialized slots.

JIT must eventually report precise slot roots.

---

## 6. Cell and Capture Roots

### 6.1 CellRoot

```text
CellRoot {
  cell_id: CellId
  owner: CellOwner
}
```

### 6.2 CellOwner

```text
CellOwner =
  | LocalCapture
  | ModuleBinding
  | ExportBinding
  | ClosureCapture
  | RuntimeInternal
```

### 6.3 Capture Root Rule

Captured bindings that may outlive a frame must be heap-traced.

A closure object must trace its capture cells.

A capture cell must trace its current value if initialized.

### 6.4 Cell Relocation

If moving GC relocates a value stored in a cell, the cell must be updated.

---

## 7. Module Roots

### 7.1 ModuleRoot

```text
ModuleRoot {
  module_id: ModuleId
  module_object: ObjRef
}
```

### 7.2 Module Environment Roots

Module roots include:

```text
module object
module slot table
export table cells
initialization error
imported module references
module-level constants
```

### 7.3 Module State

Module state itself may be immediate.

But any initialization error or export value must be traced.

### 7.4 Import Cycle Roots

During module initialization, partially initialized modules are roots.

Circular import state must remain visible at safepoints.

---

## 8. Constant Roots

### 8.1 ConstantRoot

```text
ConstantRoot {
  constant_id: ConstantId
  owning_eir_module: EirModuleId
}
```

### 8.2 Constant Pool

Heap-backed constants are roots while their EIR module or module instance is live.

Examples:

```text
string constants
function refs
record type refs
enum type refs
module refs
```

### 8.3 Interned Objects

Intern tables are host/VM roots.

Interned strings or symbols must be traced if heap-backed.

---

## 9. Region Stack Roots

### 9.1 RegionRoot

```text
RegionRoot {
  frame_id: FrameId
  region_id: ControlRegionId
  root_kind: RegionRootKind
}
```

### 9.2 RegionRootKind

```text
RegionRootKind =
  | DeferStack
  | ResourceStack
  | FinallyState
  | PatternState
  | MatchSubject
  | IteratorState
  | CleanupState
```

### 9.3 Region Stack Trace Rule

Region stack must trace:

```text
registered defer callables
resource handles
pending finally values
match subjects
iterator state
cleanup errors
suppressed errors
```

### 9.4 JIT Requirement

JIT frames that own active region state must expose it through frame maps or deopt metadata.

---

## 10. Pending Control Roots

### 10.1 PendingControlRoot

```text
PendingControlRoot {
  frame_id: FrameId
  pending_control_slot: SlotId
}
```

### 10.2 Pending Control Values

Pending control may contain:

```text
return value
raise error
break target
continue target
suppressed errors
```

Return values and errors must be traced.

Break/continue target IDs are immediate metadata.

### 10.3 Unwind Root Rule

During unwinding, pending control is always a root.

---

## 11. Error and Suppressed Error Roots

### 11.1 ErrorRoot

```text
ErrorRoot {
  error_object: ObjRef
  role: ErrorRootRole
}
```

### 11.2 ErrorRootRole

```text
ErrorRootRole =
  | ActiveRaise
  | InitializationError
  | SuppressedError
  | DiagnosticError
  | HelperError
```

### 11.3 Error Object Trace

Error objects trace:

```text
message
details map
source span metadata if heap-backed
stack trace
suppressed error list
cause/context if supported
```

### 11.4 Suppressed Error Rule

Suppressed errors attached during cleanup must remain reachable through primary error or diagnostics.

---

## 12. Defer and Resource Roots

### 12.1 DeferRoot

```text
DeferRoot {
  frame_id: FrameId
  region_id: ControlRegionId
  defer_index: UInt
}
```

A defer root traces the callable and captured environment.

### 12.2 ResourceRoot

```text
ResourceRoot {
  frame_id: FrameId
  region_id: ControlRegionId
  resource_index: UInt
}
```

A resource root traces:

```text
resource object
close callable
capability origin metadata
host resource handle wrapper
```

### 12.3 Close In Progress

A resource in `Closing` state remains a root until close completes.

---

## 13. Iterator and Pattern Roots

### 13.1 IteratorRoot

```text
IteratorRoot {
  frame_id: FrameId
  iterator_slot: SlotId
}
```

For loops must trace iterator state.

### 13.2 Pattern Temporary Roots

Pattern lowering may allocate temporary slots for:

```text
subject
record field values
enum payload values
list elements
map lookup values
or-pattern candidate bindings
```

These temporaries must be roots if live at safepoint.

### 13.3 Match Subject

Match subject is a root across pattern tests and guard evaluation.

---

## 14. Helper Argument Roots

### 14.1 HelperArgRoot

```text
HelperArgRoot {
  helper_id: RuntimeHelperId
  argument_index: UInt
}
```

### 14.2 Helper Call Rule

Before calling a helper that may allocate, collect, or move objects, all helper arguments that contain heap refs must be visible.

### 14.3 Helper Internal Roots

Helpers may maintain internal roots.

They must register them with VM root API if they can trigger allocation/collection while holding them.

---

## 15. Host Roots

### 15.1 HostRoot

```text
HostRoot {
  host_root_id: HostRootId
  capability?: CapabilityId
  description?: String
}
```

### 15.2 Host Boundary Rule

Host roots are VM-controlled.

Host code must not keep raw VM object pointers without registering roots.

### 15.3 FFI Constraint

Future FFI must use registered handles, not raw pointers.

Host roots are part of capability boundary.

### 15.4 Host Root Lifetime

Host root lifetime must be explicit:

```text
enter host call
register host root
use handle
unregister host root
exit host call
```

Leaked host roots are resource leaks and must be diagnosable.

---

## 16. JIT Frame Roots

### 16.1 JitFrameRoot

```text
JitFrameRoot {
  compiled_function_id: CompiledFunctionId
  return_address_offset: CodeOffset
  stack_map_id: StackMapId
}
```

### 16.2 StackMap

```text
StackMap {
  stack_map_id: StackMapId
  live_value_locations: List<ValueLocation>
  frame_state: FrameStateRef
  source_span?: SourceSpanId
}
```

### 16.3 ValueLocation

```text
ValueLocation =
  | StackSlot
  | Register
  | SpillSlot
  | Constant
  | Immediate
  | DerivedPointer
  | RuntimeHandle
```

### 16.4 JIT Stack Walking

At safepoint, compiled frames must provide:

```text
return address
compiled function identity
stack map
live roots
frame reconstruction metadata
deopt metadata if needed
```

### 16.5 Derived Pointer Rule

If JIT uses derived pointers, base object must be kept live and reconstructable.

Bootstrap baseline JIT should avoid derived pointers where possible.

---

## 17. Safepoint Model

### 17.1 Safepoint

A safepoint is a VM-observable execution location.

At a safepoint, VM may:

```text
run GC
walk stack
collect profile sample
process interrupt
perform deoptimization
handle cancellation
validate runtime state in debug mode
```

### 17.2 SafepointRecord

```text
SafepointRecord {
  safepoint_id: SafepointId
  kind: SafepointKind
  owner: SafepointOwner
  location: SafepointLocation
  root_map: RootMapId
  frame_map?: FrameMapId
  deopt_id?: DeoptId
  source_span?: SourceSpanId
}
```

### 17.3 SafepointOwner

```text
SafepointOwner =
  | Interpreter
  | EirFunction
  | RuntimeHelper
  | JitCompiledFunction
  | HostCall
```

### 17.4 SafepointLocation

```text
SafepointLocation =
  | EirLocation
  | HelperCallSite
  | MachineCodeOffset
  | HostBoundary
  | ModuleImportBoundary
```

### 17.5 SafepointKind

```text
SafepointKind =
  | FunctionCall
  | LoopBackedge
  | Allocation
  | HostCall
  | HelperCall
  | RaiseBoundary
  | ImportBoundary
  | DeoptExit
  | DebugPoll
```

### 17.6 Required Safepoint Candidates

The VM must be able to place safepoints at:

```text
function call boundary
loop backedge
allocation point
runtime helper call if helper may allocate/raise/collect
host call boundary
module import boundary
raise boundary
deopt side exit
long-running builtin boundary
```

---

## 18. RootMap

### 18.1 RootMap

```text
RootMap {
  root_map_id: RootMapId
  roots: List<RootLocation>
}
```

### 18.2 RootLocation

```text
RootLocation =
  | SlotRootLocation
  | CellRootLocation
  | ModuleRootLocation
  | ConstantRootLocation
  | RegionRootLocation
  | PendingControlRootLocation
  | ErrorRootLocation
  | HelperArgRootLocation
  | HostRootLocation
  | JitRootLocation
```

### 18.3 Interpreter RootMap

For interpreter frames, root map may be computed from:

```text
current EIR function
slot layout
live slot set
region stack
pending control
helper call state
```

### 18.4 JIT RootMap

For JIT frames, root map must be explicit.

It must not rely on conservative scanning if moving GC is enabled.

### 18.5 Debug RootMap

Debug mode may report more roots than strictly necessary.

This is acceptable for non-moving GC but may reduce reclamation.

Moving GC still requires updateable locations.

---

## 19. FrameMap

### 19.1 FrameMap

```text
FrameMap {
  frame_map_id: FrameMapId
  function_id?: FunctionId
  eir_function_id: EirFunctionId
  module_id: ModuleId
  slot_layout: SlotLayoutRef
  visible_bindings: List<VisibleBinding>
  region_state: RegionStateRef
  source_span?: SourceSpanId
}
```

### 19.2 VisibleBinding

```text
VisibleBinding {
  binding_id: BindingId
  slot_id: SlotId
  visibility: BindingVisibility
  value_kind_hint?: RuntimeValueKind
}
```

### 19.3 FrameMap Purpose

FrameMap supports:

```text
stack trace
debug inspection
deoptimization
GC root enumeration
error diagnostics
```

### 19.4 FrameMap and Deopt

Deopt metadata may reference FrameMap.

FrameMap must be sufficient to reconstruct source-visible frame state.

---

## 20. Allocation Protocol

### 20.1 Allocation Operation

All heap allocation goes through VM allocation API or helper.

Allocation API must:

```text
check heap state
make roots visible if needed
possibly run GC
allocate object
initialize object safely
return ObjRef handle
```

### 20.2 Allocation Safety

An object must not become visible before required fields are initialized.

If initialization can raise, partially initialized object must not escape.

### 20.3 Allocation Safepoint

Any allocation may be a safepoint if GC profile allows collection on allocation.

### 20.4 Large Object Allocation

Large objects may use separate allocation path.

Still must be handle-based and root-visible.

---

## 21. Object Tracing

### 21.1 Trace Trait Concept

Each heap object kind must define trace behavior.

Conceptual:

```text
trace(object, tracer)
```

### 21.2 Object Kinds to Trace

Required object trace coverage:

```text
StringObj
ListObj
MapObj
RecordTypeObj
RecordInstanceObj
EnumTypeObj
EnumValueObj
ReadOnlyViewObj
FunctionObj
ModuleObj
ErrorObj
ResourceObj
IteratorObj
HostObjectWrapper
```

### 21.3 Trace Fields

Examples:

```text
ListObj -> elements
MapObj -> keys and values if heap-backed
RecordInstanceObj -> fields
EnumValueObj -> payload
FunctionObj -> captures, module
ModuleObj -> slots, exports, initialization error
ErrorObj -> details, suppressed errors, stack trace
ResourceObj -> close callable, wrapper metadata
```

### 21.4 Trace Invariant

Every heap reference stored inside a heap object must be traced.

Violation is VM memory safety bug.

---

## 22. Moving GC Update Protocol

### 22.1 Relocation

When object moves, VM updates all roots and heap references.

Required update locations:

```text
slots
cells
module slots
constant pools
region stack entries
pending control
error objects
defer stacks
resource stacks
helper arguments
host roots
JIT stack maps
heap object fields
```

### 22.2 Handle Table Option

A handle-table design may avoid direct root rewriting by updating handle entries.

If used, handle lookup cost must be considered in performance architecture.

### 22.3 Direct Reference Option

A direct tagged-reference design requires precise root updates.

### 22.4 JIT Constraint

JIT code must not keep raw object addresses across safepoints unless root map can update them.

The safe default is:

```text
no raw object pointer live across safepoint
```

---

## 23. Write Barrier Model

### 23.1 Write Barrier Purpose

Write barrier supports future generational/incremental GC.

### 23.2 Barrier Sites

All heap reference writes must pass barrier hook:

```text
record field write
list element write
map key/value write
capture cell write
module slot write
object header reference write if any
resource close callable write
error suppressed list write
```

### 23.3 WriteBarrierRecord

```text
WriteBarrierRecord {
  owner_object?: ObjRef
  owner_slot?: SlotId
  written_value: Value
  write_kind: WriteKind
}
```

### 23.4 WriteKind

```text
WriteKind =
  | RecordField
  | ListElement
  | MapEntry
  | CaptureCell
  | ModuleSlot
  | ErrorSuppressed
  | ResourceState
  | RuntimeInternal
```

### 23.5 Bootstrap Barrier

In bootstrap VM, barrier may be no-op.

But mutation APIs must keep explicit barrier call site.

### 23.6 JIT Barrier Rule

JIT code that performs heap mutation must emit barrier or call helper.

It must not silently bypass barrier path.

---

## 24. Safepoint Polling

### 24.1 Poll Sites

Polling may occur at:

```text
loop backedge
function call
helper call
allocation
host call
module import
long-running builtin
```

### 24.2 Poll Actions

Poll action may include:

```text
GC request
interrupt
cancellation
profiling sample
debug trap
deopt request
```

### 24.3 Poll Overhead

Not every EIR operation should poll.

Poll placement must balance performance with responsiveness.

Loop backedges and calls are primary poll points.

---

## 25. Interpreter Root Enumeration

### 25.1 Interpreter Frames

Interpreter frames already store slots and region stack in VM-managed structures.

Root enumeration can directly traverse:

```text
CallStack
Frame.slots
Frame.region_stack
PendingControl
Helper call state
Module environment
```

### 25.2 Interpreter Safepoint

At interpreter safepoint:

1. stop at known EIR location
2. identify current frame
3. enumerate live slots
4. enumerate region stack
5. enumerate pending control
6. enumerate helper args if inside helper boundary
7. expose roots to GC

### 25.3 Bootstrap Strategy

Bootstrap interpreter may enumerate all initialized slots.

Later interpreter may use liveness-derived root maps.

---

## 26. JIT Root Enumeration

### 26.1 Compiled Frame Requirement

Compiled frames must expose stack maps.

Stack maps must describe every live heap reference.

### 26.2 Return Address Mapping

JIT stack walker uses return address or code offset to find:

```text
compiled function
safepoint record
stack map
frame map
deopt metadata
```

### 26.3 Register Roots

If live roots are in registers at safepoint, stack map must identify them.

### 26.4 Spill Roots

If live roots are spilled, stack map must identify stack slots.

### 26.5 JIT and Moving GC

Moving GC requires JIT roots to be updateable.

The JIT backend must cooperate with VM relocation protocol.

---

## 27. Host Boundary Roots

### 27.1 Host Calls

Before entering host call that may retain values, VM must register host roots.

### 27.2 Host Return

Host return values must be converted to VM values through capability-controlled boundary.

### 27.3 Host Exceptions

Host exception objects must not leak directly.

They must be converted to VM Error objects.

### 27.4 Host Root Validation

Host roots must be unregistered after use unless explicitly transferred to a longer-lived host handle.

Leaks must be diagnosable.

---

## 28. GC and Capability Boundary

GC must not invoke arbitrary capability effects.

Finalization, if later introduced, must not perform ambient authority operations.

Resource cleanup remains structured through `use`/`defer`/host resource protocols, not GC finalizers.

Phase 3 should not rely on finalizers for language resource semantics.

---

## 29. GC Validation

GC/safepoint validation must reject:

```text
safepoint without root map
JIT safepoint without stack map
helper may_collect without roots-visible contract
allocation path without safepoint possibility under collecting profile
heap mutation without barrier hook
object kind without trace implementation
root location not updateable under moving GC profile
host call retaining value without host root
deopt point without frame map where required
region stack root not represented
pending control root missing during unwind
```

---

## 30. Cache Compatibility

Changing any of the following invalidates RuntimePlan/EIR/JIT caches:

```text
value layout profile
heap profile
GC profile
root map schema
safepoint schema
frame map schema
write barrier policy
helper GC behavior
JIT stack map format
object trace layout
```

The cache key must include GC-relevant target profile data.

---

## 31. Non-Goals

This document does not define:

```text
specific GC algorithm
mark/sweep implementation
generational collector implementation
moving collector implementation
incremental collector implementation
exact register allocation
Cranelift stack map encoding
LLVM stack map encoding
native ABI
FFI ownership protocol
debugger protocol
profiler format
```

---

## 32. Next Work

Next Phase 3 documents should define:

```text
baseline JIT backend interface
fast interpreter concrete data structures
runtime helper implementation plan
GC implementation staging plan
Phase 3 consistency audit

```
