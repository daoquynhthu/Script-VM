# Phase 3 · Unified Validation Matrix

Document class: Normative specification  
Normative status: This document defines the unified Phase 3 validation matrix across SIR, RuntimePlan, EIR, helpers, modules, GC metadata, JIT metadata, and cache compatibility.

Created: 2026-06-29 09:28:58

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
R13: Create unified Phase 3 validation matrix.
```

It addresses major finding:

```text
M-10: Validation levels from Phase 2 are referenced but Phase 3 validation gates need a unified table.
```

It also supports freeze-readiness and conformance by defining when malformed internal state must be rejected.

---

## 1. Validation Boundary

Phase 3 validation is layered.

No executable tier may run unchecked data from a previous tier.

```text
Source / AST
  -> SIR
  -> RuntimePlan
  -> EIR
  -> Fast Interpreter
  -> Baseline JIT
  -> Runtime execution
```

Each boundary MUST validate its input before use.

---

## 2. Validation Passes

```text
P3-V0: Phase 2 SIR acceptance gate
P3-V1: Normative feature and document-boundary gate
P3-V2: RuntimePlan schema validation
P3-V3: RuntimePlan semantic validation
P3-V4: EIR schema validation
P3-V5: EIR semantic validation
P3-V6: Helper registry validation
P3-V7: Module runtime validation
P3-V8: Capability/effect validation
P3-V9: GC metadata validation
P3-V10: JIT metadata validation
P3-V11: Cache compatibility validation
P3-V12: Execution preflight validation
P3-V13: Conformance validation
```

---

## 3. P3-V0 · Phase 2 SIR Acceptance Gate

Input:

```text
Phase 2 frozen SIR
```

Required before:

```text
RuntimePlan construction
```

Must verify:

```text
SIR schema valid
Phase 2 validation V0-V8 passed or equivalent
feature set allowed
no unresolved IDs
source spans available for source-originated nodes
```

Failure category:

```text
DiagnosticError
```

---

## 4. P3-V1 · Normative Feature and Boundary Gate

Input:

```text
SIR feature set
VM target profile
Phase 3 document manifest
```

Must reject:

```text
public bytecode expectation
CPython ABI expectation
Python wheel compatibility expectation
native object layout ABI expectation
production SIR-walk execution path
deferred feature without explicit enablement
```

Failure category:

```text
DiagnosticError or VmStructuralError
```

---

## 5. P3-V2 · RuntimePlan Schema Validation

Input:

```text
RuntimePlan
```

Required before:

```text
EIR generation
execution
cache storage
```

Must reject:

```text
missing required RuntimePlan table
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
malformed ModulePlan
malformed FunctionPlan
malformed SlotLayout
profile mismatch
source SIR digest mismatch
```

Failure category:

```text
InvalidRuntimePlanError
```

---

## 6. P3-V3 · RuntimePlan Semantic Validation

Must verify:

```text
module initialization function exists
module slot layout exists
function parameter/default/capture layouts are consistent
record field index maps complete
enum case index maps complete
all call sites have source spans
all access sites have source spans
capability gates cover declared effects
helper requirements resolve in helper registry
safepoint seeds required by plan are present
deopt seeds required by speculative operations are present
```

Failure category:

```text
InvalidRuntimePlanError
```

---

## 7. P3-V4 · EIR Schema Validation

Input:

```text
EirModule
```

Required before:

```text
fast interpreter execution
JIT compilation
EIR cache storage
```

Must reject:

```text
unknown EIR op kind
unknown EIR terminator kind
block without terminator
fallthrough block
unknown SlotId
unknown ConstantId
unknown RuntimeHelperId
unknown CallSiteId
unknown AccessSiteId
unknown SafepointId
unknown DeoptId
invalid block target
invalid block argument count
```

Failure category:

```text
InvalidEirError
```

---

## 8. P3-V5 · EIR Semantic Validation

Must verify:

```text
may-raise operations have source mapping
may-collect helpers have root map/safepoint metadata
heap writes have barrier policy
guards have failure action
Return/Break/Continue/Raise crossing cleanup enter unwind path
Bool-only branch conditions
non-public bytecode invariant preserved
```

Failure category:

```text
InvalidEirError
```

---

## 9. P3-V6 · Helper Registry Validation

Input:

```text
RuntimeHelperRegistry
RuntimeHelperTable
helper references from RuntimePlan/EIR/JIT
```

Must reject:

```text
duplicate helper id
duplicate helper name
helper descriptor without implementation
implementation without descriptor
RuntimeHelperOp referencing missing helper
may_raise helper without source mapping policy
may_allocate/may_collect helper without roots-visible policy
JIT-callable helper without JIT call policy
capability helper without capability/effect metadata
```

Failure category:

```text
InvalidHelperError
```

---

## 10. P3-V7 · Module Runtime Validation

Input:

```text
ModulePlan
ModuleInstance
ImportPlan
ExportPlan
ModuleState
```

Must reject:

```text
module without initialization function
invalid module state transition
automatic retry after Failed
duplicate export names
export table mutation after sealing
named import of missing export
uninitialized circular export access
top-level return/break/continue
effectful resolver without capability declaration
```

Failure category:

```text
ImportError
ImportCycleError
InvalidRuntimePlanError
```

depending on whether the failure is source-level or structural.

---

## 11. P3-V8 · Capability/Effect Validation

Input:

```text
CapabilityGatePlan
FunctionPlan effects
CallSite effects
RuntimeHelperDescriptor capability metadata
Host boundary metadata
```

Must reject:

```text
effectful operation without capability metadata
host call without capability gate
JIT capability check elimination without guard/invalidation policy
mutable capability environment without epoch/digest policy
module resolver effect without capability declaration
```

Failure category:

```text
CapabilityError
InvalidRuntimePlanError
BackendViolationError
```

---

## 12. P3-V9 · GC Metadata Validation

Input:

```text
RootMap
FrameMap
SafepointRecord
StackMap
RegionStackState
PendingControlState
```

Must reject:

```text
safepoint without RootMap when GC may run
RootMap referencing unknown SlotId
FrameMap referencing unknown SlotLayout
moving GC profile with non-updateable roots
helper may_collect without SafepointRecord
JIT safepoint without StackMap
PendingControl with heap values not root-visible
cleanup state not reconstructable at deopt point
```

Failure category:

```text
InvalidRootMapError
InvalidStackMapError
InvalidFrameStateError
```

---

## 13. P3-V10 · JIT Metadata Validation

Input:

```text
JitCompileInput
CompiledFunction
StackMapTable
JitSafepointTable
JitDeoptTable
HelperCallSite metadata
```

Must reject:

```text
compiled helper call without descriptor
compiled may-collect helper without root map
compiled safepoint without stack map
compiled heap write without barrier path
compiled guard without failure path
compiled speculative op without deopt/helper fallback
compiled may-raise op without source map
compiled return that skips active cleanup
compiled raise that skips unwind
compiled host call that bypasses helper boundary
compiled capability op without check
compiled operation that assumes object address identity
compiled operation that assumes public Value layout
```

Failure category:

```text
BackendViolationError
InvalidStackMapError
InvalidDeoptError
```

---

## 14. P3-V11 · Cache Compatibility Validation

Input:

```text
RuntimePlan cache entry
EIR cache entry
JIT cache entry
helper registry digest
target/runtime profiles
module interface digests
capability profile
```

Must reject stale caches when any compatibility key component changes.

Cache validation failure is not language error.

It must cause:

```text
cache discard
rebuild
or VmStructuralError if cache is required but invalid
```

---

## 15. P3-V12 · Execution Preflight Validation

Before execution, VM MUST verify:

```text
RuntimePlan validated
EIR validated
helper registry validated
module environment initialized enough for requested entry
target profile compatible
capability environment policy known
GC metadata policy compatible with execution mode
JIT disabled or JIT metadata validated
```

Failure category:

```text
VmStructuralError
DiagnosticError
```

---

## 16. P3-V13 · Conformance Validation

Conformance test manifest MUST cover:

```text
expression semantics
binding/scope semantics
function call/default/closure semantics
record semantics
enum semantics
list/map/range semantics
ValueKey semantics
string slicing/display semantics
pattern semantics
structured unwinding
resource/defer/finally
module import/export/cycles
capability checks
runtime error categories
source diagnostics
readonly views
format strings
test blocks
```

Negative conformance MUST cover:

```text
truthiness rejected
implicit coercion rejected
integer overflow or arbitrary-precision policy validated
division by zero rejected
uninitialized binding read rejected
const assignment rejected
readonly mutation rejected
unknown record field rejected
invalid enum case rejected
non-Error raise rejected
break/continue/return invalid context rejected
missing capability rejected
uninitialized circular export rejected
public bytecode expectation rejected
CPython ABI expectation rejected
```

---

## 17. Validation Ordering

Required order:

```text
P3-V0
P3-V1
P3-V2
P3-V3
P3-V4
P3-V5
P3-V6
P3-V7
P3-V8
P3-V9
P3-V11
P3-V12
```

JIT-specific validation:

```text
P3-V10 after P3-V4/P3-V5/P3-V6/P3-V9/P3-V11
```

Conformance validation:

```text
P3-V13 before freeze
```

---

## 18. Audit Tracking

This document completes:

```text
R13
```

It addresses:

```text
M-10
M-15 partial
```

It supports:

```text
R15
```
