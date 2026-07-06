# Phase 3 · GC Implementation Staging Plan
Document class: Implementation plan
Planning status: This document defines sequencing, milestones, tests, or implementation strategy. It does not override normative specifications.


Version: 0.13 implementation-plan draft  
Depends on: Phase 3 Runtime Helper Implementation Plan v0.12  
Depends on: Phase 3 GC Root Enumeration and Safepoint Model v0.9  
Depends on: Phase 3 Fast Interpreter Concrete Data Structures v0.11  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: heap implementation stages, bootstrap heap, root scanner, object tracing, non-moving tracing GC, barrier transition, generational GC path, moving GC constraints, GC testing matrix, implementation milestones  
Out of scope: final concrete collector algorithm, concurrent GC, concrete JIT stack-map encoding, FFI ownership protocol, public native ABI

---

## 0. Status

This document turns the GC/safepoint architecture into an implementation staging plan.

It does not require a full production GC in the first VM.

It requires that the first VM be implemented in a way that does not block:

```text
non-moving tracing GC
generational GC
moving/compacting GC
JIT stack maps
precise root enumeration
deoptimization
```

The initial implementation may use a simple handle heap.

The architecture must remain GC-upgradable.

---

## 1. Implementation Principles

### 1.1 Handle First

All heap objects must be referenced through VM handles.

The VM must not rely on raw object addresses as language identity.

### 1.2 Bootstrap Is Not Architecture

Bootstrap heap may use:

```text
typed arena
generational arena
slotmap-like storage
Rc/RefCell internally in isolated components
```

But these must not become:

```text
language semantics
public ABI
native extension contract
JIT assumption
module interface identity
```

### 1.3 No CPython-Style Refcount Architecture

The VM must not expose or depend on:

```text
per-object public refcount
copy-triggered inc/dec as semantic requirement
external ownership of object lifetime
object layout coupled to extension ABI
```

### 1.4 Precise Root Direction

The long-term architecture is precise root enumeration.

Bootstrap may conservatively scan all initialized slots, but moving GC requires precise, updateable roots.

### 1.5 Resource Cleanup Is Not GC

GC must not be used to implement language resource cleanup.

`use`, `defer`, and resource close semantics remain structured runtime semantics.

Finalizers are not part of Phase 3 resource semantics.

---

## 2. GC Stage Overview

### 2.1 Stages

```text
G0: Handle heap without collection
G1: Root scanner + trace trait
G2: Non-moving tracing GC
G3: Barrier-ready heap
G4: Generational GC
G5: Moving/compacting GC readiness
G6: Moving/compacting GC prototype
G7: Incremental/concurrent research stage
```

### 2.2 Stage Rule

Each stage must preserve compatibility with previous VM semantics.

Advancing a GC stage must not change observable language behavior except memory usage, pause behavior, and performance.

### 2.3 Required From G0

Even G0 must include:

```text
ObjectId
ObjRef
Heap API
allocation helper path
root enumeration API placeholder
trace trait placeholder
write barrier hook placeholder
safepoint metadata integration
```

---

## 3. G0 · Bootstrap Handle Heap

### 3.1 Goal

Provide a simple correct heap for initial VM execution.

### 3.2 Heap Structure

Recommended conceptual structure:

```text
Heap {
  objects: ObjectStore
  object_metadata: ObjectMetadataTable
  allocation_epoch: UInt
  gc_profile: GcProfile
}
```

### 3.3 ObjectStore Options

Allowed initial implementation options:

```text
Vec<Option<HeapObject>>
generational arena
slotmap-style storage
typed arenas per object kind
```

Recommended:

```text
generational arena or slotmap-style storage
```

because stale handles can be detected.

### 3.4 ObjRef

```text
ObjRef {
  object_id: ObjectId
}
```

In G0, ObjectId may be arena index + generation.

### 3.5 ObjectMetadata

```text
ObjectMetadata {
  object_id: ObjectId
  object_kind: ObjectKind
  marked?: Bool
  generation?: Generation
  allocation_site?: SourceSpanId
  debug_name?: String
}
```

### 3.6 G0 Required API

```text
allocate(kind, payload) -> ObjRef
get(ref) -> &HeapObject
get_mut(ref) -> &mut HeapObject
kind(ref) -> ObjectKind
is_live(ref) -> Bool
trace_object(ref, tracer)
```

### 3.7 G0 Non-Goals

G0 does not require:

```text
collection
moving
generational collection
compaction
write barrier behavior beyond no-op hook
```

### 3.8 G0 Validation

G0 must detect:

```text
stale object handle
wrong object kind access
invalid object ID
mutation of read-only object through heap API
```

---

## 4. Heap Object Model

### 4.1 HeapObject

```text
HeapObject =
  | StringObj
  | ListObj
  | MapObj
  | RecordTypeObj
  | RecordInstanceObj
  | EnumTypeObj
  | EnumValueObj
  | ReadOnlyViewObj
  | FunctionObj
  | ModuleObj
  | ErrorObj
  | ResourceObj
  | IteratorObj
  | HostObjectWrapper
```

### 4.2 Object Payload Rule

Object payloads may contain:

```text
Value
ObjRef
BindingCellRef
ModuleId
ShapeId
TypeId
SourceSpanId
runtime metadata
```

Any payload field that can reach heap objects must be traced.

### 4.3 No Raw Pointers

Heap object payloads must not store raw pointers to other heap objects.

If host resources require native handles, they must be wrapped in resource/host objects and not be interpreted as VM object references.

### 4.4 ReadOnly Flag

Mutable aggregate objects should carry read-only state or readonly view should enforce mutation rejection.

Write barriers still apply where mutation is allowed.

---

## 5. G1 · Root Scanner and Trace Trait

### 5.1 Goal

Introduce root enumeration and object tracing before implementing collection.

### 5.2 Trace Trait

Conceptual Rust-like API:

```rust
trait Trace {
    fn trace(&self, tracer: &mut Tracer);
}
```

### 5.3 Tracer

```text
Tracer {
  visit_obj_ref(ref: ObjRef)
  visit_value(value: Value)
  visit_cell(cell: BindingCellRef)
}
```

### 5.4 Object Trace Coverage

Trace implementations required for:

```text
ListObj
MapObj
RecordInstanceObj
EnumValueObj
ReadOnlyViewObj
FunctionObj
ModuleObj
ErrorObj
ResourceObj
IteratorObj
HostObjectWrapper
```

`StringObj` may contain no heap refs.

`RecordTypeObj` and `EnumTypeObj` trace method tables or metadata if heap-backed.

### 5.5 Root Scanner

```text
RootScanner {
  scan_call_stack
  scan_slots
  scan_cells
  scan_modules
  scan_constants
  scan_regions
  scan_pending_control
  scan_helpers
  scan_host_roots
  scan_jit_frames
}
```

### 5.6 G1 Validation

With collection still disabled, G1 can validate:

```text
all allocated object refs reachable from known roots in simple programs
trace coverage exists for every object kind
root scanner sees pending control values
root scanner sees defer/resource values
root scanner sees helper arguments
```

This stage should include debug assertions but no reclamation yet.

---

## 6. G2 · Non-Moving Tracing GC

### 6.1 Goal

Implement first real collector without moving objects.

### 6.2 Algorithm

Recommended:

```text
stop-the-world mark-sweep over handle heap
```

Other non-moving tracing strategies are allowed.

### 6.3 Collection Steps

```text
reach safepoint
enumerate roots
mark reachable objects
trace marked objects transitively
sweep unmarked objects
reclaim unreachable slots
update allocation metadata
resume execution
```

### 6.4 Safepoint Requirement

G2 collection may run only at safepoints.

Required initial safepoints:

```text
allocation
loop backedge
function call
helper call
module import
host call
```

### 6.5 Sweep Rule

Sweeping must not invalidate live ObjRef handles.

Unreachable handles become stale.

Stale handle access is VM bug unless held only by unreachable memory.

### 6.6 Finalization Rule

No language finalizers.

Resource cleanup remains structured.

GC reclaiming ResourceObj must not be required to close external resource.

### 6.7 G2 Testing

Tests must verify:

```text
unreachable list reclaimed
cycles reclaimed
module roots preserved
closure captures preserved
pending raise error preserved
defer callable preserved
resource object preserved during use
helper arg preserved across allocating helper
```

---

## 7. G3 · Barrier-Ready Heap

### 7.1 Goal

Convert all mutation paths to explicit barrier calls.

### 7.2 Barrier Sites

Must cover:

```text
record field write
list element write
map key/value write
capture cell write
module slot write
error suppressed list write
resource state write
runtime internal heap ref write
```

### 7.3 Barrier Function

```text
write_barrier(owner, value, write_kind)
```

Initial G3 may still no-op.

The purpose is to ensure all mutation routes are structurally hooked.

### 7.4 Mutation API Centralization

All mutation must go through Heap/Cell/Slot APIs.

No direct payload mutation that bypasses barrier is allowed.

### 7.5 G3 Validation

Debug mode should count barrier events and assert expected barrier calls for mutation tests.

---

## 8. G4 · Generational GC

### 8.1 Goal

Introduce young/old generations.

### 8.2 Required Structures

```text
Generation
RememberedSet
CardTable or equivalent
MinorCollectionPlan
PromotionPolicy
```

### 8.3 Write Barrier Role

When old object stores reference to young object, barrier records remembered edge.

### 8.4 Minor Collection

Minor collection scans:

```text
young roots
remembered set
stack roots
module roots
host roots
```

### 8.5 Promotion

Objects surviving threshold may promote to old generation.

Promotion must not change language identity.

### 8.6 G4 Non-Goal

No concurrent/incremental GC required.

### 8.7 G4 Testing

Tests must verify:

```text
old-to-young record field write preserved
old-to-young list write preserved
old-to-young map write preserved
old module cell -> young value preserved
remembered set updates
promotion preserves identity
minor collection does not collect reachable young object
```

---

## 9. G5 · Moving GC Readiness

### 9.1 Goal

Prepare codebase for object movement without yet moving objects.

### 9.2 Required Refactoring

All object access must pass through handle resolution.

No subsystem may assume:

```text
ObjRef is raw pointer
object address stable across safepoint
JIT can keep raw object pointer live across safepoint
host can hold raw VM object pointer
```

### 9.3 Root Update API

Introduce conceptual API:

```text
update_root(location, new_ref)
```

### 9.4 Object Relocation Metadata

Heap object metadata must support:

```text
forwarding state
new location
relocation epoch
```

### 9.5 JIT Constraint

Compiled code must use stack maps at safepoints.

Raw derived pointers across safepoints are forbidden unless base object and derived relation are recorded.

### 9.6 G5 Testing

Use simulated relocation:

```text
change object backing storage
update handles/root locations
verify identity/equality semantics
verify no stale object access
```

---

## 10. G6 · Moving/Compacting GC Prototype

### 10.1 Goal

Implement first moving or compacting collector prototype.

### 10.2 Allowed Strategies

```text
copying young generation
compact old generation
handle-table relocation
semi-space prototype
```

### 10.3 Recommended First Moving Strategy

Recommended first moving path:

```text
handle table relocation
```

Reason:

```text
keeps ObjRef stable as handle
reduces immediate JIT pointer complexity
allows compaction behind handle table
```

### 10.4 Direct Reference Strategy

If VM later uses direct tagged references, it must implement precise root update and stack map relocation.

### 10.5 Relocation Steps

```text
stop at safepoint
enumerate roots
copy/compact objects
update handle table or roots
update heap object references
update JIT/interpreter roots
resume execution
```

### 10.6 G6 Testing

Tests must verify:

```text
identity preserved across movement
record/list/map references updated
closure captures updated
module exports updated
pending control error updated
defer/resource roots updated
host roots updated
JIT stack maps updated if JIT enabled
```

---

## 11. G7 · Incremental/Concurrent Research Stage

### 11.1 Status

Deferred beyond Phase 3.

### 11.2 Requirements Before G7

Need mature:

```text
write barriers
safepoints or handshakes
root snapshot protocol
mutator barrier protocol
threading model
host boundary protocol
JIT stack map protocol
```

### 11.3 Phase 3 Position

Phase 3 must not assume incremental or concurrent GC.

It must not block them.

---

## 12. Root Scanner Implementation Plan

### 12.1 RootScanner API

Conceptual:

```rust
struct RootScanner;

impl RootScanner {
    fn scan_vm(&mut self, vm: &mut Vm, tracer: &mut Tracer);
    fn scan_frame(&mut self, frame: &mut InterpreterFrame, tracer: &mut Tracer);
    fn scan_jit_frame(&mut self, frame: &mut CompiledFrame, tracer: &mut Tracer);
    fn scan_module(&mut self, module: &mut ModuleObj, tracer: &mut Tracer);
    fn scan_host_roots(&mut self, host: &mut HostRootRegistry, tracer: &mut Tracer);
}
```

### 12.2 Mutable Root Visiting

Moving GC needs mutable root visiting.

Conceptual:

```rust
fn visit_root(&mut self, root: &mut Value);
```

or location-based handle update.

### 12.3 Bootstrap Mode

Bootstrap root scanner may visit:

```text
all initialized slots
all module slots
all constants
all region entries
all helper args
all host roots
```

### 12.4 Production Mode

Production root scanner should use:

```text
liveness-derived RootMap
FrameMap
StackMap
SafepointRecord
```

---

## 13. Object Trace Implementation Plan

### 13.1 Trace Coverage Table

```text
ObjectKind             Trace fields
StringObj              none
ListObj                elements
MapObj                 keys, values
RecordTypeObj          method table if heap-backed
RecordInstanceObj      fields
EnumTypeObj            case metadata if heap-backed
EnumValueObj           payload
ReadOnlyViewObj        target
FunctionObj            captures, module
ModuleObj              slots, exports, initialization_error
ErrorObj               message, details, stack_trace, suppressed
ResourceObj            close_callable, metadata values
IteratorObj            iterable, current state
HostObjectWrapper      registered VM handles only
```

### 13.2 Trace Validation

Debug build should assert every ObjectKind has trace implementation.

### 13.3 Trace No-Effect Rule

Trace must not execute language code.

Trace must not trigger capability effects.

Trace must not call user-defined finalizers.

---

## 14. Allocation Implementation Plan

### 14.1 Allocation API

```text
heap.allocate(kind, payload, ctx) -> ObjRef
```

### 14.2 Allocation Context

```text
AllocationContext {
  source_span?: SourceSpanId
  object_kind: ObjectKind
  may_collect: Bool
  zero_initialize?: Bool
}
```

### 14.3 Allocation Path

```text
if allocation threshold reached and may_collect:
  require safepoint/root map
  collect
allocate object slot
initialize object metadata
store payload
return handle
```

### 14.4 Partial Initialization

Use temporary unexposed allocation state if initialization can fail.

Object becomes root-visible only after initialized or is tracked by allocation context.

---

## 15. Write Barrier Implementation Plan

### 15.1 Barrier API

```text
heap.write_barrier(owner, written_value, write_kind)
```

### 15.2 Owner Kinds

```text
ObjectOwner
CellOwner
ModuleOwner
RuntimeOwner
```

### 15.3 G3 No-Op Barrier

G3 barrier implementation:

```text
increment debug counter
record optional trace event
return
```

### 15.4 G4 Generational Barrier

G4 barrier implementation:

```text
if owner is old and written value is young:
  remembered_set.insert(owner)
```

### 15.5 Incremental Future

Incremental GC later may use pre-write or post-write barriers.

Phase 3 must keep barrier call sites explicit enough to evolve.

---

## 16. Safepoint Implementation Plan

### 16.1 Safepoint Poll

```text
poll_safepoint(safepoint_id, vm, frame)
```

### 16.2 Poll Flow

```text
load SafepointRecord
construct or retrieve RootMap
if gc_requested:
  run GC
if interrupt_requested:
  deliver interrupt
if profiling_requested:
  record sample
if debug_trap:
  enter debug trap
```

### 16.3 Root Map Availability

If safepoint has no valid root map, polling for GC must be disabled or validation must fail.

### 16.4 Loop Backedge

Loop backedge safepoints should use fast poll check:

```text
if poll_epoch changed:
  slow_poll()
```

### 16.5 Helper Safepoints

Before helper call that may allocate/collect:

```text
enter HelperCallState
make roots visible
poll/collect if required
call helper
```

---

## 17. Host Root Implementation Plan

### 17.1 HostRootRegistry

```text
HostRootRegistry {
  roots: Map<HostRootId, HostRootEntry>
}
```

### 17.2 HostRootEntry

```text
HostRootEntry {
  value: Value
  capability?: CapabilityId
  owner: HostBoundaryId
  lifetime: HostRootLifetime
}
```

### 17.3 HostRootLifetime

```text
HostRootLifetime =
  | CallScoped
  | ResourceScoped
  | ExplicitHandle
```

### 17.4 Host Root Rule

Host code must not retain VM values beyond call without HostRoot registration.

---

## 18. JIT Integration Staging

### 18.1 Before JIT

Interpreter root scanning is sufficient.

### 18.2 Baseline JIT Entry

Before baseline JIT can be enabled, required:

```text
StackMapTable
JitFrameRoot
JIT safepoint records
helper call root maps
compiled frame registration
```

### 18.3 Moving GC with JIT

Moving GC with JIT requires:

```text
updateable register/stack roots
precise stack maps
no raw object pointer across safepoint
compiled frame relocation protocol
```

### 18.4 JIT Feature Gating

If stack maps are incomplete, JIT must be disabled under moving GC profile.

---

## 19. GC Testing Matrix

### 19.1 G0 Tests

```text
allocate each object kind
stale handle detection
wrong kind access rejection
readonly mutation rejection through heap API
```

### 19.2 G1 Tests

```text
trace coverage for all object kinds
root scanner sees slots
root scanner sees module cells
root scanner sees captures
root scanner sees region stack
root scanner sees pending control
root scanner sees helper args
```

### 19.3 G2 Tests

```text
unreachable object reclaimed
cycle reclaimed
live object preserved
module root preserved
closure capture preserved
error/suppressed error preserved
resource/defer preserved
```

### 19.4 G3 Tests

```text
barrier invoked on record write
barrier invoked on list write
barrier invoked on map write
barrier invoked on cell write
barrier invoked on module write
```

### 19.5 G4 Tests

```text
old-to-young remembered edge
minor collection preserves remembered young
promotion preserves identity
module old-to-young edge preserved
```

### 19.6 G5/G6 Tests

```text
simulated relocation
handle update
slot root update
cell root update
module root update
region root update
host root update
compiled stack map update if JIT enabled
```

### 19.7 Negative Tests

```text
missing trace implementation rejected
safepoint without root map rejected
may_collect helper without roots rejected
heap mutation without barrier rejected
host retained value without HostRoot rejected
JIT under moving GC without stack maps rejected
```

---

## 20. GC Milestones

### 20.1 Milestone G0 · Handle Heap

Deliver:

```text
ObjectId
ObjRef
HeapObject enum
Heap API
ObjectMetadata
basic allocation
stale handle detection
```

### 20.2 Milestone G1 · Trace and Roots

Deliver:

```text
Trace trait
Tracer
RootScanner
root categories
trace coverage table
debug root validation
```

### 20.3 Milestone G2 · Non-Moving GC

Deliver:

```text
stop-the-world mark-sweep
safepoint-triggered collection
cycle reclamation
sweep integration
GC diagnostics
```

### 20.4 Milestone G3 · Barrier Infrastructure

Deliver:

```text
write barrier API
barrier call sites
debug barrier counters
mutation API centralization
```

### 20.5 Milestone G4 · Generational Prototype

Deliver:

```text
young/old generation tags
remembered set
minor collection
promotion policy
generational tests
```

### 20.6 Milestone G5 · Moving Readiness

Deliver:

```text
root update API
simulated relocation
handle table relocation support
no raw pointer audit
JIT moving-GC feature gates
```

### 20.7 Milestone G6 · Moving Prototype

Deliver:

```text
copy/compact prototype
root updates
heap reference updates
identity preservation tests
```

### 20.8 Milestone G7 · Incremental Research

Deferred.

---

## 21. Implementation Guardrails

The GC implementation must not:

```text
use finalizers for language resource cleanup
expose object addresses
expose object headers
make refcount observable
allow host raw VM object pointers
allow JIT raw object pointers across safepoints
bypass write barrier on heap mutation
collect at non-safepoint
run user code during trace
perform capability effects during GC
```

---

## 22. Validation

GC implementation validation must reject:

```text
object kind without trace implementation
safepoint without root map when GC can run
helper may_collect without roots-visible descriptor
mutation path without barrier hook
host retained VM value without HostRoot
JIT compiled frame without stack map under collecting profile
moving GC profile with non-updateable roots
resource cleanup depending on GC finalization
```

---

## 23. Compatibility

Changing any of the following invalidates RuntimePlan/EIR/JIT caches:

```text
ObjectId representation
ObjRef representation
Value heap-ref layout
HeapObject layout profile
Trace schema
RootMap schema
FrameMap schema
StackMap schema
WriteBarrier policy
GC profile
Allocation protocol
HostRoot protocol
```

These are VM-internal compatibility boundaries.

They are not public ABI.

---

## 24. Non-Goals

This document does not define:

```text
final collector implementation
concurrent GC
incremental GC
specific compacting algorithm
specific card table representation
specific stack map binary encoding
native extension ownership model
FFI
debugger protocol
profiler format
```

---

## 25. Next Work

Next Phase 3 documents should define:

```text
JIT lowering matrix per EIR operation
fast interpreter implementation milestones
Phase 3 consistency audit
Phase 3 freeze-readiness checklist

```
