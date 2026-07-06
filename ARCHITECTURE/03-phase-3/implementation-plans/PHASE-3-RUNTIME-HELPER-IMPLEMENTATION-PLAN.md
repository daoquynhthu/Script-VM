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
