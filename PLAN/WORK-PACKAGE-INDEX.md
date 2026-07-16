# Work Package Index

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications, `AGENT-MASTER-PLAN.md`, and `IMPLEMENTATION-CODING-PLAN.md`  
Revised: 2026-07-10 (implementation status sync)

---

## 0. Purpose

This document defines concrete implementation work packages.

It is not a general governance checklist.

Each work package must be executable by an Agent as a bounded coding unit.

For step-by-step coding order, directory layout, crate layout, and per-stage required actions, use:

```text
IMPLEMENTATION-CODING-PLAN.md
```

This index owns work package identity, scope, dependencies, max Agent mode, gates, and completion criteria.

It does not define VM semantics.

When a work package requires semantic detail, it cites frozen specification documents.

---

## 1. Execution Rule

A work package is valid only if it answers:

```text
what files or directories may be created
what crate/module area is affected
what implementation step is next
what frozen specs control the work
what tests are required
what gates must pass
what must be recorded in PROGRESS.md
what audit findings must be recorded in ISSUE.md
```

A work package is invalid if it only says:

```text
review
consider
design
think about
align
```

without concrete coding outputs.

---

## 2. Work Package Status Values

```text
DRAFT
READY_FOR_G0
GATED
READY_FOR_IMPLEMENTATION
IN_PROGRESS
BLOCKED
COMPLETE
DEFERRED
REJECTED
```

### 1.1 DRAFT

The work package exists but lacks enough detail for gate review.

### 1.2 READY_FOR_G0

The package has scope, owner, and non-goals.

### 1.3 GATED

The package has passed G0-G3.

### 1.4 READY_FOR_IMPLEMENTATION

The package has passed G4 and has executable implementation inputs.

### 1.5 IN_PROGRESS

Implementation work has started.

### 1.6 BLOCKED

A required dependency, gate, or spec-reference condition blocks progress.

### 1.7 COMPLETE

All required gates and handoff requirements are satisfied.

### 1.8 DEFERRED

The package is intentionally delayed.

### 1.9 REJECTED

The package violates boundaries or is superseded.

---

## 3. Agent Mode Values

```text
main-only
main+1
main+2
main+3
main+4
```

Default:

```text
main-only
```

Any mode beyond `main-only` requires explicit parallelism justification.

---

## 4. Work Package Families

The initial work package families are:

```text
WP-00 Agent and repository process
WP-01 Frozen spec reference ingestion
WP-02 Traceability matrix construction
WP-03 ID and schema model skeleton
WP-04 Runtime error registry implementation
WP-05 RuntimePlan model and validation
WP-06 EIR model and validation
WP-07 Helper registry and dispatch
WP-08 Value / heap / object reference model
WP-09 Frame / slot / control-state model
WP-10 Structured unwinding implementation
WP-11 Module runtime implementation
WP-12 Call execution protocol implementation
WP-13 ReadOnlyView implementation
WP-14 Host boundary skeleton
WP-15 GC metadata structures
WP-16 Cache compatibility checks
WP-17 Fast interpreter core
WP-18 Conformance test matrix
WP-19 Integration and regression gates
WP-20 Phase 1 language pipeline process and traceability
WP-21 Phase 1 lexical analysis
WP-22 Phase 1 parser and AST (bootstrap COMPLETE)
WP-23 Phase 1 semantic binding skeleton (COMPLETE)\nWP-24 Phase 1 AST to bootstrap SIR (COMPLETE)\nWP-25 Bootstrap source to EIR execute (COMPLETE)
```

---

## 5. Work Package Template

Every work package must use this template.

```text
WP-ID:
Title:
Status:
Owner:
Agent Mode:
Parallelism Justification:
Frozen Spec References:
Inputs:
Outputs:
Non-Goals:
Dependencies:
Implementation Tasks:
Validation Gates:
Tests Required:
Risks:
Completion Criteria:
Handoff Notes:
```

Invalid if missing:

```text
Frozen Spec References
Non-Goals
Validation Gates
Completion Criteria
```

---

## 5. WP-00 · Agent and Repository Process

```text
WP-ID: WP-00
Title: Agent and repository process setup
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
Parallelism Justification: none
```

Frozen Spec References:

```text
SPEC-P3-FREEZE
AGENT-MASTER-PLAN.md
AGENT-OPERATING-PROTOCOL.md
GATE-CHECKLIST.md
HANDOFF-TEMPLATE.md
```

Inputs:

```text
frozen documentation archive
current repository or future repository root
Agent implementation plan directory
```

Outputs:

```text
repository layout proposal
task log convention
branch/commit discipline
test command convention
handoff storage convention
```

Non-Goals:

```text
no VM semantics
no implementation of runtime
no specification edits
```

Dependencies:

```text
none
```

Implementation Tasks:

```text
define repository working layout
define where Agent task logs live
define naming convention for work package branches/issues
define test command reporting format
define handoff filing convention
```

Validation Gates:

```text
G0
G1
G2
G3
G7
```

Tests Required:

```text
not applicable before repository exists
```

Risks:

```text
process overhead
unclear handoff storage
premature coding before traceability
```

Completion Criteria:

```text
repository process is explicit
task log convention exists
handoff path exists
no specification files are modified
```

---

## 6. WP-01 · Frozen Spec Reference Ingestion

```text
WP-ID: WP-01
Title: Frozen specification reference ingestion
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+1
Parallelism Justification: A1 may independently verify reference aliases while main Agent prepares ingestion map.
```

Frozen Spec References:

```text
SPEC-P3-FREEZE
SPEC-P3-VM
SPEC-P3-MIN
SPEC-P3-KEYWORDS
```

Inputs:

```text
Phase 1 frozen documents
Phase 2 frozen documents
Phase 3 frozen documents
AGENT-MASTER-PLAN.md
```

Outputs:

```text
canonical reference alias map
frozen document availability check
phase-to-subsystem map
missing-reference report if any
```

Non-Goals:

```text
no rewriting frozen specs
no copying full normative text
no semantic interpretation beyond routing references
```

Dependencies:

```text
WP-00 recommended
```

Implementation Tasks:

```text
verify all aliases resolve
add Phase 1 and Phase 2 aliases
classify references by subsystem
identify references needed for early work packages
```

Validation Gates:

```text
G0
G1
G2
G3
G7
```

Tests Required:

```text
alias resolution check
missing file check
```

Risks:

```text
incorrect alias
outdated archive copy
confusing aggregate references with subsystem references
```

Completion Criteria:

```text
all aliases resolve
all early work packages can cite references
no plan text acts as normative source
```

---

## 7. WP-02 · Traceability Matrix Construction

```text
WP-ID: WP-02
Title: Initial traceability matrix construction
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A1 can verify references while A3 drafts test obligations; main Agent merges both.
```

Frozen Spec References:

```text
SPEC-P3-VALID
SPEC-P3-EIR
SPEC-P3-RTP
SPEC-P3-ERRORS
SPEC-P3-HELPERS
SPEC-P3-CONTROL
SPEC-P3-UNWIND
SPEC-P3-MODULE
```

Inputs:

```text
TRACEABILITY-MATRIX.md
WORK-PACKAGE-INDEX.md
frozen reference alias map
```

Outputs:

```text
initial traceability table
implementation item to spec reference mapping
gate mapping
test obligation mapping
```

Non-Goals:

```text
no implementation
no exhaustive conformance test body
no normative text copy
```

Dependencies:

```text
WP-01
```

Implementation Tasks:

```text
create trace rows for WP-03 through WP-19
map each row to frozen references
map each row to gates
map each row to test obligations
identify gaps
```

Validation Gates:

```text
G0
G1
G2
G3
G7
```

Tests Required:

```text
trace row completeness check
unreferenced implementation item check
```

Risks:

```text
overbroad trace rows
missing Phase 1/2 references
aggregate-only references hiding subsystem requirements
```

Completion Criteria:

```text
no early work package lacks trace references
trace rows identify validation/test obligations
gaps are recorded
```

---

## 8. WP-03 · ID and Schema Model Skeleton

```text
WP-ID: WP-03
Title: ID and schema model skeleton
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+1
Parallelism Justification: A1 may verify ID/schema spec references while main Agent designs implementation package boundaries.
```

Frozen Spec References:

```text
SPEC-P3-RTP
SPEC-P3-EIR
SPEC-P3-GC-META
SPEC-P3-PROFILE
SPEC-P3-VALID
```

Inputs:

```text
Phase 2 ID classes
RuntimePlan schema
EIR schema
target/runtime profile schemas
```

Outputs:

```text
ID type implementation package plan
schema struct package plan
validation skeleton plan
```

Non-Goals:

```text
no interpreter execution
no runtime allocation model
no JIT implementation
```

Dependencies:

```text
WP-01
WP-02 recommended
```

Implementation Tasks:

```text
identify ID type set
identify schema structs
identify ownership boundaries
identify serialization/debug policy if needed
define validation entry points
```

Validation Gates:

```text
G0
G1
G2
G3
G4
G7
```

Tests Required:

```text
ID construction tests
unknown ID rejection tests
schema validation fixture tests
```

Risks:

```text
ID aliasing
schema drift
embedding normative assumptions in code comments
```

Completion Criteria:

```text
schema skeleton can be implemented without semantic invention
validation entry points are known
```

---

## 9. WP-04 · Runtime Error Registry Implementation

```text
WP-ID: WP-04
Title: Runtime error registry implementation
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+1
Parallelism Justification: A3 can derive negative-test obligations independently.
```

Frozen Spec References:

```text
SPEC-P3-ERRORS
SPEC-P3-VALID
```

Inputs:

```text
runtime error registry
validation matrix
```

Outputs:

```text
RuntimeErrorCode implementation plan
ErrorObj implementation plan
VmStructuralError implementation plan
test matrix
```

Non-Goals:

```text
no full runtime execution
no host error normalization beyond interface placeholder
```

Dependencies:

```text
WP-03
```

Implementation Tasks:

```text
define error code enum
define language ErrorObj fields
define structural VM error type
define conversion boundaries
define source-span requirements
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
known error code tests
non-Error raise rejection
structural error non-catchability test plan
source-span diagnostics tests
```

Risks:

```text
mixing LanguageError and VmStructuralError
missing source spans
turning internal failures into catchable errors
```

Completion Criteria:

```text
runtime error model matches frozen registry
negative tests are mapped
```

---

## 10. WP-05 · RuntimePlan Model and Validation

```text
WP-ID: WP-05
Title: RuntimePlan model and validation
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A1 verifies references; A3 derives validation tests.
```

Frozen Spec References:

```text
SPEC-P3-RTP
SPEC-P3-VALID
SPEC-P3-CACHE
SPEC-P3-PROFILE
```

Inputs:

```text
RuntimePlan schema closure
target/runtime profile schemas
validation matrix
cache compatibility matrix
```

Outputs:

```text
RuntimePlan data model plan
RuntimePlan validator plan
RuntimePlan cache key plan
test obligations
```

Non-Goals:

```text
no full lowering implementation
no EIR execution
no public bytecode/cache format
```

Dependencies:

```text
WP-03
WP-04
```

Implementation Tasks:

```text
map schema structs
define required table validators
define ID resolution validators
define target profile compatibility checks
define cache key construction plan
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
missing table rejection
unknown ID rejection
profile mismatch rejection
helper reference mismatch rejection
cache key mismatch tests
```

Risks:

```text
under-validating schema
cache key omissions
confusing internal cache with public artifact
```

Completion Criteria:

```text
RuntimePlan validator plan covers P3-V2 and P3-V3
```

---

## 11. WP-06 · EIR Model and Validation

```text
WP-ID: WP-06
Title: EIR model and validation
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A1 validates references; A3 builds op/terminator negative-test obligations.
```

Frozen Spec References:

```text
SPEC-P3-EIR
SPEC-P3-VALID
SPEC-P3-HELPERS
SPEC-P3-GC-META
```

Inputs:

```text
EIR operation schema closure
helper registry
GC metadata ownership
validation matrix
```

Outputs:

```text
EIR data model plan
EIR validator plan
op/terminator test matrix
```

Non-Goals:

```text
no interpreter implementation
no JIT lowering
no new EIR op kinds
```

Dependencies:

```text
WP-03
WP-05
```

Implementation Tasks:

```text
map EirModule/EirFunction/EirBlock/EirOp/EirTerminator structs
define op validation
define terminator validation
define source-map requirement checks
define may-collect/root-map checks
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
unknown op rejection
block without terminator rejection
may-raise without source map rejection
may-collect without root map rejection
invalid helper reference rejection
```

Risks:

```text
op schema drift
missing validation for cleanup crossing
treating EIR as public bytecode
```

Completion Criteria:

```text
EIR validator plan covers P3-V4 and P3-V5
```

---

## 12. WP-07 · Helper Registry and Dispatch

```text
WP-ID: WP-07
Title: Runtime helper registry and dispatch
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A1 checks helper references; A3 derives helper validation tests.
```

Frozen Spec References:

```text
SPEC-P3-HELPERS
SPEC-P3-ERRORS
SPEC-P3-VALID
SPEC-P3-HOST
SPEC-P3-CACHE
```

Inputs:

```text
canonical helper registry
runtime error registry
host boundary contract
cache compatibility matrix
```

Outputs:

```text
helper descriptor plan
helper table plan
helper validation plan
dispatch boundary plan
```

Non-Goals:

```text
no arbitrary native helper ABI
no public helper ABI
no direct host pointer call
```

Dependencies:

```text
WP-04
WP-06
```

Implementation Tasks:

```text
define helper descriptor representation
define helper table digest
define helper lookup
define helper validation
define helper call boundary
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
missing helper rejection
duplicate helper rejection
may-raise source policy tests
may-collect root policy tests
JIT-callable helper policy tests
```

Risks:

```text
descriptor/implementation mismatch
helper ABI leakage
host boundary bypass
```

Completion Criteria:

```text
helper registry is implementable without new helper semantics
```

---

## 13. WP-08 · Value / Heap / Object Reference Model

```text
WP-ID: WP-08
Title: Value, heap, and object reference model
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+1
Parallelism Justification: A2 may review localized runtime structure against value/profile specs.
```

Frozen Spec References:

```text
SPEC-P3-VALUES
SPEC-P3-PROFILE
SPEC-P3-GC-META
SPEC-P3-READONLY
SPEC-P3-ERRORS
```

Inputs:

```text
ValueKey/string semantics
target/runtime profile schemas
GC metadata ownership
ReadOnlyView semantics
runtime error registry
```

Outputs:

```text
Value representation plan
heap object plan
ObjRef/ObjectId plan
ValueKey plan
ReadOnlyView storage plan
```

Non-Goals:

```text
no production moving GC
no public object layout ABI
no CPython PyObject compatibility
```

Dependencies:

```text
WP-03
WP-04
```

Implementation Tasks:

```text
define Value representation under selected bootstrap profile
define heap object enum/trait plan
define ObjRef handle boundary
define ValueKey hash/equality plan
define ReadOnlyView trace/mutation checks
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
non-hashable map key rejection
NaN key rejection
string slice checks
readonly mutation rejection
ObjRef invalid handle rejection plan
```

Risks:

```text
object layout leak
incorrect hash/equality
readonly/freeze confusion
```

Completion Criteria:

```text
runtime value skeleton is compatible with future GC/JIT hooks
```

---

## 14. WP-09 · Frame / Slot / Control-State Model

```text
WP-ID: WP-09
Title: Frame, slot, and control-state model
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+1
Parallelism Justification: A2 may review frame/control structure against control and GC metadata specs.
```

Frozen Spec References:

```text
SPEC-P3-CONTROL
SPEC-P3-GC-META
SPEC-P3-RTP
SPEC-P3-VALID
```

Inputs:

```text
control-state model
GC metadata ownership
RuntimePlan slot layout
validation matrix
```

Outputs:

```text
Frame structure plan
SlotArray plan
PendingControl storage plan
FrameMap/RootMap linkage plan
```

Non-Goals:

```text
no full interpreter dispatch
no JIT stack map implementation
```

Dependencies:

```text
WP-03
WP-08
```

Implementation Tasks:

```text
define frame data model
define slot states
define pending control slot/root policy
define region stack storage
define frame map references
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
uninitialized slot read rejection
pending control root visibility checks
invalid slot ID rejection
control-state mapping tests
```

Risks:

```text
heap roots hidden in pending control
invalid frame reconstruction
control-state ambiguity
```

Completion Criteria:

```text
frame/control model can support interpreter, unwind, and deopt metadata
```

---

## 15. WP-10 · Structured Unwinding Implementation

```text
WP-ID: WP-10
Title: Structured unwinding implementation
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A2 reviews structure while A3 derives unwinding tests.
```

Frozen Spec References:

```text
SPEC-P3-UNWIND
SPEC-P3-CONTROL
SPEC-P3-ERRORS
SPEC-P3-GC-META
```

Inputs:

```text
structured unwinding algorithm
control-state model
runtime error registry
GC metadata ownership
```

Outputs:

```text
unwind helper plan
cleanup state model
defer/resource/finally test plan
suppressed error test plan
```

Non-Goals:

```text
no new cleanup ordering
no finalizer-based resource cleanup
```

Dependencies:

```text
WP-04
WP-09
WP-07
```

Implementation Tasks:

```text
define cleanup stack model
define perform_unwind loop
define defer execution boundary
define resource close exactly-once state
define finally override handling
define root/deopt state requirements
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
return through finally
raise through defer
resource close raise suppression
break/continue cleanup crossing
finally override tests
```

Risks:

```text
cleanup ordering bug
suppressed error loss
deopt during cleanup state loss
```

Completion Criteria:

```text
structured unwinding behavior has executable implementation and tests
```

---

## 16. WP-11 · Module Runtime Implementation

```text
WP-ID: WP-11
Title: Module runtime implementation
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A1 checks spec references; A3 derives module/cycle tests.
```

Frozen Spec References:

```text
SPEC-P3-MODULE
SPEC-P3-HOST
SPEC-P3-ERRORS
SPEC-P3-VALID
```

Inputs:

```text
module runtime contract
host boundary contract
runtime error registry
validation matrix
```

Outputs:

```text
ModuleInstance plan
ModuleState transition plan
ImportPlan/ExportPlan execution plan
module cycle test plan
```

Non-Goals:

```text
no package manager semantics
no public module ABI
no automatic retry beyond frozen policy
```

Dependencies:

```text
WP-05
WP-07
WP-08
```

Implementation Tasks:

```text
define module instance structure
define state transition enforcement
define import helpers usage
define export table sealing
define circular import checks
define resolver capability boundary
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
duplicate export rejection
uninitialized circular export rejection
failed module import behavior
top-level control rejection
resolver capability tests
```

Risks:

```text
module state leak
partial initialization ambiguity
resolver host boundary bypass
```

Completion Criteria:

```text
module runtime can execute source-order initialization safely
```

---

## 17. WP-12 · Call Execution Protocol Implementation

```text
WP-ID: WP-12
Title: Call execution protocol implementation
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A2 reviews call frame design; A3 derives call negative tests.
```

Frozen Spec References:

```text
SPEC-P3-CALL
SPEC-P3-CONTROL
SPEC-P3-ERRORS
SPEC-P3-HOST
SPEC-P3-VALID
```

Inputs:

```text
call execution protocol
control-state model
runtime error registry
host boundary contract
validation matrix
```

Outputs:

```text
call frame plan
argument binding plan
default evaluation plan
return contract test plan
host call wrapper plan
```

Non-Goals:

```text
no native ABI
no direct host pointer call
no default precomputation unless frozen spec allows
```

Dependencies:

```text
WP-04
WP-09
WP-14
```

Implementation Tasks:

```text
define call frame input
define callability checks
define arity/named argument binding
define call-time default evaluation
define parameter/return contract checks
define builtin/host call boundary
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
wrong arity
duplicate named arg
missing required arg
default raises
return contract failure
host call capability failure
```

Risks:

```text
default argument timing bug
host call boundary bypass
return cleanup ordering bug
```

Completion Criteria:

```text
call protocol can execute user/builtin/constructor/host wrapper calls consistently
```

---

## 18. WP-13 · ReadOnlyView Implementation

```text
WP-ID: WP-13
Title: ReadOnlyView implementation
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+1
Parallelism Justification: A3 may independently derive readonly negative tests.
```

Frozen Spec References:

```text
SPEC-P3-READONLY
SPEC-P3-VALUES
SPEC-P3-ERRORS
SPEC-P3-VALID
```

Inputs:

```text
ReadOnlyView semantics
ValueKey/string semantics
runtime error registry
validation matrix
```

Outputs:

```text
ReadOnlyView object plan
readonly mutation check plan
readonly helper behavior plan
readonly tests
```

Non-Goals:

```text
no deep freezing
no hashability wrapper semantics
no object ownership transfer
```

Dependencies:

```text
WP-08
WP-07
```

Implementation Tasks:

```text
define ReadOnlyView object
define target root tracing
define read delegation
define mutation rejection
define helper checks
define JIT guard placeholder requirements
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
field mutation through view rejected
index mutation through view rejected
original object mutation reflected in view
readonly over non-hashable target remains non-hashable
```

Risks:

```text
confusing readonly with immutable
missing mutating method rejection
hash stability bug
```

Completion Criteria:

```text
ReadOnlyView behavior is shallow, guarded, and tested
```

---

## 19. WP-14 · Host Boundary Skeleton

```text
WP-ID: WP-14
Title: Host boundary skeleton
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A1 checks boundary references; A4 reviews capability and integration risks.
```

Frozen Spec References:

```text
SPEC-P3-HOST
SPEC-P3-CALL
SPEC-P3-ERRORS
SPEC-P3-PROFILE
SPEC-P3-CACHE
```

Inputs:

```text
host boundary contract
call execution protocol
runtime error registry
target/runtime profile schemas
cache compatibility matrix
```

Outputs:

```text
HostFunctionWrapper plan
HostObjectWrapper plan
HostRootRegistry plan
host error normalization plan
capability-gated call plan
```

Non-Goals:

```text
no FFI implementation
no native extension ABI
no raw VM object pointer exposure
```

Dependencies:

```text
WP-04
WP-07
WP-08
WP-12
```

Implementation Tasks:

```text
define host wrapper descriptors
define host root registry
define enter/exit host call protocol
define error normalization boundary
define capability checks
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
host call without capability rejected
host error normalized
host retaining VM value without root rejected
direct host pointer call forbidden
```

Risks:

```text
host boundary becoming hidden FFI
raw pointer leak
capability bypass
```

Completion Criteria:

```text
host boundary skeleton supports module resolver and builtin host hooks safely
```

---

## 20. WP-15 · GC Metadata Structures

```text
WP-ID: WP-15
Title: GC metadata structures
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A2 reviews structures; A3 derives validation tests.
```

Frozen Spec References:

```text
SPEC-P3-GC-META
SPEC-P3-PROFILE
SPEC-P3-VALID
SPEC-P3-UNWIND
```

Inputs:

```text
GC metadata ownership
target/runtime profile schemas
validation matrix
structured unwinding algorithm
```

Outputs:

```text
RootMap plan
FrameMap plan
SafepointRecord plan
StackMap projection placeholder
metadata validation tests
```

Non-Goals:

```text
no production moving GC
no incremental/concurrent GC
no public stack map ABI
```

Dependencies:

```text
WP-08
WP-09
WP-10
```

Implementation Tasks:

```text
define RootMap structures
define FrameMap structures
define SafepointRecord structures
define root visibility policy
define moving-GC compatibility hooks
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
safepoint without root map rejection
unknown slot root rejection
pending control root visibility
moving GC profile root policy tests
```

Risks:

```text
precise root incompleteness
metadata schema drift
future JIT incompatibility
```

Completion Criteria:

```text
metadata structures satisfy interpreter and future JIT/GC hooks
```

---

## 21. WP-16 · Cache Compatibility Checks

```text
WP-ID: WP-16
Title: Cache compatibility checks
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+1
Parallelism Justification: A3 may derive cache invalidation tests.
```

Frozen Spec References:

```text
SPEC-P3-CACHE
SPEC-P3-PROFILE
SPEC-P3-RTP
SPEC-P3-EIR
SPEC-P3-HELPERS
```

Inputs:

```text
cache compatibility matrix
profile schemas
RuntimePlan schema
EIR schema
helper registry
```

Outputs:

```text
cache key component plan
digest policy plan
stale cache rejection tests
```

Non-Goals:

```text
no public cache format
no package ABI
no stable bytecode artifact
```

Dependencies:

```text
WP-05
WP-06
WP-07
```

Implementation Tasks:

```text
define cache key construction
define RuntimePlan cache validation
define EIR cache validation
define helper registry digest usage
define profile digest usage
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
mismatched SIR digest
mismatched RuntimePlan digest
mismatched helper digest
mismatched profile digest
public bytecode cache claim rejected
```

Risks:

```text
missing invalidation component
cache becoming public artifact
unsafe JIT cache reuse
```

Completion Criteria:

```text
cache compatibility checks are explicit and testable
```

---

## 22. WP-17 · Fast Interpreter Core

```text
WP-ID: WP-17
Title: Fast interpreter core
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+3
Parallelism Justification: A2 can review runtime execution structure, A3 test obligations, A4 integration risks; main Agent owns final design.
```

Frozen Spec References:

```text
SPEC-P3-EIR
SPEC-P3-RTP
SPEC-P3-CONTROL
SPEC-P3-HELPERS
SPEC-P3-VALID
SPEC-P3-GC-META
```

Inputs:

```text
EIR model
RuntimePlan model
Frame/Slot model
helper registry
GC metadata structures
validation pipeline
```

Outputs:

```text
dispatch loop plan
op execution plan
terminator execution plan
helper bridge plan
safepoint poll plan
diagnostic mapping plan
```

Non-Goals:

```text
no optimizing JIT
no public bytecode
no direct host pointer call
```

Dependencies:

```text
WP-05
WP-06
WP-07
WP-08
WP-09
WP-10
WP-11
```

Implementation Tasks:

```text
define interpreter state
define block dispatch
define op handlers
define terminator handlers
define helper bridge
define safepoint/root interaction
define error/control propagation
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
literal execution
slot load/store
branch condition Bool check
call helper path
raise/unwind path
module import path
readonly rejection path
```

Risks:

```text
scope too large
dispatch semantics drift
cleanup bypass
helper boundary bypass
metadata omission
```

Completion Criteria:

```text
minimal EIR execution works under frozen semantics and validation gates
```

---

## 23. WP-18 · Conformance Test Matrix

```text
WP-ID: WP-18
Title: Conformance test matrix
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+3
Parallelism Justification: A1 validates spec mapping, A3 builds test categories, A4 checks integration risk.
```

Frozen Spec References:

```text
SPEC-P3-VALID
SPEC-P3-ERRORS
SPEC-P3-VALUES
SPEC-P3-UNWIND
SPEC-P3-MODULE
SPEC-P3-CALL
SPEC-P3-READONLY
SPEC-P3-HOST
```

Inputs:

```text
validation matrix
all implemented runtime subsystem plans
```

Outputs:

```text
positive conformance matrix
negative conformance matrix
diagnostic matrix
regression matrix
```

Non-Goals:

```text
no implementation shortcuts
no weakening frozen behavior to match tests
```

Dependencies:

```text
WP-04 through WP-17
```

Implementation Tasks:

```text
define test categories
map tests to frozen specs
map tests to work packages
define required negative tests
define regression suite grouping
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
this package defines tests
```

Risks:

```text
missing negative tests
diagnostic under-coverage
tests replacing spec interpretation
```

Completion Criteria:

```text
every implemented subsystem has traceable positive and negative tests
```

---

## 24. WP-19 · Integration and Regression Gates

```text
WP-ID: WP-19
Title: Integration and regression gates
Status: COMPLETE
Owner: Main Agent
Agent Mode: main+2
Parallelism Justification: A3 verifies regression checks; A4 identifies integration hazards.
```

Frozen Spec References:

```text
SPEC-P3-VALID
SPEC-P3-CACHE
SPEC-P3-GC-META
SPEC-P3-HOST
SPEC-P3-FREEZE
```

Inputs:

```text
implemented subsystems
conformance matrix
gate checklist
risk register
```

Outputs:

```text
integration gate plan
regression gate plan
release candidate criteria
post-freeze erratum trigger policy
```

Non-Goals:

```text
no freeze baseline modification
no public release policy beyond implementation gates
```

Dependencies:

```text
WP-00 through WP-18
```

Implementation Tasks:

```text
define full gate run
define cross-subsystem regression checks
define cache/profile compatibility regression
define host boundary regression
define post-freeze issue classification
```

Validation Gates:

```text
G0-G7
```

Tests Required:

```text
full suite run
negative suite run
integration regression run
```

Risks:

```text
late integration failure
untracked cross-module dependency
regression suite too slow
```

Completion Criteria:

```text
implementation can be reviewed as a coherent minimal VM candidate
```

---

## 25. Initial Sequencing

Recommended order:

```text
WP-00
WP-01
WP-02
WP-03
WP-04
WP-05
WP-06
WP-07
WP-08
WP-09
WP-10
WP-11
WP-14
WP-12
WP-13
WP-15
WP-16
WP-17
WP-18
WP-19
```

Reasoning:

```text
process before trace
trace before schema
schema before runtime
runtime before interpreter
validation before integration
host boundary before host calls
GC/cache hooks before final interpreter integration
```

---

## 26. Parallelism Ceiling by Package

| Work Package | Max Agent Mode |
|---|---|
| WP-00 | main-only |
| WP-01 | main+1 |
| WP-02 | main+2 |
| WP-03 | main+1 |
| WP-04 | main+1 |
| WP-05 | main+2 |
| WP-06 | main+2 |
| WP-07 | main+2 |
| WP-08 | main+1 |
| WP-09 | main+1 |
| WP-10 | main+2 |
| WP-11 | main+2 |
| WP-12 | main+2 |
| WP-13 | main+1 |
| WP-14 | main+2 |
| WP-15 | main+2 |
| WP-16 | main+1 |
| WP-17 | main+3 |
| WP-18 | main+3 |
| WP-19 | main+2 |

No package defaults to `main+4`.

`main+4` is reserved for exceptional audit-style reviews and requires explicit written approval.

---

## 27. Completion Criteria for This Index

This work package index is complete when:

```text
each package has scope
each package has non-goals
each package has frozen spec references
each package has dependencies
each package has gate requirements
each package has test obligations
each package has risk notes
```

This initial version satisfies those requirements at planning level.

Detailed per-package task breakdowns may be added later without changing the frozen specifications.

---

## 28. Mandatory WP-00 Concrete Bootstrap Amendment

Added: 2026-06-29 11:00:40

`WP-00` is not a process-only package.

`WP-00` must create or align the implementation workspace.

### Required concrete outputs

```text
root AGENT.md
root PROGRESS.md
root ISSUE.md
docs/frozen-specs/
docs/agent-plan/
crates/
tests/
agent/
scripts/
Cargo.toml workspace
```

### Required crate directories

```text
crates/sir/
crates/sir_validate/
crates/vm_core/
crates/vm_runtime/
crates/vm_eval/
crates/vm_diag/
crates/vm_host/
crates/vm_tests/
crates/vm_cli/
```

### Required test directories

```text
tests/conformance/
tests/negative/
tests/diagnostics/
tests/regression/
tests/fixtures/
```

### Required Agent directories

```text
agent/work-packages/
agent/handoffs/
agent/gate-records/
agent/audit-records/
agent/task-logs/
```

### Required commands

At minimum, attempt:

```text
cargo metadata
cargo check --workspace
```

If these cannot run because the repository is not yet fully initialized, record the reason in `PROGRESS.md`.

### Required records

`PROGRESS.md` must receive an append-only summary of created directories and files.

`ISSUE.md` must receive audit findings only if bootstrap checks fail.

---

## 26. Phase 1 Language Pipeline Work Packages

After WP-00..WP-19 (Phase 3 bootstrap CLOSED), new packages implement the
source to AST to SIR to RuntimePlan/EIR product path. Do not reopen WP-00..19
without documented reason.

`	ext
WP-20 Phase 1 language pipeline process and traceability
WP-21 Phase 1 lexical analysis (source text + tokens)
WP-22 Phase 1 parser and AST (bootstrap COMPLETE)
WP-23 Phase 1 semantic binding skeleton (COMPLETE)\nWP-24 Phase 1 AST to bootstrap SIR (COMPLETE)\nWP-25 Bootstrap source to EIR execute (COMPLETE)
`

### WP-20 · Phase 1 Language Pipeline Process

`	ext
WP-ID: WP-20
Title: Phase 1 language pipeline process and traceability
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-FREEZE
SPEC-P1-LANG
SPEC-P1-DESIGN
TR-GAP-001
`

Outputs:

`	ext
WP-20..23 package rows
TRACE rows TR-P1-000.. for language frontend
coding stage 15+ notes in IMPLEMENTATION-STATUS
`

Non-Goals:

`	ext
no frozen-spec edits
no Phase 3 reopen
`

Completion Criteria:

`	ext
Phase 1 frontend work packages and trace rows exist and cite SPEC-P1-*
`

### WP-21 · Phase 1 Lexical Analysis

`	ext
WP-ID: WP-21
Title: Phase 1 lexical analysis (source text + tokens)
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-FREEZE
SPEC-P1-LANG (sections 3-6 lexical)
`

Outputs:

`	ext
crates/script_lex
token model; INDENT/DEDENT/NEWLINE; keyword/ident/literal/operator lexing
positive and negative lexer tests
`

Non-Goals:

`	ext
no parser/AST; no SIR lowering; no public bytecode
`

Dependencies:

`	ext
WP-20
`

Validation Gates:

`	ext
G0-G5, G7
`

Tests Required:

`	ext
UTF-8/comment/indent/keyword/integer/float/string/operators
negative: tab indent, bad int, unclosed string, indent mismatch
`

Completion Criteria:

`	ext
lexer implements PHASE-1-LANGUAGE-SPEC lexical subset with tests
`

### WP-22 · Phase 1 Parser and AST (bootstrap)

`	ext
WP-ID: WP-22
Title: Phase 1 parser and AST (bootstrap subset)
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-FREEZE
SPEC-P1-LANG (module/declarations/statements/expressions bootstrap)
`

Outputs:

`	ext
crates/script_parse
AST nodes; recursive-descent parser
minimal surface: let/const/def/if/while/return/assign/call/arith/list
tests include fib-shaped module
`

Non-Goals:

`	ext
full grammar (match/record/enum/import/export/...)
semantic analysis
SIR lowering
`

Dependencies:

`	ext
WP-21
`

Completion Criteria:

`	ext
bootstrap subset parses fib-shaped scripts with tests
`

### WP-23 · Phase 1 Semantic Binding Skeleton

`	ext
WP-ID: WP-23
Title: Phase 1 semantic binding and scope skeleton
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-FREEZE
SPEC-P1-LANG section 2.1, 2.2, let/const/def and assignment rules
`

Outputs:

`	ext
crates/script_sema
ScopeStack / BindingKind
no assignment without let; const/def immutable; block scope; duplicates; unresolved names
prelude print for bootstrap samples
10 unit tests including fib check
`

Non-Goals:

`	ext
full type contracts, import graph, record/enum members
`

Dependencies:

`	ext
WP-22
`

Completion Criteria:

`	ext
bootstrap AST passes binding/scope checks per SPEC-P1 design corrections
`

### WP-24 · Phase 1 AST to bootstrap SIR

`	ext
WP-ID: WP-24
Title: Phase 1 AST to bootstrap SIR materialization
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-LANG
SPEC-P2-FREEZE
SPEC-P2-IR
PHASE-2-SIR-SEMANTICS-ROUND1 (binding table kinds)
`

Outputs:

`	ext
sir: IrUnit, SirNode, symbol/scope/binding tables
script_lower: compile_to_sir / lower_module
Phase 1 surface: import, export, raise, assert
`

Non-Goals:

`	ext
full SIR node schema rounds
public IR ABI / bytecode
SIR to RuntimePlan (WP-25)
`

Dependencies:

`	ext
WP-21, WP-22, WP-23
`

Completion Criteria:

`	ext
analyzed bootstrap modules lower to IrUnit with tests (fib, import/export)
`

### WP-25 · SIR/AST to EIR codegen and fib end-to-end

`	ext
WP-ID: WP-25
Title: Bootstrap AST/EIR codegen for vm_eval (source to execute)
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-LANG
SPEC-P3-EIR
SPEC-P3-CALL
SPEC-P3-FREEZE
`

Outputs:

`	ext
crates/script_codegen
compile_source -> EirModule + callables
vm_eval binary ops extended (arith + compare)
fib(10) == 55 end-to-end test
`

Non-Goals:

`	ext
full RuntimePlan production path
for/break/continue/list/and-or short-circuit
production overflow policy
`

Completion Criteria:

`	ext
PROJECT-OVERVIEW fib sample executes via source pipeline
`


---

## 27. Unified Plan Work Packages (WP-L* / SUPERSEDED note)

Authority: PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md  
Legacy WP-00..19: Phase 3 bootstrap **COMPLETE (archived)**.  
Legacy WP-20..25: **SUPERSEDED** as plan IDs (prototype/demo assets may remain in tree).

### Active series (T-P1)

`	ext
WP-L00 T-P1 plan landing + P1-GAP-MATRIX
WP-L01 Lexical SPEC alignment
WP-L02 Grammar/AST v0 surface
WP-L03 Semantic analysis v0
WP-L04 Frontend diagnostics + AnalyzedModule API
WP-L05 T-P1 acceptance
`

### WP-L00 · Plan Landing and Gap Baseline

`	ext
WP-ID: WP-L00
Title: T-P1 plan landing + P1-GAP-MATRIX
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-FREEZE
SPEC-P1-LANG
UNIFIED-IMPLEMENTATION-GUIDANCE.md
`

Outputs:

`	ext
PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md
docs/phase-1/P1-GAP-MATRIX.md
WP-L* registry rows
WP-20..25 marked SUPERSEDED
`

### WP-L01 · Lexical SPEC Alignment

`	ext
WP-ID: WP-L01
Title: Lexical analysis SPEC-P1 section 3-6 alignment
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-LANG sections 3-6
`

Outputs:

`	ext
script_lex convergence
docs/phase-1/P1-TEST-MATRIX.md (lexical rows)
positive/negative lexer tests mapped to SPEC sections
`

Dependencies:

`	ext
WP-L00
`

Completion Criteria:

`	ext
P1-GAP lexical rows updated; v0 lexical surface YES or explicit PARTIAL with tests
`

### WP-L02 · Grammar/AST v0 surface

`	ext
WP-ID: WP-L02
Title: Grammar and AST v0 surface expansion
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-LANG sections 4.4, 6.7, 9.6, 10.2
`

Outputs:

`	ext
from-import, aug-assign, map literal, empty-block rejection
script_parse tests expanded
`

### WP-L03 · Semantic analysis v0

`	ext
WP-ID: WP-L03
Title: Semantic analysis v0 (Bool conditions, NFC, export)
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P1-LANG sections 2.1-2.3, 3.3, export
`

Outputs:

`	ext
script_sema Bool condition checks
NFC binding identity
Binding.exported
22 unit tests
`


### WP-L04 · Frontend diagnostics and AnalyzedModule API

`	ext
WP-ID: WP-L04
Title: Frontend diagnostics + AnalyzedModule API
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

### WP-L05 · T-P1 acceptance

`	ext
WP-ID: WP-L05
Title: T-P1 acceptance (P1-A..F)
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Notes: T-P1 complete for v0 frontend with residual DEFER items only in P1-GAP-MATRIX (match/record/full types, etc.).

---

## 28. T-P2 SIR Work Packages (WP-S*)

Authority: UNIFIED-IMPLEMENTATION-GUIDANCE.md Track T-P2

`	ext
WP-S00 AnalyzedModule → SIR materialization (bootstrap tables)
WP-S01 SIR structural validation (sir_validate)
WP-S02 SIR depth (types/patterns/control_regions) — planned
`

### WP-S00 · SIR materialization from AnalyzedModule

`	ext
WP-ID: WP-S00
Title: Materialize IrUnit from AnalyzedModule
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P2-IR section 4
SPEC-P1-LANG (input via AnalyzedModule)
`

Outputs:

`	ext
script_lower::materialize_sir / compile_to_sir via analyze_source
IrUnit.sources + interface_exports required tables
`

### WP-S01 · SIR structural validation

`	ext
WP-ID: WP-S01
Title: sir_validate structural checks
Status: COMPLETE
Owner: Main Agent
Agent Mode: main-only
`

Frozen Spec References:

`	ext
SPEC-P2-IR section 4.2 required tables
`

Outputs:

`	ext
validate_ir_unit (SIR001–SIR011)
tests: fib unit, missing sources, exports
`
