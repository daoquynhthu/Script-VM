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
