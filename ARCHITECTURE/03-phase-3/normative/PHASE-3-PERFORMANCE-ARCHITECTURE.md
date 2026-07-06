# Phase 3 · Performance and JIT Architecture
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.3 architecture draft  
Depends on: Phase 3 VM Framework v0.1  
Depends on: Phase 3 VM Runtime Semantics Round 1 v0.2  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: high-performance VM architecture, RuntimePlan, EIR execution target, JIT readiness, GC readiness, safepoints, deoptimization metadata, inline caches  
Out of scope: complete JIT implementation, concrete machine-code backend, final GC implementation, public bytecode, native ABI

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

This document amends the Phase 3 VM direction.

The previous Phase 3 framework correctly prioritized semantic closure, but its wording around JIT and performance was too weak.

The corrected position is:

```text
JIT implementation: staged
JIT architecture: mandatory
```

The VM may begin with a SIR interpreter, but the VM architecture must not be shaped around a permanently slow AST/SIR-walk evaluator.

The VM must be designed from the beginning as a high-performance adaptive runtime.

---

## 1. Core Correction

The Phase 3 VM must not become:

```text
Rust-written CPython
SIR AST-walk interpreter as final runtime
Rc<RefCell<Value>> everywhere
HashMap<String, Value> locals in hot path
dynamic dict object model
string-based field lookup
no safepoints
no deopt metadata
no call-site identity
no GC root maps
```

The correct architecture is:

```text
SIR
  -> validation
  -> RuntimePlan
  -> EIR
  -> fast interpreter
  -> baseline JIT
  -> optimizing JIT
```

SIR remains canonical semantic IR.

EIR is internal executable IR.

EIR is not public bytecode.

RuntimePlan is the static bridge between semantic SIR and efficient execution.

---

## 2. Architectural Split

The VM is divided into two cooperating runtimes:

```text
Semantic Runtime
Execution Runtime
```

### 2.1 Semantic Runtime

The Semantic Runtime owns:

```text
SIR validation
module interface compatibility
capability enforcement
diagnostics
source mapping
runtime error categories
language-level object semantics
structured semantics
```

It preserves correctness.

It is allowed to be slower.

### 2.2 Execution Runtime

The Execution Runtime owns:

```text
RuntimePlan
EIR
fast interpreter
profiling
inline caches
quickening
baseline JIT
optimizing JIT
safepoints
GC coordination
deoptimization
```

It preserves performance.

It must not change Phase 1 or Phase 2 semantics.

### 2.3 Boundary Rule

The Execution Runtime may specialize, lower, cache, and JIT.

It must not redefine language semantics.

All execution-layer optimizations must be recoverable to the Semantic Runtime model through diagnostics, source mapping, and deoptimization metadata where required.

---

## 3. Performance Tiers

The VM has explicit execution tiers.

```text
Tier 0: SIR correctness interpreter
Tier 1: RuntimePlan-driven interpreter
Tier 2: EIR fast interpreter
Tier 3: baseline JIT
Tier 4: optimizing JIT
```

### 3.1 Tier 0 · SIR Correctness Interpreter

Purpose:

```text
semantic validation
early implementation
diagnostic clarity
conformance baseline
```

Tier 0 is allowed to be slow.

Tier 0 must not be treated as the final production execution engine.

### 3.2 Tier 1 · RuntimePlan-Driven Interpreter

Purpose:

```text
remove repeated table lookups
resolve slots and indices
precompute execution metadata
prepare for EIR
```

Tier 1 still walks high-level structure but uses precomputed execution plan data.

### 3.3 Tier 2 · EIR Fast Interpreter

Purpose:

```text
production interpreter
compact execution form
quickening target
JIT input source
profile collection
```

Tier 2 is the long-term non-JIT execution target.

### 3.4 Tier 3 · Baseline JIT

Purpose:

```text
compile hot EIR functions quickly
reduce dispatch overhead
use simple guards
emit deopt points
call runtime helpers
preserve fast compilation
```

The recommended first backend target is a Cranelift-compatible backend interface, but the VM must not be architecturally locked to one backend.

### 3.5 Tier 4 · Optimizing JIT

Purpose:

```text
specialize hot paths
inline functions
eliminate redundant checks
specialize record/enum access
use type and shape feedback
perform guarded optimizations
deopt on failed assumptions
```

Tier 4 is deferred.

The metadata it needs is not deferred.

---

## 4. RuntimePlan

### 4.1 Purpose

RuntimePlan converts validated SIR into execution-friendly metadata.

RuntimePlan is required before hot execution.

RuntimePlan is not public bytecode.

RuntimePlan is discardable and implementation-private.

### 4.2 RuntimePlan Contents

A RuntimePlan contains:

```text
module plan
function plans
binding slot layout
scope layout
field index layout
enum case index layout
type descriptor layout
call-site table
access-site table
control-region plan
pattern plan
source mapping
deopt metadata seeds
safepoint map seeds
capability gate plan
```

### 4.3 Binding Slot Layout

SIR uses `BindingId`.

Execution uses slots.

```text
BindingId -> SlotId
```

Slot kinds:

```text
local slot
capture slot
module slot
global builtin slot
temporary slot
```

Hot-path variable access must not perform textual lookup.

### 4.4 Field Index Layout

Record access uses field indices.

```text
FieldId -> FieldIndex
```

Hot-path field access should be:

```text
check shape
load field index
```

It should not be:

```text
lookup field name string
```

### 4.5 Enum Case Layout

Enum access uses case indices.

```text
CaseId -> CaseIndex
```

Pattern matching against enums should compile to:

```text
check enum shape
check case index
load payload slots
```

### 4.6 Type Descriptor Layout

Type contracts use runtime type descriptors.

```text
TypeId -> RuntimeTypeDesc
```

Type checks may be cached or lowered.

### 4.7 Control Region Plan

Structured control regions are preplanned.

```text
ControlRegionId -> RegionPlan
```

RegionPlan includes:

```text
region kind
parent region
cleanup ownership
loop target
finally target
defer stack slot
resource cleanup slot
source span
```

### 4.8 Pattern Plan

Patterns are precompiled.

```text
PatternId -> PatternPlan
```

PatternPlan may contain:

```text
literal checks
record shape checks
field loads
enum case checks
payload loads
binding writes
guard references
failure continuation
```

### 4.9 RuntimePlan Validation

RuntimePlan generation must reject:

```text
unresolved BindingId
unresolved FieldId
unresolved CaseId
unresolved TypeId
unresolved PatternId
unresolved ControlRegionId
invalid slot assignment
invalid cleanup ownership
invalid source mapping
```

---

## 5. EIR · Executable Internal IR

### 5.1 EIR Identity

EIR is the VM's internal executable representation.

EIR is not:

```text
public bytecode
package artifact
stable ABI
external compiler target
user-visible instruction set
```

EIR is:

```text
compact internal execution form
fast interpreter input
JIT input
quickening target
profile carrier
deopt source map carrier
```

### 5.2 EIR Requirements

EIR must support:

```text
slot-based locals
explicit temporaries
explicit control regions
explicit type guards
explicit capability gates
explicit call sites
explicit access sites
explicit safepoints
explicit deopt points
source span mapping
runtime helper calls
```

### 5.3 EIR Generation

EIR is generated from validated SIR plus RuntimePlan.

```text
SIR + RuntimePlan -> EIR
```

EIR must preserve:

```text
evaluation order
binding identity
scope semantics
function default evaluation time
assignment target single evaluation
Bool-only condition semantics
structured unwinding
primary/suppressed error semantics
capability checks
type contract checks
read-only view checks
module initialization order
```

### 5.4 EIR Caching

EIR may be cached.

EIR cache is discardable.

EIR cache is not package ABI.

EIR cache key must include:

```text
SIR semantic digest
runtime target profile
VM version
feature flags
capability environment digest
dependency interface digests
EIR schema version
backend-relevant options
```

### 5.5 EIR Non-Goals

EIR must not introduce:

```text
public bytecode compatibility
native ABI commitment
CPython ABI dependence
unvalidated execution
source-invisible semantic change
```

---

## 6. Fast Interpreter Architecture

### 6.1 Interpreter Target

The long-term interpreter target is EIR, not SIR.

SIR interpreter is correctness tier.

EIR interpreter is production tier.

### 6.2 Dispatch Strategy

Allowed dispatch strategies:

```text
match dispatch
computed dispatch where available
threaded dispatch where safe and portable
function-pointer dispatch
quickened node dispatch
```

The specification does not mandate one dispatch mechanism.

### 6.3 Quickening

The interpreter may rewrite internal EIR operations after observing stable behavior.

Examples:

```text
generic field access -> shape-checked field-index load
generic call -> monomorphic call
generic type check -> cached descriptor check
generic binary add -> int add with overflow guard
generic enum match -> case-index check
```

Quickening must preserve deopt or fallback path.

### 6.4 Hot Path Prohibitions

Hot interpreter paths must avoid:

```text
textual name lookup
string field lookup where field ID is known
HashMap locals where slots are available
repeated SIR table traversal
repeated scope graph traversal
unnecessary Rc clone/drop in every operation
unnecessary dynamic borrow checks in stable hot loops
```

### 6.5 Cold/Warm/Hot Performance

The VM must optimize three performance domains:

```text
cold performance
warm performance
hot performance
```

Cold performance:

```text
fast validation
fast module initialization
low startup overhead
compact RuntimePlan
```

Warm performance:

```text
EIR interpreter
quickening
inline caches
low dispatch overhead
```

Hot performance:

```text
baseline JIT
optimizing JIT
type specialization
shape specialization
deoptimization
```

---

## 7. Call Sites and Inline Caches

### 7.1 CallSiteId

Every call expression that can become hot must have a `CallSiteId`.

```text
CallSiteId
```

CallSiteId is VM-internal and may be derived from `NodeId`.

### 7.2 CallSiteRecord

```text
CallSiteRecord {
  call_site_id
  node_id
  argument_layout
  observed_targets
  inline_cache_state
  type_feedback
  source_span
}
```

### 7.3 Inline Cache States

```text
InlineCacheState =
  | Uninitialized
  | Monomorphic
  | Polymorphic
  | Megamorphic
  | Disabled
```

### 7.4 Call Target Kinds

Observed call targets may include:

```text
user function
builtin function
record constructor
enum case constructor
bound method
host function
```

### 7.5 AccessSiteId

Attribute/index access sites should have stable VM-internal IDs.

```text
AccessSiteId
```

Access sites may collect:

```text
receiver shape
field index
map key class
list index class
readonly state
miss count
```

### 7.6 Inline Cache Rule

Inline caches are optimization state.

They must be invalidated or guarded when assumptions fail.

They must not change semantics.

---

## 8. Type and Shape Feedback

### 8.1 TypeFeedback

Type feedback records observed value categories.

Examples:

```text
Int
Float
String
List
Map
RecordShape
EnumShape
Function
BuiltinFunction
Module
Error
```

### 8.2 ShapeFeedback

Shape feedback records object layout observations.

```text
RecordShapeId
EnumShapeId
MapShapeClass
ListElementClass
```

### 8.3 Feedback Consumers

Feedback may be consumed by:

```text
quickening
baseline JIT
optimizing JIT
diagnostics
specialized checks
```

### 8.4 Feedback Safety

Feedback is speculative.

Optimizations based on feedback must have guards.

Guard failure must deopt or fall back to generic execution.

---

## 9. Object Shapes

### 9.1 Shape-First Model

The VM should use shape-first object representation.

Records and enums are not dynamic dictionaries.

### 9.2 RecordShape

```text
RecordShape {
  shape_id
  record_id
  field_count
  field_ids
  field_indices
  mutability_bitmap
  method_table
}
```

### 9.3 Record Instance Layout

```text
RecordInstance {
  shape_id
  fields: Vec<Value>
}
```

### 9.4 EnumShape

```text
EnumShape {
  shape_id
  enum_id
  case_count
  case_ids
  payload_layouts
}
```

### 9.5 Enum Value Layout

```text
EnumValue {
  shape_id
  case_index
  payload: Vec<Value>
}
```

### 9.6 Shape Guard

A shape guard checks that an object still has the expected layout.

For fixed-shape records and closed enums, shape guards can be highly stable.

### 9.7 Shape Non-Goal

The VM should not implement Python-style arbitrary per-object dictionaries for records.

If dynamic host objects exist later, they must be explicitly foreign/opaque objects, not ordinary records.

---

## 10. Value Physical Layout

### 10.1 Conceptual vs Physical Value

The `Value` enum in Phase 3 Round 1 is conceptual.

It is not ABI.

It is not final physical layout.

### 10.2 Allowed Physical Representations

Future physical representations may include:

```text
Rust enum
tagged pointer
NaN boxing
compressed handles
split immediate/object reference
```

### 10.3 Forbidden Commitments

The VM must not expose:

```text
Rust enum memory layout
object pointer layout
heap object header layout
GC metadata layout
JIT value register convention
```

as language ABI.

### 10.4 JIT Constraint

JIT-generated code must interact with values through VM-defined internal calling conventions and runtime helpers.

Those conventions are VM-internal and versioned with the VM, not public ABI.

---

## 11. Memory Management Architecture

### 11.1 Core Position

The VM must not be architecturally based on CPython-style reference counting.

Reference-counted handles may be used only as bootstrap implementation detail.

They must not define language semantics, object layout, extension ABI, or JIT assumptions.

### 11.2 Bootstrap Heap

Allowed bootstrap strategies:

```text
typed arena
generational arena
slotmap-like heap
Rc/RefCell only in isolated bootstrap layers
```

Preferred bootstrap strategy:

```text
typed handle heap with generational indices
```

### 11.3 GC-Ready Architecture

The heap model must preserve a path to:

```text
tracing GC
generational GC
moving GC
compacting GC
incremental GC
```

Not all are required in Phase 3.

The architecture must not block them.

### 11.4 Required GC Hooks

Even before a full GC exists, the VM should reserve:

```text
root set enumeration
frame maps
stack maps
safepoints
allocation hooks
write barrier hooks
object tracing hooks
object relocation abstraction
```

### 11.5 Write Barrier Hook

A write barrier may initially be a no-op.

But mutation APIs must pass through a place where a future barrier can be inserted:

```text
record field write
list element write
map entry write
capture cell write
module binding write
```

### 11.6 Root Set

The root set includes:

```text
call stack frames
local slots
capture cells
module environments
region stacks
pending control values
active errors
defer callables
resource handles
JIT frame maps
host roots
```

### 11.7 Allocation Points

Allocation points are safepoint candidates.

Examples:

```text
string allocation
list allocation
map allocation
record instance allocation
enum value allocation
function object allocation
error object allocation
module object allocation
boxing large integer
```

---

## 12. Safepoints

### 12.1 Safepoint Identity

A safepoint is a location where the VM can safely observe or control execution.

Safepoints are needed for:

```text
GC
deoptimization
stack walking
interrupts
debugging
profiling
host cancellation
```

### 12.2 Required Safepoint Candidates

The VM must reserve safepoint positions at:

```text
function call boundary
loop backedge
allocation point
host call boundary
potential raise boundary
module import boundary
long-running builtin boundary
JIT side-exit boundary
```

### 12.3 SafepointMap

```text
SafepointMap {
  safepoint_id
  code_location
  live_slots
  live_objects
  frame_state
  source_span
}
```

For the interpreter, `code_location` may be an EIR instruction index or node index.

For JIT code, it may be a machine-code offset.

### 12.4 Safepoint Non-Goal

Phase 3 does not require preemptive multithreading.

Safepoints are still required as architecture points for future GC, profiling, interrupts, and JIT deopt.

---

## 13. Deoptimization

### 13.1 Deopt Principle

Any speculative optimized execution must be able to return to a semantically correct lower tier.

```text
optimizing JIT -> baseline JIT
baseline JIT -> EIR interpreter
EIR interpreter -> generic EIR path
```

### 13.2 DeoptPoint

```text
DeoptPoint {
  deopt_id
  source_span
  eir_location
  frame_map
  local_slot_map
  value_stack_map
  region_stack_state
  pending_control_state
}
```

### 13.3 Guard

```text
Guard {
  guard_id
  condition
  failure_target
  deopt_point
}
```

Guards may check:

```text
type
shape
call target
capability state
module state
readonly state
integer overflow
division by zero
```

### 13.4 Frame Reconstruction

Deoptimization requires reconstructing:

```text
function frame
local slots
capture references
pending expression values
region stack
pending control effect
source span
```

### 13.5 Deopt Non-Goal

Round 0/1 VM does not implement deopt.

But all JIT-facing plans must reserve enough metadata for future deopt.

---

## 14. Runtime Helper ABI

### 14.1 Runtime Helper Boundary

JIT code must call VM runtime helpers for operations that are too complex, rare, or semantic-heavy to inline.

Examples:

```text
generic call
generic type check
record construction
enum construction
map lookup
string slicing
pattern fallback
raise error
perform unwinding
capability check
allocate object
write barrier
```

### 14.2 Internal ABI

Runtime helper ABI is internal.

It is not public native ABI.

It may change across VM versions.

### 14.3 Helper Table

```text
RuntimeHelperTable {
  helper_id
  name
  signature
  effect
  safepoint_behavior
  may_raise
}
```

### 14.4 Helper Requirements

A helper must declare:

```text
whether it may allocate
whether it may raise
whether it is a safepoint
which roots must be visible
which capabilities it may require
```

---

## 15. JIT Backend Abstraction

### 15.1 Backend Interface

The VM should define a JIT backend abstraction.

```text
JitBackend {
  compile_function(EirFunction, JitContext) -> CompiledFunction
}
```

### 15.2 JitContext

```text
JitContext {
  runtime_plan
  type_feedback
  shape_feedback
  deopt_metadata
  safepoint_map
  helper_table
  target_profile
}
```

### 15.3 Recommended First Backend

The recommended first backend is:

```text
Cranelift-compatible baseline backend
```

Reason:

```text
suitable for Rust implementation
appropriate for baseline JIT
lower engineering overhead than full optimizing compiler infrastructure
```

### 15.4 Later Backends

Possible later backends:

```text
LLVM ORC optimizing backend
custom baseline compiler
WASM backend
AOT backend
```

### 15.5 Backend Lock-In Rule

The VM architecture must not depend on a single backend.

JIT backend choice is implementation strategy, not language semantics.

---

## 16. Profiling Model

### 16.1 Profile Data

The VM may collect:

```text
call counts
loop counts
branch bias
type feedback
shape feedback
call target feedback
allocation counts
error frequency
deopt frequency
inline cache state
```

### 16.2 Hotness

Hotness may be determined by:

```text
function entry count
loop backedge count
call site count
allocation pressure
combined heuristic
```

### 16.3 Profile Storage

Profile data is VM-internal.

It must not affect observable semantics.

It may affect optimization decisions.

### 16.4 Profile Invalidation

Profile data must be invalidated when:

```text
module interface changes
function body changes
EIR cache invalidates
runtime target profile changes
shape assumptions fail globally
```

---

## 17. Capability and JIT

### 17.1 Capability Guards

JIT code must not bypass capability checks.

Effectful operations require explicit capability guards or runtime helper calls.

### 17.2 Host Calls

Host calls are safepoint candidates.

Host calls may allocate, raise, block, or require capability checks.

### 17.3 FFI

FFI remains out of scope.

Future FFI must pass through capability-gated helper boundary.

JIT code must not directly call arbitrary native pointers without a VM-controlled foreign boundary.

---

## 18. Diagnostics and Optimized Code

### 18.1 Source Mapping

Optimized execution must preserve enough source mapping for diagnostics.

Required mapping:

```text
compiled code offset -> EIR location -> SIR NodeId -> SourceSpan
```

### 18.2 Stack Trace

Stack traces must be reconstructable across:

```text
interpreter frames
baseline JIT frames
optimizing JIT frames
runtime helper frames
```

### 18.3 Diagnostic Stability

Optimization must not erase diagnostic categories.

A type error in JIT code must still produce the same language-level error category as interpreter execution.

---

## 19. Performance Architecture Non-Goals

This document does not require immediate implementation of:

```text
full JIT
optimizing JIT
moving GC
generational GC
inline cache machinery
deoptimization runtime
native backend
LLVM
Cranelift integration
threaded interpreter
computed goto
```

But it requires the VM design not to block them.

---

## 20. Required Phase 3 Patch Commitments

Phase 3 VM specification must be updated to reflect:

```text
SIR interpreter is correctness tier only.
RuntimePlan is required before hot execution.
EIR fast interpreter is production interpreter target.
JIT architecture is mandatory.
JIT implementation is staged.
Value physical layout is VM-private.
Rust enum Value is conceptual.
CPython-style reference counting is rejected as architecture.
Rc/RefCell is bootstrap-only if used.
Heap handles must preserve path to tracing/moving GC.
Safepoints are architectural.
CallSiteId and AccessSiteId are required for hot execution.
Inline cache state is reserved.
Type/shape feedback is reserved.
Deopt metadata is reserved.
Frame maps and root maps are reserved.
Write barrier hooks are reserved.
Runtime helper ABI is internal and VM-controlled.
Cranelift-compatible backend is recommended first baseline JIT target.
Backend lock-in is forbidden.
```

---

## 21. Next Specification Work

After this architecture patch, Phase 3 should define:

```text
RuntimePlan concrete schema
EIR framework
fast interpreter execution model
safepoint/root map schema
call/access site cache schema
GC root enumeration model
baseline JIT backend interface
node execution semantics adjusted to RuntimePlan/EIR path
```

The next work should not return to naive SIR-walk execution without this architecture in place.
