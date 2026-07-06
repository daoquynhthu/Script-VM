# Phase 3 · Normative Consistency Audit

Document class: Administrative audit  
Planning status: This document records a normative-specification audit. It is not itself a normative specification and does not modify Phase 3 semantics.

Updated: 2026-06-29 09:28:58

## Repair Progress

```text
R1 complete
R2 complete
R3 complete
R4 complete
R5 complete
R6 complete
R7 complete
R8 complete
R9 complete
R10 complete
R11 complete
R12 complete
R13 complete
R14 complete
R15 complete
```

Detailed progress is tracked in:

```text
PHASE-3-AUDIT-REPAIR-LOG.md
```

Second audit result is recorded in:

```text
PHASE-3-SECOND-NORMATIVE-AUDIT.md
```

Original blocker set B-01 through B-08 has been addressed.

---

## 0. Audit Verdict

Phase 3 is **not yet normative-freeze-ready**.

Current state:

```text
Normative architecture skeleton: present
Normative boundary taxonomy: repaired
Normative completeness: incomplete
Cross-document consistency: not yet sufficient
Implementation-plan contamination in aggregates: repaired
Implementation-plan residue inside some normative docs: still needs review
Freeze readiness: no
```

The next action should be **normative repair**, not implementation planning.

---

## 1. Scope

This audit covers only current Phase 3 normative specification documents.

| File | Present | Title |
|---|---:|---|
| `PHASE-3-VM-FRAMEWORK.md` | yes | Phase 3 · Minimal VM Framework Specification |
| `PHASE-3-VM-RUNTIME-ROUND1.md` | yes | Phase 3 · VM Runtime Semantics · Round 1 |
| `PHASE-3-PERFORMANCE-ARCHITECTURE.md` | yes | Phase 3 · Performance and JIT Architecture |
| `PHASE-3-RUNTIMEPLAN-EIR-FRAMEWORK.md` | yes | Phase 3 · RuntimePlan and EIR Framework |
| `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` | yes | Phase 3 · EIR Operation Semantics · Round 1 |
| `PHASE-3-SIR-LOWERING-ROUND1.md` | yes | Phase 3 · SIR to RuntimePlan / EIR Lowering · Round 1 |
| `PHASE-3-CONTROL-LOWERING-ROUND2.md` | yes | Phase 3 · Structured Control and Unwinding Lowering · Round 2 |
| `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` | yes | Phase 3 · Runtime Helper Contracts |
| `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md` | yes | Phase 3 · GC Root Enumeration and Safepoint Model |
| `PHASE-3-BASELINE-JIT-BACKEND-INTERFACE.md` | yes | Phase 3 · Baseline JIT Backend Interface |
| `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md` | yes | Phase 3 · Fast Interpreter Concrete Data Structures |
| `PHASE-3-JIT-LOWERING-MATRIX.md` | yes | Phase 3 · JIT Lowering Matrix per EIR Operation |

Implementation plan documents are not audited as normative sources:

```text
PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md
PHASE-3-GC-IMPLEMENTATION-STAGING-PLAN.md
PHASE-3-FAST-INTERPRETER-IMPLEMENTATION-MILESTONES.md
```

They are referenced only for boundary verification.

---

## 2. Severity Definitions

```text
BLOCKER
  Prevents Phase 3 normative freeze.

MAJOR
  Must be fixed before freeze unless explicitly deferred with a normative rationale.

MINOR
  Wording, classification, or local clarity issue. Does not usually block freeze alone.

INFO
  Tracking note or later-stage concern.
```

---

## 3. Blockers

## B-01 · EIR instruction schema is not closed

Severity: BLOCKER  
Area: EIR, RuntimePlan, fast interpreter, JIT

### Finding

The current EIR specification defines operation families and semantic expectations, but does not yet define a final closed instruction schema.

Current documents define families such as:

```text
LoadOp
StoreOp
UnaryOp
BinaryOp
LogicalOp
CheckOp
CallOp
AccessOp
ConstructOp
PatternOp
RuntimeHelperOp
SafepointOp
GuardOp
```

and terminators such as:

```text
Jump
Branch
Return
Raise
LoopBackedge
Switch
Unwind
Unreachable
```

However, the normative shape of each concrete operation is not fully closed.

### Why this blocks freeze

The fast interpreter, JIT lowering matrix, validation rules, source mapping, safepoints, and deopt metadata all depend on precise EIR op schemas.

A freeze-ready VM spec must answer:

```text
What exact fields does each EIR op contain?
Which fields are required?
Which fields are optional?
Which IDs must resolve?
Which checks are semantic vs speculative?
Which operations can raise?
Which operations can allocate?
Which operations are safepoints?
```

### Required repair

Create or revise a normative EIR schema section that defines a closed minimal EIR instruction set.

At minimum, close:

```text
ConstantOp
LoadSlot
LoadCell
LoadCapture
LoadModuleSlot
LoadField
LoadEnumPayload
StoreSlot
StoreCell
StoreModuleSlot
StoreField
StoreListIndex
StoreMapEntry
UnaryOp
BinaryOp
CheckOp
CallOp
AccessOp
ConstructOp
PatternOp
RuntimeHelperOp
SafepointOp
GuardOp
Jump
Branch
Return
Raise
LoopBackedge
Switch
Unwind
Unreachable
```

---

## B-02 · RuntimePlan schema is still framework-level, not freeze-level

Severity: BLOCKER  
Area: RuntimePlan, lowering, validation

### Finding

RuntimePlan has a strong architectural framework, but not all tables and fields are fully closed with validation rules.

Examples needing closure:

```text
ModulePlan
FunctionPlan
TypePlan
ShapePlan
SlotLayout
CaptureLayout
CallSiteTable
AccessSiteTable
SafepointSeedTable
DeoptSeedTable
CapabilityGatePlan
RuntimeHelperTable link
```

### Why this blocks freeze

RuntimePlan is the bridge from frozen SIR to EIR. If RuntimePlan is not closed, SIR lowering cannot be freeze-stable.

The current documents define enough direction, but not enough final schema detail to guarantee independent implementation conformance.

### Required repair

Add a normative RuntimePlan schema closure pass.

Each RuntimePlan table should define:

```text
schema
required fields
ID references
ownership
validation rules
cache digest participation
source mapping role
lowering responsibility
```

---

## B-03 · Helper names are referenced outside the helper contract without a canonical helper registry

Severity: BLOCKER  
Area: Runtime helpers, JIT, interpreter

### Finding

`PHASE-3-RUNTIME-HELPER-CONTRACTS.md` defines helper families and many helper names.

Other normative documents introduce additional helper names or helper-like concepts, including examples such as:

```text
helper_load_cell
helper_store_cell
helper_string_concat
helper_membership
helper_construct_function
helper_load_module_slot
helper_check_arity
helper_numeric_unary
```

These are not all canonically registered in one normative helper registry.

### Why this blocks freeze

The JIT lowering matrix and EIR validation refer to helper IDs. If helper names can be introduced ad hoc across documents, the VM cannot validate `RuntimeHelperOp` deterministically.

### Required repair

Create a canonical normative helper registry inside `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` or a dedicated normative helper registry section.

For every helper, define:

```text
helper_id policy
canonical name
family
signature
result type
may_allocate
may_raise
may_unwind
is_safepoint
requires_roots_visible
capability/effect metadata
source mapping policy
JIT call policy
```

Then remove or normalize all helper names outside the registry.

---

## B-04 · Error taxonomy is not centralized

Severity: BLOCKER  
Area: runtime errors, diagnostics, Phase 1/2 compatibility

### Finding

Phase 3 uses and sometimes introduces error categories across multiple documents.

Examples include:

```text
NameError
UninitializedBindingError
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
CapabilityError
InternalVMError
StackOverflowError
ResourceStateError
```

Some appear as required core runtime errors. Others appear as examples or "equivalent" categories.

### Why this blocks freeze

Error categories are observable through diagnostics and exception handling. They are part of runtime semantics.

The spec needs a central registry that distinguishes:

```text
language-level Error codes
VM internal structural errors
diagnostic-only errors
host-boundary translated errors
future/deferred error categories
```

### Required repair

Add a normative runtime error registry.

Each error must define:

```text
code
category
language-observable status
where it can be raised
source mapping requirement
relationship to Phase 1/Phase 2
whether it is required in minimal VM
```

---

## B-05 · PendingControl / VmControl / TerminatorResult are not fully unified

Severity: BLOCKER  
Area: control flow, unwinding, interpreter, deopt

### Finding

Several related control-state concepts exist:

```text
VmControl
Control
PendingControl
PendingControlValue
TerminatorResult
OpResult
```

They are directionally aligned, but not yet canonically unified.

Examples of possible drift:

```text
Break / Continue sometimes appear as bare states.
Break / Continue sometimes carry ControlRegionId.
PendingControl may use Raise(ErrorHandle).
TerminatorResult may use Raise(ErrorHandle) or Unwind(VmControl).
```

### Why this blocks freeze

Structured unwinding, deopt reconstruction, JIT control transfer, and interpreter dispatch all depend on a single control-state model.

### Required repair

Define one canonical control-state model and explicitly map all execution-layer result types to it.

At minimum:

```text
Expression result
Statement result
EIR op result
Terminator result
Pending control
Frame return result
Helper return
Unwind result
```

must be normalized.

---

## B-06 · Structured unwinding algorithm is not yet fully executable-spec closed

Severity: BLOCKER  
Area: defer/use/finally/unwind

### Finding

The documents specify the intended unwinding order:

```text
defers
resources
finally
```

and rules such as:

```text
defer LIFO
use close exactly once
finally override
suppressed cleanup errors
```

But the exact state machine is still split across runtime semantics, control lowering, helper contracts, and fast interpreter structures.

### Why this blocks freeze

Unwinding is one of the highest-risk semantic areas. Small ambiguity changes observable behavior.

The spec needs a single canonical algorithm for:

```text
Normal exit
Return exit
Break exit
Continue exit
Raise exit
defer raise
resource close raise
finally return
finally raise
suppressed error attachment
cleanup progress across reentrancy/deopt
```

### Required repair

Create a canonical normative "Structured Unwinding Algorithm" section.

It should define:

```text
state machine
cleanup ordering
pending control update
primary/suppressed error rules
finally override
deopt reconstruction state
validation requirements
```

---

## B-07 · Module initialization semantics are not closed at EIR/runtime level

Severity: BLOCKER  
Area: modules, imports, circular imports

### Finding

Phase 2 defines module states including:

```text
Unloaded
Loading
Initializing
Initialized
Failed
```

and allows `Failed -> Loading` only under explicit host retry.

Phase 3 documents define module helpers and module init lowering, but the exact EIR/runtime execution contract remains spread across documents.

Potential gap:

```text
Failed -> Loading retry behavior is not consistently carried into Phase 3 helper/module specs.
```

### Why this blocks freeze

Module initialization order and circular import behavior are observable semantics.

### Required repair

Add a canonical Phase 3 module runtime section covering:

```text
state transitions
retry policy
module init function shape
import helper behavior
export cell creation
export table sealing
circular import access
initialization error persistence
source-order execution
capability interaction with module resolver
```

---

## B-08 · SIR lowering coverage is not demonstrably complete

Severity: BLOCKER  
Area: SIR lowering, Phase 2 compatibility

### Finding

The lowering rules are split between:

```text
SIR Lowering Round 1
Structured Control and Unwinding Lowering Round 2
```

Together they cover much of Phase 2, but the spec does not yet provide a complete SIR-node coverage matrix.

### Why this blocks freeze

Phase 3 depends on Phase 2 frozen SIR. A freeze-ready VM spec must show every required SIR node is either:

```text
lowered
executed by defined fallback
rejected by validation
explicitly deferred outside minimal VM
```

### Required repair

Add a normative SIR-to-RuntimePlan/EIR coverage matrix for all Phase 2 SIR node variants:

```text
ModuleBodyNode
DeclarationNode
BindingNode
ExpressionNode
AssignmentNode
FunctionNode
RecordNode
EnumNode
BlockNode
IfNode
WhileNode
ForNode
MatchNode
ReturnNode
BreakNode
ContinueNode
RaiseNode
TryNode
UseNode
DeferNode
AssertNode
TestNode
CheckNode
PatternTable variants
```

---

## 4. Major Findings

## M-01 · Normative documents still contain plan-like language

Severity: MAJOR  
Area: document taxonomy

### Finding

The aggregate separation has been repaired, but some normative documents still contain language such as:

```text
recommended first backend
staged
bootstrap may
implementation order
initial implementation
later implementation
```

Not all such language is invalid. Some is legitimate architectural staging. But the spec currently lacks a formal convention distinguishing:

```text
normative requirement
permitted implementation option
non-normative implementation advice
future design direction
```

### Required repair

Introduce a normative keyword policy:

```text
MUST
MUST NOT
SHOULD
MAY
RECOMMENDED
DEFERRED
NON-NORMATIVE NOTE
```

Then mark planning-style text accordingly.

---

## M-02 · `PHASE-3-MINIMAL-VM.md` now aggregates all normative architecture, including non-minimal future hooks

Severity: MAJOR  
Area: aggregate semantics

### Finding

`PHASE-3-MINIMAL-VM.md` is a normative aggregate, but it includes advanced architecture documents:

```text
GC safepoint model
Baseline JIT backend interface
JIT lowering matrix
```

This is acceptable only if "minimal" means:

```text
minimal VM design with mandatory future-proofing hooks
```

not:

```text
minimal implementation subset
```

### Required repair

Clarify the title or boundary.

Possible repair:

```text
PHASE-3-MINIMAL-VM.md = minimal normative VM architecture
```

and explicitly state that:

```text
JIT implementation is not required in minimal VM.
JIT architecture hooks are required.
GC implementation is staged.
GC/safepoint architecture hooks are required.
```

---

## M-03 · RootMap / FrameMap / SafepointRecord definitions are duplicated across documents

Severity: MAJOR  
Area: GC, JIT, interpreter, RuntimePlan

### Finding

These concepts appear in multiple documents:

```text
RootMap
FrameMap
SafepointRecord
StackMap
DeoptPointRecord
RegionStackState
```

The definitions are compatible in direction, but not canonically owned by a single document.

### Risk

Schema drift.

### Required repair

Assign canonical ownership:

```text
GC/safepoint model owns RootMap and SafepointRecord.
RuntimePlan/EIR framework owns EIR-level references.
Baseline JIT interface owns JIT stack-map projection.
Fast interpreter data structures owns interpreter runtime use.
```

Then all secondary documents must reference canonical schema instead of redefining it.

---

## M-04 · ValueLayoutProfile is referenced before being normatively defined

Severity: MAJOR  
Area: JIT, value representation, cache compatibility

### Finding

JIT and cache documents reference:

```text
ValueLayoutProfile
HeapProfile
GcProfile
JitTargetProfile
CallingConventionProfile
```

but some are only described conceptually.

### Required repair

Create a normative target/profile schema section defining:

```text
ValueLayoutProfile
HeapProfile
GcProfile
InterpreterProfile
JitProfile
TargetProfile
```

with digest participation and invalidation rules.

---

## M-05 · Capability environment mutability is underspecified

Severity: MAJOR  
Area: capability checks, JIT guards, cache invalidation

### Finding

Capability checks appear in:

```text
lowering
helpers
JIT
host boundary
module import
```

But the spec does not fully define whether the capability environment is immutable for a VM execution context, dynamically mutable, module-local, frame-local, or host-call scoped.

### Risk

JIT may incorrectly cache capability checks.

### Required repair

Define:

```text
capability environment lifetime
mutation policy
digest/epoch policy
JIT guard invalidation
module import interaction
host boundary interaction
```

---

## M-06 · Map key/hash/equality semantics need closure

Severity: MAJOR  
Area: maps, hashability, equality

### Finding

Map semantics depend on:

```text
hashability
ValueKey
equality
duplicate replacement
insertion order
Float handling
EnumValue handling
mutable payload restriction
```

The current spec gives direction but not a complete canonical key model.

### Required repair

Define `ValueKey` normatively.

Clarify:

```text
allowed key types
Float key behavior
NaN/finite float policy
-0.0 / 0.0 policy
EnumValue hash/equality policy
record/list/map non-hashability
readonly view hashability
hash stability
```

---

## M-07 · String model needs sharper runtime constraints

Severity: MAJOR  
Area: strings, slicing, indexing, display

### Finding

The spec says string indexing is not core and slicing must not split invalid scalar boundaries.

However, string representation needs closure:

```text
Unicode scalar sequence vs UTF-8 bytes
slice bounds unit
source span for invalid bounds
display conversion
constant interning and identity
```

### Required repair

Define:

```text
string length unit
slice index unit
valid boundary rule
identity behavior under interning
display representation contract
```

---

## M-08 · Function call protocol is split across helper, EIR, interpreter, lowering

Severity: MAJOR  
Area: calls, defaults, closures, builtins

### Finding

Function call semantics are repeated in several documents.

The central concerns are:

```text
callee evaluation
argument evaluation
named argument layout
default evaluation at call time
parameter contract checks
frame creation
capture binding
return contract
builtin call
host call
call-site feedback
```

### Required repair

Create a canonical "Call Execution Protocol" section and make EIR/helper/interpreter/JIT documents reference it.

---

## M-09 · `ReadOnlyView` identity and mutation semantics need single ownership

Severity: MAJOR  
Area: readonly views, identity, mutation

### Finding

The spec states readonly views are shallow and `readonly(x) is x` is false unless future optimization allows otherwise.

Other parts discuss readonly view delegation and mutation rejection.

### Risk

A VM might optimize readonly view identity incorrectly.

### Required repair

Centralize readonly-view rules:

```text
identity
equality
delegated read
mutation through view
mutation through original object
nested object behavior
JIT guard assumptions
helper behavior
```

---

## M-10 · Validation levels from Phase 2 are referenced but Phase 3 validation gates need a unified table

Severity: MAJOR  
Area: validation

### Finding

Phase 2 defines V0-V8 validation levels. Phase 3 adds:

```text
RuntimePlan validation
EIR validation
helper validation
GC validation
JIT validation
fast interpreter structure validation
```

These are currently distributed.

### Required repair

Create a unified Phase 3 validation matrix:

```text
input
validation pass
required before execution
required before lowering
required before JIT
required before GC
failure category
diagnostic category
```

---

## M-11 · Internal cache compatibility rules are distributed and need consolidation

Severity: MAJOR  
Area: cache invalidation

### Finding

Cache keys and invalidation rules appear in:

```text
RuntimePlan/EIR
helpers
GC
JIT
interpreter
```

### Required repair

Add a canonical cache compatibility section covering:

```text
RuntimePlan cache
EIR cache
helper table digest
GC profile digest
ValueLayoutProfile digest
JIT cache
module dependency interface digest
capability environment digest/epoch
stdlib interface digest
```

---

## M-12 · Some normative docs use "recommended" implementation choices without marking non-normative status

Severity: MAJOR  
Area: document wording

### Finding

Examples:

```text
Cranelift-compatible backend recommended first
generational arena recommended
handle table relocation recommended
```

Recommendations can be acceptable, but they must be marked as:

```text
RECOMMENDED implementation option
```

not semantic requirement.

### Required repair

Adopt normative keyword policy and annotate.

---

## M-13 · Host boundary and future FFI need clearer "deferred but constrained" status

Severity: MAJOR  
Area: host boundary, FFI, capabilities

### Finding

FFI is repeatedly deferred, but host functions and HostObjectWrapper are already referenced.

### Required repair

Define:

```text
host function wrapper status in minimal VM
host object wrapper status
FFI deferred boundary
which host operations are permitted before FFI
which capability checks are mandatory
```

---

## M-14 · JIT lowering matrix depends on helper and op registries not yet canonical

Severity: MAJOR  
Area: JIT

### Finding

The JIT lowering matrix is useful, but it is downstream of:

```text
closed EIR op registry
canonical helper registry
canonical error registry
canonical safepoint/root map schema
```

Since those are not yet closed, the JIT matrix cannot itself be freeze-stable.

### Required repair

Treat JIT lowering matrix as normative only after B-01, B-03, B-04, and M-03 are repaired.

---

## M-15 · Testing/conformance requirements are outside normative spec

Severity: MAJOR  
Area: freeze readiness

### Finding

Testing/conformance is currently mostly in plan documents.

For freeze readiness, the normative spec needs at least a conformance requirement map, not implementation milestones.

### Required repair

Create a normative conformance manifest that maps:

```text
language feature
Phase 2 SIR source
Phase 3 execution component
required positive tests
required negative tests
required diagnostics
```

This should not be an implementation plan; it is a normative freeze criterion.

---

## 5. Minor Findings

## m-01 · Naming convention drift

Severity: MINOR

Examples:

```text
EIR / Eir
VMControl / VmControl
ObjectId / ObjRef / ObjRef<T>
SafepointTable / JitSafepointTable
RuntimeHelperOp / helper call
```

### Repair

Add a terminology appendix or glossary.

---

## m-02 · Version dependency headers need normalization

Severity: MINOR

Some documents depend on specific earlier drafts; after repair, dependency headers should be checked for consistency.

### Repair

Normalize all headers to the current Phase 3 document graph.

---

## m-03 · "Out of scope" sections can appear contradictory across layered docs

Severity: MINOR

Example pattern:

```text
early framework says JIT implementation out of scope
later normative docs define JIT architecture/interface
```

This is not necessarily wrong. It needs clearer distinction between:

```text
implementation out of scope
architecture in scope
```

---

## m-04 · Administrative tracking files should not use specification-like titles

Severity: MINOR

`PHASE-3-CHANGELOG.md` starts as a VM specification changelog. It is correctly administrative now, but future wording should keep this clear.

---

## m-05 · Some "bootstrap" language should be converted to explicit MAY clauses

Severity: MINOR

Examples:

```text
bootstrap may use Rc/RefCell
bootstrap may enumerate all initialized slots
bootstrap may implement unwinding through helper
```

These should use consistent normative wording.

---

## m-06 · StackOverflowError status is uncertain

Severity: MINOR

The milestones document is now a plan, but normative runtime docs should avoid "or equivalent" language for errors.

If StackOverflowError is required, add it to the error registry. If not, mark as deferred.

---

## m-07 · Test blocks and ordinary module initialization need a sharper boundary

Severity: MINOR

The direction is clear: tests do not execute during ordinary module init.

The runtime representation of TestPlan and test runner entry should be classified as either normative minimal VM or host tooling.

---

## 6. Info Findings

## i-01 · Document taxonomy repair was necessary and should remain enforced

The new taxonomy is correct:

```text
PHASE-3-VM-SPEC.md -> normative aggregate only
PHASE-3-MINIMAL-VM.md -> normative aggregate only
PHASE-3-IMPLEMENTATION-PLAN.md -> implementation plans only
```

Future generation must preserve this separation.

---

## i-02 · The architecture direction remains strong

Despite the blockers, the main architecture direction is coherent:

```text
source-first
no public bytecode
RuntimePlan/EIR internal only
slot-based execution
fixed-shape records
closed enums
helper slow paths
GC root/safepoint awareness
JIT backend abstraction
no CPython ABI
no CPython refcount architecture
```

The remaining work is closure and consistency, not a restart.

---

## i-03 · Plans should remain available but non-normative

The existing implementation plans are useful for later execution.

They should not be deleted.

They should remain subordinate to normative specs.

---

## 7. Required Repair Order

Recommended repair sequence:

```text
R1: Add normative keyword policy and terminology glossary.
R2: Create central runtime error registry.
R3: Close EIR operation schema.
R4: Close RuntimePlan schema.
R5: Create canonical helper registry.
R6: Unify control-state model.
R7: Canonicalize structured unwinding algorithm.
R8: Close module initialization/runtime contract.
R9: Add SIR node lowering coverage matrix.
R10: Canonicalize RootMap/FrameMap/SafepointRecord ownership.
R11: Define ValueLayoutProfile / HeapProfile / GcProfile / TargetProfile schemas.
R12: Define map key/hash/equality and string runtime constraints.
R13: Create unified Phase 3 validation matrix.
R14: Create canonical cache compatibility matrix.
R15: Run second audit after repairs.
```

---

## 8. Freeze Readiness Criteria

Phase 3 should not freeze until all blocker findings are resolved.

Freeze-readiness requires:

```text
no blocker findings
all major findings fixed or explicitly deferred with normative rationale
closed EIR op schema
closed RuntimePlan schema
canonical helper registry
canonical error registry
canonical control-state model
canonical unwinding algorithm
closed module runtime semantics
complete SIR lowering coverage matrix
normative/plan separation preserved
no public bytecode leak
no CPython ABI leak
no refcount architecture leak
no production SIR-walk path leak
```

---

## 9. Explicit Non-Actions

This audit does not:

```text
modify normative specifications
modify implementation plans
freeze Phase 3
approve implementation start
delete existing planning documents
```

It creates a repair reference for the next normative-specification pass.
