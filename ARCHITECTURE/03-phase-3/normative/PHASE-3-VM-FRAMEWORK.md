# Phase 3 · Minimal VM Framework Specification
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.1 framework draft  
Depends on: Phase 1 High-Level Language Specification, Version 1.0 frozen baseline  
Depends on: Phase 2 IR Design, Version 1.0 frozen baseline  
Scope: VM architecture framework, execution boundary, runtime object model boundary, implementation technology stack  
Out of scope for this document: complete VM instruction set, optimizer IR, JIT, native ABI, full standard library, package manager

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

This document begins Phase 3.

Phase 1 and Phase 2 are frozen baselines.

The VM design must treat the following as fixed semantic inputs:

```text
Phase 1: source language semantics
Phase 2: canonical SIR semantics and compatibility rules
```

Phase 3 does not redefine the language.

Phase 3 does not redefine SIR.

Phase 3 specifies how a minimal runtime executes or lowers the frozen SIR baseline while preserving Phase 1 and Phase 2 semantics.

---

## 1. VM Identity

The Phase 3 VM is a semantic execution engine.

It is not:

```text
public bytecode VM
JVM-style class-file VM
CLR-style metadata VM
CPython-compatible runtime
CPython C-extension host
Python wheel runtime
native ABI surface
optimizer-first JIT runtime
```

It is:

```text
SIR-consuming runtime
structured-control executor
runtime value manager
module initializer
capability gate
diagnostic producer
future lowering target
```

The initial VM may execute SIR directly.

A later VM may execute NIR or EIR, but such layers must preserve Phase 2 SIR semantics.

---

## 2. Primary VM Goal

The primary goal of Phase 3 is semantic closure, not peak performance.

The minimal VM must prove that the frozen language and IR can be executed correctly.

Success means:

```text
Phase 1 source
  -> frontend
  -> Phase 2 SIR
  -> validation
  -> VM execution
  -> correct observable behavior
```

The VM must support the complete frozen core semantics sufficiently to run conformance programs, but implementation can stage performance improvements.

---

## 3. Rust as Implementation Language

Rust is selected as the default implementation language for the VM.

Rationale:

```text
memory safety without garbage-collected host runtime
strong enum/pattern modeling for SIR and runtime values
explicit ownership model for VM object lifetime
good fit for capability boundaries
good fit for deterministic validation pipelines
good tooling for testing, fuzzing, formatting, and CI
suitable for future native embedding and WASM targets
```

Rust is not selected because it makes the VM automatically fast.

Rust is selected because it gives a disciplined substrate for:

```text
runtime safety
schema validation
explicit error handling
structured control flow
future optimization
native boundary control
```

The VM must not expose Rust internal data layout as language ABI.

---

## 4. Technology Stack Boundary

The VM technology stack is divided into mandatory, recommended, and deferred layers.

### 4.1 Mandatory

```text
Rust stable toolchain
Cargo workspace
rustfmt
clippy
unit tests
integration tests
SIR schema model
SIR validator
runtime value model
structured evaluator
diagnostic system
```

### 4.2 Recommended

```text
property-based tests
snapshot tests for diagnostics
fuzzing for parser/SIR validation boundary
benchmark harness
Miri checks for unsafe-sensitive code
cargo-deny or equivalent dependency auditing
CI matrix for major targets
```

### 4.3 Deferred

```text
JIT implementation
AOT compiler
native ABI
WASM extension host
moving GC
parallel runtime
async scheduler
debugger
profiler
package manager
standard library implementation
```

### 4.4 Explicit Non-Goals

The Phase 3 minimal VM does not implement:

```text
public bytecode
CPython C API
Python ABI
Python binary wheel loader
native plugin ABI
industrial JIT
industrial garbage collector
full standard library
full package manager
```

---

## 5. Workspace Architecture

The recommended Rust workspace structure is:

```text
vm-workspace/
  crates/
    sir/
    sir_validate/
    vm_core/
    vm_runtime/
    vm_eval/
    vm_diag/
    vm_host/
    vm_tests/
```

### 5.1 `sir`

Owns Rust data structures for frozen Phase 2 SIR.

Responsibilities:

```text
IRUnit model
NodeId / BindingId / ScopeId / TypeId model
SIR node enums
pattern model
control region model
module interface descriptor model
canonical schema compatibility helpers
```

### 5.2 `sir_validate`

Owns SIR validation.

Responsibilities:

```text
V0 schema validation
V1 reference validation
V2 table validation
V3 node validation
V4 control-flow validation
V5 module-interface validation
V6 dependency compatibility validation
V7 capability safety validation
V8 lowering precondition validation
```

### 5.3 `vm_core`

Owns VM-wide abstract definitions.

Responsibilities:

```text
VM configuration
execution mode
feature support
capability environment
module environment
runtime error categories
control-flow effect representation
```

### 5.4 `vm_runtime`

Owns runtime values and heap objects.

Responsibilities:

```text
RuntimeValue
String value
List value
Map value
Record instance
Enum value
Function object
Builtin function
Module object
Error object
ReadOnly view
Resource handle abstraction
```

### 5.5 `vm_eval`

Owns execution.

Responsibilities:

```text
SIR evaluator
expression evaluation
statement execution
function call
module initialization
structured unwinding
type contract checks
capability checks
resource cleanup
```

### 5.6 `vm_diag`

Owns diagnostics.

Responsibilities:

```text
diagnostic records
source spans
related spans
stack traces
runtime error formatting
validation error formatting
snapshot-friendly diagnostic rendering
```

### 5.7 `vm_host`

Owns host boundary.

Responsibilities:

```text
capability injection
module resolver interface
host function interface
resource abstraction
clock/random/fs/net/process/env boundaries
foreign boundary stubs
```

### 5.8 `vm_tests`

Owns conformance and integration tests.

Responsibilities:

```text
SIR validation tests
runtime semantics tests
module tests
unwinding tests
capability tests
diagnostic tests
negative tests
```

---

## 6. VM Execution Layers

The VM may have multiple execution layers, but Phase 3 only requires the first.

```text
Layer 0: SIR correctness interpreter
Layer 1: RuntimePlan-driven interpreter
Layer 2: EIR fast interpreter
Layer 3: baseline JIT
Layer 4: optimizing JIT
```

### 6.1 Layer 0 · SIR Interpreter

Required for Phase 3 as correctness tier.

Executes canonical SIR directly.

Advantages:

```text
maximal semantic visibility
simpler diagnostics
less lowering complexity
direct validation of Phase 2
```

Disadvantages:

```text
slower execution
more runtime branching
less optimized control flow
```

Layer 0 is acceptable only as a correctness tier. It must not become the final production execution architecture.

### 6.2 Layer 1 · RuntimePlan-Driven Interpreter

Deferred.

RuntimePlan precomputes:

```text
binding slots
field indices
enum case indices
call sites
access sites
control-region plans
pattern plans
type descriptor lookups
```

### 6.3 Layer 2 · EIR Fast Interpreter

Deferred.

EIR may be closer to executable control states.

Still not public bytecode.

### 6.4 Layer 3/4 · Baseline and Optimizing JIT

JIT implementation is staged, but JIT architecture is mandatory. It must not affect Phase 3 semantic baseline.

---

## 7. Runtime Value Model

The VM must represent all Phase 1 value kinds:

```text
Nil
Bool
Int
Float
String
List
Map
Range
RecordType
RecordInstance
EnumType
EnumValue
ReadOnlyView
Function
BuiltinFunction
Module
Error
```

### 7.1 Rust Representation Principle

The Rust representation should prefer explicit enums and handles.

Conceptual shape:

```rust
enum Value {
    Nil,
    Bool(bool),
    Int(IntValue),
    Float(FloatValue),
    String(StringHandle),
    List(ListHandle),
    Map(MapHandle),
    Range(RangeValue),
    RecordType(RecordTypeHandle),
    RecordInstance(RecordHandle),
    EnumType(EnumTypeHandle),
    EnumValue(EnumValueHandle),
    ReadOnlyView(ReadOnlyViewHandle),
    Function(FunctionHandle),
    BuiltinFunction(BuiltinFunctionId),
    Module(ModuleHandle),
    Error(ErrorHandle),
}
```

This is conceptual, not final Rust API.

### 7.2 Object Identity

Runtime object identity must not be based on exposed raw pointers.

The VM may internally use arena indices, generational IDs, handles, or reference-counted objects.

The language-level `is` semantics must be implemented independently from host pointer exposure.

### 7.3 Heap Strategy

The first VM may use simple host-managed allocation.

Acceptable initial strategies:

```text
typed arena
generational arena
index-based heap
Rc/Arc handles only as bootstrap detail
```

A moving GC is not required for Phase 3.

However, the runtime representation must preserve a path to tracing, generational, and moving GC. CPython-style reference counting must not become the architecture.

### 7.4 Interior Mutability

Mutable runtime aggregates require controlled mutation.

Rust implementation may use:

```text
RefCell
RwLock
custom heap borrow protocol
arena-mediated mutation
```

The chosen strategy must prevent unsound aliasing in the VM implementation.

---

## 8. Execution State

### 8.1 VM

```text
VM {
  config
  feature_set
  module_environment
  capability_environment
  heap
  call_stack
  diagnostics
}
```

### 8.2 ExecutionContext

```text
ExecutionContext {
  current_module
  current_scope
  current_function
  current_region
  pending_control
}
```

### 8.3 Frame

```text
Frame {
  function
  module
  locals
  region_stack
  source_span
}
```

Frames are required for:

```text
function calls
return handling
stack traces
diagnostics
future debugging
```

### 8.4 Region Stack

The VM must track structured regions:

```text
FunctionRegion
LoopRegion
TryRegion
CatchRegion
FinallyRegion
UseRegion
BlockRegion
TestRegion
```

The region stack is necessary for:

```text
return
break
continue
raise
defer
use
finally
suppressed errors
```

---

## 9. Control Flow Execution

The VM must not use Rust panics for normal language control flow.

The VM must represent language control flow explicitly:

```rust
enum Control {
    Normal(Value),
    Return(Value),
    Break,
    Continue,
    Raise(ErrorHandle),
}
```

This is conceptual, not final API.

### 9.1 Required Control Semantics

The VM must preserve:

```text
left-to-right evaluation
Bool-only conditions
short-circuit logical operators
single-evaluation chained comparisons
default arguments evaluated at call time
assignment target evaluated once
structured unwinding
finally override behavior
use close behavior
defer LIFO behavior
primary/suppressed error behavior
```

### 9.2 Unwinding

The VM must implement Phase 2 structured unwinding.

Required order:

```text
block-local defers
use cleanup
finally
pending control propagation
```

The exact implementation may use a region stack, explicit cleanup frames, or lowered NIR later.

For Layer 0 SIR interpreter, a region stack is the preferred design.

---

## 10. Module Execution

The VM must implement Phase 2 module states:

```text
Unloaded
Loading
Initializing
Initialized
Failed
```

### 10.1 Module Environment

The module environment stores:

```text
module identity
module state
module scope
exports
interface descriptor
initialization error
```

### 10.2 Import Execution

The VM must execute imports in source order.

Named imports bind immutable values.

Whole-module imports bind module values.

Wildcard import is impossible by Phase 1 and Phase 2.

### 10.3 Circular Imports

The VM must detect access to uninitialized exports and raise `ImportCycleError`.

---

## 11. Capability Environment

The VM must not grant ambient authority by default.

Host effects must be capability-gated.

Capability environment:

```text
CapabilityEnvironment {
  fs?
  net?
  process?
  env?
  clock?
  random?
  ffi?
}
```

For Phase 3, capability objects may be stubs.

The VM must still enforce that effectful host access cannot occur without granted capability.

### 11.1 FFI

Phase 3 does not implement FFI.

It reserves the boundary.

Any future FFI must remain separate from SIR and must not expose Rust or VM object layout as ABI.

---

## 12. Diagnostics

The VM must produce structured diagnostics.

Diagnostics must include where possible:

```text
diagnostic code
message
source span
related spans
phase
stack trace
runtime value category
```

Runtime errors should map to Phase 1 categories:

```text
NameError
TypeError
TypeContractError
PatternMatchError
ReadOnlyError
AssertionError
ArityError
IndexError
KeyError
FieldError
ImportError
ImportCycleError
DivisionByZeroError
NumericOverflowError
```

---

## 13. Validation Gate

The VM must not execute invalid SIR.

Before execution, the VM must require validation through:

```text
V0 Schema
V1 References
V2 Tables
V3 Node Semantics
V4 Control Flow
V5 Module Interface
V6 Dependency Compatibility
V7 Capability Safety
V8 Lowering Preconditions
```

For early Phase 3, the validator may be incomplete only if execution is limited to a declared subset and the subset is explicitly documented.

---

## 14. Minimal VM Subset

The minimal Phase 3 VM should be staged.

### 14.1 Stage A · Structural Runtime

Supports:

```text
SIR loading from in-memory structures
SIR validation subset
RuntimeValue core
diagnostics
module environment skeleton
```

### 14.2 Stage B · Expressions and Bindings

Supports:

```text
literals
bindings
let/const
assignment
unary/binary/logical expressions
calls
functions
type checks
```

### 14.3 Stage C · Aggregates

Supports:

```text
list
map
record
enum
attribute/index/slice
format string
readonly view
```

### 14.4 Stage D · Control Flow

Supports:

```text
block
if
while
for
return
break
continue
raise
```

### 14.5 Stage E · Structured Runtime Semantics

Supports:

```text
match/patterns
try/catch/finally
use/defer
assert/test
structured unwinding
primary/suppressed errors
```

### 14.6 Stage F · Modules and Capabilities

Supports:

```text
module initialization
imports/exports
interface validation
capability checks
host boundary stubs
```

These stages are implementation staging, not language feature staging.

The frozen semantics remain the target throughout.

---

## 15. Performance Strategy

Phase 3 performance strategy:

```text
correctness first
then structural efficiency
then profiling
then normalization
then specialization
```

The VM must avoid early optimization that changes architecture.

Accepted early optimizations:

```text
symbol interning
ID-indexed tables
arena handles
prevalidated references
field ID lookup
enum case ID lookup
cached type descriptor checks
```

Deferred optimizations:

```text
inline caches
object shape specialization
SSA
JIT implementation
native code generation
escape analysis
moving GC
parallel execution
```

---

## 16. Safety Strategy

Safety means:

```text
memory safety
capability safety
schema safety
module compatibility safety
foreign boundary safety
diagnostic safety
```

The VM must prefer rejection over guessed execution.

The VM must not execute:

```text
unknown required feature
invalid SIR
missing capability
incompatible module interface
foreign access without ffi boundary
malformed control region
malformed pattern table
```

---

## 17. Compatibility Strategy

The VM must support Phase 2 compatibility rules.

It must understand:

```text
IR schema version
feature flags
required extensions
optional extensions
module interface digest
dependency interface digest
cache digest
capability environment digest
```

The VM may reject unsupported SIR.

It must not reinterpret unsupported SIR.

---

## 18. VM Framework Non-Goals

Phase 3 framework does not define:

```text
public bytecode
bytecode loader
binary package format
CPython C-extension compatibility
Python wheel compatibility
native plugin ABI
JIT
AOT
moving GC
async runtime
thread scheduler
debugger protocol
profiler format
full standard library
package manager
```

---

## 19. Framework Completion Criteria

The Phase 3 framework is ready when it defines:

```text
technology stack
workspace architecture
VM identity
runtime value model
execution state
control-flow execution model
module execution model
capability environment
diagnostic model
validation gate
minimal implementation stages
performance strategy
safety strategy
compatibility strategy
non-goals
```

This document provides the initial framework.

Concrete VM semantics should be filled in later rounds.

---

## 20. Next Work

Next Phase 3 documents should define:

```text
VM runtime value representation
heap/object handle model
module environment concrete schema
call frame and region stack semantics
evaluator function contracts
runtime error model
builtin function boundary
capability host API
minimal conformance test plan
```

---

## 21. Performance Architecture Amendment

The VM must be high-performance aware from the beginning.

The corrected Phase 3 rule is:

```text
JIT implementation: staged
JIT architecture: mandatory
```

The VM may start with a SIR interpreter, but SIR interpretation is a correctness tier only.

The production execution direction is:

```text
SIR
  -> validation
  -> RuntimePlan
  -> EIR
  -> fast interpreter
  -> baseline JIT
  -> optimizing JIT
```

Required architectural commitments:

```text
RuntimePlan before hot execution
EIR fast interpreter as production interpreter target
slot-based locals
field-index access
enum case-index access
CallSiteId
AccessSiteId
inline cache state
type feedback
shape feedback
safepoints
deopt metadata
frame maps
root maps
write barrier hook
runtime helper table
backend abstraction
```

The VM must not become dependent on:

```text
CPython-style reference counting
Rust enum memory layout as ABI
HashMap<String, Value> hot locals
string-based record field lookup
SIR table traversal in hot paths
public bytecode
single JIT backend lock-in
```

The Rust `Value` enum is a conceptual semantic model, not a frozen physical ABI.

A Cranelift-compatible backend is the recommended first baseline JIT direction, but backend lock-in is forbidden.
