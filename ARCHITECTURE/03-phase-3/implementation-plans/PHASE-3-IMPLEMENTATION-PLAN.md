# Phase 3 · Implementation Plan Aggregate

Document class: Implementation plan aggregate  
Planning status: This aggregate includes only non-normative implementation planning documents. It does not override normative specifications.

Updated: 2026-06-29 09:33:23

## Boundary

This file aggregates implementation plans only.

If this file conflicts with `PHASE-3-VM-SPEC.md`, the normative specification wins.

## Included Implementation Plans

- `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md`
- `PHASE-3-GC-IMPLEMENTATION-STAGING-PLAN.md`
- `PHASE-3-FAST-INTERPRETER-IMPLEMENTATION-MILESTONES.md`


---

<!-- BEGIN IMPLEMENTATION PLAN: PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md -->


# Phase 3 · Runtime Helper Implementation Plan
Document class: Implementation plan
Planning status: This document defines sequencing, milestones, tests, or implementation strategy. It does not override normative specifications.


Version: 0.12 implementation-plan draft  
Depends on: Phase 3 Fast Interpreter Concrete Data Structures v0.11  
Depends on: Phase 3 Runtime Helper Contracts v0.8  
Depends on: Phase 3 GC Root Enumeration and Safepoint Model v0.9  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: runtime helper implementation priority, bootstrap helper set, Rust module organization, helper table construction, generic call/access/allocation/unwind/module/capability helpers, testing matrix, implementation milestones  
Out of scope: full standard library, concrete host fs/net/process implementations, concrete JIT backend, concrete moving GC, public native ABI, FFI implementation

---

## 0. Status

This document converts the runtime helper contracts into an implementation plan.

It does not redefine helper semantics.

It defines the order and structure in which helpers should be implemented for the first VM.

The plan is optimized for:

```text
semantic correctness first
EIR fast interpreter integration
GC/root visibility correctness
JIT readiness
minimal bootstrap surface
no public native ABI leakage
```

---

## 1. Implementation Principles

### 1.1 Helper Role

Runtime helpers are VM-internal semantic slow paths.

They are called by:

```text
EIR fast interpreter
future baseline JIT
module initialization runtime
capability boundary
GC-aware allocation paths
structured unwinding
```

They are not:

```text
public user API
native extension ABI
FFI ABI
CPython C API
stable plugin boundary
```

### 1.2 Bootstrap Discipline

The first implementation may be direct Rust.

However:

```text
helper names remain internal
helper signatures remain VM-versioned
helper table digest controls compatibility
raw object pointers must not escape
Rc/RefCell is bootstrap-only if used
capability checks must not be bypassed
write barrier call sites must exist even when no-op
```

### 1.3 Fail-Closed Rule

If a helper cannot preserve semantics, it must fail with structured VM diagnostic.

It must not approximate language behavior.

### 1.4 Error Discipline

Language errors return:

```text
VmControl::Raise(ErrorObj)
```

VM structural failures return:

```text
VmError
```

Rust panic is not a language error mechanism.

---

## 2. Rust Module Organization

Recommended crate/module layout inside the VM workspace:

```text
crates/
  vm_runtime/
    src/
      helper/
        mod.rs
        table.rs
        descriptor.rs
        result.rs
        call.rs
        access.rs
        construct.rs
        type_check.rs
        pattern.rs
        error.rs
        unwind.rs
        resource.rs
        module.rs
        capability.rs
        alloc.rs
        barrier.rs
        display.rs
        numeric.rs
        validate.rs
      heap/
      frame/
      value/
      module/
      capability/
      diag/
```

### 2.1 `helper/mod.rs`

Exports the helper subsystem internally.

Must not expose public ABI.

### 2.2 `helper/table.rs`

Builds `RuntimeHelperTable`.

Responsibilities:

```text
register helper descriptors
map RuntimeHelperId to implementation
compute helper table digest
validate descriptor/implementation consistency
```

### 2.3 `helper/descriptor.rs`

Defines descriptor structs matching the spec.

### 2.4 `helper/result.rs`

Defines helper return normalization.

```rust
enum HelperReturn {
    Value(Value),
    Control(VmControl),
    Unit,
    Error(VmError),
}
```

### 2.5 Family Modules

Each helper family owns implementation and family-specific tests.

---

## 3. Helper Table Construction

### 3.1 Bootstrap Table

The bootstrap VM must construct a fixed helper table at VM startup.

```text
RuntimeHelperTable::bootstrap()
```

### 3.2 Helper ID Stability

Within one VM version, helper IDs are stable.

Across VM versions, IDs may change, but helper table digest must change.

### 3.3 Descriptor/Implementation Pairing

Every helper must register:

```text
descriptor
implementation function
validation metadata
test coverage marker
```

### 3.4 Helper Digest

The helper table digest includes:

```text
helper id
name
family
signature
may_allocate
may_raise
may_unwind
is_safepoint
requires_roots_visible
required capability
effect
GC behavior
JIT call policy
```

### 3.5 Missing Helper Policy

If EIR references a helper absent from the table, EIR validation fails.

Execution must not attempt late dynamic helper lookup.

---

## 4. Helper Implementation Priority

### 4.1 Priority Levels

```text
P0: required for minimal expression/declaration execution
P1: required for structured control and modules
P2: required for full Phase 3 semantic closure
P3: performance/JIT support helpers
P4: host/standard-library expansion helpers
```

### 4.2 P0 Helpers

Required first:

```text
helper_alloc_object
helper_construct_error
helper_check_type_contract
helper_check_callable
helper_check_hashable
helper_numeric_binary
helper_compare
helper_get_attribute
helper_set_attribute
helper_index_read
helper_index_write
helper_slice_read
helper_generic_call
helper_construct_record
helper_construct_enum
helper_construct_map
helper_display
helper_write_barrier
```

### 4.3 P1 Helpers

Required for structured control:

```text
helper_raise
helper_perform_unwind
helper_attach_suppressed
helper_register_defer
helper_execute_defer
helper_register_resource
helper_close_resource
helper_assert_fail
```

### 4.4 P2 Helpers

Required for module completeness and tests:

```text
helper_resolve_module
helper_initialize_module
helper_import_named
helper_import_module
helper_seal_exports
helper_match_pattern
```

### 4.5 P3 Helpers

Required for optimization readiness:

```text
helper_check_shape
helper_bind_method
helper_call_builtin
helper_enter_host_call
helper_exit_host_call
specialized allocation helpers
JIT helper bridge adapters
```

### 4.6 P4 Helpers

Deferred host/domain helpers:

```text
fs helpers
net helpers
process helpers
env helpers
random helpers
clock helpers
future FFI helpers
```

P4 helpers require explicit capability integration.

---

## 5. Helper Invocation Flow

### 5.1 Interpreter Invocation

Fast interpreter helper call flow:

```text
lookup helper descriptor
validate helper availability
marshal arguments from slots
update source mapping
if helper requires capability -> check capability
if helper may allocate/collect -> make roots visible
enter HelperCallState
call helper implementation
normalize HelperReturn
exit HelperCallState
dispatch result
```

### 5.2 Helper Argument Rules

Arguments passed to helpers should be:

```text
Value
ObjRef
SlotRef
FrameRef
VmRef
RuntimePlanRef
CallSiteId
AccessSiteId
TypeId
ShapeId
SourceSpanId
```

depending on helper descriptor.

Helpers must not receive raw pointers to VM heap objects as stable identity.

### 5.3 Helper Result Dispatch

```text
Value -> write destination slot
Unit -> continue
VmControl::Raise -> enter unwind
VmControl::Return/Break/Continue -> enter control path if permitted
VmError -> abort current execution with VM diagnostic
```

### 5.4 Source Mapping

Every helper that may raise must receive source span or source mapping reference.

---

## 6. Allocation Helper Plan

### 6.1 `helper_alloc_object`

Initial implementation:

```text
allocate through Heap API
return ObjRef handle
```

Required behavior:

```text
no raw pointer escape
object kind recorded
allocation source span optional
GC hook called if profile requires
```

### 6.2 Specialized Allocation Helpers

Implement after generic allocation:

```text
alloc_string
alloc_list
alloc_map
alloc_record
alloc_enum_value
alloc_function
alloc_module
alloc_error
alloc_resource
```

### 6.3 Allocation Safety

No partially initialized object may become source-visible.

Recommended pattern:

```text
allocate unexposed object
initialize required fields
publish handle only after initialization succeeds
```

### 6.4 Root Visibility

If allocation may trigger GC:

```text
all live interpreter roots visible
all helper args visible
destination slot not treated as initialized until allocation succeeds
```

### 6.5 Bootstrap Heap

Bootstrap may use non-moving handle arena.

API must still preserve future moving-GC compatibility.

---

## 7. Write Barrier Helper Plan

### 7.1 `helper_write_barrier`

Initial implementation may be no-op.

The function still exists.

### 7.2 Required Call Sites

Barrier must be invoked or syntactically present at:

```text
record field write
list element write
map entry write
capture cell write
module slot write
error suppressed list write
resource state write
```

### 7.3 Barrier API

Conceptual:

```rust
fn write_barrier(
    vm: &mut Vm,
    owner: BarrierOwner,
    value: Value,
    kind: WriteKind,
) -> Result<(), VmError>
```

### 7.4 Future GC Compatibility

When generational GC is added, barrier implementation updates remembered sets.

When incremental GC is added, barrier implementation enforces tri-color/incremental invariants.

---

## 8. Type and Check Helper Plan

### 8.1 `helper_check_type_contract`

Required for:

```text
let/const type contracts
parameter contracts
return contracts
record field contracts
enum payload contracts
explicit TypeCheckNode
```

### 8.2 Implementation Strategy

Dispatch by `RuntimeTypeDesc`.

```text
ImmediateKindCheck
ShapeCheck
UnionCheck
OptionalCheck
FunctionSignatureCheck
ExtensionCheck
```

### 8.3 Error Strategy

Failure raises:

```text
TypeContractError
```

unless caller supplies more specific failure code.

### 8.4 `helper_check_hashable`

Initial allowed key categories should match current VM hashability policy.

Must reject mutable/hash-unstable objects.

### 8.5 `helper_check_callable`

Recognizes VM callable categories only.

No implicit host callable access without capability-controlled wrapper.

---

## 9. Numeric Helper Plan

### 9.1 `helper_numeric_binary`

Handles unsupported or generic numeric paths.

Must preserve:

```text
no implicit coercion
checked Int overflow
division by zero error
operator-specific TypeError
```

### 9.2 Fast Path Split

Interpreter/JIT may inline common numeric cases.

Fallback calls helper.

### 9.3 Integer Representation

If first VM uses checked i64, overflow raises `NumericOverflowError`.

If it uses arbitrary precision Int, overflow helper behavior changes but no silent wrap is allowed.

---

## 10. Access Helper Plan

### 10.1 `helper_get_attribute`

Supported initial categories:

```text
RecordInstance
ReadOnlyView over RecordInstance
Module
EnumType case constructor
RecordType method
```

Must not implement dynamic dict fields for records.

### 10.2 `helper_set_attribute`

Supported initial categories:

```text
RecordInstance field write
ReadOnlyView rejection
```

Must enforce:

```text
fixed shape
field mutability
type contract
write barrier
```

### 10.3 `helper_index_read`

Supported initial categories:

```text
List[Int]
Map[Hashable]
```

String indexing remains non-core.

### 10.4 `helper_index_write`

Supported initial categories:

```text
List[Int existing index]
Map[Hashable insert/replace]
```

Must enforce readonly and barrier.

### 10.5 `helper_slice_read`

Supported initial categories:

```text
List
String
```

Must enforce:

```text
half-open slicing
negative bound error
out-of-range error
no invalid string scalar splitting
```

---

## 11. Construction Helper Plan

### 11.1 `helper_construct_record`

Inputs:

```text
shape_id
field value list
source span
```

Required checks:

```text
required fields
unknown fields
duplicate fields
default evaluation already completed or delegated explicitly
field type contracts
readonly state initial false
```

### 11.2 `helper_construct_enum`

Inputs:

```text
shape_id
case_index
payload values
source span
```

Required checks:

```text
case exists
payload arity
payload contracts
closed enum
```

### 11.3 `helper_construct_map`

Must preserve:

```text
source-order entry evaluation already done
hashability
duplicate-key replacement
first insertion position preservation
```

### 11.4 `helper_construct_error`

Must create ErrorObj with:

```text
code
message
details
source span
stack trace if enabled
suppressed list
```

---

## 12. Call Helper Plan

### 12.1 `helper_generic_call`

Generic call is central to bootstrap execution.

Supported initial callable categories:

```text
FunctionObj
BuiltinFunction
Record constructor
Enum case constructor
BoundMethod
HostFunction wrapper
```

### 12.2 Call Steps

Helper must perform:

```text
callee kind dispatch
arity check
named argument resolution
default argument evaluation at call time
parameter binding
parameter contract checks
frame push
body execution through interpreter
return contract check
frame pop
```

Some steps may be delegated to VM call engine.

### 12.3 CallSite Feedback

The helper updates CallSite feedback:

```text
observed callee kind
function id
builtin id
receiver shape if method
arity shape
miss count
```

### 12.4 Recursive Calls

Recursive calls use logical call stack.

Stack depth limit is enforced.

### 12.5 Builtin Calls

Builtin calls must follow builtin descriptor:

```text
arity
type expectations if declared
capability requirement
may_allocate
may_raise
safepoint behavior
```

---

## 13. Pattern Helper Plan

### 13.1 `helper_match_pattern`

Bootstrap pattern fallback may implement all pattern kinds conservatively.

Supported patterns:

```text
wildcard
literal
binding
record
enum
list
map
or-pattern
```

### 13.2 Match vs Destructuring Mode

Helper receives mode:

```text
PatternMode =
  | MatchCase
  | DestructuringDeclaration
```

Match failure returns branch failure.

Destructuring failure raises `PatternMatchError`.

### 13.3 Binding Commit

Recommended implementation:

```text
write candidate bindings into temporary buffer
commit to binding slots only after full pattern success
```

This avoids partial binding leakage on failure.

### 13.4 Guard Separation

Pattern helper does not evaluate guard expressions.

Guard lowering handles Bool check and case continuation.

---

## 14. Unwind Helper Plan

### 14.1 `helper_perform_unwind`

The bootstrap VM may centralize unwinding in this helper.

Inputs:

```text
frame
region_stack
pending_control
source_span
```

### 14.2 Algorithm

```text
while region stack not empty and pending control unresolved:
  run defers LIFO
  close resources reverse acquisition
  run finally if present
  apply finally override
  handle cleanup errors
  pop region if complete
resolve break/continue/return/raise target
```

### 14.3 Cleanup Error Policy

Must preserve:

```text
pending Raise + cleanup Raise -> cleanup suppressed
pending Normal + cleanup Raise -> cleanup primary
pending Return/Break/Continue + cleanup Raise -> cleanup Raise supersedes non-error control unless amended
finally non-normal -> overrides prior pending control
```

### 14.4 Reentrancy

If defer/resource close calls back into VM, active cleanup progress must remain explicit and root-visible.

### 14.5 JIT Readiness

JIT can call unwind helper.

Compiled frames must expose region/defer/resource state.

---

## 15. Resource Helper Plan

### 15.1 `helper_register_resource`

Registers resource only after acquisition succeeds.

### 15.2 `helper_close_resource`

State machine:

```text
Open -> Closing -> Closed
Open -> Closing -> Failed
Closed -> Closed or ResourceStateError depending resource policy
```

Initial policy should prefer idempotent close for already-closed resources unless language semantics require error.

### 15.3 Capability Origin

Resource object records capability origin.

Closing a resource generally should not require re-granting capability if the resource was validly acquired.

### 15.4 `helper_register_defer`

Must check:

```text
callable
zero arity
owning region exists
```

### 15.5 `helper_execute_defer`

Executes callable with no arguments.

If it raises, returns `VmControl::Raise`.

---

## 16. Module Helper Plan

### 16.1 `helper_resolve_module`

Uses deterministic module resolver.

The resolver is host-defined but environment-stable.

### 16.2 `helper_initialize_module`

Implements module state machine:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
```

### 16.3 Cycle Handling

If module is already Initializing:

```text
allow access to initialized exports
reject uninitialized export access with ImportCycleError
```

### 16.4 `helper_import_named`

Checks:

```text
provider export exists
interface compatibility
export initialized
local binding duplicate already validated
```

### 16.5 `helper_seal_exports`

Seals export table after successful initialization.

Export table mutation after sealing is VM bug or module runtime error depending layer.

---

## 17. Capability Helper Plan

### 17.1 `helper_check_capability`

Checks capability environment.

Missing capability raises `CapabilityError`.

### 17.2 Capability Environment

Capability environment is explicit.

No ambient authority.

### 17.3 Host Boundary Helpers

`helper_enter_host_call` and `helper_exit_host_call` normalize host boundary.

They must:

```text
make roots visible
register host roots if needed
convert host errors to VM Error
release/transmit host roots explicitly
```

### 17.4 P4 Host Helpers

Actual fs/net/process/env/random/clock helpers remain deferred.

They must be implemented behind capability-gated descriptors.

---

## 18. Display Helper Plan

### 18.1 `helper_display`

Used by:

```text
print
format strings
debug output
diagnostics
```

### 18.2 Display Is Not Coercion

Display conversion does not introduce implicit coercion for binary operators or type contracts.

### 18.3 Error Handling

If display fails, helper raises language error or diagnostic error according to call site.

---

## 19. Helper Testing Matrix

### 19.1 Test Categories

Each helper family requires:

```text
positive tests
negative tests
error category tests
source span tests
GC root visibility tests
capability tests where applicable
write barrier tests where applicable
reentrancy tests where applicable
determinism tests where applicable
```

### 19.2 P0 Test Matrix

P0 must test:

```text
allocation success/failure
type contract pass/fail
hashable pass/fail
generic call arity errors
default argument call-time evaluation
record fixed shape
enum closed case behavior
map duplicate key order
readonly access rejection
numeric overflow/division-by-zero
display in format string
write barrier invocation
```

### 19.3 P1 Test Matrix

P1 must test:

```text
return cleanup
raise cleanup
break/continue cleanup
defer LIFO
use close exactly once
finally override
suppressed cleanup errors
assert message lazy evaluation
```

### 19.4 P2 Test Matrix

P2 must test:

```text
module source-order import
named import
whole-module import
circular import initialized export access
circular import uninitialized export error
export sealing
pattern match order
or-pattern binding consistency
destructuring PatternMatchError
```

### 19.5 GC/JIT Readiness Tests

Even before full GC/JIT, tests must verify:

```text
roots visible before may_allocate helper
barrier hook called on mutation
helper descriptor marks safepoint correctly
helper digest changes on signature change
JIT-callable helper has JIT policy
```

---

## 20. Implementation Milestones

### 20.1 Milestone H0 · Helper Table Skeleton

Deliver:

```text
RuntimeHelperId
RuntimeHelperDescriptor
RuntimeHelperTable
helper digest
descriptor validation
dummy no-op helper registry
```

### 20.2 Milestone H1 · Allocation/Error/Type Helpers

Deliver:

```text
allocation helper
error construction helper
type contract helper
callable/hashable helpers
write barrier no-op helper
```

### 20.3 Milestone H2 · Access/Construction/Numeric Helpers

Deliver:

```text
get/set attribute
index read/write
slice read
construct record
construct enum
construct map
numeric binary
compare
display
```

### 20.4 Milestone H3 · Call Engine Helpers

Deliver:

```text
generic call
builtin call
method bind
frame push/pop integration
default evaluation
parameter/return contract integration
call-site feedback update
```

### 20.5 Milestone H4 · Control Helpers

Deliver:

```text
raise
perform_unwind
attach_suppressed
register/execute defer
register/close resource
assert_fail
```

### 20.6 Milestone H5 · Module Helpers

Deliver:

```text
resolve module
initialize module
import named
import module
seal exports
cycle handling
interface compatibility checks
```

### 20.7 Milestone H6 · Capability/Host Boundary

Deliver:

```text
capability check
enter host call
exit host call
host error conversion
host root registration
```

### 20.8 Milestone H7 · Optimization Readiness

Deliver:

```text
shape check helper
JIT helper-call descriptors
helper JIT readiness matrix
helper deopt metadata links
helper safepoint/root map validation
```

---

## 21. Implementation Guardrails

The implementation must not:

```text
expose helper table to user code
treat helper IDs as public ABI
perform effectful host access without capability
store raw object pointers in helper-visible state
throw Rust panic for language errors
skip source span propagation
skip write barrier call sites
skip root visibility before allocation/collection
let helper behavior diverge from interpreter fast path
```

---

## 22. Validation

Helper implementation validation must reject:

```text
registered descriptor without implementation
implementation without descriptor
declared no-raise helper that can raise language error
declared no-allocate helper that allocates
declared non-safepoint helper that may collect
capability-using helper without required capability descriptor
JIT-callable helper without JIT policy
may-move helper returning raw pointer
helper missing source mapping for diagnostics
helper missing tests at required milestone
```

---

## 23. Compatibility

Changing helper implementation alone may not invalidate caches if descriptor digest and behavior remain compatible.

Changing any of the following invalidates RuntimePlan/EIR/JIT caches:

```text
helper signature
helper result type
may_allocate
may_raise
may_unwind
is_safepoint
requires_roots_visible
GC behavior
JIT call policy
required capability
effect
```

Behavioral bug fixes should still bump VM build metadata.

---

## 24. Non-Goals

This document does not define:

```text
actual Rust code
full standard library helpers
host fs/net/process implementation
FFI
native extension ABI
Cranelift helper lowering
LLVM helper lowering
moving GC implementation
debugger protocol
profiler format
```

---

## 25. Next Work

Next Phase 3 documents should define:

```text
GC implementation staging plan
JIT lowering matrix per EIR operation
fast interpreter implementation milestones
Phase 3 consistency audit
Phase 3 freeze-readiness checklist

```


<!-- END IMPLEMENTATION PLAN: PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md -->


---

<!-- BEGIN IMPLEMENTATION PLAN: PHASE-3-GC-IMPLEMENTATION-STAGING-PLAN.md -->


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


<!-- END IMPLEMENTATION PLAN: PHASE-3-GC-IMPLEMENTATION-STAGING-PLAN.md -->


---

<!-- BEGIN IMPLEMENTATION PLAN: PHASE-3-FAST-INTERPRETER-IMPLEMENTATION-MILESTONES.md -->


# Phase 3 · Fast Interpreter Implementation Milestones
Document class: Implementation plan
Planning status: This document defines sequencing, milestones, tests, or implementation strategy. It does not override normative specifications.


Version: 0.15 implementation-milestone draft  
Depends on: Phase 3 JIT Lowering Matrix per EIR Operation v0.14  
Depends on: Phase 3 Fast Interpreter Concrete Data Structures v0.11  
Depends on: Phase 3 Runtime Helper Implementation Plan v0.12  
Depends on: Phase 3 GC Implementation Staging Plan v0.13  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: fast interpreter implementation sequence, milestone gates, execution subset ordering, helper/GC/JIT integration order, conformance tests, failure criteria, implementation readiness  
Out of scope: concrete Rust code, concrete parser/frontend implementation, concrete JIT backend, production GC, public bytecode

---

## 0. Status

This document defines the implementation milestones for the EIR fast interpreter.

The fast interpreter is the production non-JIT execution tier.

It consumes:

```text
RuntimePlan
EIR
RuntimeHelperTable
FrameMap
RootMap
SafepointTable
FeedbackTable
InlineCacheTable
```

It must not execute SIR by naive tree walking except in a temporary correctness-tier interpreter.

The implementation sequence is staged so that correctness, diagnostics, GC visibility, and later JIT integration are not retrofitted after the fact.

---

## 1. Implementation Principle

### 1.1 Fast Interpreter Is Not a Prototype Dump

The first working interpreter may be incomplete.

It must still preserve the core architectural constraints:

```text
slot-based execution
explicit frames
explicit region stack
explicit helper bridge
explicit safepoint polling points
explicit root visibility API
explicit source mapping
explicit error/control separation
```

### 1.2 No Hidden Semantic Shortcuts

The implementation must not use shortcuts that would later contradict the spec:

```text
HashMap<String, Value> locals
string field lookup for fixed records in hot path
host exceptions as language errors
Rust panic for language control
raw object pointers as identity
refcount as semantic ownership model
implicit truthiness
implicit coercion
```

### 1.3 Milestone Gate Rule

Each milestone has:

```text
implementation deliverables
required tests
failure criteria
next-milestone gate
```

A milestone is not complete until its tests and validation pass.

---

## 2. Milestone Overview

```text
I0: Workspace and skeleton
I1: Core runtime values, heap handles, frames, slots
I2: RuntimePlan/EIR loader and validation gate
I3: Constants, load/store, checks
I4: Unary/binary/logical expression execution
I5: Helper bridge and P0 helpers
I6: Aggregates, records, enums, access
I7: Function call engine
I8: Structured control and unwinding
I9: Modules, imports, exports, tests
I10: Feedback, inline caches, safepoints, root integration
I11: Conformance gate and freeze-readiness
```

I11 is included because Phase 3 cannot freeze merely because documents exist.

It must have an implementation-facing closure checklist.

---

## 3. I0 · Workspace and Skeleton

### 3.1 Goal

Create the Rust workspace skeleton and internal crate boundaries.

### 3.2 Deliverables

```text
Cargo workspace
sir crate placeholder
sir_validate crate placeholder
vm_core crate
vm_runtime crate
vm_eval crate
vm_diag crate
vm_host crate
vm_tests crate
vm_cli optional
```

### 3.3 Required Modules

```text
value
heap
slot
frame
region
control
helper
eir
runtime_plan
diag
source_map
```

### 3.4 Required Policies

```text
rustfmt enabled
clippy enabled
unsafe forbidden in core by default
panic policy documented
Result/VmControl distinction encoded
```

### 3.5 Tests

```text
workspace builds
crate dependency graph has no cycles
no unsafe in core runtime modules
basic diagnostic type compiles
basic Value type compiles
```

### 3.6 Failure Criteria

I0 fails if:

```text
workspace requires nightly without justification
core crate depends on JIT backend
runtime depends on host fs/net/process directly
public API exposes internal object layout
```

---

## 4. I1 · Core Runtime Values, Heap Handles, Frames, Slots

### 4.1 Goal

Implement minimal runtime state required for execution.

### 4.2 Deliverables

```text
Value conceptual enum or equivalent
ObjectId / ObjRef
HeapObject enum
Heap API G0
BindingCell
SlotArray
InterpreterFrame
InterpreterCallStack
RegionStack skeleton
PendingControl storage
VmControl
VmError
ErrorObj
```

### 4.3 Heap Requirements

G0 heap must support:

```text
allocate
get
get_mut
kind check
stale handle detection if generational handles used
object metadata
```

### 4.4 Slot Requirements

SlotArray must support:

```text
uninitialized state
value state
cell state
runtime-internal state
slot read
slot write
initialized tracking
heap-ref root visibility placeholder
```

### 4.5 Frame Requirements

Frame must include:

```text
frame id
module id
function id optional
eir function id
slot array
region stack
frame map id placeholder
source span
pending control slot optional
```

### 4.6 Tests

```text
allocate object and retrieve by handle
detect wrong object kind
slot uninitialized read produces correct runtime error path
cell read/write respects mutability
frame push/pop preserves logical call stack
pending control stores Return/Raise with heap refs
```

### 4.7 Gate

I1 complete when a VM can create frames, allocate heap objects, store values in slots, and enumerate a placeholder root set without executing EIR.

---

## 5. I2 · RuntimePlan/EIR Loader and Validation Gate

### 5.1 Goal

Load RuntimePlan and EIR structures and reject invalid execution inputs before interpretation.

### 5.2 Deliverables

```text
RuntimePlan data structures
EirModule
EirFunction
EirBlock
EirOp family enum
EirTerminator
ConstantPool
SlotLayout
CallSiteTable
AccessSiteTable
SafepointTable placeholder
DeoptPointTable placeholder
EIR validator
RuntimePlan validator
```

### 5.3 Validation Requirements

Reject:

```text
missing function entry block
block without terminator
unknown SlotId
unknown helper id
unknown call site id
unknown access site id
invalid block target
invalid constant id
slot layout mismatch
```

### 5.4 Tests

```text
valid minimal EIR module accepted
block without terminator rejected
LoadSlot unknown slot rejected
CallOp unknown CallSiteId rejected
RuntimeHelperOp unknown helper rejected
branch to missing block rejected
```

### 5.5 Gate

I2 complete when invalid EIR cannot enter the interpreter.

---

## 6. I3 · Constants, Load/Store, Checks

### 6.1 Goal

Execute the smallest useful EIR subset.

### 6.2 Required Operations

```text
ConstantOp
LoadSlot
LoadCell
LoadCapture
StoreSlot
StoreCell
CheckBool
CheckReadonly
CheckHashable placeholder/helper call
CheckType simple builtins
Jump
Branch
Return without cleanup
```

### 6.3 Source Mapping

May-raise operations must carry source span.

Diagnostics must include EIR location and source span when available.

### 6.4 Tests

```text
load nil/bool/int/float/string constants
copy slot value
read/write mutable cell
immutable cell write rejected
CheckBool accepts Bool
CheckBool rejects Int/String/List
branch uses Bool only
return value exits frame
source span attached to CheckBool failure
```

### 6.5 Gate

I3 complete when expression-free straight-line EIR with branches and simple return can execute.

---

## 7. I4 · Unary/Binary/Logical Expression Execution

### 7.1 Goal

Execute core expression operations.

### 7.2 Required Operations

```text
UnaryOp Plus
UnaryOp Minus
UnaryOp Not
BinaryOp Add/Subtract/Multiply/Divide/Modulo
BinaryOp Equal/NotEqual
BinaryOp Less/LessEqual/Greater/GreaterEqual
BinaryOp Identity/NotIdentity
Logical lowering via Branch
CheckOverflow
CheckDivisionByZero
```

### 7.3 Required Semantics

```text
no implicit coercion
no truthiness
checked integer overflow
division by zero error
logical short-circuit preserved by block lowering
logical result Bool
```

### 7.4 Tests

```text
Int arithmetic
Float arithmetic
unsupported mixed arithmetic rejected if no promotion defined
division by zero raises DivisionByZeroError
overflow raises NumericOverflowError under checked fixed-width Int
not requires Bool
and/or short-circuit does not evaluate skipped side
comparison returns Bool
identity differs from equality where applicable
```

### 7.5 Gate

I4 complete when lowered expression EIR can execute core arithmetic, comparison, identity, and logical flows with correct error categories.

---

## 8. I5 · Helper Bridge and P0 Helpers

### 8.1 Goal

Make RuntimeHelperOp and helper bridge operational.

### 8.2 Deliverables

```text
RuntimeHelperTable
RuntimeHelperDescriptor
HelperBridge
HelperCallState
HelperReturn normalization
helper digest
P0 helper skeletons
P0 helper implementation subset
```

### 8.3 Required P0 Helpers

```text
helper_alloc_object
helper_construct_error
helper_check_type_contract
helper_check_callable
helper_check_hashable
helper_numeric_binary
helper_compare
helper_get_attribute
helper_set_attribute
helper_index_read
helper_index_write
helper_slice_read
helper_generic_call skeleton
helper_construct_record
helper_construct_enum
helper_construct_map
helper_display
helper_write_barrier no-op
```

### 8.4 Helper Bridge Rules

Before helper call:

```text
validate helper descriptor
marshal args
make roots visible if needed
check capability if declared
enter HelperCallState
```

After helper call:

```text
normalize result
exit HelperCallState
dispatch Value/VmControl/VmError
```

### 8.5 Tests

```text
unknown helper rejected by validation
helper may_raise returns VmControl::Raise
helper structural failure returns VmError
helper source span preserved
helper may_allocate makes root visibility hook observable
write barrier helper called on mutation paths
helper digest changes when descriptor changes
```

### 8.6 Gate

I5 complete when interpreter can execute RuntimeHelperOp and P0 helpers support access/construction fallback.

---

## 9. I6 · Aggregates, Records, Enums, Access

### 9.1 Goal

Execute aggregate and nominal object operations.

### 9.2 Required Operations

```text
ConstructList
ConstructMap
ConstructRecord
ConstructEnumValue
LoadField
StoreField
LoadEnumPayload
AttributeRead
AttributeWrite
IndexRead
IndexWrite
SliceRead
MethodRead skeleton
```

### 9.3 Required Semantics

```text
record fixed shape
field index access
field mutability
field type contracts
enum closed cases
enum payload index access
map insertion order
duplicate map key replacement
readonly mutation rejection
string indexing non-core
slice half-open
negative bounds error
out-of-range error
```

### 9.4 Tests

```text
list literal preserves order
map literal duplicate key replaces value and preserves first position
record construction rejects missing/unknown/duplicate fields
record field read by FieldIndex
record field write checks mutability and type
readonly view rejects mutation
enum construction rejects invalid payload
enum payload load checks case
list index read/write
map key read/write
slice list/string valid case
slice negative bound rejected
```

### 9.5 Gate

I6 complete when aggregate/object semantics are executable through EIR and helper fallback without dynamic record dictionaries.

---

## 10. I7 · Function Call Engine

### 10.1 Goal

Implement user function calls, builtins, default arguments, parameter binding, closures, and return contracts.

### 10.2 Deliverables

```text
call_value engine
FunctionObj
BuiltinFunction descriptor
ParameterLayout execution
CaptureLayout execution
frame push/pop
return contract check
call-site feedback update skeleton
```

### 10.3 Required Semantics

```text
callee evaluated first
positional args left-to-right
named args left-to-right
defaults evaluated at call time only when omitted
function declaration not hoisted
parameter contract checks
return contract checks
closure captures
stack depth limit
```

### 10.4 Tests

```text
simple function call
recursive function stack depth limit
default evaluated at call time
omitted default not evaluated when argument provided
parameter contract failure
return contract failure
closure captures immutable value
mutable capture uses cell
function declaration unavailable before execution point
builtin arity check
```

### 10.5 Gate

I7 complete when lowered function declarations and calls work through EIR, with call-site identity present even if caches are disabled.

---

## 11. I8 · Structured Control and Unwinding

### 11.1 Goal

Implement structured control and cleanup semantics.

### 11.2 Required Constructs

```text
Block
If
While
For
Return with cleanup
Break
Continue
Raise
Try/Catch/Finally
Use
Defer
Match
Assert
```

### 11.3 Required Runtime Pieces

```text
RegionStack
RuntimeRegionFrame
CleanupState
PendingControl
helper_perform_unwind
helper_register_defer
helper_execute_defer
helper_register_resource
helper_close_resource
helper_attach_suppressed
```

### 11.4 Required Semantics

```text
Bool-only conditions
loop break/continue targets
defer LIFO
use close exactly once
finally executes on all exits
finally non-normal overrides pending control
pending Raise + cleanup Raise -> suppressed cleanup error
match subject evaluated once
case order preserved
guard after bindings
assert message evaluated only on failure
```

### 11.5 Tests

```text
if condition non-Bool rejected
while loop break
while loop continue
for List/Map/Range iteration
return runs defer
raise runs defer
defer LIFO
use close exactly once
finally overrides return
catch guard Bool check
match subject single evaluation
match case order
or-pattern binding consistency
assert lazy message
suppressed cleanup error recorded
```

### 11.6 Gate

I8 complete when structured control passes conformance tests and unwinding is represented explicitly enough for GC and future JIT.

---

## 12. I9 · Modules, Imports, Exports, Tests

### 12.1 Goal

Implement module initialization and test execution semantics.

### 12.2 Required Runtime Pieces

```text
ModuleObj
ModuleState
ModuleEnvironment
ImportPlan execution
ExportPlan execution
helper_resolve_module
helper_initialize_module
helper_import_named
helper_import_module
helper_seal_exports
TestPlan
test runner entry
```

### 12.3 Required Semantics

```text
source-order top-level execution
source-order imports
module state transitions
Failed state records initialization error
whole module import
named import
export table sealed after successful init
circular import initialized export allowed
circular import uninitialized export raises ImportCycleError
test blocks do not run during ordinary module init
```

### 12.4 Tests

```text
module initializes once
module failed state stores error
named import binds exported value
whole import binds module object
missing export raises ImportError
circular initialized export access works
circular uninitialized export raises ImportCycleError
export table sealed after initialization
test runner executes test blocks separately
test failure reports source span
```

### 12.5 Gate

I9 complete when module lifecycle and import/export semantics match Phase 2 integration rules.

---

## 13. I10 · Feedback, Inline Caches, Safepoints, Root Integration

### 13.1 Goal

Enable performance-adaptive interpreter infrastructure without changing semantics.

### 13.2 Deliverables

```text
FeedbackStore
InlineCacheStore
hotness counters
CallCacheRuntime
AccessCacheRuntime
SafepointPoller
RootScanner G1
FrameMap runtime use
RootMap runtime use
write barrier debug counters
deterministic mode
```

### 13.3 Feedback Requirements

Collect:

```text
function entry count
loop backedge count
call target feedback
type feedback
shape feedback
branch feedback
guard failure count
allocation count
```

### 13.4 Inline Cache Requirements

Implement at least:

```text
call cache skeleton
record field access cache
list index strategy cache
type check cache
```

Caches may be disabled by deterministic mode.

### 13.5 Safepoint Requirements

Poll at:

```text
loop backedge
function call
helper call
allocation
module import
host call boundary
```

### 13.6 Root Integration

At safepoint, root scanner must see:

```text
live slots
cells
module roots
constant roots
region stack
pending control
errors
defer/resource roots
helper args
host roots
```

### 13.7 Tests

```text
feedback updates do not change semantics
deterministic mode disables adaptive changes
call cache transitions Uninitialized -> Monomorphic
access cache records shape
safepoint poll can enumerate roots
allocating helper preserves live roots
barrier counter increments on heap mutation
GC G1 root scanner sees region/defer/resource/pending control roots
```

### 13.8 Gate

I10 complete when fast interpreter is GC-visible, feedback-enabled, cache-ready, and deterministic-mode capable.

---

## 14. I11 · Conformance Gate and Freeze-Readiness

### 14.1 Goal

Prepare Phase 3 for design freeze or implementation baseline freeze.

### 14.2 Required Conformance Areas

```text
expression semantics
binding/scope semantics
function semantics
record semantics
enum semantics
list/map/range semantics
pattern semantics
structured control
resource/defer/finally unwinding
module import/export
capability checks
runtime error categories
source diagnostics
readonly views
format strings
test blocks
```

### 14.3 Required Negative Tests

```text
truthiness rejected
implicit coercion rejected
integer overflow rejected or arbitrary precision documented
division by zero rejected
uninitialized binding read rejected
const assignment rejected
readonly mutation rejected
unknown record field rejected
invalid enum case rejected
non-error raise rejected
break outside loop rejected
return outside function rejected
missing capability rejected
uninitialized circular export rejected
```

### 14.4 Performance Architecture Checks

Verify:

```text
no hot textual lookup for locals
record field access can use FieldIndex
enum case access can use CaseIndex
CallSiteId exists for calls
AccessSiteId exists for accesses
feedback tables wired
inline cache tables wired
safepoint/root APIs wired
helper table digest wired
```

### 14.5 GC/JIT Readiness Checks

Verify:

```text
root scanner exists
write barrier hooks exist
safepoint records exist
frame maps exist
JIT lowering classes exist
compiled-code metadata interfaces exist
JIT can be disabled safely
```

### 14.6 Freeze-Readiness Gate

Phase 3 may be considered freeze-ready only when:

```text
all spec contradictions resolved
all mandatory runtime structures specified
all implementation milestones defined
all conformance areas mapped to tests
all deferred items explicitly labeled
no public bytecode leak exists
no CPython/Python ABI compatibility claim exists
performance/JIT/GC paths remain open
```

---

## 15. Implementation Order Dependency Graph

```text
I0
  -> I1
    -> I2
      -> I3
        -> I4
          -> I5
            -> I6
              -> I7
                -> I8
                  -> I9
                    -> I10
                      -> I11
```

Some helper and GC work can run in parallel:

```text
H0/H1 can run during I1-I3
H2 can run during I5-I6
H3 can run during I7
H4 can run during I8
H5 can run during I9
G0 can run during I1
G1 can run during I10
J0/J1 can run after I2
```

---

## 16. Minimal Executable Slice

The first meaningful executable slice should be:

```text
module init function
constants
let/const
slots
simple arithmetic
if/branch
function call
return
print/display builtin
diagnostic source spans
```

This slice should not include:

```text
records
modules across files
JIT
GC collection
full unwinding
```

But it must already use the same Frame/Slot/Helper/Safepoint architecture.

---

## 17. Forbidden Implementation Shortcuts

Implementation must not introduce:

```text
AST/SIR-walk execution as production path
dynamic string-key locals
dynamic dict records
ambient host authority
native object pointer identity
public EIR files
public bytecode cache
CPython extension compatibility
resource cleanup through GC finalizers
helper table as user API
```

---

## 18. Milestone Reporting Format

Each implementation milestone should report:

```text
milestone id
implemented files/modules
implemented EIR ops
implemented helpers
implemented tests
known unsupported cases
validation status
conformance status
performance architecture status
GC/JIT readiness status
```

### 18.1 Example

```text
Milestone: I3
Implemented:
  ConstantOp, LoadSlot, StoreSlot, CheckBool, Branch, Jump, Return
Tests:
  42 passed, 0 failed
Unsupported:
  Function calls, aggregates, unwinding
Architecture:
  SlotArray used
  FrameMap placeholder wired
  Safepoint API placeholder wired
```

---

## 19. Non-Goals

This document does not define:

```text
actual Rust code
parser/frontend implementation
full standard library
concrete GC implementation
concrete JIT implementation
debugger protocol
profiler protocol
package manager
public bytecode
native ABI
```

---

## 20. Next Work

Next Phase 3 documents should define:

```text
Phase 3 consistency audit
Phase 3 freeze-readiness checklist
implementation issue tracker template
conformance test manifest

```


<!-- END IMPLEMENTATION PLAN: PHASE-3-FAST-INTERPRETER-IMPLEMENTATION-MILESTONES.md -->
