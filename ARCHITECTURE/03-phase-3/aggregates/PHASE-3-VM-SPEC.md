# Phase 3 · VM Specification Aggregate

Document class: Normative specification aggregate  
Normative status: This aggregate includes only Phase 3 normative specification documents. Implementation plans and administrative audits are deliberately excluded.

Updated: 2026-06-29 09:33:23

## Boundary

This file aggregates normative Phase 3 VM design documents only.



Included document classes:

```text
Normative specification
```

Excluded document classes:

```text
Implementation plan
Administrative tracking
Administrative audit
```

Implementation plans and audits do not override the normative VM specification.

## Included Normative Documents

- `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md`
- `PHASE-3-RUNTIME-ERROR-REGISTRY.md`
- `PHASE-3-EIR-SCHEMA-CLOSURE.md`
- `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md`
- `PHASE-3-RUNTIME-HELPER-REGISTRY.md`
- `PHASE-3-CONTROL-STATE-MODEL.md`
- `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md`
- `PHASE-3-MODULE-RUNTIME-CONTRACT.md`
- `PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md`
- `PHASE-3-GC-METADATA-OWNERSHIP.md`
- `PHASE-3-TARGET-PROFILE-SCHEMAS.md`
- `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md`
- `PHASE-3-VALIDATION-MATRIX.md`
- `PHASE-3-CACHE-COMPATIBILITY-MATRIX.md`
- `PHASE-3-CALL-EXECUTION-PROTOCOL.md`
- `PHASE-3-READONLY-VIEW-SEMANTICS.md`
- `PHASE-3-HOST-BOUNDARY-CONTRACT.md`
- `PHASE-3-VM-FRAMEWORK.md`
- `PHASE-3-VM-RUNTIME-ROUND1.md`
- `PHASE-3-PERFORMANCE-ARCHITECTURE.md`
- `PHASE-3-RUNTIMEPLAN-EIR-FRAMEWORK.md`
- `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md`
- `PHASE-3-SIR-LOWERING-ROUND1.md`
- `PHASE-3-CONTROL-LOWERING-ROUND2.md`
- `PHASE-3-RUNTIME-HELPER-CONTRACTS.md`
- `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md`
- `PHASE-3-BASELINE-JIT-BACKEND-INTERFACE.md`
- `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md`
- `PHASE-3-JIT-LOWERING-MATRIX.md`


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md -->


# Phase 3 · Normative Keywords and Terminology Glossary

Document class: Normative specification  
Normative status: This document defines normative keyword usage, terminology ownership, and naming conventions for Phase 3 VM specifications.

Created: 2026-06-29 09:18:30

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
R1: Add normative keyword policy and terminology glossary.
```

It supports the following audit findings:

```text
M-01 Normative documents still contain plan-like language.
M-12 Some normative docs use "recommended" implementation choices without marking non-normative status.
m-01 Naming convention drift.
m-05 Some "bootstrap" language should be converted to explicit MAY clauses.
```

This document is normative for document interpretation.

If another Phase 3 normative document uses ambiguous planning language, this document defines how that language must be interpreted until the wording is repaired.

---

## 1. Normative Keyword Policy

Phase 3 normative documents use the following keyword levels.

### 1.1 MUST

`MUST` defines a required semantic, interface, validation, or compatibility constraint.

A conforming VM implementation cannot violate a `MUST`.

Example:

```text
A RuntimeHelperOp MUST reference a helper in the RuntimeHelperTable.
```

### 1.2 MUST NOT

`MUST NOT` defines a forbidden behavior.

A conforming VM implementation cannot perform behavior marked `MUST NOT`.

Example:

```text
The VM MUST NOT expose EIR as public bytecode.
```

### 1.3 REQUIRED

`REQUIRED` is equivalent to `MUST` when used as an adjective.

Example:

```text
A source span is REQUIRED for may-raise diagnostics.
```

### 1.4 SHOULD

`SHOULD` defines a strong default expectation.

A conforming implementation may diverge only if it preserves all stronger normative constraints and documents the reason.

Example:

```text
The interpreter SHOULD use liveness-derived root maps after bootstrap.
```

### 1.5 SHOULD NOT

`SHOULD NOT` defines a discouraged design that is not absolutely forbidden.

A VM may use it only with explicit justification and only if no `MUST` or `MUST NOT` is violated.

### 1.6 MAY

`MAY` defines an allowed implementation option.

A `MAY` clause does not create a required feature.

Example:

```text
A bootstrap heap MAY use a slotmap-style object store.
```

### 1.7 OPTIONAL

`OPTIONAL` is equivalent to `MAY`.

### 1.8 RECOMMENDED

`RECOMMENDED` means `SHOULD`.

It indicates the preferred implementation path, not a semantic dependency.

Example:

```text
A Cranelift-compatible baseline backend is RECOMMENDED as the first JIT backend.
```

This does not make Cranelift part of language semantics.

### 1.9 DEFERRED

`DEFERRED` marks a design space intentionally excluded from the current freeze boundary.

A deferred feature must not be required for the Phase 3 minimal VM.

A deferred feature may still impose negative constraints.

Example:

```text
FFI is DEFERRED, but future FFI MUST NOT expose VM object layout as public ABI.
```

### 1.10 NON-NORMATIVE NOTE

A `NON-NORMATIVE NOTE` provides explanation, rationale, or implementation advice.

It does not define semantics.

If a non-normative note conflicts with a normative rule, the normative rule wins.

### 1.11 BOOTSTRAP

`BOOTSTRAP` marks an implementation allowance for early VM construction.

Bootstrap allowances are not architecture commitments.

The following rule is mandatory:

```text
A BOOTSTRAP allowance MUST NOT leak into public ABI, language semantics, or long-term compatibility boundary.
```

Example:

```text
BOOTSTRAP: The first heap MAY use Rc/RefCell internally.
```

This does not allow refcount to become language-visible or ABI-visible.

---

## 2. Document Class Policy

### 2.1 Normative Specification

A normative specification document defines:

```text
semantics
interfaces
required invariants
validation rules
compatibility boundaries
runtime contracts
forbidden behaviors
```

### 2.2 Implementation Plan

An implementation plan defines:

```text
milestones
staging
priority order
testing sequence
project execution strategy
```

Implementation plans do not override normative specifications.

### 2.3 Administrative Tracking

Administrative tracking documents define:

```text
status
changelog
manifest
audit progress
repair tracking
```

Administrative tracking documents do not override normative specifications.

### 2.4 Precedence

```text
Normative specification > Implementation plan > Administrative tracking
```

If an implementation plan conflicts with a normative specification, the normative specification wins.

If a status/changelog/audit-progress file conflicts with a normative specification, the normative specification wins.

---

## 3. Normative vs Planning Language

Normative documents MUST avoid unmarked project-planning language.

The following terms are allowed in normative documents only when marked as implementation guidance, bootstrap allowance, or deferred staging:

```text
stage
milestone
implementation priority
deliver
later
first implementation
roadmap
plan
```

Allowed forms:

```text
BOOTSTRAP: ...
RECOMMENDED implementation option: ...
DEFERRED: ...
NON-NORMATIVE NOTE: ...
```

Forbidden unmarked form inside normative documents:

```text
Milestone X must deliver Y.
```

unless the document is explicitly a conformance or validation specification and the milestone is a normative acceptance gate.

---

## 4. Terminology Ownership

The following terms require canonical ownership.

Secondary documents may reference them, but MUST NOT redefine their schema incompatibly.

| Term | Canonical owner |
|---|---|
| `RuntimePlan` | RuntimePlan / EIR Framework |
| `EIR` | RuntimePlan / EIR Framework + EIR Operation Semantics |
| `EirModule` | RuntimePlan / EIR Framework |
| `EirFunction` | RuntimePlan / EIR Framework |
| `EirBlock` | RuntimePlan / EIR Framework |
| `EirOp` | EIR Operation Semantics |
| `EirTerminator` | EIR Operation Semantics |
| `SlotId` | RuntimePlan / EIR Framework |
| `SlotArray` | Fast Interpreter Data Structures |
| `FrameMap` | GC Safepoint Root Model, with interpreter/JIT projections |
| `RootMap` | GC Safepoint Root Model |
| `SafepointRecord` | GC Safepoint Root Model |
| `StackMap` | Baseline JIT Backend Interface, under GC root rules |
| `DeoptPoint` | RuntimePlan / EIR Framework, with JIT projection |
| `RuntimeHelperDescriptor` | Runtime Helper Contracts |
| `RuntimeHelperTable` | Runtime Helper Contracts |
| `Value` | VM Runtime Semantics |
| `ObjectId` | VM Runtime Semantics + GC Safepoint Root Model |
| `ObjRef` | VM Runtime Semantics + GC Safepoint Root Model |
| `RegionStack` | Fast Interpreter Data Structures + Control Lowering |
| `PendingControl` | Control Lowering + Fast Interpreter Data Structures |
| `CapabilityEnvironment` | VM Framework + Helper/Host Boundary documents |
| `ModuleState` | Phase 2 Integration + Phase 3 module runtime contract |
| `ValueLayoutProfile` | Target/Profile schema to be added by repair R11 |
| `RuntimeErrorCode` | Runtime error registry to be added by repair R2 |

---

## 5. Naming Conventions

### 5.1 Acronyms

Canonical capitalization:

```text
SIR
EIR
OIR
NIR
VM
GC
JIT
ABI
API
FFI
```

### 5.2 Rust-like Type Names

Rust-like type names use UpperCamelCase:

```text
RuntimePlan
EirModule
EirFunction
EirBlock
EirOp
EirTerminator
VmControl
VmError
ObjRef
ObjectId
RootMap
FrameMap
SafepointRecord
RuntimeHelperDescriptor
```

### 5.3 ID Names

ID names use UpperCamelCase with `Id` suffix:

```text
SlotId
FrameId
ModuleId
FunctionId
CallSiteId
AccessSiteId
SafepointId
DeoptId
RuntimeHelperId
```

### 5.4 Table Names

Table names use UpperCamelCase with `Table` suffix:

```text
CallSiteTable
AccessSiteTable
RuntimeHelperTable
SafepointTable
DeoptPointTable
StackMapTable
```

### 5.5 Avoided Aliases

The following aliases SHOULD NOT be introduced in new normative text:

```text
VMControl
ControlResult
Bytecode
InstructionSet ABI
Native helper ABI
PyObject-compatible value
```

If an older document uses `Control`, it must be mapped to the canonical control-state model during R6.

---

## 6. Meaning of "Minimal VM"

`Minimal VM` in Phase 3 means:

```text
minimal normative VM architecture with mandatory future-proofing hooks
```

It does not mean:

```text
smallest possible implementation
interpreter-only toy runtime
JIT implementation required immediately
production GC required immediately
```

The minimal VM MAY defer actual JIT and advanced GC implementation.

The minimal VM MUST preserve required hooks for:

```text
RuntimePlan
EIR
slot-based execution
helper slow paths
GC root enumeration
safepoints
write barriers
JIT metadata
deoptimization metadata
capability checks
```

---

## 7. Meaning of "Internal"

A structure marked internal is not public ABI and not package ABI.

Internal structures include:

```text
RuntimePlan
EIR
OIR
RuntimeHelperTable
Value layout
Object layout
Frame layout
RootMap encoding
StackMap encoding
JIT helper call ABI
```

Internal does not mean unspecified.

Internal structures may be fully specified for VM correctness while remaining non-public.

---

## 8. Meaning of "Public Bytecode"

The language has no public bytecode commitment in Phase 3.

The following are explicitly not public bytecode:

```text
SIR
RuntimePlan
EIR
OIR
JIT machine code
cache files
helper IDs
stack maps
deopt maps
```

A VM may cache internal representations.

Such caches MUST be:

```text
VM-versioned
target-profile-versioned
discardable
not package ABI
not user-facing bytecode artifacts
```

---

## 9. Meaning of "CPython Compatibility Rejected"

The following are rejected:

```text
CPython C API compatibility
CPython ABI compatibility
Python wheel compatibility
Python extension module compatibility
PyObject layout compatibility
CPython refcount architecture as VM architecture
GIL compatibility as design target
```

This does not forbid:

```text
source-level inspiration
foreign process interop
data-format interop
future capability-gated FFI
```

Any future FFI remains separate from CPython compatibility.

---

## 10. Audit Tracking

This document completes repair item:

```text
R1: Add normative keyword policy and terminology glossary.
```

It partially addresses:

```text
M-01
M-12
m-01
m-05
```

It does not resolve blockers B-01 through B-08.


<!-- END NORMATIVE DOCUMENT: PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-RUNTIME-ERROR-REGISTRY.md -->


# Phase 3 · Runtime Error Registry

Document class: Normative specification  
Normative status: This document defines the canonical runtime error taxonomy for Phase 3 VM specifications.

Created: 2026-06-29 09:21:35

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
R2: Create central runtime error registry.
```

It resolves blocker:

```text
B-04: Error taxonomy is not centralized.
```

All Phase 3 normative documents that mention runtime errors MUST reference this registry.

---

## 1. Error Layering

Phase 3 distinguishes four error layers.

```text
LanguageError
  Observable language-level error value.

VmStructuralError
  VM invariant failure or malformed internal input. Not ordinary source-level control flow.

DiagnosticError
  Error produced by compiler/validator/diagnostic subsystem before or around execution.

HostBoundaryError
  Host exception/effect failure normalized into LanguageError or VmStructuralError.
```

### 1.1 LanguageError

A LanguageError is represented as a language-level `ErrorObj`.

It can be raised through:

```text
VmControl::Raise(ErrorHandle)
```

### 1.2 VmStructuralError

A VmStructuralError is represented as:

```text
VmError
```

It indicates a VM bug, invalid internal IR, failed validation, corrupted runtime structure, or backend violation.

It MUST NOT be catchable as ordinary source-level error unless explicitly converted.

### 1.3 DiagnosticError

A DiagnosticError belongs to validation/compile/source reporting.

It may prevent execution.

### 1.4 HostBoundaryError

Host errors MUST be normalized at host boundary.

Raw host exceptions MUST NOT leak into language runtime.

---

## 2. RuntimeErrorCode Registry

### 2.1 Required LanguageError Codes

The minimal Phase 3 VM MUST support these language-visible error codes:

| Code | Category | Required? | Raised by |
|---|---|---:|---|
| `NameError` | LanguageError | yes | unresolved source-visible name after validation boundary where applicable |
| `UninitializedBindingError` | LanguageError | yes | initialized binding read before initialization |
| `TypeError` | LanguageError | yes | unsupported operation, non-Bool condition, non-callable call, non-Error raise |
| `TypeContractError` | LanguageError | yes | failed runtime type contract |
| `PatternMatchError` | LanguageError | yes | failed declaration destructuring pattern |
| `ReadOnlyError` | LanguageError | yes | mutation through read-only view or read-only target |
| `AssertionError` | LanguageError | yes | failed assert |
| `ArityError` | LanguageError | yes | wrong function/builtin/constructor arity |
| `IndexError` | LanguageError | yes | invalid list/string slice/index bounds |
| `KeyError` | LanguageError | yes | missing map key where read requires presence |
| `FieldError` | LanguageError | yes | invalid record/module/attribute field access |
| `ImportError` | LanguageError | yes | module resolution/import/export failure |
| `ImportCycleError` | LanguageError | yes | uninitialized export access in circular import |
| `DivisionByZeroError` | LanguageError | yes | division or modulo by zero |
| `NumericOverflowError` | LanguageError | yes | checked fixed-width integer overflow |
| `CapabilityError` | LanguageError | yes | missing required capability |
| `StackOverflowError` | LanguageError | yes | logical VM call stack depth exceeded |
| `ResourceStateError` | LanguageError | yes | invalid resource state transition if policy requires error |
| `InternalVMError` | LanguageError | restricted | source-visible wrapper only when VM elects to expose internal failure safely |

### 2.2 Required VmStructuralError Codes

| Code | Category | Required? | Raised by |
|---|---|---:|---|
| `InvalidEirError` | VmStructuralError | yes | malformed EIR reaching execution/JIT |
| `InvalidRuntimePlanError` | VmStructuralError | yes | malformed RuntimePlan reaching execution |
| `InvalidSlotError` | VmStructuralError | yes | unknown or incompatible SlotId |
| `InvalidObjectHandleError` | VmStructuralError | yes | stale/invalid ObjRef/ObjectId |
| `InvalidHelperError` | VmStructuralError | yes | unknown helper ID or descriptor mismatch |
| `InvalidFrameStateError` | VmStructuralError | yes | corrupted frame/stack/root/deopt state |
| `InvalidRootMapError` | VmStructuralError | yes | missing or invalid root map at required safepoint |
| `InvalidStackMapError` | VmStructuralError | yes | JIT safepoint without valid stack map |
| `InvalidDeoptError` | VmStructuralError | yes | deopt point missing reconstruction data |
| `BackendViolationError` | VmStructuralError | yes | compiled code violates VM metadata contract |

---

## 3. Error Object Requirements

A language-level ErrorObj MUST contain:

```text
error_code: RuntimeErrorCode
message: String
source_span?: SourceSpanId
stack_trace?: StackTrace
details?: Map[String, Value]
cause?: ErrorHandle
suppressed?: List[ErrorHandle]
```

`source_span` is REQUIRED for source-originated may-raise operations when available.

`stack_trace` MAY be disabled in restricted runtime mode, but diagnostics MUST still be source-oriented where possible.

---

## 4. Raise Rules

A source-level `raise` MUST raise only language Error values.

Raising a non-Error value MUST raise:

```text
TypeError
```

Runtime helpers that produce language failure MUST return:

```text
VmControl::Raise(ErrorHandle)
```

Runtime helpers that detect structural VM failure MUST return:

```text
VmError
```

---

## 5. Cleanup and Suppressed Errors

During cleanup:

```text
pending Raise + cleanup Raise
```

MUST preserve the original primary error and attach the cleanup error as suppressed unless the structured unwinding algorithm explicitly defines override.

```text
pending Normal + cleanup Raise
```

MUST make cleanup error the primary error.

```text
pending Return/Break/Continue + cleanup Raise
```

MUST convert pending control to Raise unless the canonical structured unwinding algorithm defines a stronger rule.

---

## 6. Source Mapping

Errors raised during execution MUST map to:

```text
SourceSpanId
SIR NodeId where available
EIR location
frame stack
helper context if relevant
```

VM-internal helper frames MAY be hidden from ordinary source stack traces.

---

## 7. Deferred Error Codes

Future features MAY add error codes for:

```text
FFI boundary
async/cancellation
debugger traps
concurrent execution
native resource ownership
```

Such codes MUST be added to this registry before becoming normative.

---

## 8. Compatibility

Adding a new language-visible error code is a normative language/runtime change.

Changing an existing error's category, visibility, or required status requires reopening Phase 3 or a later compatibility revision.

---

## 9. Audit Tracking

This document completes:

```text
R2
```

It resolves:

```text
B-04
```

It partially supports:

```text
B-06
M-10
M-15
m-06
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-RUNTIME-ERROR-REGISTRY.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-EIR-SCHEMA-CLOSURE.md -->


# Phase 3 · EIR Operation Schema Closure

Document class: Normative specification  
Normative status: This document defines the canonical closed minimal EIR operation and terminator schema for Phase 3 VM specifications.

Created: 2026-06-29 09:21:35

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
R3: Close EIR operation schema.
```

It resolves blocker:

```text
B-01: EIR instruction schema is not closed.
```

All Phase 3 documents that mention EIR operations MUST conform to this schema.

---

## 1. EIR Boundary

EIR is:

```text
VM-internal
RuntimePlan-dependent
discardable
cacheable only under VM cache compatibility rules
not public bytecode
not package ABI
not native ABI
```

EIR is the canonical fast execution input after SIR lowering and RuntimePlan construction.

---

## 2. EirModule

```text
EirModule {
  eir_version: Version
  source_runtime_plan_digest: Digest
  functions: List<EirFunction>
  constants: ConstantPool
  source_map: EirSourceMap
  safepoints: SafepointTable
  deopt_points: DeoptPointTable
}
```

Validation MUST reject an EIR module whose digest does not match the RuntimePlan used for execution.

---

## 3. EirFunction

```text
EirFunction {
  eir_function_id: EirFunctionId
  function_id?: FunctionId
  module_id: ModuleId
  entry_block: EirBlockId
  blocks: List<EirBlock>
  slot_layout: SlotLayoutRef
  frame_map: FrameMapId
  source_span?: SourceSpanId
}
```

Every function MUST have exactly one entry block.

---

## 4. EirBlock

```text
EirBlock {
  block_id: EirBlockId
  parameters: List<BlockParameter>
  ops: List<EirOp>
  terminator: EirTerminator
  source_span?: SourceSpanId
}
```

Blocks MUST NOT fall through.

A block MUST have exactly one terminator.

---

## 5. Common Operation Fields

Every EIR operation has:

```text
op_id?: EirOpId
source_span?: SourceSpanId
debug_origin?: NodeId
```

Any operation that may raise a LanguageError MUST have a source span or a source mapping through `debug_origin`.

---

## 6. EirOp Union

```text
EirOp =
  | ConstantOp
  | LoadOp
  | StoreOp
  | UnaryOp
  | BinaryOp
  | LogicalOp
  | CheckOp
  | CallOp
  | AccessOp
  | ConstructOp
  | PatternOp
  | RuntimeHelperOp
  | SafepointOp
  | GuardOp
  | DebugOp
```

No other EIR op kind is allowed in Phase 3 minimal EIR.

Unknown op kinds MUST be rejected by EIR validation.

---

## 7. ConstantOp

```text
ConstantOp {
  dest: SlotId
  constant: ConstantId
}
```

Validation:

```text
dest resolves to writable temporary/local slot
constant resolves in ConstantPool
```

---

## 8. LoadOp

```text
LoadOp =
  | LoadSlot
  | LoadCell
  | LoadCapture
  | LoadModuleSlot
  | LoadField
  | LoadEnumPayload
```

### 8.1 LoadSlot

```text
LoadSlot {
  dest: SlotId
  source: SlotId
  require_initialized: Bool
}
```

### 8.2 LoadCell

```text
LoadCell {
  dest: SlotId
  cell_slot: SlotId
  binding_id?: BindingId
}
```

### 8.3 LoadCapture

```text
LoadCapture {
  dest: SlotId
  capture_index: UInt
  binding_id?: BindingId
}
```

### 8.4 LoadModuleSlot

```text
LoadModuleSlot {
  dest: SlotId
  module_id: ModuleId
  module_slot: SlotId
  binding_id?: BindingId
}
```

### 8.5 LoadField

```text
LoadField {
  dest: SlotId
  receiver: SlotId
  record_shape: ShapeId
  field_id: FieldId
  field_index: FieldIndex
  access_site_id: AccessSiteId
}
```

### 8.6 LoadEnumPayload

```text
LoadEnumPayload {
  dest: SlotId
  receiver: SlotId
  enum_shape: ShapeId
  case_id: CaseId
  case_index: CaseIndex
  payload_index: UInt
}
```

---

## 9. StoreOp

```text
StoreOp =
  | StoreSlot
  | StoreCell
  | StoreModuleSlot
  | StoreField
  | StoreListIndex
  | StoreMapEntry
```

### 9.1 StoreSlot

```text
StoreSlot {
  dest: SlotId
  value: SlotId
  check_initialized?: Bool
}
```

### 9.2 StoreCell

```text
StoreCell {
  cell_slot: SlotId
  value: SlotId
  binding_id?: BindingId
  check_mutability: Bool
  type_check?: TypeId
}
```

### 9.3 StoreModuleSlot

```text
StoreModuleSlot {
  module_id: ModuleId
  module_slot: SlotId
  value: SlotId
  binding_id?: BindingId
  check_mutability: Bool
  type_check?: TypeId
}
```

### 9.4 StoreField

```text
StoreField {
  receiver: SlotId
  value: SlotId
  record_shape: ShapeId
  field_id: FieldId
  field_index: FieldIndex
  access_site_id: AccessSiteId
  check_readonly: Bool
  check_mutability: Bool
  type_check?: TypeId
}
```

### 9.5 StoreListIndex

```text
StoreListIndex {
  receiver: SlotId
  index: SlotId
  value: SlotId
  access_site_id: AccessSiteId
  check_readonly: Bool
}
```

### 9.6 StoreMapEntry

```text
StoreMapEntry {
  receiver: SlotId
  key: SlotId
  value: SlotId
  access_site_id: AccessSiteId
  check_readonly: Bool
  check_hashable: Bool
}
```

All heap-reference mutation paths MUST route through write barrier policy.

---

## 10. UnaryOp

```text
UnaryOp {
  dest: SlotId
  op: UnaryOperator
  operand: SlotId
}
```

```text
UnaryOperator =
  | Plus
  | Minus
  | Not
```

`Not` MUST require Bool operand.

---

## 11. BinaryOp

```text
BinaryOp {
  dest: SlotId
  op: BinaryOperator
  left: SlotId
  right: SlotId
  overflow_policy?: NumericOverflowPolicy
}
```

```text
BinaryOperator =
  | Add
  | Subtract
  | Multiply
  | Divide
  | Modulo
  | Equal
  | NotEqual
  | Less
  | LessEqual
  | Greater
  | GreaterEqual
  | Identity
  | NotIdentity
  | Contains
```

No implicit coercion is allowed.

---

## 12. LogicalOp

Logical operations SHOULD lower to branches.

If retained as EIR op:

```text
LogicalOp {
  dest: SlotId
  op: LogicalOperator
  left_block: EirBlockId
  right_block: EirBlockId
  merge_block: EirBlockId
}
```

```text
LogicalOperator =
  | And
  | Or
```

A simple eager boolean binary op MUST NOT replace short-circuit semantics.

---

## 13. CheckOp

```text
CheckOp =
  | CheckBool
  | CheckType
  | CheckCallable
  | CheckArity
  | CheckHashable
  | CheckReadonly
  | CheckCapability
  | CheckShape
  | CheckOverflow
  | CheckDivisionByZero
```

Every CheckOp defines:

```text
failure_error: RuntimeErrorCode
```

Semantic check failure raises a LanguageError, not deopt, unless the check is explicitly speculative GuardOp.

---

## 14. CallOp

```text
CallOp {
  dest?: SlotId
  callee: SlotId
  positional_args: List<SlotId>
  named_args: List<NamedArgumentSlot>
  call_site_id: CallSiteId
  result_type_check?: TypeId
  call_kind: CallKind
}
```

```text
CallKind =
  | Generic
  | KnownFunction
  | KnownBuiltin
  | RecordConstructor
  | EnumConstructor
  | BoundMethod
  | HostFunction
```

Argument evaluation MUST already be represented in prior EIR ops in source order.

---

## 15. AccessOp

```text
AccessOp =
  | AttributeRead
  | AttributeWrite
  | MethodRead
  | IndexRead
  | IndexWrite
  | SliceRead
```

AccessOps MUST include `AccessSiteId`.

Generic or megamorphic access MAY lower to RuntimeHelperOp.

---

## 16. ConstructOp

```text
ConstructOp =
  | ConstructList
  | ConstructMap
  | ConstructRecord
  | ConstructEnumValue
  | ConstructFunction
  | ConstructError
```

Construction operations that may allocate MUST be safepoint-compatible and root-map-compatible.

---

## 17. PatternOp

```text
PatternOp =
  | PatternCheckLiteral
  | PatternCheckRecordShape
  | PatternCheckEnumCase
  | PatternCheckListLength
  | PatternCheckMapKey
  | PatternBind
  | PatternBranch
  | PatternCommit
  | PatternRollback
```

Pattern operations MUST distinguish:

```text
MatchCase failure
DestructuringDeclaration failure
```

---

## 18. RuntimeHelperOp

```text
RuntimeHelperOp {
  dest?: SlotId
  helper_id: RuntimeHelperId
  args: List<SlotId>
  call_site?: CallSiteId
  access_site?: AccessSiteId
  safepoint_id?: SafepointId
  deopt_id?: DeoptId
}
```

Validation MUST ensure helper_id resolves in the canonical RuntimeHelperRegistry.

---

## 19. SafepointOp

```text
SafepointOp {
  safepoint_id: SafepointId
  kind: SafepointKind
}
```

The referenced SafepointRecord MUST have a valid RootMap when GC can run.

---

## 20. GuardOp

```text
GuardOp {
  guard_kind: GuardKind
  inputs: List<SlotId>
  on_failure: GuardFailureAction
  deopt_id?: DeoptId
  helper_id?: RuntimeHelperId
  failure_error?: RuntimeErrorCode
}
```

```text
GuardFailureAction =
  | Raise
  | HelperFallback
  | Deopt
  | Branch
```

Every guard MUST have a failure action.

---

## 21. DebugOp

```text
DebugOp {
  debug_kind: DebugKind
  source_span?: SourceSpanId
}
```

DebugOp MUST NOT change source-visible semantics.

---

## 22. EirTerminator Union

```text
EirTerminator =
  | Jump
  | Branch
  | Return
  | Raise
  | LoopBackedge
  | Switch
  | Unwind
  | Unreachable
```

### 22.1 Jump

```text
Jump {
  target: EirBlockId
  args: List<SlotId>
}
```

Block argument transfer MUST be simultaneous.

### 22.2 Branch

```text
Branch {
  condition: SlotId
  then_block: EirBlockId
  else_block: EirBlockId
}
```

Condition MUST be Bool.

### 22.3 Return

```text
Return {
  value?: SlotId
}
```

If active cleanup exists, Return MUST enter PendingControl/unwind path.

### 22.4 Raise

```text
Raise {
  error: SlotId
}
```

Non-Error raise MUST raise TypeError.

### 22.5 LoopBackedge

```text
LoopBackedge {
  target: EirBlockId
  args: List<SlotId>
  safepoint_id: SafepointId
}
```

### 22.6 Switch

```text
Switch {
  scrutinee: SlotId
  cases: List<SwitchCase>
  default: EirBlockId
}
```

### 22.7 Unwind

```text
Unwind {
  pending_control_slot: SlotId
  target_region?: ControlRegionId
}
```

### 22.8 Unreachable

```text
Unreachable {
  reason: String
}
```

Reaching Unreachable is `InternalVMError` or VmStructuralError.

---

## 23. Validation Requirements

EIR validation MUST reject:

```text
unknown op kind
unknown terminator kind
unknown SlotId
unknown TypeId
unknown ShapeId
unknown FieldId
unknown CaseId
unknown RuntimeHelperId
unknown CallSiteId
unknown AccessSiteId
unknown SafepointId
unknown DeoptId
block without terminator
invalid block argument count
may-raise op without source mapping
may-collect helper without root map
heap write without barrier policy
guard without failure action
unreachable public bytecode marker
```

---

## 24. Audit Tracking

This document completes:

```text
R3
```

It resolves:

```text
B-01
```

It partially supports:

```text
M-14
M-10
M-15
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-EIR-SCHEMA-CLOSURE.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md -->


# Phase 3 · RuntimePlan Schema Closure

Document class: Normative specification  
Normative status: This document defines the canonical closed minimal RuntimePlan schema for Phase 3 VM specifications.

Created: 2026-06-29 09:21:35

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
R4: Close RuntimePlan schema.
```

It resolves blocker:

```text
B-02: RuntimePlan schema is still framework-level.
```

RuntimePlan is the internal semantic-to-execution bridge from frozen SIR to EIR.

---

## 1. RuntimePlan Boundary

RuntimePlan is:

```text
VM-internal
SIR-digest-bound
target-profile-bound
discardable
cacheable only under compatibility rules
not public bytecode
not package ABI
not native ABI
```

---

## 2. RuntimePlan Schema

```text
RuntimePlan {
  plan_version: Version
  source_sir_digest: Digest
  phase2_schema_version: Version
  vm_version: Version
  target_profile: RuntimeTargetProfile
  module_plans: ModulePlanTable
  function_plans: FunctionPlanTable
  type_plan: TypePlan
  shape_plan: ShapePlan
  slot_layouts: SlotLayoutTable
  call_site_table: CallSiteTable
  access_site_table: AccessSiteTable
  safepoint_seed_table: SafepointSeedTable
  deopt_seed_table: DeoptSeedTable
  helper_requirements: RuntimeHelperRequirementTable
  capability_gate_plan: CapabilityGatePlan
  source_map: RuntimePlanSourceMap
  diagnostics?: DiagnosticTable
}
```

Every field except diagnostics is REQUIRED.

---

## 3. RuntimeTargetProfile

```text
RuntimeTargetProfile {
  vm_version: Version
  architecture: TargetArchitecture
  pointer_width: UInt
  value_layout_profile: ValueLayoutProfileRef
  heap_profile: HeapProfileRef
  gc_profile: GcProfileRef
  interpreter_profile: InterpreterProfileRef
  jit_profile?: JitProfileRef
  capability_environment_digest?: Digest
}
```

The full profile participates in RuntimePlan cache identity.

---

## 4. ModulePlanTable

```text
ModulePlanTable {
  modules: Map<ModuleId, ModulePlan>
}
```

### 4.1 ModulePlan

```text
ModulePlan {
  module_id: ModuleId
  module_slot_layout: SlotLayoutId
  initialization_function: EirFunctionId
  import_plan: ImportPlan
  export_plan: ExportPlan
  module_state_slot: SlotId
  module_object_slot: SlotId
  source_order: List<NodeId>
  source_span?: SourceSpanId
}
```

### 4.2 ImportPlan

```text
ImportPlan {
  imports: List<ImportPlanEntry>
}
```

```text
ImportPlanEntry {
  node_id: NodeId
  imported_module: ModuleId
  import_kind: ImportKind
  local_binding_slot: SlotId
  required_interface?: InterfaceId
  source_span: SourceSpanId
}
```

### 4.3 ExportPlan

```text
ExportPlan {
  exports: List<ExportPlanEntry>
  seal_after_init: Bool
}
```

```text
ExportPlanEntry {
  exported_name: String
  binding_id: BindingId
  slot_id: SlotId
  interface_type?: TypeId
  source_span: SourceSpanId
}
```

---

## 5. FunctionPlanTable

```text
FunctionPlanTable {
  functions: Map<FunctionId, FunctionPlan>
}
```

### 5.1 FunctionPlan

```text
FunctionPlan {
  function_id: FunctionId
  module_id: ModuleId
  entry_eir_function: EirFunctionId
  parameter_layout: ParameterLayout
  local_slot_layout: SlotLayoutId
  capture_layout: CaptureLayout
  default_argument_plan: DefaultArgumentPlan
  return_type?: TypeId
  effect?: EffectId
  required_capabilities: List<CapabilityId>
  call_site_range: IdRange<CallSiteId>
  access_site_range: IdRange<AccessSiteId>
  safepoint_range: IdRange<SafepointId>
  deopt_range: IdRange<DeoptId>
  source_span?: SourceSpanId
}
```

---

## 6. SlotLayoutTable

```text
SlotLayoutTable {
  layouts: Map<SlotLayoutId, SlotLayout>
}
```

### 6.1 SlotLayout

```text
SlotLayout {
  slot_layout_id: SlotLayoutId
  owner: SlotLayoutOwner
  slots: List<SlotDescriptor>
  hidden_slots: List<HiddenSlotDescriptor>
  max_slot_count: UInt
}
```

### 6.2 SlotDescriptor

```text
SlotDescriptor {
  slot_id: SlotId
  kind: SlotKind
  binding_id?: BindingId
  type_hint?: TypeId
  mutability?: Mutability
  storage: SlotStorage
  initialized_at_entry: Bool
  source_span?: SourceSpanId
}
```

```text
SlotKind =
  | Local
  | Parameter
  | Capture
  | Module
  | Temporary
  | Builtin
  | HiddenRuntime
```

```text
SlotStorage =
  | Value
  | Cell
  | Constant
  | TypeDescriptor
  | ModuleRef
  | RuntimeInternal
```

---

## 7. TypePlan

```text
TypePlan {
  runtime_types: Map<TypeId, RuntimeTypeDesc>
  type_check_strategies: Map<TypeId, TypeCheckStrategy>
}
```

Every TypeId used by EIR or RuntimePlan MUST resolve here or in the frozen SIR TypeTable by explicit reference.

---

## 8. ShapePlan

```text
ShapePlan {
  record_shapes: Map<RecordId, RecordShape>
  enum_shapes: Map<EnumId, EnumShape>
}
```

### 8.1 RecordShape

```text
RecordShape {
  record_id: RecordId
  shape_id: ShapeId
  fields: List<RecordFieldShape>
  methods: List<MethodShape>
  field_index_by_id: Map<FieldId, FieldIndex>
}
```

### 8.2 EnumShape

```text
EnumShape {
  enum_id: EnumId
  shape_id: ShapeId
  cases: List<EnumCaseShape>
  case_index_by_id: Map<CaseId, CaseIndex>
}
```

Normal execution MUST use FieldId/CaseId-derived indices, not string lookup.

---

## 9. CallSiteTable

```text
CallSiteTable {
  call_sites: Map<CallSiteId, CallSiteRecord>
}
```

```text
CallSiteRecord {
  call_site_id: CallSiteId
  owner_function: EirFunctionId
  node_id: NodeId
  call_kind_hint?: CallKind
  arity_shape: ArityShape
  effect?: EffectId
  required_capabilities: List<CapabilityId>
  source_span: SourceSpanId
}
```

---

## 10. AccessSiteTable

```text
AccessSiteTable {
  access_sites: Map<AccessSiteId, AccessSiteRecord>
}
```

```text
AccessSiteRecord {
  access_site_id: AccessSiteId
  owner_function: EirFunctionId
  node_id: NodeId
  access_kind: AccessKind
  receiver_type_hint?: TypeId
  shape_hint?: ShapeId
  field_id?: FieldId
  field_index?: FieldIndex
  source_span: SourceSpanId
}
```

---

## 11. SafepointSeedTable

```text
SafepointSeedTable {
  safepoints: Map<SafepointId, SafepointSeed>
}
```

```text
SafepointSeed {
  safepoint_id: SafepointId
  owner_function: EirFunctionId
  kind: SafepointKind
  source_span?: SourceSpanId
  requires_root_map: Bool
  may_trigger_gc: Bool
}
```

---

## 12. DeoptSeedTable

```text
DeoptSeedTable {
  deopts: Map<DeoptId, DeoptSeed>
}
```

```text
DeoptSeed {
  deopt_id: DeoptId
  owner_function: EirFunctionId
  source_eir_location?: EirLocation
  frame_map_id: FrameMapId
  reason: DeoptReason
  source_span?: SourceSpanId
}
```

---

## 13. RuntimeHelperRequirementTable

```text
RuntimeHelperRequirementTable {
  required_helpers: Map<RuntimeHelperId, RuntimeHelperRequirement>
}
```

```text
RuntimeHelperRequirement {
  helper_id: RuntimeHelperId
  reason: HelperRequirementReason
  required_by: List<EirLocationOrPlanRef>
}
```

All helpers referenced by RuntimePlan/EIR MUST exist in the canonical helper registry.

---

## 14. CapabilityGatePlan

```text
CapabilityGatePlan {
  gates: List<CapabilityGate>
  environment_digest?: Digest
  mutability_policy: CapabilityEnvironmentMutability
}
```

```text
CapabilityGate {
  capability_id: CapabilityId
  effect?: EffectId
  checked_at: CapabilityCheckLocation
  source_span: SourceSpanId
}
```

---

## 15. Validation Requirements

RuntimePlan validation MUST reject:

```text
missing required table
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
slot layout mismatch
function plan without entry EIR function
module plan without initialization function
record shape without field index map
enum shape without case index map
capability gate without capability
cache profile mismatch
source SIR digest mismatch
```

---

## 16. Cache Identity

RuntimePlan cache key MUST include:

```text
source_sir_digest
phase2_schema_version
vm_version
plan_version
target_profile
feature set
capability environment digest or epoch policy
dependency interface digests
stdlib interface digest
helper registry digest
```

---

## 17. Audit Tracking

This document completes:

```text
R4
```

It resolves:

```text
B-02
```

It partially supports:

```text
M-04
M-10
M-11
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-RUNTIME-HELPER-REGISTRY.md -->


# Phase 3 · Runtime Helper Registry

Document class: Normative specification  
Normative status: This document defines the canonical runtime helper registry for Phase 3 VM specifications.

Created: 2026-06-29 09:21:35

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
R5: Create canonical helper registry.
```

It resolves blocker:

```text
B-03: Helper names are referenced outside the helper contract without a canonical helper registry.
```

Runtime helpers are VM-internal semantic slow paths. They are not public ABI.

---

## 1. Registry Rule

Every `RuntimeHelperOp` MUST reference a helper declared in this registry.

No Phase 3 normative document may introduce a helper name outside this registry without amending this registry.

Helper IDs are internal and VM-versioned.

Helper names are descriptive and internal.

---

## 2. Helper Descriptor Schema

Every helper has:

```text
RuntimeHelperEntry {
  helper_id: RuntimeHelperId
  name: HelperName
  family: RuntimeHelperFamily
  signature: RuntimeHelperSignature
  result: HelperResultType
  may_allocate: Bool
  may_raise: Bool
  may_unwind: Bool
  is_safepoint: Bool
  requires_roots_visible: Bool
  required_capability?: CapabilityId
  effect?: EffectId
  gc_behavior: HelperGcBehavior
  jit_call_policy: HelperJitCallPolicy
  source_mapping_policy: HelperSourceMappingPolicy
}
```

---

## 3. Canonical Helper Registry

| Canonical name | Family | Result | Allocate | Raise | Unwind | Safepoint | Roots |
|---|---|---|---:|---:|---:|---:|---:|
| `helper_alloc_object` | Allocation | Value | yes | yes | no | yes | yes |
| `helper_write_barrier` | WriteBarrier | Unit | no | no | no | no | no |
| `helper_construct_error` | Error | Value | yes | no | no | yes | yes |
| `helper_raise` | Error | VmControl | no | yes | yes | no | no |
| `helper_attach_suppressed` | Error | Unit | yes | no | no | yes | yes |
| `helper_assert_fail` | Error | VmControl | yes | yes | yes | yes | yes |
| `helper_check_type_contract` | TypeCheck | Value | no | yes | no | no | no |
| `helper_check_callable` | TypeCheck | Value | no | yes | no | no | no |
| `helper_check_hashable` | TypeCheck | Value | no | yes | no | no | no |
| `helper_check_shape` | TypeCheck | Bool | no | no | no | no | no |
| `helper_numeric_unary` | Numeric | Value | no | yes | no | no | no |
| `helper_numeric_binary` | Numeric | Value | no | yes | no | no | no |
| `helper_compare` | Numeric | Value | no | yes | no | no | no |
| `helper_get_attribute` | Access | Value | maybe | yes | no | maybe | maybe |
| `helper_set_attribute` | Access | Unit | maybe | yes | no | maybe | maybe |
| `helper_bind_method` | Access | Value | yes | yes | no | yes | yes |
| `helper_index_read` | Access | Value | maybe | yes | no | maybe | maybe |
| `helper_index_write` | Access | Unit | maybe | yes | no | maybe | maybe |
| `helper_slice_read` | Access | Value | yes | yes | no | yes | yes |
| `helper_membership` | Access | Value | maybe | yes | no | maybe | maybe |
| `helper_construct_list` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_map` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_record` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_enum` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_function` | Construction | Value | yes | yes | no | yes | yes |
| `helper_generic_call` | Call | VmControl | maybe | yes | yes | yes | yes |
| `helper_call_builtin` | Call | VmControl | maybe | yes | yes | maybe | maybe |
| `helper_check_arity` | Call | Unit | no | yes | no | no | no |
| `helper_match_pattern` | Pattern | HelperInternal | maybe | yes | no | maybe | maybe |
| `helper_perform_unwind` | Unwind | VmControl | maybe | yes | yes | yes | yes |
| `helper_register_defer` | Resource | Unit | maybe | yes | no | maybe | maybe |
| `helper_execute_defer` | Resource | VmControl | maybe | yes | yes | yes | yes |
| `helper_register_resource` | Resource | Unit | maybe | yes | no | maybe | maybe |
| `helper_close_resource` | Resource | VmControl | maybe | yes | yes | yes | yes |
| `helper_resolve_module` | Module | Value | yes | yes | no | yes | yes |
| `helper_initialize_module` | Module | VmControl | yes | yes | yes | yes | yes |
| `helper_import_named` | Module | Value | maybe | yes | no | maybe | maybe |
| `helper_import_module` | Module | Value | maybe | yes | no | maybe | maybe |
| `helper_seal_exports` | Module | Unit | no | yes | no | no | no |
| `helper_check_capability` | Capability | Unit | no | yes | no | no | no |
| `helper_enter_host_call` | Capability | Unit | maybe | yes | no | yes | yes |
| `helper_exit_host_call` | Capability | Value | maybe | yes | no | yes | yes |
| `helper_display` | Display | Value | yes | yes | no | yes | yes |
| `helper_string_concat` | Display | Value | yes | yes | no | yes | yes |
| `helper_load_cell` | Access | Value | no | yes | no | no | no |
| `helper_store_cell` | Access | Unit | maybe | yes | no | maybe | maybe |
| `helper_load_module_slot` | Module | Value | no | yes | no | no | no |

`maybe` means descriptor MUST resolve to true/false in a concrete VM profile or helper specialization.

---

## 4. Helper Result Types

```text
HelperResultType =
  | Value
  | VmControl
  | Unit
  | Bool
  | ErrorRef
  | HelperInternal
```

`HelperInternal` MUST NOT escape as language-visible value.

---

## 5. Source Mapping Policy

Any helper with `may_raise = yes` MUST receive or reconstruct:

```text
SourceSpanId
EirLocation
SIR NodeId where available
```

---

## 6. JIT Policy

A helper callable from JIT MUST have a JIT call policy.

JIT MUST call helpers through the VM-controlled helper table or VM trampoline.

Compiled code MUST NOT directly call arbitrary host/native pointers.

---

## 7. Capability Policy

A helper that performs host/effectful access MUST declare:

```text
required_capability
effect
```

Missing capability raises:

```text
CapabilityError
```

---

## 8. Validation

Helper registry validation MUST reject:

```text
duplicate helper name
duplicate helper id
RuntimeHelperOp referencing missing helper
helper descriptor without implementation
implementation without descriptor
may_collect helper without roots-visible policy
may_raise helper without source mapping policy
JIT-callable helper without JIT policy
capability helper without capability/effect metadata
```

---

## 9. Audit Tracking

This document completes:

```text
R5
```

It resolves:

```text
B-03
```

It partially supports:

```text
M-14
M-11
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-RUNTIME-HELPER-REGISTRY.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-CONTROL-STATE-MODEL.md -->


# Phase 3 · Unified Control State Model

Document class: Normative specification  
Normative status: This document defines the canonical control-state model for Phase 3 VM execution, EIR, helpers, unwinding, interpreter, and JIT.

Created: 2026-06-29 09:21:35

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
R6: Unify control-state model.
```

It resolves blocker:

```text
B-05: PendingControl / VmControl / TerminatorResult are not fully unified.
```

---

## 1. Canonical Control Layers

Phase 3 has these control layers:

```text
ExpressionResult
OpResult
TerminatorResult
PendingControl
VmControl
HelperReturn
FrameExit
```

All layers MUST map to the canonical `ControlState`.

---

## 2. ControlState

```text
ControlState =
  | Normal(Value?)
  | Return(Value?)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
  | Halt
  | Deopt(DeoptId)
  | VmError(VmError)
```

### 2.1 Normal

`Normal` means ordinary execution.

For expression context, `Normal` usually carries a value.

For statement context, `Normal` may carry no value.

### 2.2 Return

`Return` carries optional return value.

If cleanup regions are active, Return MUST first become PendingControl and enter unwinding.

### 2.3 Break / Continue

Break and Continue MUST carry a `ControlRegionId`.

Bare break/continue states are forbidden in canonical Phase 3 control state.

### 2.4 Raise

Raise carries `ErrorHandle`.

Non-Error raise is converted to:

```text
TypeError
```

### 2.5 Halt

Halt is VM-internal.

Source programs MUST NOT observe arbitrary Halt.

### 2.6 Deopt

Deopt is execution-tier transition, not language control.

Deopt MUST reconstruct frame/slot/region/pending-control state.

### 2.7 VmError

VmError is structural failure, not language Error.

---

## 3. ExpressionResult

```text
ExpressionResult =
  | Value(Value)
  | Raise(ErrorHandle)
  | VmError(VmError)
```

Expression evaluation MUST NOT produce Return, Break, Continue, Halt, or Deopt directly except through internal lowering transition.

---

## 4. OpResult

```text
OpResult =
  | Continue
  | Raise(ErrorHandle)
  | Deopt(DeoptId)
  | VmError(VmError)
```

EIR ops write results to slots.

They do not return source values directly except through slot writes.

---

## 5. TerminatorResult

```text
TerminatorResult =
  | NextBlock(EirBlockId)
  | Return(Value?)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
  | Unwind(PendingControl)
  | Deopt(DeoptId)
  | Halt
  | VmError(VmError)
```

A terminator that exits a region with cleanup MUST produce or update PendingControl and enter Unwind.

---

## 6. PendingControl

```text
PendingControl =
  | PendingReturn(Value?)
  | PendingBreak(ControlRegionId)
  | PendingContinue(ControlRegionId)
  | PendingRaise(ErrorHandle)
```

PendingControl is stored in a hidden runtime slot or frame field.

PendingControl MUST be root-visible if it contains heap references.

---

## 7. VmControl

```text
VmControl =
  | Normal(Value?)
  | Return(Value?)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
```

VmControl is the language-level control result used across helpers and function/frame execution.

VmControl MUST NOT carry Deopt or VmError.

---

## 8. HelperReturn

```text
HelperReturn =
  | Value(Value)
  | Control(VmControl)
  | Unit
  | Deopt(DeoptId)
  | Error(VmError)
```

Language errors from helpers MUST be returned as:

```text
Control(Raise(ErrorHandle))
```

Structural failures MUST be returned as:

```text
Error(VmError)
```

---

## 9. FrameExit

```text
FrameExit =
  | Returned(Value?)
  | Raised(ErrorHandle)
  | PropagateBreak(ControlRegionId)
  | PropagateContinue(ControlRegionId)
  | VmError(VmError)
```

A source-level Break/Continue escaping its valid region is a validation failure or LanguageError depending on phase boundary.

---

## 10. Mapping Rules

### 10.1 Return Mapping

```text
TerminatorResult::Return
  -> if cleanup active: PendingReturn -> Unwind
  -> else FrameExit::Returned
```

### 10.2 Raise Mapping

```text
Raise(ErrorHandle)
  -> PendingRaise if cleanup active
  -> FrameExit::Raised if no handler/cleanup remains
```

### 10.3 Break/Continue Mapping

```text
Break(target)
Continue(target)
  -> PendingBreak/PendingContinue if cleanup active or target outside current region
  -> direct branch only if no cleanup is crossed
```

### 10.4 Helper Mapping

```text
HelperReturn::Value(v) -> OpResult::Continue after dest write
HelperReturn::Unit -> OpResult::Continue
HelperReturn::Control(Normal(v)) -> continue or dest write
HelperReturn::Control(Return/Break/Continue/Raise) -> Terminator/control path
HelperReturn::Deopt(id) -> Deopt
HelperReturn::Error(e) -> VmError
```

---

## 11. Deopt and Control

Deopt MUST preserve:

```text
current EIR location
frame slots
region stack
pending control
source span
live roots
```

Deopt is not catchable by source code.

---

## 12. JIT Requirements

Compiled code MUST use the same control-state model.

Compiled return/raise/break/continue MUST NOT skip active cleanup.

Compiled code MUST expose pending control and region stack state at deopt/unwind safepoints.

---

## 13. Validation

Control-state validation MUST reject:

```text
bare Break without ControlRegionId
bare Continue without ControlRegionId
Return outside function
Break outside loop
Continue outside loop
Raise with non-Error value unless converted to TypeError
Deopt without DeoptId
PendingControl containing heap value without root visibility
compiled control transfer that skips cleanup
```

---

## 14. Audit Tracking

This document completes:

```text
R6
```

It resolves:

```text
B-05
```

It partially supports:

```text
B-06
M-10
M-14
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-CONTROL-STATE-MODEL.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md -->


# Phase 3 · Structured Unwinding Algorithm

Document class: Normative specification  
Normative status: This document defines the canonical structured unwinding algorithm for Phase 3 VM execution.

Created: 2026-06-29 09:24:10

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
R7: Canonicalize structured unwinding algorithm.
```

It resolves blocker:

```text
B-06: Structured unwinding algorithm is not yet fully executable-spec closed.
```

This document is the canonical source for cleanup ordering, PendingControl updates, finally override, suppressed errors, and deopt-visible cleanup state.

---

## 1. Scope

This algorithm applies to exits caused by:

```text
Normal block exit
Return
Break
Continue
Raise
defer callable raise
resource close raise
finally non-normal completion
host/resource cleanup failure
```

It covers cleanup registered by:

```text
defer
use/resource
finally
try/catch/finally
loop regions crossed by break/continue
function regions crossed by return
```

---

## 2. Canonical Data Structures

### 2.1 PendingControl

```text
PendingControl =
  | PendingReturn(Value?)
  | PendingBreak(ControlRegionId)
  | PendingContinue(ControlRegionId)
  | PendingRaise(ErrorHandle)
```

PendingControl MUST be root-visible if it contains heap references.

### 2.2 RuntimeRegionFrame

```text
RuntimeRegionFrame {
  region_id: ControlRegionId
  region_kind: ControlRegionKind
  cleanup_state: CleanupState
  loop_target?: LoopTarget
  finally_entry?: EirBlockId
  catch_entries?: List<CatchEntry>
  source_span?: SourceSpanId
}
```

### 2.3 CleanupState

```text
CleanupState {
  defer_stack: List<DeferredCallable>
  resource_stack: List<ResourceCleanup>
  finally_state: FinallyState
  cleanup_progress: CleanupProgress
}
```

### 2.4 CleanupProgress

```text
CleanupProgress =
  | NotStarted
  | RunningDefers
  | RunningResources
  | RunningFinally
  | Complete
```

CleanupProgress MUST be preserved across deopt, helper calls, reentrant host calls, and safepoints.

---

## 3. Cleanup Ordering

When leaving a region, cleanup order is:

```text
1. defer callables in LIFO order
2. resources in reverse acquisition order
3. finally block if present
```

This order is canonical for Phase 3.

A future revision may change it only by reopening structured control semantics.

---

## 4. Entry to Unwinding

Unwinding starts when a control transfer crosses a region that owns cleanup.

```text
Return(value)     -> PendingReturn(value)
Break(region)     -> PendingBreak(region)
Continue(region)  -> PendingContinue(region)
Raise(error)      -> PendingRaise(error)
```

Normal block fall-through does not create PendingControl unless the block exits a cleanup-owning region.

---

## 5. Main Algorithm

Canonical pseudocode:

```text
perform_unwind(frame, pending_control):
  while frame.region_stack is not empty:
    region = frame.region_stack.top()

    if pending_control target is inside region and no cleanup remains:
      return resolve_control_inside_region(pending_control)

    if region.cleanup_state.cleanup_progress == NotStarted:
      region.cleanup_state.cleanup_progress = RunningDefers

    if cleanup_progress == RunningDefers:
      while region.defer_stack not empty:
        defer = pop last defer
        result = call defer()
        if result is non-normal:
          pending_control = combine_cleanup_result(pending_control, result)
      cleanup_progress = RunningResources

    if cleanup_progress == RunningResources:
      while region.resource_stack not empty:
        resource = pop last resource
        result = close resource exactly once
        if result is non-normal:
          pending_control = combine_cleanup_result(pending_control, result)
      cleanup_progress = RunningFinally

    if cleanup_progress == RunningFinally:
      if region has finally and finally not yet run:
        result = run finally block
        if result is non-normal:
          pending_control = finally_override(pending_control, result)
      cleanup_progress = Complete

    if cleanup_progress == Complete:
      pop region
      if pending_control target resolved by popped region:
        return resolve_control_after_region(pending_control)

  return propagate_from_frame(pending_control)
```

---

## 6. Defer Semantics

### 6.1 Registration

A defer statement MUST:

```text
evaluate callable immediately
check callable
check zero-argument call compatibility
register callable in current block/function cleanup region
```

Module top-level defer remains rejected unless later normatively amended.

### 6.2 Execution

Defer callables execute in LIFO order.

A defer callable may:

```text
complete normally
raise
return/break/continue only if callable semantics permit, then it exits that callable frame first
```

The defer result visible to the unwinding region is either Normal or Raise unless the VM permits non-local control from defer callables. Phase 3 minimal VM SHOULD normalize defer callable non-return control to LanguageError or VmStructuralError according to call boundary.

### 6.3 Defer Raise

If defer raises during pending Raise:

```text
primary = existing pending raise
suppressed += defer raise
pending remains primary raise
```

If defer raises during pending Return/Break/Continue/Normal:

```text
pending becomes PendingRaise(defer_error)
```

---

## 7. Resource Semantics

### 7.1 Registration

A resource is registered only after acquisition succeeds.

If acquisition fails, no close is registered.

### 7.2 Close Exactly Once

A registered resource MUST be closed exactly once by structured unwinding unless ownership is explicitly transferred by a future normative mechanism.

Resource states:

```text
Open
Closing
Closed
Failed
```

### 7.3 Close Ordering

Resources close in reverse acquisition order.

### 7.4 Close Raise

If resource close raises during pending Raise:

```text
primary = existing pending raise
suppressed += close raise
pending remains primary raise
```

If resource close raises during pending Return/Break/Continue/Normal:

```text
pending becomes PendingRaise(close_error)
```

---

## 8. Finally Semantics

### 8.1 Execution

A finally block MUST execute when control exits its try/finally region, regardless of whether the exit is:

```text
Normal
Return
Break
Continue
Raise
```

### 8.2 Finally Override

If finally completes normally:

```text
pending_control remains unchanged
```

If finally produces non-normal control:

```text
pending_control = finally_result
```

This is the canonical finally override rule.

Examples:

```text
pending Return + finally Raise -> Raise
pending Raise + finally Return -> Return
pending Break + finally Continue -> Continue
```

### 8.3 Finally Error Suppression

A finally Raise overrides a previous pending Raise.

The previous pending Raise SHOULD be attached as suppressed/context if ErrorObj supports it.

If suppressed/context support is unavailable in bootstrap, diagnostic metadata MUST preserve the overwritten error.

---

## 9. Catch Semantics

A catch region handles PendingRaise only.

When PendingRaise reaches a try/catch region:

```text
for catch in source order:
  bind catch error
  if guard absent or guard evaluates Bool true:
    clear PendingRaise
    execute catch body
    result becomes current control
```

Catch guard condition MUST be Bool.

If guard raises, guard raise becomes current PendingRaise.

If no catch matches, PendingRaise continues unwinding.

---

## 10. Break / Continue Target Resolution

Break and Continue carry target `ControlRegionId`.

When PendingBreak or PendingContinue reaches the target loop region:

```text
run all cleanup crossed so far
resolve to loop break/continue target block
clear PendingControl
```

Break/continue MUST NOT skip cleanup.

---

## 11. Return Resolution

PendingReturn resolves when it exits the current function region after all cleanup has run.

Before final frame exit:

```text
return contract check MUST run
```

If return contract check raises, PendingReturn becomes PendingRaise.

---

## 12. Raise Propagation

PendingRaise propagates outward until:

```text
a matching catch handles it
or function/module/test boundary is crossed
```

At outer boundary, PendingRaise becomes frame/module/test failure.

---

## 13. Normal Exit With Cleanup

Normal exit from a cleanup-owning region runs:

```text
defers
resources
finally
```

If cleanup produces Raise, Normal becomes PendingRaise.

---

## 14. Reentrancy and Safepoints

During cleanup:

```text
pending_control
region_stack
defer_stack
resource_stack
current cleanup progress
active errors
```

MUST be visible to GC and deopt.

If a defer/resource/finally calls into VM or host, cleanup state MUST remain explicitly represented.

---

## 15. JIT Requirements

Compiled code MUST NOT inline or elide cleanup unless it preserves this algorithm.

If compiled code cannot prove cleanup correctness, it MUST call:

```text
helper_perform_unwind
```

JIT deopt metadata at cleanup boundaries MUST reconstruct:

```text
PendingControl
RegionStack
CleanupProgress
active resource/defer state
source span
```

---

## 16. Validation

Validation MUST reject:

```text
Return crossing cleanup without unwind path
Break/Continue crossing cleanup without unwind path
Raise crossing cleanup without unwind path
finally block without override handling
defer registration without cleanup region
resource registration without cleanup region
resource close path without exactly-once state
PendingControl with heap references but no root visibility
cleanup state not reconstructable at deopt point
```

---

## 17. Audit Tracking

This document completes:

```text
R7
```

It resolves:

```text
B-06
```

It partially supports:

```text
B-05
M-10
M-15
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-MODULE-RUNTIME-CONTRACT.md -->


# Phase 3 · Module Runtime Contract

Document class: Normative specification  
Normative status: This document defines the canonical module initialization, import, export, and circular import runtime contract for Phase 3 VM specifications.

Created: 2026-06-29 09:24:10

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
R8: Close module initialization/runtime contract.
```

It resolves blocker:

```text
B-07: Module initialization semantics are not closed at EIR/runtime level.
```

---

## 1. Module Runtime Objects

### 1.1 ModuleInstance

```text
ModuleInstance {
  module_id: ModuleId
  state: ModuleState
  module_object: ObjRef
  module_slots: SlotArray
  export_table: ExportTable
  interface_descriptor: ModuleInterfaceDescriptor
  initialization_error?: ErrorHandle
  initialization_function: EirFunctionId
  source_span?: SourceSpanId
}
```

### 1.2 ModuleState

```text
ModuleState =
  | Unloaded
  | Loading
  | Initializing
  | Initialized
  | Failed
```

No other module states are allowed in Phase 3.

---

## 2. State Transitions

Allowed transitions:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
Loading -> Failed
Failed -> Loading only by explicit host retry
```

Forbidden transitions:

```text
Initialized -> Loading
Initialized -> Initializing
Failed -> Initializing directly
Failed -> Initialized directly
Unloaded -> Initializing
```

---

## 3. Explicit Retry Policy

A Failed module MUST NOT automatically retry initialization.

Retry is allowed only if the host explicitly requests retry through a VM-controlled module reload/retry operation.

On retry:

```text
previous initialization_error remains diagnosable
new attempt uses fresh initialization state
module identity policy must be host-defined and deterministic
```

If Phase 3 minimal VM does not expose retry, `Failed -> Loading` remains a reserved transition.

---

## 4. Module Initialization Function

Each module has one synthetic initialization function:

```text
module_init(module_id) -> VmControl
```

It executes top-level declarations and imports in source order.

Top-level `return`, `break`, and `continue` are invalid and MUST be rejected before execution.

A top-level raise fails module initialization.

---

## 5. Import Execution

Imports execute in source order.

Import resolution is deterministic and host-defined.

No implicit relative import is allowed unless the module resolver explicitly defines it.

Import kinds:

```text
WholeModuleImport
NamedImport
AliasedNamedImport
```

---

## 6. Whole Module Import

Whole module import binds a module value to the local import binding.

If provider module is not initialized, VM MUST initialize or continue initializing it according to the module graph rules.

If provider initialization fails, importer fails with ImportError or propagated initialization error.

---

## 7. Named Import

Named import binds an exported binding value from provider module.

Named import MUST check:

```text
provider module resolved
export exists
export initialized
interface compatible
local binding slot valid
```

If export exists but is uninitialized due to circular import, VM raises:

```text
ImportCycleError
```

---

## 8. Export Table

### 8.1 ExportTable

```text
ExportTable {
  entries: Map<String, ExportEntry>
  sealed: Bool
}
```

### 8.2 ExportEntry

```text
ExportEntry {
  name: String
  binding_id: BindingId
  slot_id: SlotId
  initialized: Bool
  type_id?: TypeId
  source_span: SourceSpanId
}
```

### 8.3 Sealing

Export table MUST be sealed after successful module initialization.

After sealing, export table shape MUST NOT change.

Exported binding values may remain live through their slots/cells.

---

## 9. Circular Imports

Circular imports are permitted only under strict access rules.

If module A imports module B while B is Initializing:

```text
A may access already initialized exports of B.
A MUST NOT access uninitialized exports of B.
```

Accessing uninitialized circular export raises:

```text
ImportCycleError
```

This applies to both named imports and module object export access.

---

## 10. Initialization Failure

If module initialization raises:

```text
state = Failed
initialization_error = raised error
export table remains unsealed or marked failed
partially initialized exports are not considered successfully initialized for future imports unless explicitly allowed by retry policy
```

Future ordinary imports of Failed module MUST fail.

They MUST NOT silently reinitialize.

---

## 11. Module Rooting

During Loading/Initializing/Initialized/Failed, module instance and its live values MUST be GC roots if reachable from module environment or import graph.

Rooted values include:

```text
module object
module slots
export table entries
initialization_error
imported module references
module constants
```

---

## 12. Module Resolver and Capabilities

Module resolver is host-defined.

If resolver performs effectful host access, it MUST be capability-gated.

Missing capability raises:

```text
CapabilityError
```

The capability environment policy must be included in RuntimePlan/module cache compatibility when relevant.

---

## 13. Interface Compatibility

A module import MUST validate provider interface against required `ModuleInterfaceDescriptor`.

If interface contains unknown required fields, conservative rejection is required.

Compatibility failure raises:

```text
ImportError
```

or a more specific module-interface error if added to the runtime error registry.

---

## 14. EIR Runtime Contract

Module initialization EIR MUST:

```text
use ModulePlan initialization_function
use module slot layout
write exports through export slots/cells
execute source order
call module helpers for import/resolve/seal
preserve source spans
use RuntimePlan ImportPlan and ExportPlan
```

Module import operations MUST go through canonical helpers:

```text
helper_resolve_module
helper_initialize_module
helper_import_named
helper_import_module
helper_seal_exports
```

---

## 15. Test Blocks

Test blocks are not executed during ordinary module initialization.

Test runner MAY execute test blocks through separate test entry functions.

Test failures MUST report source spans.

---

## 16. Validation

Module runtime validation MUST reject:

```text
module without initialization function
module plan without module slot layout
import entry without source span
export entry without binding slot
duplicate export names
export table mutation after sealing
named import of missing export
access to uninitialized circular export
automatic retry after Failed
top-level return/break/continue
module helper missing from helper registry
effectful resolver without capability declaration
```

---

## 17. Audit Tracking

This document completes:

```text
R8
```

It resolves:

```text
B-07
```

It partially supports:

```text
M-10
M-11
M-13
M-15
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-MODULE-RUNTIME-CONTRACT.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md -->


# Phase 3 · SIR Lowering Coverage Matrix

Document class: Normative specification  
Normative status: This document defines required Phase 2 SIR node coverage for Phase 3 RuntimePlan/EIR lowering.

Created: 2026-06-29 09:24:10

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
R9: Add SIR node lowering coverage matrix.
```

It resolves blocker:

```text
B-08: SIR lowering coverage is not demonstrably complete.
```

Every Phase 2 frozen SIR node kind MUST be either:

```text
Lowered
RuntimePlan-only
Validation-rejected
Deferred outside Phase 3 minimal VM
```

---

## 1. Coverage Status Values

```text
Lowered
  Must lower to RuntimePlan and/or EIR.

RuntimePlan-only
  Represented in RuntimePlan metadata, not as executable EIR op.

Validation-rejected
  Invalid in the relevant context and must be rejected before execution.

Deferred
  Not required for Phase 3 minimal VM. Must not be generated unless feature is enabled.
```

---

## 2. Top-Level Nodes

| SIR node | Phase 3 coverage | Required target |
|---|---|---|
| `ModuleBodyNode` | Lowered | ModulePlan + module init EIR |
| `DeclarationNode` | Lowered | binding/slot/type/module plans |
| `BindingNode` | Lowered | SlotDescriptor / StoreOp / PatternOp |
| `ExpressionNode` | Lowered | EirOp / EirTerminator where applicable |
| `AssignmentNode` | Lowered | StoreOp / AccessOp |
| `FunctionNode` | Lowered | FunctionPlan + ConstructFunction + EirFunction |
| `RecordNode` | Lowered | ShapePlan + TypePlan + constructor path |
| `EnumNode` | Lowered | ShapePlan + TypePlan + enum constructor path |
| `BlockNode` | Lowered | EirBlock graph + RegionPlan |
| `IfNode` | Lowered | Branch + blocks |
| `WhileNode` | Lowered | loop blocks + LoopBackedge safepoint |
| `ForNode` | Lowered | iterator plan + loop blocks |
| `MatchNode` | Lowered | PatternOp graph + branch/case blocks |
| `ReturnNode` | Lowered | Return terminator + unwind path if needed |
| `BreakNode` | Lowered | PendingBreak / branch / unwind |
| `ContinueNode` | Lowered | PendingContinue / branch / unwind |
| `RaiseNode` | Lowered | Raise terminator + type check |
| `TryNode` | Lowered | region/catch/finally plan + unwind |
| `UseNode` | Lowered | resource registration + cleanup region |
| `DeferNode` | Lowered | defer registration in cleanup region |
| `AssertNode` | Lowered | branch/check + helper_assert_fail |
| `TestNode` | Lowered | TestPlan / test entry EIR |
| `CheckNode` | Lowered | CheckOp / RuntimePlan metadata |

---

## 3. Declarations

| Declaration | Coverage | Target |
|---|---|---|
| `let` | Lowered | SlotDescriptor + initializer + StoreSlot/StoreCell |
| `const` | Lowered | immutable SlotDescriptor + initializer |
| `def` | Lowered | FunctionPlan + function object construction when reached |
| `record` | Lowered | RecordShape + TypePlan + constructor |
| `enum` | Lowered | EnumShape + TypePlan + case constructors |
| `import` | Lowered | ImportPlan + module helper calls |
| `export` | Lowered | ExportPlan + export table entry |
| `test` | Lowered | TestPlan + test entry |

Function declarations MUST NOT be hoisted unless already specified by Phase 1/2. Function binding is created when declaration executes.

---

## 4. Expressions

| Expression | Coverage | Target |
|---|---|---|
| nil/bool/int/float/string literal | Lowered | ConstantOp |
| list literal | Lowered | ConstructList |
| map literal | Lowered | ConstructMap |
| binding reference | Lowered | LoadSlot/LoadCell/LoadModuleSlot |
| unary expression | Lowered | UnaryOp |
| binary expression | Lowered | BinaryOp / helper fallback |
| logical and/or | Lowered | Branch-based short-circuit graph |
| call expression | Lowered | CallOp + CallSiteId |
| attribute access | Lowered | AccessOp / LoadField |
| index access | Lowered | AccessOp |
| slice access | Lowered | AccessOp / helper_slice_read |
| record construction | Lowered | ConstructRecord |
| enum construction | Lowered | ConstructEnumValue |
| function expression | Lowered | ConstructFunction |
| format string | Lowered | ordered expression eval + helper_display/string concat |
| readonly view | Lowered | helper or ConstructReadOnlyView if represented |
| error construction | Lowered | ConstructError/helper_construct_error |

---

## 5. Assignment Targets

| Assignment target | Coverage | Target |
|---|---|---|
| binding target | Lowered | StoreSlot/StoreCell |
| field target | Lowered | StoreField |
| index target | Lowered | StoreListIndex/StoreMapEntry |
| destructuring target | Lowered | PatternOp + commit/rollback |

Augmented assignment MUST evaluate target once.

---

## 6. Pattern Variants

| Pattern | Coverage | Target |
|---|---|---|
| Wildcard | Lowered | PatternBranch/PatternOp no binding |
| Literal | Lowered | PatternCheckLiteral |
| Binding | Lowered | PatternBind + commit |
| Record | Lowered | PatternCheckRecordShape + field loads |
| Enum | Lowered | PatternCheckEnumCase + payload loads |
| List | Lowered | PatternCheckListLength + element checks |
| Map | Lowered | PatternCheckMapKey + value checks |
| Or | Lowered | alternative subgraphs with same binding set |

Pattern lowering MUST distinguish match failure from declaration destructuring failure.

---

## 7. Structured Control

| Construct | Coverage | Target |
|---|---|---|
| block | Lowered | EirBlock + optional RegionFrame |
| if | Lowered | Branch + merge |
| while | Lowered | loop header/body/exit + LoopBackedge |
| for list | Lowered | iterator loop preserving order |
| for map | Lowered | key iteration in insertion order |
| for range | Lowered | range iterator loop |
| match | Lowered | subject once + ordered case graph |
| try/catch | Lowered | catch region + raise matching |
| finally | Lowered | cleanup region + finally override |
| use | Lowered | resource acquisition + cleanup |
| defer | Lowered | deferred callable registration |
| return | Lowered | Return/PendingReturn/unwind |
| break | Lowered | Break/PendingBreak/unwind |
| continue | Lowered | Continue/PendingContinue/unwind |
| raise | Lowered | Raise/PendingRaise/unwind |

String iteration is not core and MUST NOT be lowered unless later amended.

---

## 8. Module Integration

| SIR source | Coverage | Target |
|---|---|---|
| top-level source order | Lowered | module init EIR |
| import source order | Lowered | ImportPlan order |
| whole module import | Lowered | helper_import_module |
| named import | Lowered | helper_import_named |
| export | Lowered | ExportPlan |
| circular initialized export access | Lowered | module runtime check |
| circular uninitialized export access | Lowered | ImportCycleError |
| module failure | Lowered | ModuleState Failed + initialization_error |

---

## 9. Capability and Effects

| SIR item | Coverage | Target |
|---|---|---|
| `requires` metadata | RuntimePlan-only | CapabilityGatePlan |
| `effect[...]` metadata | RuntimePlan-only | FunctionPlan/CallSite/helper metadata |
| host capability access | Lowered | CheckCapability/helper boundary |
| missing capability | Lowered | CapabilityError |

---

## 10. Validation-Rejected Contexts

The following MUST be rejected before execution:

```text
return outside function
break outside loop
continue outside loop
module top-level defer unless amended
top-level source control transfer
non-Bool condition
or-pattern alternatives with different binding sets
record declaration duplicate fields
enum declaration duplicate cases
unknown import/export binding
assignment to const
assignment to read-only view
```

---

## 11. Deferred Features

The following are not required in Phase 3 minimal VM unless explicitly enabled:

```text
public bytecode
FFI
native extension ABI
async/await
threads
generator/yield
string iteration
user-defined operator protocols
dynamic record field creation
```

If a deferred feature appears in SIR without feature enablement, validation MUST reject it.

---

## 12. Coverage Validation

The lowering validator MUST reject any SIR node that lacks a declared coverage route in this matrix.

Each lowering route MUST preserve:

```text
evaluation order
binding identity
source spans
type checks
capability/effect metadata
module import order
nominal record/enum identity
pattern binding semantics
defer/use/finally order
primary/suppressed error behavior
```

---

## 13. Audit Tracking

This document completes:

```text
R9
```

It resolves:

```text
B-08
```

It partially supports:

```text
M-15
M-10
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-GC-METADATA-OWNERSHIP.md -->


# Phase 3 · GC Metadata Ownership

Document class: Normative specification  
Normative status: This document defines canonical ownership and projection rules for RootMap, FrameMap, SafepointRecord, StackMap, and DeoptPoint metadata.

Created: 2026-06-29 09:26:37

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
R10: Canonicalize RootMap/FrameMap/SafepointRecord ownership.
```

It addresses major finding:

```text
M-03: RootMap / FrameMap / SafepointRecord definitions are duplicated across documents.
```

This document is the canonical cross-document ownership map for GC/JIT/interpreter metadata.

---

## 1. Canonical Ownership Rule

Each metadata family has one canonical owner.

Other documents MAY define projections, uses, or lowering references, but MUST NOT redefine the canonical schema incompatibly.

| Metadata | Canonical owner | Secondary users |
|---|---|---|
| `RootMap` | GC Metadata Ownership + GC Safepoint Root Model | interpreter, JIT, helpers |
| `FrameMap` | GC Metadata Ownership + GC Safepoint Root Model | interpreter, diagnostics, deopt |
| `SafepointRecord` | GC Metadata Ownership + GC Safepoint Root Model | interpreter, JIT, helpers |
| `StackMap` | Baseline JIT Backend Interface under GC metadata constraints | GC, deopt, compiled code |
| `DeoptPoint` | RuntimePlan Schema Closure + JIT projection | interpreter, JIT, diagnostics |
| `RegionStackState` | Structured Unwinding Algorithm + Control State Model | GC, JIT, deopt |
| `PendingControlState` | Control State Model | GC, unwind, JIT, deopt |

---

## 2. RootMap

### 2.1 Canonical Schema

```text
RootMap {
  root_map_id: RootMapId
  owner: RootMapOwner
  safepoint_id?: SafepointId
  frame_map_id?: FrameMapId
  roots: List<RootLocation>
  source_span?: SourceSpanId
}
```

### 2.2 RootMapOwner

```text
RootMapOwner =
  | InterpreterFrame
  | EirFunction
  | RuntimeHelper
  | JitCompiledFunction
  | HostBoundary
  | ModuleInitialization
```

### 2.3 RootLocation

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

### 2.4 RootMap Requirements

A RootMap MUST be available at every safepoint where GC may run.

A RootMap MUST be updateable if moving GC is enabled.

A RootMap MUST NOT rely on conservative scanning under moving GC profile.

---

## 3. FrameMap

### 3.1 Canonical Schema

```text
FrameMap {
  frame_map_id: FrameMapId
  owner_function: EirFunctionId
  source_function?: FunctionId
  module_id: ModuleId
  slot_layout: SlotLayoutId
  visible_bindings: List<VisibleBinding>
  region_state_schema: RegionStateSchema
  source_span?: SourceSpanId
}
```

### 3.2 VisibleBinding

```text
VisibleBinding {
  binding_id: BindingId
  slot_id: SlotId
  visibility: BindingVisibility
  value_kind_hint?: RuntimeValueKind
  source_span?: SourceSpanId
}
```

### 3.3 FrameMap Uses

FrameMap supports:

```text
source stack trace
debug inspection
deopt reconstruction
GC root enumeration
error diagnostics
interpreter/JIT bridge
```

FrameMap is internal metadata and not public ABI.

---

## 4. SafepointRecord

### 4.1 Canonical Schema

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

### 4.2 SafepointKind

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

### 4.3 SafepointOwner

```text
SafepointOwner =
  | Interpreter
  | EirFunction
  | RuntimeHelper
  | JitCompiledFunction
  | HostCall
```

### 4.4 Safepoint Requirements

A SafepointRecord MUST link to RootMap when GC can run.

A SafepointRecord MUST link to FrameMap when deopt, stack trace, or debugging may inspect the frame.

---

## 5. StackMap Projection

StackMap is JIT-specific projection of RootMap and FrameMap.

```text
StackMap {
  stack_map_id: StackMapId
  compiled_function_id: CompiledFunctionId
  code_offset: CodeOffset
  live_value_locations: List<ValueLocation>
  frame_state: FrameStateRef
  source_span?: SourceSpanId
}
```

StackMap MUST be sufficient to reconstruct/update live heap references at compiled safepoints.

StackMap MUST NOT replace RootMap as canonical cross-tier root metadata.

---

## 6. DeoptPoint Projection

DeoptPoint canonical semantic seed belongs to RuntimePlan.

JIT deopt records project that seed to code offsets.

```text
JitDeoptRecord {
  deopt_id: DeoptId
  code_offset: CodeOffset
  source_eir_location: EirLocation
  frame_map: FrameMapId
  root_map?: RootMapId
  region_stack_state: RegionStackState
  pending_control_state?: PendingControlState
  resume_target: DeoptResumeTarget
}
```

Deopt metadata MUST preserve enough state to resume in EIR interpreter or unwind helper.

---

## 7. Ownership Constraints

Secondary documents MUST follow these rules:

```text
RuntimePlan documents may reference FrameMapId/RootMapId/SafepointId but MUST NOT redefine their schema.
EIR documents may attach safepoint/root/frame references but MUST NOT redefine canonical GC metadata.
JIT documents may define StackMap projection but MUST preserve RootMap/FrameMap semantics.
Interpreter documents may define runtime storage but MUST preserve FrameMap/RootMap visibility.
Helper documents may define helper safepoint behavior but MUST reference SafepointRecord.
```

---

## 8. Validation

GC metadata validation MUST reject:

```text
safepoint without RootMap when GC may run
JIT safepoint without StackMap
FrameMap referencing unknown SlotLayout
RootMap referencing unknown SlotId
RootMap non-updateable under moving GC profile
DeoptPoint without FrameMap
helper may_collect without SafepointRecord
compiled helper call without stack/root metadata
duplicate incompatible schema definitions
```

---

## 9. Audit Tracking

This document completes:

```text
R10
```

It addresses:

```text
M-03
```

It supports:

```text
R13
R14
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-GC-METADATA-OWNERSHIP.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-TARGET-PROFILE-SCHEMAS.md -->


# Phase 3 · Target and Runtime Profile Schemas

Document class: Normative specification  
Normative status: This document defines ValueLayoutProfile, HeapProfile, GcProfile, InterpreterProfile, JitProfile, TargetProfile, and their cache compatibility roles.

Created: 2026-06-29 09:26:37

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
R11: Define ValueLayoutProfile / HeapProfile / GcProfile / TargetProfile schemas.
```

It addresses major finding:

```text
M-04: ValueLayoutProfile is referenced before being normatively defined.
```

---

## 1. Profile Boundary

Profiles are VM-internal compatibility descriptors.

They are not public ABI.

They participate in:

```text
RuntimePlan cache key
EIR cache key
JIT cache key
helper table compatibility
GC metadata compatibility
module dependency compatibility
```

---

## 2. RuntimeTargetProfile

```text
RuntimeTargetProfile {
  profile_version: Version
  vm_version: Version
  architecture: TargetArchitecture
  operating_system?: TargetOperatingSystem
  pointer_width: UInt
  endianness: Endianness
  value_layout_profile: ValueLayoutProfile
  heap_profile: HeapProfile
  gc_profile: GcProfile
  interpreter_profile: InterpreterProfile
  jit_profile?: JitProfile
  capability_profile: CapabilityProfile
}
```

---

## 3. ValueLayoutProfile

```text
ValueLayoutProfile {
  value_layout_id: String
  representation: ValueRepresentation
  immediate_kinds: List<RuntimeValueKind>
  heap_ref_kinds: List<RuntimeValueKind>
  identity_policy: IdentityPolicy
  numeric_int_policy: NumericIntPolicy
  float_policy: FloatPolicy
}
```

### 3.1 ValueRepresentation

```text
ValueRepresentation =
  | RustEnumBootstrap
  | TaggedPointer
  | NaNBoxing
  | CompressedHandle
  | OpaqueHandle
```

No document may assume Rust enum layout unless profile explicitly says `RustEnumBootstrap`.

Even then, Rust enum layout remains internal and not public ABI.

### 3.2 NumericIntPolicy

```text
NumericIntPolicy =
  | CheckedI64
  | ArbitraryPrecision
```

Silent integer wrap is forbidden.

### 3.3 FloatPolicy

```text
FloatPolicy {
  format: FloatFormat
  finite_only_for_serializable: Bool
  nan_key_policy: FloatNaNKeyPolicy
  negative_zero_policy: NegativeZeroPolicy
}
```

---

## 4. HeapProfile

```text
HeapProfile {
  heap_profile_id: String
  object_reference_model: ObjectReferenceModel
  object_store_model: ObjectStoreModel
  object_identity_model: ObjectIdentityModel
  moving_allowed: Bool
  stale_handle_detection: Bool
}
```

```text
ObjectReferenceModel =
  | ObjRefHandle
  | HandleTable
  | DirectTaggedReference
  | CompressedReference
```

```text
ObjectStoreModel =
  | Arena
  | GenerationalArena
  | SlotMap
  | MovingHeap
  | Custom
```

---

## 5. GcProfile

```text
GcProfile {
  gc_profile_id: String
  collection_model: CollectionModel
  moving: Bool
  generational: Bool
  incremental: Bool
  concurrent: Bool
  requires_write_barrier: Bool
  requires_precise_roots: Bool
  safepoint_policy: SafepointPolicy
}
```

```text
CollectionModel =
  | NoCollectionBootstrap
  | NonMovingTracing
  | GenerationalTracing
  | MovingCompacting
  | Incremental
  | Concurrent
```

Moving GC requires:

```text
requires_precise_roots = true
```

Generational or incremental GC requires:

```text
requires_write_barrier = true
```

---

## 6. InterpreterProfile

```text
InterpreterProfile {
  interpreter_profile_id: String
  dispatch_model: InterpreterDispatchModel
  root_mode: InterpreterRootMode
  quickening_enabled: Bool
  feedback_enabled: Bool
  deterministic_mode: Bool
}
```

```text
InterpreterDispatchModel =
  | MatchDispatch
  | FunctionTableDispatch
  | ThreadedDispatch
  | QuickenedDispatch
```

```text
InterpreterRootMode =
  | AllInitializedSlots
  | LivenessDerivedSlots
  | DebugAllSlots
```

---

## 7. JitProfile

```text
JitProfile {
  jit_profile_id: String
  enabled: Bool
  backend_kind?: JitBackendKind
  backend_version?: Version
  compile_mode?: BaselineCompileMode
  stack_maps_required: Bool
  deopt_required: Bool
  helper_trampoline_abi: HelperTrampolineAbiProfile
}
```

If JIT is disabled, JitProfile may be absent.

If JIT is enabled under moving GC:

```text
stack_maps_required = true
```

---

## 8. CapabilityProfile

```text
CapabilityProfile {
  environment_model: CapabilityEnvironmentModel
  mutability_policy: CapabilityEnvironmentMutability
  digest_or_epoch_policy: CapabilityDigestPolicy
}
```

```text
CapabilityEnvironmentMutability =
  | ImmutableForVmRun
  | ImmutablePerModule
  | MutableWithEpoch
  | HostControlledWithInvalidation
```

JIT may cache capability checks only if mutability policy and invalidation are defined.

---

## 9. TargetArchitecture

```text
TargetArchitecture =
  | X86_64
  | AArch64
  | Wasm
  | InterpreterPseudoTarget
  | Other
```

---

## 10. Digest Participation

RuntimeTargetProfile digest MUST include:

```text
ValueLayoutProfile
HeapProfile
GcProfile
InterpreterProfile
JitProfile if present
CapabilityProfile
architecture
pointer width
endianness
VM version
profile version
```

Changing any profile field invalidates dependent RuntimePlan/EIR/JIT caches unless explicitly marked non-semantic and non-layout-affecting.

---

## 11. Validation

Profile validation MUST reject:

```text
moving GC without precise roots
generational GC without write barrier requirement
JIT enabled under moving GC without stack maps
RustEnumBootstrap exposed as public ABI
CheckedI64 without overflow error policy
capability mutable without epoch/invalidation policy
direct tagged refs with host raw pointer exposure
```

---

## 12. Audit Tracking

This document completes:

```text
R11
```

It addresses:

```text
M-04
M-05 partial
M-11 partial
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-TARGET-PROFILE-SCHEMAS.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-VALUE-KEY-STRING-SEMANTICS.md -->


# Phase 3 · Value Key and String Runtime Semantics

Document class: Normative specification  
Normative status: This document defines canonical map key/hash/equality semantics and string runtime constraints for Phase 3 VM specifications.

Created: 2026-06-29 09:26:37

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
R12: Define map key/hash/equality and string runtime constraints.
```

It addresses major findings:

```text
M-06: Map key/hash/equality semantics need closure.
M-07: String model needs sharper runtime constraints.
```

---

## 1. ValueKey

Map keys are represented by canonical `ValueKey`.

```text
ValueKey =
  | BoolKey(Bool)
  | IntKey(IntCanonical)
  | FloatKey(FloatCanonical)
  | StringKey(StringCanonical)
  | EnumKey(EnumIdentity, CaseIndex, PayloadKey?)
  | NilKey
```

Phase 3 minimal VM MUST NOT permit mutable aggregate values as map keys.

Forbidden as map keys:

```text
List
Map
RecordInstance
ReadOnlyView over mutable aggregate
Function
BuiltinFunction
Module
Resource
Error
HostObjectWrapper
```

unless a future explicit hash protocol is added.

---

## 2. Hashability Rule

A value is hashable only if:

```text
its equality is stable
its hash is stable
it cannot be mutated in a way that changes equality/hash while stored as key
```

Readonly view does not automatically make an underlying mutable object hashable.

---

## 3. Int Keys

Int keys use mathematical integer value.

If implementation uses checked i64, key canonicalization is i64 value.

If implementation uses arbitrary precision, key canonicalization is arbitrary-precision integer value.

---

## 4. Float Keys

Float keys are allowed only for finite Float values.

NaN keys are forbidden in Phase 3 minimal VM.

Infinity handling follows Float runtime policy; if Infinity is supported as ordinary Float, it MAY be allowed as FloatKey only if equality/hash are stable. Serializable values still require finite Float.

### 4.1 Negative Zero

`-0.0` and `0.0` MUST compare equal as Float values unless FloatPolicy later defines stricter distinction.

If they compare equal, they MUST hash to the same FloatKey.

---

## 5. String Keys

String keys use string scalar sequence equality.

String identity/interning MUST NOT affect key equality.

Two strings with the same scalar sequence are the same StringKey.

---

## 6. Enum Keys

Enum values MAY be hashable only if:

```text
enum identity is nominal and stable
case identity is stable
payload is absent or payload is itself hashable
```

If payload contains non-hashable value, enum value is non-hashable.

---

## 7. Equality and Hash Consistency

For all hashable keys:

```text
a == b implies hash(a) == hash(b)
```

Hash collision is allowed.

Hash collision MUST NOT imply equality.

---

## 8. Map Duplicate Key Rule

When constructing a map:

```text
later value replaces earlier value
first insertion position is preserved
```

Example semantic sequence:

```text
insert k -> position p
insert k again -> update value at p
```

Iteration order uses first insertion position.

---

## 9. Map Iteration

Map iteration yields keys in insertion order.

For `for` over Map:

```text
iteration value = key
order = insertion order
```

---

## 10. String Runtime Model

A String is an immutable Unicode scalar sequence.

The VM may store strings as UTF-8, UTF-16, rope, interned object, or other internal representation.

Internal representation is not public ABI.

---

## 11. String Length

`len(String)` returns the number of Unicode scalar values.

It does not return bytes.

If future language revision changes length semantics, this document must be reopened.

---

## 12. String Indexing

String indexing is not core Phase 3.

A source program MUST NOT assume:

```text
s[i] returns character
s[i] returns byte
s[i] returns scalar
```

String indexing must be rejected unless a later feature explicitly enables it.

---

## 13. String Slicing

String slicing is core if slice operation is available for strings.

Slice bounds are scalar indices.

Rules:

```text
slice is half-open [start, end)
start and end must be Int
negative bounds raise IndexError
out-of-range bounds raise IndexError
start > end raises IndexError
slice result preserves scalar sequence
```

The VM MUST NOT split invalid internal encoding boundaries.

If internal representation is UTF-8, scalar index to byte offset mapping is VM-internal.

---

## 14. String Constants and Interning

The VM MAY intern string constants.

Interning MUST NOT change:

```text
equality
hashing
display
serialization
source-visible behavior
```

Identity of strings is implementation-defined unless the language later defines string identity semantics.

JIT MUST NOT rely on string object identity for equality unless guarded by canonical interning policy.

---

## 15. Display Semantics

`helper_display` converts values to display strings.

Display conversion does not create implicit coercion for:

```text
binary operators
type contracts
map keys
pattern matching
```

String display of a String value returns its scalar sequence or escaped diagnostic representation depending on call site.

---

## 16. Format Strings

Format strings evaluate embedded expressions left-to-right.

Each embedded value is converted through display semantics.

Display failures raise language error at the embedded expression source span if possible.

---

## 17. Validation

Validation/runtime MUST reject:

```text
non-hashable map key
NaN map key
mutable aggregate as map key
string indexing in core Phase 3
negative string slice bound
out-of-range string slice bound
string slice start > end
hash/equality mismatch for key type
```

---

## 18. Audit Tracking

This document completes:

```text
R12
```

It addresses:

```text
M-06
M-07
```

It supports:

```text
R13
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-VALUE-KEY-STRING-SEMANTICS.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-VALIDATION-MATRIX.md -->


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


<!-- END NORMATIVE DOCUMENT: PHASE-3-VALIDATION-MATRIX.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-CACHE-COMPATIBILITY-MATRIX.md -->


# Phase 3 · Cache Compatibility Matrix

Document class: Normative specification  
Normative status: This document defines canonical cache compatibility and invalidation rules for RuntimePlan, EIR, helper table, GC metadata, JIT artifacts, module interfaces, and capability profiles.

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
R14: Create canonical cache compatibility matrix.
```

It addresses major finding:

```text
M-11: Internal cache compatibility rules are distributed and need consolidation.
```

---

## 1. Cache Boundary

All Phase 3 caches are internal.

No cache is:

```text
public bytecode
package ABI
native ABI
foreign extension ABI
portable user artifact
```

Caches are discardable.

If compatibility cannot be proven, the VM MUST discard and rebuild the cache.

---

## 2. Cache Kinds

```text
RuntimePlanCache
EirCache
HelperRegistryCache
ModuleInterfaceCache
GcMetadataCache
JitCodeCache
ProfileCache
DiagnosticSourceMapCache
```

---

## 3. Universal Cache Key Components

Every cache key MUST include or be derived from:

```text
vm_version
phase3_schema_version
document/spec revision id or equivalent
target/runtime profile digest where relevant
feature set
source digest where relevant
dependency digest where relevant
```

---

## 4. RuntimePlan Cache Key

RuntimePlan cache key MUST include:

```text
source_sir_digest
phase2_schema_version
phase3_runtimeplan_schema_version
vm_version
RuntimeTargetProfile digest
feature set
dependency module interface digests
stdlib interface digest
helper registry digest
capability profile digest or epoch policy
```

Invalidation required when any component changes.

---

## 5. EIR Cache Key

EIR cache key MUST include:

```text
RuntimePlan digest
EIR schema version
VM version
target profile digest
helper registry digest
GC metadata schema version
source map digest
```

Invalidation required when:

```text
EIR op schema changes
RuntimePlan changes
helper signatures change
target/value layout changes
GC/safepoint profile changes
source mapping changes in a way that affects diagnostics
```

---

## 6. Helper Registry Digest

Helper registry digest MUST include for each helper:

```text
helper id
canonical name
family
signature
result type
may_allocate
may_raise
may_unwind
is_safepoint
requires_roots_visible
required_capability
effect
gc_behavior
jit_call_policy
source_mapping_policy
```

Changing helper implementation without descriptor change MAY avoid cache invalidation only if behavior is semantically identical.

Changing helper descriptor MUST invalidate dependent RuntimePlan/EIR/JIT caches.

---

## 7. Module Interface Cache Key

Module interface cache key MUST include:

```text
module source digest or semantic digest
export table shape
exported names
exported binding identities
exported type/interface descriptors
feature set
dependency interface digests
stdlib interface digest
```

Changing implementation body without interface change MAY preserve dependent import compatibility.

Changing exported shape or required interface MUST invalidate dependents.

---

## 8. GC Metadata Cache Key

GC metadata cache key MUST include:

```text
RootMap schema version
FrameMap schema version
SafepointRecord schema version
StackMap schema version if JIT
GcProfile digest
HeapProfile digest
ValueLayoutProfile digest
write barrier policy
moving/nonmoving policy
```

Invalidation required when:

```text
root representation changes
slot layout changes
frame layout changes
GC profile changes
moving policy changes
barrier policy changes
JIT stack map schema changes
```

---

## 9. JIT Code Cache Key

JIT code cache key MUST include:

```text
EIR digest
RuntimePlan digest
helper registry digest
ValueLayoutProfile digest
HeapProfile digest
GcProfile digest
JitProfile digest
target architecture
pointer width
endianness
backend kind
backend version
compile mode
source map digest
deopt metadata digest
stack map digest
capability profile digest/epoch policy
```

JIT cache MUST be invalidated when any safety-relevant metadata changes.

JIT cache MUST NOT be reused across incompatible target profiles.

---

## 10. Capability Cache Compatibility

Capability environment policy controls cacheability.

```text
ImmutableForVmRun
  cache may assume capability set for one VM run.

ImmutablePerModule
  cache may assume capability set per module, keyed by module capability digest.

MutableWithEpoch
  cache must include capability epoch and invalidate on epoch change.

HostControlledWithInvalidation
  cache valid only if host invalidation protocol is active.
```

JIT may cache capability checks only under an explicit capability profile.

---

## 11. Diagnostic Source Map Cache

Diagnostic source map cache key MUST include:

```text
source file digest
SIR source map digest
RuntimePlan source map digest
EIR source map digest
VM version
```

Source map cache mismatch MUST NOT produce incorrect source diagnostics.

If mismatch is detected, diagnostics cache must be rebuilt or execution rejected in checked mode.

---

## 12. Cache Failure Policy

Cache compatibility failure MUST result in one of:

```text
discard and rebuild
fallback to interpreter/lower tier
VmStructuralError if no safe fallback exists
```

Cache compatibility failure MUST NOT be converted to ordinary language Error.

---

## 13. Public Artifact Rule

The VM MUST NOT promise users that cached RuntimePlan/EIR/JIT artifacts are stable portable files.

A package manager or build system MUST treat such caches as internal, rebuildable artifacts.

---

## 14. Validation

Cache validation MUST reject:

```text
RuntimePlan cache with mismatched SIR digest
EIR cache with mismatched RuntimePlan digest
JIT cache with mismatched ValueLayoutProfile
JIT cache with mismatched helper registry digest
GC metadata cache with mismatched RootMap schema
module interface cache with incompatible export shape
capability-sensitive cache without digest/epoch policy
cache file claiming public bytecode status
cache crossing VM version without compatibility marker
```

---

## 15. Audit Tracking

This document completes:

```text
R14
```

It addresses:

```text
M-11
```

It supports:

```text
R15
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-CACHE-COMPATIBILITY-MATRIX.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-CALL-EXECUTION-PROTOCOL.md -->


# Phase 3 · Call Execution Protocol

Document class: Normative specification  
Normative status: This document defines the canonical call execution protocol for Phase 3 VM specifications.

Created: 2026-06-29 09:31:28

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

This document repairs second-stage audit item:

```text
S2-R1: Add canonical call execution protocol.
```

It addresses residual finding:

```text
S2-M01: Call execution protocol still deserves canonical extraction.
```

All Phase 3 documents that define or use function calls, builtin calls, host calls, constructor calls, default arguments, parameter binding, call-site feedback, or return contracts MUST conform to this protocol.

---

## 1. Call Boundary

The call protocol applies to:

```text
user function calls
builtin function calls
record constructor calls
enum case constructor calls
bound method calls
host function wrapper calls
future FFI calls if enabled
```

The protocol is internal VM semantics.

It is not:

```text
native ABI
public function ABI
CPython C API
Python wheel compatibility
public bytecode call convention
```

---

## 2. Canonical Call Inputs

A call site is represented by:

```text
CallFrameInput {
  callee: Value
  positional_args: List<Value>
  named_args: List<NamedArgumentValue>
  call_site_id: CallSiteId
  source_span: SourceSpanId
  expected_result_type?: TypeId
}
```

Argument values MUST already be evaluated according to source order before entering the actual callee frame.

---

## 3. Evaluation Order

Call expression evaluation order is canonical:

```text
1. evaluate callee
2. evaluate positional arguments left-to-right
3. evaluate named arguments left-to-right
4. perform callability check
5. resolve callable target
6. bind arguments
7. evaluate required defaults at call time
8. check parameter contracts
9. enter frame or helper boundary
10. execute body or builtin/host call
11. check return contract
12. return, raise, or propagate control
```

If any step raises, later steps MUST NOT execute.

---

## 4. Callable Categories

```text
CallableTarget =
  | UserFunction
  | BuiltinFunction
  | RecordConstructor
  | EnumCaseConstructor
  | BoundMethod
  | HostFunctionWrapper
```

Calling a non-callable value raises:

```text
TypeError
```

---

## 5. User Function Call

### 5.1 FunctionObj

```text
FunctionObj {
  function_id: FunctionId
  module_id: ModuleId
  entry_eir_function: EirFunctionId
  parameter_layout: ParameterLayout
  capture_layout: CaptureLayout
  default_argument_plan: DefaultArgumentPlan
  return_type?: TypeId
  effect?: EffectId
  required_capabilities: List<CapabilityId>
  source_span?: SourceSpanId
}
```

### 5.2 Frame Creation

A user function call MUST create a logical VM frame.

The frame MUST contain:

```text
function identity
module identity
slot layout
parameter slots
local slots
capture slots/cells
region stack
source span
frame map
pending control slot
```

The logical VM call stack is independent of host language call stack.

---

## 6. Argument Binding

### 6.1 Positional Arguments

Positional arguments bind to positional parameters in declaration order.

Too many positional arguments raise:

```text
ArityError
```

### 6.2 Named Arguments

Named arguments bind by parameter name.

Duplicate named arguments raise:

```text
ArityError
```

Unknown named arguments raise:

```text
ArityError
```

A parameter receiving both positional and named values raises:

```text
ArityError
```

### 6.3 Missing Arguments

A missing required argument raises:

```text
ArityError
```

A missing optional argument with default MUST evaluate its default at call time.

---

## 7. Default Argument Evaluation

Default expressions are evaluated:

```text
at call time
only when the argument is omitted
in parameter declaration order
within the caller-visible evaluation context defined by lowering
with source spans preserved
```

Default evaluation may raise.

If default evaluation raises, the function body MUST NOT start.

Default expressions MUST NOT be precomputed at function declaration time unless the language later explicitly marks them as constant defaults.

---

## 8. Parameter Contract Checks

After argument/default binding and before body execution:

```text
parameter type contracts MUST be checked
```

Failure raises:

```text
TypeContractError
```

Parameter contract checks MUST preserve source span of the parameter or call site.

---

## 9. Captures and Closures

Closure captures MUST preserve BindingId identity.

Capture storage may be:

```text
Value capture
Cell capture
Module slot capture
Runtime internal capture
```

Mutable captures MUST use cell-like storage.

JIT/interpreter optimizations MUST NOT break observable closure semantics.

---

## 10. Function Body Execution

The function body executes as EIR entry function from the FunctionPlan.

Return, raise, break, and continue are interpreted through the unified control-state model.

A Break/Continue escaping a valid target is invalid and MUST be rejected by validation or converted to a source-level error according to validation boundary.

---

## 11. Return Contract

Before a function returns to its caller:

```text
return type contract MUST be checked if present
```

Failure raises:

```text
TypeContractError
```

If cleanup is active, cleanup runs before final frame exit, but return contract check MUST still happen before exposing the returned value to caller.

---

## 12. Builtin Function Call

BuiltinFunction descriptors MUST define:

```text
builtin_id
arity
parameter contract policy
return contract policy
required_capabilities
effect
may_allocate
may_raise
may_unwind
source mapping policy
```

Builtin calls MAY execute through:

```text
helper_call_builtin
direct VM builtin dispatch
JIT builtin fast path
```

but all paths MUST preserve descriptor semantics.

Effectful builtins MUST be capability-gated.

---

## 13. Constructor Calls

### 13.1 Record Constructor

Record constructor calls MUST enforce:

```text
fixed shape
known fields only
required fields
duplicate fields rejected
field type contracts
field mutability initialization rules
```

Failure raises:

```text
ArityError
FieldError
TypeContractError
```

according to failure kind.

### 13.2 Enum Case Constructor

Enum case constructor calls MUST enforce:

```text
closed enum
case exists
payload arity
payload contracts
```

---

## 14. Bound Method Call

A bound method call MUST preserve:

```text
receiver identity
method function identity
receiver binding position
source span
call-site feedback
```

Method binding may allocate a BoundMethod object, or may be represented internally without allocation if semantics are preserved.

---

## 15. Host Function Wrapper Call

Host calls MUST pass through the host boundary contract.

Compiled code MUST NOT directly call arbitrary host/native pointers.

Host call steps:

```text
check capability
enter host boundary
make roots visible
register host roots if needed
call host wrapper
normalize host result/error
exit host boundary
```

Host exceptions MUST be normalized.

---

## 16. Call-Site Feedback

Every source call expression lowered to EIR MUST have a CallSiteId.

Call-site feedback MAY collect:

```text
observed callable kind
function id
builtin id
receiver shape
arity shape
return type observation
miss count
exception count
```

Feedback MUST NOT change semantics.

Deterministic mode MAY disable adaptive feedback.

---

## 17. Source Mapping

A call that may raise MUST preserve:

```text
call site source span
callee source span where available
argument source spans where relevant
helper/builtin/host context where relevant
```

Stack traces SHOULD hide VM-internal helper frames unless debug mode requests them.

---

## 18. JIT Requirements

JIT call lowering MUST preserve this protocol.

Generic calls initially SHOULD call:

```text
helper_generic_call
```

Monomorphic calls MAY use guarded fast paths if they preserve:

```text
callee guard
arity check
parameter contracts
return contract
cleanup/unwind path
source mapping
deopt state
capability checks
```

---

## 19. Validation

Call validation MUST reject:

```text
CallOp without CallSiteId
call site without source span
unknown callable target kind
arity layout mismatch
default argument plan mismatch
parameter slot mismatch
return contract without TypeId resolution
host call without capability metadata
builtin call without descriptor
compiled call that skips cleanup/unwind
compiled call that bypasses helper/host boundary
```

---

## 20. Audit Tracking

This document completes:

```text
S2-R1
```

It addresses:

```text
S2-M01
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-CALL-EXECUTION-PROTOCOL.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-READONLY-VIEW-SEMANTICS.md -->


# Phase 3 · ReadOnlyView Semantics

Document class: Normative specification  
Normative status: This document defines canonical ReadOnlyView identity, equality, delegation, mutation, and optimization rules for Phase 3 VM specifications.

Created: 2026-06-29 09:31:28

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

This document repairs second-stage audit item:

```text
S2-R2: Add canonical ReadOnlyView semantics.
```

It addresses residual finding:

```text
S2-M02: ReadOnlyView semantics still need a canonical document.
```

---

## 1. ReadOnlyView Boundary

`ReadOnlyView` is a language-visible runtime value that provides shallow read-only access to a target value.

It is not:

```text
deep immutable copy
frozen object
hashability wrapper
ownership transfer
GC lifetime boundary
security sandbox by itself
```

---

## 2. Construction

```text
readonly(value) -> ReadOnlyView
```

If `value` is already a ReadOnlyView, the VM MAY return the same view or create another view.

This choice MUST NOT change observable mutation restrictions.

---

## 3. Target

```text
ReadOnlyViewObj {
  target: Value
  source_span?: SourceSpanId
}
```

The target MUST be root-visible.

If target is heap-backed, ReadOnlyView traces the target.

---

## 4. Identity

A ReadOnlyView has its own runtime identity.

Canonical rule:

```text
readonly(x) is x == false
```

unless the value is an immutable immediate where the VM explicitly defines identity as value identity.

The VM MUST NOT optimize ReadOnlyView identity in a way that makes a mutable aggregate view identical to the original mutable aggregate.

---

## 5. Equality

Equality through ReadOnlyView delegates to target equality unless the language explicitly performs identity comparison.

```text
readonly(x) == x
```

MAY be true if target equality says so.

```text
readonly(x) is x
```

MUST be false for heap-backed mutable aggregates.

---

## 6. Shallow Read-Only Semantics

ReadOnlyView is shallow.

For aggregate target:

```text
view.field read delegates to target field read
view[index] read delegates to target index read
iteration if supported delegates to target iteration
```

Mutation through the view is forbidden.

Nested mutable objects reached through a read operation are not automatically wrapped unless an operation explicitly returns a readonly view.

---

## 7. Mutation Through View

The following MUST raise:

```text
ReadOnlyError
```

when target is reached through a ReadOnlyView:

```text
field write
index write
map insert/replace
list element write
resource state mutation if exposed
any mutating method call
```

Mutating methods accessed through ReadOnlyView MUST be rejected unless the method is explicitly marked non-mutating.

---

## 8. Mutation Through Original Object

ReadOnlyView does not freeze the original object.

If original object is still reachable, mutating the original object changes what the view reads.

Example semantic rule:

```text
let x = [1]
let v = readonly(x)
x[0] = 2
v[0] == 2
```

This is allowed.

---

## 9. Hashability

ReadOnlyView does not make a target hashable.

If target is non-hashable, ReadOnlyView over that target is non-hashable.

If target is hashable immutable value, the view MAY delegate hash/equality according to ValueKey rules only if doing so cannot violate hash stability.

---

## 10. Records

For record instances:

```text
ReadOnlyView(record).field
```

delegates to field read.

```text
ReadOnlyView(record).field = value
```

raises ReadOnlyError.

Fixed-shape record field indexing remains valid under ReadOnlyView if the receiver guard accounts for view unwrap.

---

## 11. Lists and Maps

List and Map reads through ReadOnlyView are allowed.

Writes through ReadOnlyView are forbidden.

Map key hashing/equality still follows ValueKey semantics.

ReadOnlyView over Map does not make map itself hashable.

---

## 12. Functions and Modules

ReadOnlyView over Function or Module is allowed only if the operation is semantically meaningful.

Calling a ReadOnlyView over a function MUST NOT bypass callability checks.

Module mutation through view is forbidden.

---

## 13. Resources

ReadOnlyView over Resource MUST NOT allow resource state mutation.

Closing a resource through a readonly view is a mutating operation and MUST raise ReadOnlyError unless future resource policy explicitly defines close as permitted through readonly handle.

---

## 14. Helper Behavior

Helpers MUST preserve ReadOnlyView rules.

```text
helper_get_attribute
helper_index_read
helper_slice_read
```

may unwrap readonly for read operations.

```text
helper_set_attribute
helper_index_write
helper_close_resource
mutating builtin/helper paths
```

MUST reject readonly mutation.

---

## 15. JIT Requirements

JIT fast paths may unwrap ReadOnlyView for reads only if guard includes:

```text
receiver is ReadOnlyView
target shape/type
operation is non-mutating
```

JIT MUST NOT remove readonly mutation checks.

Compiled mutation through ReadOnlyView MUST raise ReadOnlyError.

---

## 16. GC Requirements

ReadOnlyView target is a root edge.

Moving GC MUST update ReadOnlyView target reference.

---

## 17. Validation

Readonly validation/runtime MUST reject:

```text
mutating operation through ReadOnlyView
mutating method through ReadOnlyView
hashing readonly view over non-hashable target
JIT mutation path missing readonly guard/check
helper mutation path missing readonly check
```

---

## 18. Audit Tracking

This document completes:

```text
S2-R2
```

It addresses:

```text
S2-M02
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-READONLY-VIEW-SEMANTICS.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-HOST-BOUNDARY-CONTRACT.md -->


# Phase 3 · Host Boundary Contract

Document class: Normative specification  
Normative status: This document defines the canonical host boundary, host function wrapper, host object wrapper, host root registry, host error normalization, capability gating, and deferred FFI boundary for Phase 3 VM specifications.

Created: 2026-06-29 09:31:28

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

This document repairs second-stage audit item:

```text
S2-R3: Add canonical host boundary contract.
```

It addresses residual finding:

```text
S2-M03: Host boundary / FFI deferred status should be sharpened.
```

---

## 1. Host Boundary

The host boundary is the VM-controlled interface through which VM execution may interact with host-provided services.

Examples:

```text
module resolver
builtin host functions
fs/net/process/env/random/clock providers
test runner hooks
debug/profiling hooks
future FFI gateway
```

The host boundary is not a public native extension ABI.

---

## 2. Host Boundary Non-Goals

Phase 3 host boundary MUST NOT imply:

```text
CPython C API compatibility
CPython ABI compatibility
Python wheel compatibility
native plugin ABI
stable VM object layout
raw pointer ownership by host
public helper ABI
```

FFI remains DEFERRED unless explicitly enabled by a later normative document.

---

## 3. HostFunctionWrapper

```text
HostFunctionWrapper {
  host_function_id: HostFunctionId
  descriptor: HostFunctionDescriptor
  capability?: CapabilityId
  effect?: EffectId
  source_span?: SourceSpanId
}
```

### 3.1 HostFunctionDescriptor

```text
HostFunctionDescriptor {
  arity: ArityShape
  parameter_policy: HostParameterPolicy
  result_policy: HostResultPolicy
  may_allocate: Bool
  may_raise: Bool
  may_block: Bool
  may_reenter_vm: Bool
  requires_roots_visible: Bool
}
```

---

## 4. HostObjectWrapper

```text
HostObjectWrapper {
  host_object_id: HostObjectId
  descriptor: HostObjectDescriptor
  capability_origin?: CapabilityId
  lifetime: HostObjectLifetime
}
```

HostObjectWrapper may hold native host state.

It MUST NOT expose raw VM object pointers to host.

It MUST NOT let host retain VM values except through HostRootRegistry.

---

## 5. HostRootRegistry

```text
HostRootRegistry {
  roots: Map<HostRootId, HostRootEntry>
}
```

```text
HostRootEntry {
  value: Value
  owner: HostBoundaryId
  lifetime: HostRootLifetime
  capability?: CapabilityId
}
```

```text
HostRootLifetime =
  | CallScoped
  | ResourceScoped
  | ExplicitHandle
```

Host code MUST NOT retain VM values beyond a call unless a HostRootEntry exists.

---

## 6. Host Call Protocol

Host call execution order:

```text
1. resolve HostFunctionWrapper
2. check required capability
3. make VM roots visible if descriptor requires
4. register host call frame
5. marshal VM values to host boundary representation
6. call host function
7. normalize host result or error
8. unregister call-scoped host roots
9. return Value, VmControl::Raise, or VmError
```

Compiled code MUST NOT skip this protocol.

---

## 7. Capability Gating

Every effectful host operation MUST declare:

```text
CapabilityId
EffectId
```

Missing capability raises:

```text
CapabilityError
```

Capability environment mutability and cache invalidation follow Target/Profile schema and cache compatibility matrix.

---

## 8. Host Error Normalization

Host exceptions/errors MUST be normalized.

Possible outcomes:

```text
LanguageError as VmControl::Raise(ErrorHandle)
VmStructuralError as VmError
host cancellation/interruption if later defined
```

Raw host exceptions MUST NOT cross into VM interpreter/JIT as untyped host exceptions.

---

## 9. Host Reentrancy

If host function may reenter VM:

```text
may_reenter_vm = true
```

VM MUST preserve:

```text
call stack
region stack
pending control
roots
helper state
source diagnostics
```

Host reentrancy MUST NOT corrupt structured unwinding.

---

## 10. Host Blocking and Safepoints

If host function may block:

```text
may_block = true
```

The VM SHOULD treat entry/exit as safepoint-capable.

If host call may allocate/trigger GC/reenter VM, roots MUST be visible.

---

## 11. Module Resolver

Module resolver is a host boundary component.

If resolver touches filesystem, network, environment, or other effects, it MUST be capability-gated.

Module resolver failures normalize to:

```text
ImportError
CapabilityError
VmStructuralError
```

depending on failure kind.

---

## 12. Resource Ownership

Host resources exposed to VM MUST be represented by ResourceObj or HostObjectWrapper with explicit lifetime policy.

GC finalization MUST NOT be required for language resource cleanup.

Structured `use`/`defer` cleanup remains canonical.

---

## 13. FFI Deferred Boundary

FFI is DEFERRED in Phase 3 minimal VM.

Deferred FFI is constrained by:

```text
no stable VM object layout exposure
no direct ObjRef raw pointer ownership
no CPython API/ABI compatibility promise
capability-gated effects
host root registration for retained VM values
error normalization
source diagnostics where possible
```

---

## 14. JIT Requirements

JIT compiled code MUST NOT directly call arbitrary host/native pointers.

Host calls from JIT MUST go through:

```text
helper_enter_host_call
host call trampoline
helper_exit_host_call
```

or equivalent VM-controlled boundary preserving this contract.

---

## 15. Validation

Host boundary validation MUST reject:

```text
effectful host call without capability metadata
host function descriptor missing arity policy
host call retaining VM value without HostRoot
host object exposing raw VM pointer
JIT direct host pointer call
host error path without normalization
module resolver effect without capability
FFI feature use without explicit enablement
```

---

## 16. Audit Tracking

This document completes:

```text
S2-R3
```

It addresses:

```text
S2-M03
```

It partially supports:

```text
M-13
M-05
M-11
```


<!-- END NORMATIVE DOCUMENT: PHASE-3-HOST-BOUNDARY-CONTRACT.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-VM-FRAMEWORK.md -->


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


<!-- END NORMATIVE DOCUMENT: PHASE-3-VM-FRAMEWORK.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-VM-RUNTIME-ROUND1.md -->


# Phase 3 · VM Runtime Semantics · Round 1
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.2 runtime draft  
Depends on: Phase 3 VM Framework v0.1  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Depends on: Phase 1 Language Specification v1.0 frozen baseline  
Scope: runtime value model, heap/handle model, environment model, frame model, region stack model, evaluator contracts, runtime error model  
Out of scope: full module resolver, full standard library, FFI implementation, native ABI, NIR/EIR schema, JIT, moving GC

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



## 0. Round 1 Scope

This document fills the first concrete VM layer.

Round 1 defines the runtime substrate required before node-by-node SIR execution can be specified:

1. Rust representation strategy
2. runtime value model
3. heap and handle model
4. object identity model
5. aggregate object model
6. record and enum runtime model
7. function and closure model
8. module object model
9. binding and environment model
10. frame and call stack model
11. region stack model
12. control result model
13. runtime error model
14. read-only view model
15. resource handle model
16. evaluator function contracts
17. validation/execution boundary
18. implementation invariants

Round 1 does not yet define complete evaluation semantics for every SIR node. That belongs to later VM execution rounds.

---

## 1. Rust Representation Strategy

### 1.1 Core Principle

The VM should use explicit Rust enums, typed IDs, typed handles, and table-indexed storage.

The VM must not use raw pointers as language-level identity.

The VM must not expose Rust memory layout as ABI.

### 1.2 Internal IDs

Use compact newtype IDs internally.

Conceptual shape:

```rust
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BindingId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScopeId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TypeId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ObjectId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FrameId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RegionId(u32);
```

String-form IDs from SIR serialization should be decoded into typed IDs before execution.

### 1.3 Result Discipline

All VM operations that can fail return explicit results.

Conceptual shape:

```rust
type VmResult<T> = Result<T, VmError>;
```

Rust panics must not be used for ordinary language-level failures.

Language failures include:

```text
TypeError
NameError
IndexError
KeyError
ImportError
AssertionError
PatternMatchError
ReadOnlyError
```

These are VM errors or language `Error` values, not Rust panics.

### 1.4 Unsafe Policy

Round 1 assumes an unsafe-free core VM.

If future implementation introduces `unsafe`, it must not be required for:

```text
ordinary value evaluation
ordinary binding lookup
ordinary module execution
ordinary control flow
ordinary error propagation
```

Unsafe code, if any, must be isolated behind documented invariants.

---

## 2. Runtime Value Model

### 2.1 Value Enum

The VM must represent all frozen Phase 1 runtime value kinds.

Conceptual Rust shape:

```rust
pub enum Value {
    Nil,
    Bool(bool),
    Int(IntValue),
    Float(FloatValue),
    String(ObjRef<StringObj>),
    List(ObjRef<ListObj>),
    Map(ObjRef<MapObj>),
    Range(RangeValue),
    RecordType(ObjRef<RecordTypeObj>),
    RecordInstance(ObjRef<RecordInstanceObj>),
    EnumType(ObjRef<EnumTypeObj>),
    EnumValue(ObjRef<EnumValueObj>),
    ReadOnlyView(ObjRef<ReadOnlyViewObj>),
    Function(ObjRef<FunctionObj>),
    BuiltinFunction(BuiltinFunctionId),
    Module(ObjRef<ModuleObj>),
    Error(ObjRef<ErrorObj>),
    Resource(ObjRef<ResourceObj>),
}
```

`Resource` is a VM-level value kind used to model host resources and `use` semantics. It is not a general user-constructible Phase 1 literal.

### 2.2 Immediate vs Heap Values

Immediate values:

```text
Nil
Bool
Int
Float
Range
BuiltinFunctionId
```

Heap-backed values:

```text
String
List
Map
RecordType
RecordInstance
EnumType
EnumValue
ReadOnlyView
Function
Module
Error
Resource
```

An implementation may choose different physical storage, but language-visible semantics must match this split.

### 2.3 IntValue

Phase 1 requires integers not to silently overflow.

The first VM may choose one of:

```text
checked fixed-width integer
arbitrary precision integer
hybrid small/big integer
```

For semantic simplicity, the recommended initial strategy is arbitrary precision or checked `i64` with explicit `NumericOverflowError`.

If checked `i64` is used, integer literals or operations outside range must raise `NumericOverflowError`.

The choice must be documented in the VM target profile.

### 2.4 FloatValue

Float values are IEEE-754 binary64.

The VM must preserve Phase 1 float semantics.

NaN and infinity serialization remains governed by Phase 1 serialization boundary rules.

### 2.5 String Value

Strings are immutable Unicode scalar sequences.

String indexing is not a core operation.

String slicing uses Unicode scalar positions according to Phase 1.

The VM may internally store UTF-8, but slicing semantics must not split invalid scalar boundaries.

### 2.6 Value Equality

The VM must distinguish:

```text
equality: ==
identity: is
```

Equality uses Phase 1 value equality.

Identity uses VM object identity for heap values and canonical identity for immediate singletons where defined.

`nil`, `true`, and `false` have stable singleton identity.

Numeric value identity must not be used as equality substitute.

### 2.7 Value Display

Display conversion is explicit and used by:

```text
print
format strings
debug(value)
```

The VM must not implement implicit string coercion for `+`.

---

## 3. Heap and Handle Model

### 3.1 Heap Purpose

The heap owns runtime objects that outlive a single expression evaluation.

The heap supports:

```text
object allocation
object lookup
object mutation
object identity
read-only view enforcement
future GC replacement
```

### 3.2 Object Handle

Conceptual handle:

```rust
pub struct ObjRef<T> {
    raw: ObjectId,
    marker: PhantomData<T>,
}
```

The handle is typed in Rust but may erase to a common object reference internally.

### 3.3 ObjectId

`ObjectId` is VM-internal.

It must not be exposed as language ABI.

It may be implemented as:

```text
arena index
generational arena key
Rc allocation identity wrapper
Arc allocation identity wrapper
custom heap handle
```

Recommended first implementation:

```text
generational arena or typed arena with stable indices
```

Reason:

```text
prevents accidental stale references
keeps object identity explicit
does not require moving GC
keeps future GC transition possible
```

### 3.4 Heap Object Header

Every heap object should conceptually contain:

```text
object_id
object_kind
mutable_flag
readonly_origin?
debug_origin?
```

The actual Rust layout may differ.

### 3.5 Mutation Protocol

Mutable objects must be mutated through the heap API.

The evaluator should not directly mutate arbitrary shared structures.

Conceptual API:

```rust
heap.get(obj_ref) -> VmResult<&Obj>
heap.get_mut(obj_ref) -> VmResult<&mut Obj>
heap.set_field(record_ref, field_id, value) -> VmResult<()>
heap.list_set(list_ref, index, value) -> VmResult<()>
heap.map_set(map_ref, key, value) -> VmResult<()>
```

If `RefCell` or `RwLock` is used, borrow failures are VM internal bugs unless caused by a documented reentrancy rule.

### 3.6 Future GC Constraint

The first heap does not need moving GC.

However, the handle model must avoid exposing raw addresses so future moving or compacting GC remains possible.

Forbidden design:

```text
language identity = raw Rust pointer
native extension stores direct pointer to object internals
SIR/EIR cache stores raw address
module interface stores object address
```

---

## 4. Aggregate Objects

### 4.1 ListObj

```text
ListObj {
  elements: Vec<Value>
  readonly: Bool
}
```

Lists preserve order.

List mutation requires:

```text
not readonly
valid index
value accepted by optional element contract where enforced
```

### 4.2 MapObj

```text
MapObj {
  entries: OrderedMap<ValueKey, Value>
  readonly: Bool
}
```

Maps preserve insertion order.

Map keys must be hashable.

### 4.3 ValueKey

Map keys cannot be arbitrary mutable values.

A key must satisfy hashability rules.

Recommended key representation:

```text
Nil
Bool
Int
Float where allowed and canonicalized
String
EnumValue where payload is hashable
RecordInstance only if future Hashable protocol allows it
```

For Phase 3 minimal VM, restrict hashable keys to:

```text
Nil
Bool
Int
Float
String
EnumValue without mutable/hash-unstable payload
```

If this restriction is narrower than future full language intent, it must be documented as a minimal VM limitation.

### 4.4 RangeValue

```text
RangeValue {
  start: IntValue
  end: IntValue
  step: IntValue
}
```

Phase 1 uses `range` as a core builtin boundary.

A range is immutable and iterable.

---

## 5. Record Runtime Model

### 5.1 RecordTypeObj

```text
RecordTypeObj {
  record_id: RecordId
  name: SymbolId
  fields: Vec<RecordFieldRuntimeDesc>
  methods: MethodTable
  interface_id?: InterfaceId
}
```

### 5.2 RecordFieldRuntimeDesc

```text
RecordFieldRuntimeDesc {
  field_id: FieldId
  name: SymbolId
  index: FieldIndex
  mutability: FieldMutability
  type_contract?: TypeId
  has_default: Bool
}
```

### 5.3 RecordInstanceObj

```text
RecordInstanceObj {
  record_type: ObjRef<RecordTypeObj>
  fields: Vec<Value>
  readonly: Bool
}
```

Fields are stored by field index.

Field access should resolve to `FieldId` during validation or pre-execution preparation.

Dynamic field addition is not allowed.

### 5.4 Record Construction

Construction requires:

```text
known record type
known field names
all required fields initialized
defaults evaluated at construction time
field contracts checked
```

Unknown field names raise `FieldError`.

Duplicate initializers are validation errors when statically known and runtime errors otherwise.

### 5.5 Record Identity

A record type is nominal.

Two records with identical field shape are not the same type unless they share the same `RecordId` and module identity.

---

## 6. Enum Runtime Model

### 6.1 EnumTypeObj

```text
EnumTypeObj {
  enum_id: EnumId
  name: SymbolId
  cases: Vec<EnumCaseRuntimeDesc>
  interface_id?: InterfaceId
}
```

### 6.2 EnumCaseRuntimeDesc

```text
EnumCaseRuntimeDesc {
  case_id: CaseId
  name: SymbolId
  payload_fields: Vec<EnumPayloadRuntimeDesc>
}
```

### 6.3 EnumValueObj

```text
EnumValueObj {
  enum_type: ObjRef<EnumTypeObj>
  case_id: CaseId
  payload: Vec<Value>
}
```

### 6.4 Enum Closure

Enums are closed.

The VM must not permit cases to be added dynamically.

Pattern matching may rely on enum closure.

Adding enum cases is a module-interface breaking change under Phase 2.

---

## 7. Function and Closure Runtime Model

### 7.1 FunctionObj

```text
FunctionObj {
  function_id: FunctionId
  name?: SymbolId
  parameters: Vec<ParameterRuntimeDesc>
  return_type?: TypeId
  body: NodeId
  module: ModuleId
  lexical_scope: ScopeId
  captures: CaptureEnv
  effects: Vec<EffectId>
  required_capabilities: Vec<CapabilityId>
}
```

### 7.2 ParameterRuntimeDesc

```text
ParameterRuntimeDesc {
  binding_id: BindingId
  name: SymbolId
  type_contract?: TypeId
  default_value?: NodeId
}
```

### 7.3 CaptureEnv

```text
CaptureEnv {
  entries: Vec<CaptureEntry>
}
```

```text
CaptureEntry {
  binding_id: BindingId
  cell: BindingCellRef
  capture_kind: CaptureKind
}
```

Captured variables are represented as binding cells, not copied values, when write capture is possible.

Immutable captured values may be optimized later but must preserve semantics.

### 7.4 Function Creation

A function declaration creates a function object when execution reaches the declaration.

Functions are not hoisted.

Default argument expressions are stored as SIR nodes and evaluated at call time when omitted.

### 7.5 Call Semantics

A call creates a new frame.

Steps:

1. evaluate callee
2. evaluate arguments left to right
3. resolve positional and named arguments
4. evaluate omitted defaults at call time
5. check parameter contracts
6. create frame and local binding cells
7. execute function body
8. handle return or implicit nil
9. check return contract
10. pop frame

### 7.6 BuiltinFunction

A builtin function is a VM-hosted callable with explicit signature and capability metadata.

Conceptual descriptor:

```text
BuiltinFunctionDesc {
  id: BuiltinFunctionId
  name: SymbolId
  arity: Arity
  effects: Vec<EffectId>
  required_capabilities: Vec<CapabilityId>
  implementation: BuiltinImpl
}
```

Builtins must return `VmControl` or equivalent so they can raise language errors.

---

## 8. Module Runtime Model

### 8.1 ModuleObj

```text
ModuleObj {
  module_id: ModuleId
  name: QualifiedName
  state: ModuleState
  scope: EnvRef
  exports: ExportRuntimeTable
  interface: ModuleInterfaceDescriptor
  initialization_error?: ErrorHandle
}
```

### 8.2 ModuleState

The VM implements Phase 2 module states:

```text
Unloaded
Loading
Initializing
Initialized
Failed
```

### 8.3 ExportRuntimeTable

```text
ExportRuntimeTable {
  entries: OrderedMap<SymbolId, BindingCellRef>
  sealed: Bool
}
```

Exports reference binding cells.

They do not copy values.

### 8.4 Module Initialization

Top-level execution initializes module bindings in source order.

The export table is sealed after successful initialization.

Access to an uninitialized export during circular import raises `ImportCycleError`.

---

## 9. Binding and Environment Model

### 9.1 BindingCell

A binding cell stores runtime binding state.

```text
BindingCell {
  binding_id: BindingId
  state: BindingState
  mutability: BindingMutability
  type_contract?: TypeId
}
```

```text
BindingState =
  | Uninitialized
  | Initialized(Value)
```

### 9.2 Environment

```text
Environment {
  scope_id: ScopeId
  parent?: EnvRef
  cells: Map<BindingId, BindingCellRef>
}
```

The VM uses resolved `BindingId`, not textual lookup, for ordinary variable access.

Textual lookup may exist only for diagnostics or host/tooling.

### 9.3 Binding Initialization

Declaration execution initializes a binding cell.

Reading an uninitialized binding raises `NameError` or `UninitializedBindingError`.

Writing to immutable binding raises `TypeError` or a more specific assignment error.

### 9.4 Scope Chain

The runtime environment chain must match the Phase 2 scope graph.

Function calls create function environments.

Blocks create block environments where required by Phase 1 block scope.

Loop iteration variables are scoped to the loop body.

Pattern bindings are scoped to match case or destructuring context.

### 9.5 Environment Optimization

The VM may optimize local lookup with arrays indexed by precomputed slots.

Such optimization must preserve `BindingId` semantics.

---

## 10. Frame and Call Stack Model

### 10.1 Frame

```text
Frame {
  frame_id: FrameId
  function?: ObjRef<FunctionObj>
  module: ObjRef<ModuleObj>
  env: EnvRef
  region_stack: RegionStack
  call_span?: SourceSpanId
  return_type?: TypeId
}
```

### 10.2 Call Stack

```text
CallStack {
  frames: Vec<Frame>
}
```

The call stack is used for:

```text
function calls
return targeting
runtime diagnostics
stack traces
structured unwinding
debugging later
```

### 10.3 Top-Level Frame

Module initialization executes in a top-level module frame.

Top-level frame is not a function frame.

`return`, `break`, and `continue` are invalid at top level.

### 10.4 Stack Overflow

The VM must detect excessive recursion or stack growth.

If the host stack is used recursively, the VM should still maintain its own logical call stack for diagnostics.

A future implementation may use trampoline or explicit stack evaluation.

---

## 11. Region Stack Model

### 11.1 RegionFrame

```text
RegionFrame {
  region_id: ControlRegionId
  kind: ControlRegionKind
  scope?: EnvRef
  defers: Vec<DeferredCallable>
  resources: Vec<ResourceCleanup>
  finally_block?: NodeId
  loop_target?: LoopTarget
}
```

### 11.2 RegionStack

```text
RegionStack {
  regions: Vec<RegionFrame>
}
```

### 11.3 DeferredCallable

```text
DeferredCallable {
  callable: Value
  registered_at: SourceSpanId?
}
```

The callable must be zero-argument callable at execution time.

### 11.4 ResourceCleanup

```text
ResourceCleanup {
  resource: Value
  close_method: SymbolId
  acquired_at: SourceSpanId?
  closed: Bool
}
```

### 11.5 Region Stack Use

The region stack implements:

```text
return target lookup
break/continue target lookup
defer LIFO execution
use close ordering
finally execution
suppressed error attachment
```

### 11.6 Region Invariant

Every region pushed at runtime must correspond to a valid Phase 2 `ControlRegionDescriptor`.

Synthetic implementation regions may exist internally but must not alter language semantics.

---

## 12. Control Result Model

### 12.1 VmControl

Evaluator functions return control results.

Conceptual shape:

```rust
pub enum VmControl {
    Normal(Value),
    Return(Value),
    Break { target: ControlRegionId },
    Continue { target: ControlRegionId },
    Raise(ErrorHandle),
}
```

### 12.2 Statement Normal Value

Statements that complete normally return `Value::Nil` unless they are expression statements whose value is intentionally preserved by a host mode.

Ordinary language semantics do not expose statement values.

### 12.3 Control Propagation

Control propagation follows Phase 2 structured unwinding.

Evaluator functions must not collapse `Return`, `Break`, `Continue`, or `Raise` into ordinary values.

### 12.4 Rust Panic Boundary

Rust panic indicates VM implementation bug or unrecoverable host failure.

It is not language `raise`.

---

## 13. Runtime Error Model

### 13.1 ErrorObj

```text
ErrorObj {
  code: ErrorCode
  message: String
  details: Map<String, Value>
  source_span?: SourceSpanId
  stack_trace?: StackTrace
  suppressed: Vec<ErrorHandle>
}
```

### 13.2 ErrorCode

Required core error codes:

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
```

### 13.3 Primary and Suppressed Errors

The VM must represent primary and suppressed errors.

If a cleanup operation raises while another error is pending, the cleanup error is suppressed unless Phase 2 finally override semantics say otherwise.

### 13.4 Error Raising

Language `raise` requires an `Error` value.

If a program attempts to raise a non-error value, the VM raises `TypeError`.

### 13.5 Diagnostic Conversion

Runtime errors can be converted into diagnostics.

The error object is semantic; diagnostic rendering is presentation.

---

## 14. Read-Only View Model

### 14.1 ReadOnlyViewObj

```text
ReadOnlyViewObj {
  target: Value
}
```

A read-only view is shallow.

It prevents mutation through the view.

It does not recursively freeze the target value.

### 14.2 Mutation Through View

If a mutation target is a read-only view, the VM raises `ReadOnlyError`.

Examples:

```text
readonly(list)[0] = x
readonly(record).field = x
readonly(map)[k] = v
```

### 14.3 Identity

A read-only view is its own object.

`readonly(x) is x` is false unless a future optimization explicitly preserves identity in an observationally equivalent way and Phase 1 permits it.

### 14.4 Read Access

Read access through a read-only view delegates to the target.

---

## 15. Resource Runtime Model

### 15.1 ResourceObj

```text
ResourceObj {
  resource_id: ResourceId
  state: ResourceState
  close_callable: Value
  capability_origin?: CapabilityId
}
```

### 15.2 ResourceState

```text
ResourceState =
  | Open
  | Closing
  | Closed
  | Failed
```

### 15.3 Close Rule

A resource acquired by `use` must be closed exactly once if acquisition succeeds.

Closing an already closed resource is either:

```text
idempotent success
or ResourceStateError
```

The chosen policy must be documented by the resource implementation.

### 15.4 Phase 3 Minimal Policy

The VM core only defines resource protocol.

Concrete resources are host-provided.

No filesystem/network/process resource implementation is required in Round 1.

---

## 16. Evaluator Contracts

### 16.1 Core Evaluator Functions

Conceptual API:

```rust
eval_node(vm: &mut VM, node_id: NodeId, ctx: &mut ExecutionContext) -> VmResult<VmControl>

eval_expr(vm: &mut VM, node_id: NodeId, ctx: &mut ExecutionContext) -> VmResult<Value>

exec_stmt(vm: &mut VM, node_id: NodeId, ctx: &mut ExecutionContext) -> VmResult<VmControl>

call_value(vm: &mut VM, callee: Value, args: CallArgs, ctx: &mut ExecutionContext) -> VmResult<VmControl>
```

### 16.2 Expression Contract

`eval_expr` must return a `Value` or a VM error.

If expression evaluation raises a language error, the evaluator may return either:

```text
VmResult::Ok(VmControl::Raise(error))
```

through a unified control path, or

```text
VmResult::Err(VmError::Language(error))
```

The implementation must choose one consistent convention.

Recommended convention:

```text
language raise is VmControl::Raise
VM structural failure is VmResult::Err
```

### 16.3 Statement Contract

`exec_stmt` returns `VmControl`.

Statement execution may produce:

```text
Normal
Return
Break
Continue
Raise
```

### 16.4 Node Dispatch

Node dispatch should be explicit by SIR node kind.

The VM must reject unknown required node kinds before execution.

### 16.5 Type Contract Checks

Type contract checks are runtime semantics.

Evaluator must check:

```text
let/const declared type
function parameter type
function return type
record field type
enum payload type
list/map contracts where enforced
explicit TypeCheckNode
```

### 16.6 Capability Checks

Evaluator or host boundary must check required capabilities before performing effectful host operation.

Missing capability raises `CapabilityError`.

### 16.7 Source Mapping

Evaluator should preserve current source span for diagnostics.

---

## 17. Validation and Execution Boundary

### 17.1 Required Boundary

The VM must validate SIR before execution.

The minimal VM may initially support a subset, but it must declare that subset.

Executing unvalidated full SIR is invalid.

### 17.2 Prevalidated Runtime Plan

The VM may build a runtime plan after validation.

A runtime plan may include:

```text
node kind cache
binding slot layout
field index layout
enum case index layout
type check cache
region ownership map
module import plan
```

The runtime plan is implementation-private.

It is not public bytecode.

### 17.3 Rejection Rule

If the VM cannot validate or execute required SIR semantics, it must reject the program with diagnostic rather than reinterpret semantics.

---

## 18. Internal Invariants

The VM implementation must preserve:

```text
all BindingId references resolve to binding cells
all ScopeId references map to environment structure or static scope data
all TypeId references map to runtime type descriptors
all ControlRegionId references map to runtime or static region descriptors
all PatternId references map to pattern descriptors
all heap handles are valid or rejected as stale internal errors
all exported bindings are module-scope cells
all read-only view writes are rejected
all capability-gated host operations check capability
```

Violation of these invariants is an implementation bug, not user program behavior.

---

## 19. Minimal Implementation Order

Recommended implementation order:

```text
1. sir crate data model
2. runtime Value and heap handles
3. diagnostics and ErrorObj
4. binding cells and environments
5. frames and call stack
6. region stack
7. literal/binding evaluator
8. declaration evaluator
9. expression evaluator
10. function call evaluator
11. aggregate evaluator
12. control-flow evaluator
13. unwinding evaluator
14. module evaluator
15. capability stubs
```

This order keeps runtime substrate ahead of language node coverage.

---

## 20. Round 1 Completion Criteria

Round 1 is complete when the VM specification defines:

```text
Value representation
heap/handle model
object identity model
aggregate objects
record runtime model
enum runtime model
function/closure model
module object model
binding/environment model
frame/call stack model
region stack model
control result model
runtime error model
read-only view model
resource model
evaluator contracts
validation/execution boundary
internal invariants
```

This document satisfies those criteria at the specification level.

---

## 21. Next Work

Round 2 should define node-by-node SIR execution for:

```text
literals
binding references
let/const
assignment
unary/binary/logical expressions
calls
functions
record construction/access
enum construction
list/map/range operations
format strings
type checks
```

Round 3 should define:

```text
blocks
if
while
for
match
try/catch/finally
raise
return
break
continue
use
defer
assert
test
module initialization

```


---

## 22. Performance Architecture Constraints

Round 1 runtime structures are semantic models unless explicitly declared physical.

The following constraints are mandatory:

```text
Value enum is conceptual, not ABI.
ObjectId/ObjRef identity is VM-internal.
Heap handle model must not block moving or tracing GC.
Rc/RefCell may be used only as bootstrap detail.
Hot-path execution must not rely on textual lookup.
BindingId must lower to slots before hot execution.
FieldId must lower to field indices before hot execution.
CaseId must lower to case indices before hot execution.
Region stack must be mappable to safepoint/deopt metadata.
Frame layout must support root enumeration.
Mutation APIs must have future write-barrier insertion points.
```

The runtime substrate must support future:

```text
RuntimePlan
EIR fast interpreter
inline caches
type feedback
shape feedback
baseline JIT
deoptimization
GC safepoints
root maps
frame maps
```

This document's Rust-like types are conceptual unless explicitly designated implementation API.


<!-- END NORMATIVE DOCUMENT: PHASE-3-VM-RUNTIME-ROUND1.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-PERFORMANCE-ARCHITECTURE.md -->


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


<!-- END NORMATIVE DOCUMENT: PHASE-3-PERFORMANCE-ARCHITECTURE.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-RUNTIMEPLAN-EIR-FRAMEWORK.md -->


# Phase 3 · RuntimePlan and EIR Framework
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.4 framework draft  
Depends on: Phase 3 Performance and JIT Architecture v0.3  
Depends on: Phase 3 VM Runtime Semantics Round 1 v0.2  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: RuntimePlan schema framework, EIR identity, EIR function/block/operation model, slot layout, call/access site tables, safepoint/root map/deopt metadata framework, fast interpreter boundary  
Out of scope: full EIR instruction semantics, complete node lowering rules, concrete JIT backend implementation, concrete GC implementation, public bytecode

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

This document defines the bridge between frozen SIR semantics and high-performance VM execution.

The VM execution pipeline is:

```text
SIR
  -> validation
  -> RuntimePlan
  -> EIR
  -> fast interpreter
  -> baseline JIT
  -> optimizing JIT
```

RuntimePlan and EIR are internal VM structures.

They are not public bytecode.

They are not package ABI.

They are not external compiler targets.

They may be cached, but caches are discardable and invalidated by VM/runtime compatibility checks.

---

## 1. Design Position

### 1.1 Why RuntimePlan Exists

SIR is semantic and table-rich.

Directly executing SIR would repeatedly consult:

```text
SymbolTable
ScopeTable
BindingTable
TypeTable
Record descriptors
Enum descriptors
PatternTable
ControlRegionTable
ModuleInterfaceDescriptor
```

That is correct for validation and diagnostics, but unsuitable as a hot execution path.

RuntimePlan converts validated SIR into execution-ready metadata.

### 1.2 Why EIR Exists

EIR is the compact internal execution form.

It exists so that the VM can have:

```text
fast interpreter
quickening
inline caches
type feedback
shape feedback
safepoints
deopt points
baseline JIT input
optimizing JIT input
```

without making executable IR public.

### 1.3 Core Rule

RuntimePlan and EIR may optimize representation.

They must not alter semantics.

The source of semantic authority remains:

```text
Phase 1 source semantics
Phase 2 SIR semantics
```

---

## 2. RuntimePlan Overview

### 2.1 RuntimePlan

```text
RuntimePlan {
  plan_version: Version
  source_sir_digest: Digest
  target_profile: RuntimeTargetProfile
  module_plans: List<ModulePlan>
  function_plans: List<FunctionPlan>
  type_plan: TypePlan
  shape_plan: ShapePlan
  call_site_table: CallSiteTable
  access_site_table: AccessSiteTable
  safepoint_seed_table: SafepointSeedTable
  deopt_seed_table: DeoptSeedTable
  helper_table: RuntimeHelperTable
  capability_gate_plan: CapabilityGatePlan
  diagnostics?: DiagnosticTable
}
```

### 2.2 RuntimeTargetProfile

```text
RuntimeTargetProfile {
  vm_version: Version
  architecture?: TargetArchitecture
  pointer_width?: UInt
  value_layout_profile: ValueLayoutProfile
  heap_profile: HeapProfile
  interpreter_profile: InterpreterProfile
  jit_profile?: JitProfile
  gc_profile?: GcProfile
}
```

The target profile records VM execution assumptions.

It is not public ABI.

### 2.3 RuntimePlan Digest

RuntimePlan cache keys must include:

```text
SIR semantic digest
SIR interface digest
Phase 2 schema version
VM version
RuntimePlan version
target profile
feature set
capability environment digest
dependency interface digests
stdlib interface digest
```

If any input changes incompatibly, RuntimePlan must be regenerated.

---

## 3. ModulePlan

### 3.1 ModulePlan

```text
ModulePlan {
  module_id: ModuleId
  module_name: QualifiedName
  module_scope: ScopeId
  module_slots: SlotLayout
  initialization_function: EirFunctionId
  import_plan: ImportPlan
  export_plan: ExportPlan
  module_state_slot: ModuleStateSlot
  source_map: SourceMapRef
}
```

### 3.2 ImportPlan

```text
ImportPlan {
  imports: List<ImportPlanEntry>
}
```

```text
ImportPlanEntry {
  import_node: NodeId
  import_descriptor: ImportDescriptor
  required_interface_digest?: Digest
  target_module_name: QualifiedName
  local_slot?: SlotId
  import_kind: ImportKind
  source_span?: SourceSpanId
}
```

```text
ImportKind =
  | WholeModule
  | NamedValue
```

### 3.3 ExportPlan

```text
ExportPlan {
  exports: List<ExportPlanEntry>
  sealed_after_init: Bool
}
```

```text
ExportPlanEntry {
  exported_name: SymbolId
  binding_id: BindingId
  source_slot: SlotId
  stable_interface_id: InterfaceId
}
```

Exports reference slots/cells.

They do not copy values.

### 3.4 Module Initialization Function

A module body is lowered to an internal initialization function.

This function is not source-visible.

It owns top-level execution order.

It must preserve:

```text
top-level declaration order
import execution order
export binding visibility
module state transitions
circular import checks
capability gates
```

---

## 4. FunctionPlan

### 4.1 FunctionPlan

```text
FunctionPlan {
  function_id: FunctionId
  name?: SymbolId
  source_function_node?: NodeId
  module_id: ModuleId
  parameter_layout: ParameterLayout
  local_layout: SlotLayout
  capture_layout: CaptureLayout
  return_type?: TypeId
  entry_eir_function: EirFunctionId
  call_site_range: CallSiteRange
  access_site_range: AccessSiteRange
  safepoint_range: SafepointRange
  deopt_range: DeoptRange
  source_map: SourceMapRef
}
```

### 4.2 ParameterLayout

```text
ParameterLayout {
  parameters: List<ParameterSlot>
}
```

```text
ParameterSlot {
  binding_id: BindingId
  name: SymbolId
  slot_id: SlotId
  type_contract?: TypeId
  default_node?: NodeId
  default_eir?: EirBlockId
}
```

Default argument expressions are evaluated at call time.

They must not be moved to function creation.

### 4.3 CaptureLayout

```text
CaptureLayout {
  captures: List<CaptureSlot>
}
```

```text
CaptureSlot {
  binding_id: BindingId
  capture_index: UInt
  capture_kind: CaptureKind
  cell_slot: SlotId
}
```

Captured mutable bindings must use cells.

Captured immutable values may later be optimized, but semantics must remain cell-compatible.

---

## 5. Slot Layout

### 5.1 SlotId

```text
SlotId
```

Encoded internally as compact integer.

SlotId is VM-internal.

### 5.2 SlotLayout

```text
SlotLayout {
  slots: List<SlotDescriptor>
  temporary_count: UInt
  max_live_slots: UInt
}
```

### 5.3 SlotDescriptor

```text
SlotDescriptor {
  slot_id: SlotId
  slot_kind: SlotKind
  binding_id?: BindingId
  type_hint?: TypeId
  mutability?: BindingMutability
  storage: SlotStorage
  source_span?: SourceSpanId
}
```

### 5.4 SlotKind

```text
SlotKind =
  | Local
  | Parameter
  | Capture
  | Module
  | Temporary
  | Builtin
  | HiddenRuntime
```

### 5.5 SlotStorage

```text
SlotStorage =
  | Value
  | Cell
  | Constant
  | TypeDescriptor
  | ModuleRef
  | RuntimeInternal
```

### 5.6 Slot Rules

Hot-path local access must use slots.

SIR `BindingId` lookup is resolved during RuntimePlan generation.

Execution must not perform ordinary variable lookup by string.

### 5.7 Slot Cell Rule

A slot stores a cell when:

```text
binding is captured mutably
binding may be observed by closure after frame creation
binding is exported by module
binding is shared across module import/export boundary
```

Otherwise, a slot may store direct `Value`.

---

## 6. TypePlan and ShapePlan

### 6.1 TypePlan

```text
TypePlan {
  runtime_types: List<RuntimeTypeDesc>
}
```

```text
RuntimeTypeDesc {
  type_id: TypeId
  type_kind: RuntimeTypeKind
  check_strategy: TypeCheckStrategy
}
```

### 6.2 TypeCheckStrategy

```text
TypeCheckStrategy =
  | AlwaysAccept
  | NeverAccept
  | ImmediateKindCheck
  | ShapeCheck
  | UnionCheck
  | OptionalCheck
  | FunctionSignatureCheck
  | ExtensionCheck
```

### 6.3 ShapePlan

```text
ShapePlan {
  record_shapes: List<RecordShape>
  enum_shapes: List<EnumShape>
}
```

### 6.4 RecordShape

```text
RecordShape {
  shape_id: ShapeId
  record_id: RecordId
  type_id: TypeId
  field_count: UInt
  fields: List<RecordFieldShape>
  method_table: MethodTableRef
}
```

```text
RecordFieldShape {
  field_id: FieldId
  field_index: FieldIndex
  name: SymbolId
  mutability: FieldMutability
  type_contract?: TypeId
}
```

### 6.5 EnumShape

```text
EnumShape {
  shape_id: ShapeId
  enum_id: EnumId
  type_id: TypeId
  cases: List<EnumCaseShape>
}
```

```text
EnumCaseShape {
  case_id: CaseId
  case_index: CaseIndex
  payload_layout: PayloadLayout
}
```

### 6.6 Shape Rule

Record and enum access must lower from symbolic IDs to shape/index access.

```text
FieldId -> FieldIndex
CaseId -> CaseIndex
```

String lookup is not the normal execution path.

---

## 7. Call Site Table

### 7.1 CallSiteTable

```text
CallSiteTable {
  sites: List<CallSiteRecord>
}
```

### 7.2 CallSiteRecord

```text
CallSiteRecord {
  call_site_id: CallSiteId
  node_id: NodeId
  enclosing_function: FunctionId
  argument_layout: ArgumentLayout
  expected_callee_kind?: CallableKindSet
  inline_cache_slot: InlineCacheSlotId
  type_feedback_slot: FeedbackSlotId
  source_span?: SourceSpanId
}
```

### 7.3 ArgumentLayout

```text
ArgumentLayout {
  positional_count: UInt
  named_arguments: List<NamedArgumentLayout>
  has_defaults_possible: Bool
}
```

```text
NamedArgumentLayout {
  name: SymbolId
  argument_index: UInt
}
```

### 7.4 CallableKindSet

```text
CallableKindSet {
  user_function: Bool
  builtin_function: Bool
  record_constructor: Bool
  enum_case_constructor: Bool
  bound_method: Bool
  host_function: Bool
}
```

### 7.5 Call Site Requirement

Every call expression in EIR must reference a CallSiteId.

Even if the first implementation does not use inline caches, the identity must exist.

---

## 8. Access Site Table

### 8.1 AccessSiteTable

```text
AccessSiteTable {
  sites: List<AccessSiteRecord>
}
```

### 8.2 AccessSiteRecord

```text
AccessSiteRecord {
  access_site_id: AccessSiteId
  node_id: NodeId
  enclosing_function: FunctionId
  access_kind: AccessKind
  expected_receiver_shape?: ShapeId
  resolved_field_index?: FieldIndex
  inline_cache_slot: InlineCacheSlotId
  feedback_slot: FeedbackSlotId
  source_span?: SourceSpanId
}
```

### 8.3 AccessKind

```text
AccessKind =
  | AttributeRead
  | AttributeWrite
  | MethodRead
  | IndexRead
  | IndexWrite
  | Slice
```

### 8.4 Access Site Requirement

Attribute, method, index, and slice operations that can become hot should have AccessSiteId.

Record field access should resolve to field indices when statically known.

Generic access may remain, but it must be cacheable.

---

## 9. Inline Cache Model

### 9.1 InlineCacheSlot

```text
InlineCacheSlot {
  slot_id: InlineCacheSlotId
  state: InlineCacheState
  payload: InlineCachePayload
}
```

### 9.2 InlineCacheState

```text
InlineCacheState =
  | Uninitialized
  | Monomorphic
  | Polymorphic
  | Megamorphic
  | Disabled
```

### 9.3 InlineCachePayload

```text
InlineCachePayload =
  | Empty
  | CallCache
  | AttributeCache
  | IndexCache
  | TypeCheckCache
```

### 9.4 Cache Rule

Inline cache state is optimization state.

It must be guarded.

It must be invalidated or bypassed when assumptions fail.

It must not change observable semantics.

---

## 10. Feedback Tables

### 10.1 FeedbackTable

```text
FeedbackTable {
  slots: List<FeedbackSlot>
}
```

### 10.2 FeedbackSlot

```text
FeedbackSlot {
  feedback_slot_id: FeedbackSlotId
  kind: FeedbackKind
  state: FeedbackState
}
```

### 10.3 FeedbackKind

```text
FeedbackKind =
  | TypeFeedback
  | ShapeFeedback
  | CallTargetFeedback
  | BranchFeedback
  | AllocationFeedback
  | ErrorFeedback
```

### 10.4 FeedbackState

```text
FeedbackState =
  | Empty
  | Stable
  | Polymorphic
  | Megamorphic
  | Unstable
```

### 10.5 Feedback Rule

Feedback is speculative.

Any optimization derived from feedback must have fallback or deopt path.

---

## 11. EIR Overview

### 11.1 EirModule

```text
EirModule {
  eir_version: Version
  source_runtime_plan_digest: Digest
  functions: List<EirFunction>
  constants: ConstantPool
  source_map: EirSourceMap
  safepoints: SafepointTable
  deopt_points: DeoptPointTable
}
```

### 11.2 EirFunction

```text
EirFunction {
  eir_function_id: EirFunctionId
  function_id?: FunctionId
  name?: SymbolId
  slot_layout: SlotLayout
  blocks: List<EirBlock>
  entry_block: EirBlockId
  call_site_range: CallSiteRange
  access_site_range: AccessSiteRange
  source_span?: SourceSpanId
}
```

### 11.3 EirBlock

```text
EirBlock {
  block_id: EirBlockId
  parameters: List<EirBlockParam>
  operations: List<EirOp>
  terminator: EirTerminator
  source_span?: SourceSpanId
}
```

### 11.4 EirBlockParam

```text
EirBlockParam {
  slot_id: SlotId
  type_hint?: TypeId
}
```

### 11.5 EIR Structure Rule

EIR is block-structured.

A block contains operations and ends with exactly one terminator.

Operations produce values into slots.

Terminators control flow.

---

## 12. EIR Operation Families

### 12.1 EirOp

Round 0 EIR defines operation families, not every final opcode.

```text
EirOp =
  | LoadOp
  | StoreOp
  | ConstantOp
  | UnaryOp
  | BinaryOp
  | LogicalOp
  | CheckOp
  | CallOp
  | AccessOp
  | ConstructOp
  | PatternOp
  | RuntimeHelperOp
  | SafepointOp
  | GuardOp
  | DebugOp
```

### 12.2 LoadOp

Load operations read from:

```text
local slot
capture slot
module slot
constant pool
field index
enum payload index
```

### 12.3 StoreOp

Store operations write to:

```text
local slot
capture cell
module cell
record field
list index
map entry
temporary slot
```

Store operations must pass through write-barrier hooks where heap mutation may occur.

### 12.4 CheckOp

Check operations include:

```text
type check
Bool condition check
callable check
hashable check
readonly check
capability check
arity check
overflow check
division by zero check
```

### 12.5 CallOp

Call operations reference:

```text
CallSiteId
callee slot
argument slots
result slot
fallback helper
```

### 12.6 AccessOp

Access operations reference:

```text
AccessSiteId
receiver slot
field index or key slot
result slot
fallback helper
```

### 12.7 ConstructOp

Construct operations include:

```text
list construction
map construction
record construction
enum value construction
function object construction
error construction
```

### 12.8 PatternOp

Pattern operations include:

```text
literal match
record shape match
field pattern load
enum case match
payload pattern load
list length match
map key match
or-pattern dispatch
binding write
```

### 12.9 RuntimeHelperOp

Runtime helper operations call VM helper functions.

They must declare:

```text
may allocate
may raise
is safepoint
required capability
```

### 12.10 GuardOp

Guards verify speculative assumptions.

On failure, they branch to fallback or deopt.

---

## 13. EIR Terminators

### 13.1 EirTerminator

```text
EirTerminator =
  | Jump
  | Branch
  | Return
  | Raise
  | LoopBackedge
  | Switch
  | Unwind
  | Unreachable
```

### 13.2 Jump

Unconditional block transition.

### 13.3 Branch

Bool-based branch.

Must not use truthiness.

### 13.4 Return

Returns a value from the function.

Must preserve return contract checking.

### 13.5 Raise

Raises an error value.

Must preserve language error semantics.

### 13.6 LoopBackedge

Loop transition and safepoint candidate.

May update hotness counters.

### 13.7 Switch

Used for enum cases, pattern decisions, or dense dispatch.

### 13.8 Unwind

Transfers control into structured unwinding logic.

May target runtime helper if lowering remains high-level.

---

## 14. Safepoint Framework

### 14.1 SafepointTable

```text
SafepointTable {
  safepoints: List<SafepointRecord>
}
```

### 14.2 SafepointRecord

```text
SafepointRecord {
  safepoint_id: SafepointId
  eir_function_id: EirFunctionId
  eir_location: EirLocation
  source_span?: SourceSpanId
  live_slots: List<SlotId>
  live_objects: List<RootRef>
  frame_state: FrameStateRef
  safepoint_kind: SafepointKind
}
```

### 14.3 SafepointKind

```text
SafepointKind =
  | FunctionCall
  | LoopBackedge
  | Allocation
  | HostCall
  | RaiseBoundary
  | ImportBoundary
  | HelperCall
  | JitSideExit
```

### 14.4 RootRef

```text
RootRef =
  | SlotRoot { slot_id: SlotId }
  | CaptureRoot { slot_id: SlotId }
  | ModuleRoot { module_id: ModuleId }
  | RegionRoot { region_id: ControlRegionId }
  | PendingControlRoot
  | ErrorRoot
  | HostRoot
```

### 14.5 Safepoint Rule

Every allocation, loop backedge, host call, and function call should have or be able to synthesize a safepoint.

---

## 15. Deopt Metadata Framework

### 15.1 DeoptPointTable

```text
DeoptPointTable {
  deopt_points: List<DeoptPointRecord>
}
```

### 15.2 DeoptPointRecord

```text
DeoptPointRecord {
  deopt_id: DeoptId
  eir_function_id: EirFunctionId
  eir_location: EirLocation
  source_span?: SourceSpanId
  frame_map: FrameMap
  local_slot_map: LocalSlotMap
  value_stack_map: ValueStackMap
  region_stack_state: RegionStackState
  pending_control_state?: PendingControlState
}
```

### 15.3 FrameMap

```text
FrameMap {
  function_id?: FunctionId
  module_id: ModuleId
  call_site_id?: CallSiteId
  visible_slots: List<VisibleSlot>
}
```

### 15.4 VisibleSlot

```text
VisibleSlot {
  binding_id?: BindingId
  slot_id: SlotId
  value_kind_hint?: RuntimeValueKind
}
```

### 15.5 RegionStackState

```text
RegionStackState {
  active_regions: List<ControlRegionId>
  defer_state_refs: List<DeferStateRef>
  resource_state_refs: List<ResourceStateRef>
  finally_state_refs: List<FinallyStateRef>
}
```

### 15.6 Deopt Rule

Any EIR operation or JIT code that depends on speculative type, shape, call-target, module-state, readonly, or arithmetic assumptions must be able to fall back or deopt.

---

## 16. Runtime Helper Table

### 16.1 RuntimeHelperTable

```text
RuntimeHelperTable {
  helpers: List<RuntimeHelperDescriptor>
}
```

### 16.2 RuntimeHelperDescriptor

```text
RuntimeHelperDescriptor {
  helper_id: RuntimeHelperId
  name: String
  signature: RuntimeHelperSignature
  may_allocate: Bool
  may_raise: Bool
  is_safepoint: Bool
  required_capability?: CapabilityId
  effect?: EffectId
}
```

### 16.3 Helper Families

Required helper families:

```text
generic call
method bind
record construction
enum construction
generic field access
generic index access
map lookup
string slicing
type contract check
pattern fallback
raise construction
unwinding
defer execution
resource close
module import
capability check
allocation
write barrier
```

### 16.4 Helper Rule

Runtime helpers are internal.

They do not define public native ABI.

JIT code may call helpers only through VM-controlled helper table.

---

## 17. Fast Interpreter Framework

### 17.1 Interpreter State

```text
FastInterpreterState {
  vm
  eir_module
  current_function
  current_block
  instruction_pointer
  slots
  frame
  region_stack
  feedback_table
}
```

### 17.2 Dispatch

The interpreter dispatches over EIR operations.

It must preserve:

```text
left-to-right evaluation
slot semantics
control terminator semantics
structured unwinding
safepoint behavior
runtime helper behavior
diagnostic source mapping
```

### 17.3 Quickening

The interpreter may quicken:

```text
generic call
generic field access
generic type check
generic binary op
generic pattern op
```

Quickened ops must retain fallback or deopt metadata.

### 17.4 Interpreter and JIT Sharing

The fast interpreter and baseline JIT consume the same RuntimePlan and EIR.

They must not diverge semantically.

---

## 18. RuntimePlan Generation Validation

RuntimePlan generation must check:

```text
all BindingId values have slots
all captured bindings have capture layout
all module exports have module slots or cells
all FieldId values map to field indices
all CaseId values map to case indices
all TypeId values map to runtime type descriptors
all ControlRegionId values map to region plans
all PatternId values map to pattern plans
all call nodes have CallSiteId
all access nodes have AccessSiteId where applicable
all safepoint candidates have seed entries
all source spans are preserved where available
```

Failure is a VM planning diagnostic.

It must not be ignored.

---

## 19. EIR Validation

Before fast interpretation or JIT, EIR must validate:

```text
every block has one terminator
all slot reads dominate or are initialized before use
all block targets exist
all helper IDs exist
all call site IDs exist
all access site IDs exist
all safepoint IDs exist where referenced
all deopt IDs exist where referenced
all type checks reference valid type descriptors
all shape guards reference valid shapes
all write operations have barrier hook path
all raise/unwind paths preserve control semantics
```

---

## 20. Compatibility and Cache Rules

RuntimePlan and EIR are VM-internal.

They may change across VM versions.

Cache invalidation must occur on:

```text
SIR digest change
Phase 2 schema incompatibility
VM version change
RuntimePlan version change
EIR version change
target profile change
dependency interface digest change
capability environment change
stdlib interface change
helper table incompatibility
GC/value layout profile change
JIT backend profile change
```

No RuntimePlan or EIR cache may be treated as package ABI.

---

## 21. Non-Goals

This document does not define:

```text
public bytecode
public EIR files
external compiler target
native plugin ABI
concrete Cranelift lowering
concrete LLVM lowering
complete GC
complete JIT
complete opcode list
complete node lowering rules
debugger protocol
profiler format
```

---

## 22. Next Work

Next documents should define:

```text
EIR operation semantics round 1
SIR-to-RuntimePlan lowering rules
SIR-to-EIR lowering rules
fast interpreter execution semantics
GC root enumeration details
baseline JIT backend interface
runtime helper function contracts

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-RUNTIMEPLAN-EIR-FRAMEWORK.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md -->


# Phase 3 · EIR Operation Semantics · Round 1
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.5 operation draft  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 3 Performance and JIT Architecture v0.3  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: EIR operation semantics, terminator semantics, fast interpreter execution loop, operation validation rules  
Out of scope: full SIR-to-EIR lowering rules, concrete JIT backend lowering, concrete GC implementation, final opcode encoding, public bytecode

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



## 0. Round 1 Scope

This document defines the first concrete semantics for EIR operations.

It covers:

1. EIR execution model
2. slot read/write rules
3. constant operations
4. load operations
5. store operations
6. unary operations
7. binary operations
8. logical operations
9. check operations
10. call operations
11. access operations
12. construction operations
13. pattern operations
14. runtime helper operations
15. safepoint operations
16. guard operations
17. terminator semantics
18. fast interpreter loop
19. EIR validation additions

EIR remains internal VM execution IR.

EIR is not public bytecode.

---

## 1. EIR Execution Model

### 1.1 EIR Function Execution

An EIR function executes inside a VM frame.

Execution state:

```text
EirExecutionState {
  current_function: EirFunctionId
  current_block: EirBlockId
  instruction_index: UInt
  slots: SlotArray
  region_stack: RegionStack
  pending_control?: VmControl
  feedback_table: FeedbackTable
}
```

### 1.2 Operation Rule

Each `EirOp` performs one of:

```text
read slots
write slots
check condition
call helper
update feedback
register safepoint
perform guarded specialization
```

An operation may complete with:

```text
OpResult =
  | Continue
  | Raise(ErrorHandle)
  | Deopt(DeoptId)
  | InternalError(VmError)
```

Ordinary control flow is handled by terminators.

### 1.3 Terminator Rule

Each `EirBlock` ends with exactly one terminator.

A terminator selects the next control state:

```text
TerminatorResult =
  | NextBlock(EirBlockId)
  | Return(Value)
  | Raise(ErrorHandle)
  | Unwind(VmControl)
  | Deopt(DeoptId)
  | Halt
```

### 1.4 No Fallthrough

EIR blocks do not implicitly fall through.

Every block must terminate explicitly.

---

## 2. Slot Semantics

### 2.1 SlotArray

```text
SlotArray {
  values: List<SlotState>
}
```

### 2.2 SlotState

```text
SlotState =
  | Uninitialized
  | Value(Value)
  | Cell(BindingCellRef)
  | RuntimeInternal(RuntimeValue)
```

### 2.3 Slot Read

Reading an uninitialized slot is invalid unless the operation explicitly permits it.

User-visible uninitialized binding reads raise:

```text
UninitializedBindingError
```

Internal uninitialized temporary reads are VM validation errors or InternalVMError.

### 2.4 Slot Write

Writing a slot must respect:

```text
slot kind
mutability
cell/value storage mode
type contract where attached
write barrier where heap mutation occurs
```

### 2.5 Cell Slot

If a slot stores a cell, ordinary binding read loads the cell's current value.

Writing to an immutable cell raises assignment error.

Writing to a mutable cell updates the cell and triggers write barrier hook if required.

### 2.6 Slot Validation

EIR validation must reject:

```text
read before initialized where statically provable
write to constant slot
write to readonly slot
slot kind mismatch
slot index out of function slot layout
```

---

## 3. Constant Operations

### 3.1 ConstantPool

```text
ConstantPool {
  constants: List<ConstantValue>
}
```

### 3.2 ConstantValue

```text
ConstantValue =
  | Nil
  | Bool
  | Int
  | Float
  | String
  | Symbol
  | TypeRef
  | FunctionRef
  | RecordTypeRef
  | EnumTypeRef
  | ModuleRef
```

### 3.3 ConstantOp

```text
ConstantOp {
  dest: SlotId
  constant_id: ConstantId
}
```

### 3.4 Constant Semantics

Execution loads the constant into `dest`.

Heap-backed constants may be:

```text
interned
allocated at module initialization
loaded from constant pool handle
```

Observable semantics must be identical.

### 3.5 String Constants

String constants are immutable.

The VM may intern strings.

String interning must not change equality semantics.

If identity semantics expose string object identity, interning policy must be documented and stable within a VM run.

---

## 4. Load Operations

### 4.1 LoadOp

```text
LoadOp =
  | LoadSlot
  | LoadCell
  | LoadCapture
  | LoadModuleSlot
  | LoadField
  | LoadEnumPayload
  | LoadConstant
```

### 4.2 LoadSlot

```text
LoadSlot {
  dest: SlotId
  source: SlotId
}
```

Reads value from source and writes to dest.

If source is a cell, this operation copies the cell reference unless explicitly designated as value load.

### 4.3 LoadCell

```text
LoadCell {
  dest: SlotId
  cell_slot: SlotId
}
```

Reads the current value from a cell.

If cell is uninitialized, raises `UninitializedBindingError`.

### 4.4 LoadCapture

```text
LoadCapture {
  dest: SlotId
  capture_index: UInt
}
```

Loads a captured binding value.

Mutable captures read from capture cell.

### 4.5 LoadModuleSlot

```text
LoadModuleSlot {
  dest: SlotId
  module_id: ModuleId
  slot: SlotId
}
```

Loads a module binding slot.

If module is not initialized and the slot is not initialized, raises `ImportCycleError` or `UninitializedBindingError` depending on context.

### 4.6 LoadField

```text
LoadField {
  dest: SlotId
  receiver: SlotId
  expected_shape: ShapeId
  field_index: FieldIndex
  access_site_id?: AccessSiteId
}
```

Semantics:

1. read receiver
2. require record instance or read-only view over record instance
3. check shape
4. load field by index
5. write dest

If shape check fails, use fallback helper or raise `FieldError`.

### 4.7 LoadEnumPayload

```text
LoadEnumPayload {
  dest: SlotId
  enum_value: SlotId
  expected_shape: ShapeId
  expected_case: CaseIndex
  payload_index: PayloadIndex
}
```

Semantics:

1. read enum value
2. check enum shape
3. check case index
4. load payload index
5. write dest

Failure raises `PatternMatchError` or uses pattern fallback depending on context.

---

## 5. Store Operations

### 5.1 StoreOp

```text
StoreOp =
  | StoreSlot
  | StoreCell
  | StoreModuleSlot
  | StoreField
  | StoreListIndex
  | StoreMapEntry
```

### 5.2 StoreSlot

```text
StoreSlot {
  dest: SlotId
  value: SlotId
}
```

Copies value from source slot to destination slot.

Destination must be writable.

### 5.3 StoreCell

```text
StoreCell {
  cell_slot: SlotId
  value: SlotId
}
```

Writes into binding cell.

Checks:

```text
cell initialized rules
mutability
type contract
write barrier
```

### 5.4 StoreModuleSlot

```text
StoreModuleSlot {
  module_id: ModuleId
  slot: SlotId
  value: SlotId
}
```

Used for top-level declarations and module initialization.

Exported module slots must remain cells if external imports can observe them.

### 5.5 StoreField

```text
StoreField {
  receiver: SlotId
  expected_shape: ShapeId
  field_index: FieldIndex
  value: SlotId
  access_site_id?: AccessSiteId
}
```

Semantics:

1. read receiver
2. reject read-only view
3. require record instance
4. check shape
5. check field mutability
6. check field contract if present
7. write field
8. execute write barrier hook

### 5.6 StoreListIndex

```text
StoreListIndex {
  receiver: SlotId
  index: SlotId
  value: SlotId
  access_site_id?: AccessSiteId
}
```

Semantics:

1. require list or read-only view over list
2. reject read-only view
3. require Int index
4. check index bounds
5. write element
6. execute write barrier hook

### 5.7 StoreMapEntry

```text
StoreMapEntry {
  receiver: SlotId
  key: SlotId
  value: SlotId
  access_site_id?: AccessSiteId
}
```

Semantics:

1. require map or read-only view over map
2. reject read-only view
3. require hashable key
4. insert or replace entry
5. preserve map insertion order
6. execute write barrier hook

---

## 6. Unary Operations

### 6.1 UnaryOp

```text
UnaryOp {
  dest: SlotId
  operator: UnaryOperator
  operand: SlotId
}
```

### 6.2 UnaryOperator

```text
UnaryOperator =
  | Plus
  | Minus
  | Not
```

### 6.3 Unary Plus

Allowed for numeric values.

For Int, returns same Int.

For Float, returns same Float.

Other types raise `TypeError`.

### 6.4 Unary Minus

Allowed for numeric values.

Int negation must check overflow if fixed-width integer representation is used.

Float negation follows binary64 behavior.

Other types raise `TypeError`.

### 6.5 Not

`not` requires Bool.

No truthiness.

Other types raise `TypeError`.

---

## 7. Binary Operations

### 7.1 BinaryOp

```text
BinaryOp {
  dest: SlotId
  operator: BinaryOperator
  left: SlotId
  right: SlotId
  feedback_slot?: FeedbackSlotId
}
```

### 7.2 BinaryOperator

```text
BinaryOperator =
  | Add
  | Subtract
  | Multiply
  | Divide
  | Modulo
  | Equal
  | NotEqual
  | Less
  | LessEqual
  | Greater
  | GreaterEqual
  | Identity
  | NotIdentity
  | Membership
```

### 7.3 Numeric Operators

Numeric operators require valid numeric operands.

No implicit string/number coercion.

Int operations must not silently overflow.

Division by zero raises `DivisionByZeroError`.

### 7.4 Add

Allowed categories:

```text
Int + Int
Float + Float
String + String
List + List where language semantics define concatenation
```

If mixed numeric promotion is not defined by Phase 1, mixed `Int + Float` must be rejected unless explicitly lowered by a future amendment.

### 7.5 Comparisons

Comparison operators return Bool.

Operands must be comparable under language rules.

Unsupported comparisons raise `TypeError`.

### 7.6 Equality

`Equal` and `NotEqual` use language equality.

They do not imply identity.

### 7.7 Identity

`Identity` and `NotIdentity` compare language identity.

For heap objects, identity is VM object identity.

For singleton immediates, identity follows singleton semantics.

### 7.8 Membership

Membership uses language membership semantics.

Required initial categories:

```text
value in List
key in Map
```

String membership is not defined unless Phase 1 or standard library later defines it.

---

## 8. Logical Operations

### 8.1 LogicalOp

Logical operators may appear as EIR operations only when short-circuit structure is preserved.

```text
LogicalOp =
  | LogicalAnd
  | LogicalOr
```

### 8.2 Bool Rule

Logical operators require Bool operands.

No truthiness.

### 8.3 Short-Circuit Rule

The preferred lowering of logical expressions is control-flow lowering:

```text
evaluate left
check Bool
branch
evaluate right only if needed
check Bool
write Bool result
```

A single LogicalOp is valid only if it does not evaluate both operands eagerly.

### 8.4 Result Rule

Logical operators return Bool.

They do not return arbitrary operand values.

---

## 9. Check Operations

### 9.1 CheckOp

```text
CheckOp =
  | CheckBool
  | CheckType
  | CheckCallable
  | CheckArity
  | CheckHashable
  | CheckReadonly
  | CheckCapability
  | CheckShape
  | CheckOverflow
  | CheckDivisionByZero
```

### 9.2 CheckBool

Requires value is Bool.

Used for:

```text
if
while
logical operators
match guards
catch guards
assert
```

Failure raises `TypeError`.

### 9.3 CheckType

```text
CheckType {
  value: SlotId
  type_id: TypeId
  failure_code: ErrorCode
}
```

Used for type contracts.

Failure raises `TypeContractError` unless a more specific failure code is supplied.

### 9.4 CheckCallable

Requires callable value.

Callable kinds:

```text
Function
BuiltinFunction
RecordConstructor
EnumCaseConstructor
BoundMethod
HostFunction
```

Failure raises `TypeError`.

### 9.5 CheckArity

Checks positional/named/default argument layout.

Failure raises `ArityError`.

### 9.6 CheckHashable

Checks map key legality.

Failure raises `TypeError`.

### 9.7 CheckReadonly

Fails if mutation target is read-only view.

Failure raises `ReadOnlyError`.

### 9.8 CheckCapability

Requires capability in current capability environment.

Failure raises `CapabilityError`.

### 9.9 CheckShape

Checks record or enum shape.

Failure may:

```text
fall back to generic helper
raise FieldError
raise PatternMatchError
deopt
```

depending on operation context.

### 9.10 Arithmetic Checks

Overflow and division-by-zero checks must precede committing arithmetic results.

---

## 10. Call Operations

### 10.1 CallOp

```text
CallOp {
  dest: SlotId
  call_site_id: CallSiteId
  callee: SlotId
  arguments: List<SlotId>
  named_argument_layout?: ArgumentLayout
  fallback_helper: RuntimeHelperId
}
```

### 10.2 Call Semantics

Execution:

1. read callee
2. read arguments in already-lowered left-to-right order
3. consult call-site cache if enabled
4. verify callable kind
5. check arity
6. evaluate omitted defaults if required
7. bind parameters
8. call target
9. write result or propagate control

### 10.3 Function Calls

User function call:

```text
push frame
bind parameters
execute target EIR function
handle return
check return contract
pop frame
```

### 10.4 Builtin Calls

Builtin calls go through VM builtin table.

Builtins may:

```text
allocate
raise
require capability
be safepoints
```

### 10.5 Constructor Calls

Record constructor calls must:

```text
allocate record instance
evaluate defaults at construction time
check field contracts
initialize fields by FieldIndex
```

Enum case constructor calls must:

```text
allocate enum value
check payload contracts
store payload by payload index
```

### 10.6 Method Calls

Method calls must preserve receiver binding.

A method call may be lowered to:

```text
load method
bind receiver
call function
```

or to a specialized method-call operation.

Semantics must match Phase 1.

### 10.7 Call Cache Rule

A call cache may specialize on:

```text
callee identity
callee kind
function id
arity
argument layout
receiver shape
```

All specializations require guard/fallback.

---

## 11. Access Operations

### 11.1 AccessOp

```text
AccessOp =
  | AttributeRead
  | AttributeWrite
  | MethodRead
  | IndexRead
  | IndexWrite
  | SliceRead
```

### 11.2 AttributeRead

```text
AttributeRead {
  dest: SlotId
  receiver: SlotId
  access_site_id: AccessSiteId
  field_index?: FieldIndex
  fallback_helper: RuntimeHelperId
}
```

If field index is available, use shape-checked field load.

Otherwise call fallback helper.

### 11.3 AttributeWrite

Attribute writes lower to `StoreField` or fallback helper.

Must reject dynamic undeclared fields on records.

### 11.4 MethodRead

Loads or constructs a bound method.

Method lookup should use method table and shape/type descriptor, not string dictionary lookup.

### 11.5 IndexRead

Required categories:

```text
List[Int]
Map[Hashable]
```

String indexing is not core.

Invalid index raises `IndexError` or `KeyError`.

### 11.6 IndexWrite

Required categories:

```text
List[Int] existing index
Map[Hashable] insert or replace
```

Read-only targets raise `ReadOnlyError`.

### 11.7 SliceRead

Required categories:

```text
List[Int?:Int?]
String[Int?:Int?]
```

Slice is half-open.

Negative bounds are errors under current Phase 2 semantics.

Out-of-range behavior follows Phase 2: error, not clamp.

---

## 12. Construction Operations

### 12.1 ConstructOp

```text
ConstructOp =
  | ConstructList
  | ConstructMap
  | ConstructRecord
  | ConstructEnumValue
  | ConstructFunction
  | ConstructError
```

### 12.2 ConstructList

```text
ConstructList {
  dest: SlotId
  elements: List<SlotId>
}
```

Allocates list and stores elements in order.

### 12.3 ConstructMap

```text
ConstructMap {
  dest: SlotId
  entries: List<MapEntrySlots>
}
```

```text
MapEntrySlots {
  key: SlotId
  value: SlotId
}
```

Checks hashable keys.

Duplicate keys replace values while preserving first insertion position.

### 12.4 ConstructRecord

```text
ConstructRecord {
  dest: SlotId
  shape_id: ShapeId
  field_values: List<FieldValueSlot>
}
```

```text
FieldValueSlot {
  field_index: FieldIndex
  value: SlotId
}
```

Checks all field contracts.

Allocates record instance with fixed shape.

### 12.5 ConstructEnumValue

```text
ConstructEnumValue {
  dest: SlotId
  shape_id: ShapeId
  case_index: CaseIndex
  payload_values: List<PayloadValueSlot>
}
```

Checks payload contracts.

Allocates enum value.

### 12.6 ConstructFunction

```text
ConstructFunction {
  dest: SlotId
  function_id: FunctionId
  capture_slots: List<SlotId>
}
```

Creates function object when execution reaches declaration.

Functions are not hoisted.

### 12.7 ConstructError

Creates language Error object.

Used by runtime errors, assertions, and explicit error construction where lowered.

---

## 13. Pattern Operations

### 13.1 PatternOp

```text
PatternOp =
  | PatternCheckLiteral
  | PatternCheckRecordShape
  | PatternCheckEnumCase
  | PatternCheckListLength
  | PatternCheckMapKey
  | PatternBind
  | PatternBranch
```

### 13.2 Pattern Failure

Pattern failure is not always language error.

Inside `match`, pattern failure branches to the next case.

Inside declaration destructuring, pattern failure raises `PatternMatchError`.

### 13.3 PatternBind

Writes matched value into pattern binding slot.

Pattern bindings are immutable unless Phase 1 amendment states otherwise.

### 13.4 Guard Interaction

Pattern guard is ordinary Bool check after pattern bindings are established.

Guard false continues to next case.

Guard non-Bool raises `TypeError`.

---

## 14. Runtime Helper Operations

### 14.1 RuntimeHelperOp

```text
RuntimeHelperOp {
  dest?: SlotId
  helper_id: RuntimeHelperId
  arguments: List<SlotId>
  safepoint_id?: SafepointId
}
```

### 14.2 Helper Semantics

Execution:

1. load helper descriptor
2. check capability if required
3. make roots visible if safepoint
4. call helper
5. handle returned value or control
6. write dest if value returned

### 14.3 Helper Result

Helpers may return:

```text
Value
VmControl
VmError
```

Language errors should become `VmControl::Raise`.

Structural VM failures become `VmError`.

### 14.4 Helper Constraints

Helpers must declare:

```text
may_allocate
may_raise
is_safepoint
required_capability
effect
```

The fast interpreter and JIT must respect those declarations.

---

## 15. Safepoint Operations

### 15.1 SafepointOp

```text
SafepointOp {
  safepoint_id: SafepointId
}
```

### 15.2 Safepoint Semantics

Execution reaches a VM observation point.

The VM may:

```text
poll cancellation
run GC
walk stack
record profile sample
handle interrupt
prepare deopt state
```

Phase 3 does not require all actions.

But the safepoint must expose correct root information.

### 15.3 Required Safepoints

Required candidates:

```text
function call boundary
loop backedge
allocation point
host call boundary
potential raise boundary
module import boundary
helper call if helper declares safepoint
```

---

## 16. Guard Operations

### 16.1 GuardOp

```text
GuardOp {
  guard_id: GuardId
  condition: GuardCondition
  success_next: EirLocation
  failure: GuardFailure
}
```

### 16.2 GuardCondition

```text
GuardCondition =
  | IsType
  | HasShape
  | IsCallTarget
  | HasCapability
  | ModuleStateIs
  | NotReadOnly
  | NoOverflow
  | NonZeroDivisor
```

### 16.3 GuardFailure

```text
GuardFailure =
  | Fallback(EirBlockId)
  | Helper(RuntimeHelperId)
  | Deopt(DeoptId)
  | Raise(ErrorCode)
```

### 16.4 Guard Semantics

If condition holds, continue.

If condition fails, take failure path.

Guard failure must not silently continue along optimized path.

### 16.5 Guard Feedback

Guard failures may update feedback state.

Repeated guard failures may disable quickened path.

---

## 17. Terminator Semantics

### 17.1 Jump

```text
Jump {
  target: EirBlockId
  arguments?: List<SlotId>
}
```

Transfers control to target block.

Block arguments are assigned to block parameters.

### 17.2 Branch

```text
Branch {
  condition: SlotId
  true_target: EirBlockId
  false_target: EirBlockId
}
```

Requires Bool.

No truthiness.

### 17.3 Return

```text
Return {
  value: SlotId
}
```

Returns from current function.

Must execute function-region unwinding and return contract check as required by lowering strategy.

### 17.4 Raise

```text
Raise {
  error: SlotId
}
```

Requires Error value.

If non-error, raises TypeError.

### 17.5 LoopBackedge

```text
LoopBackedge {
  target: EirBlockId
  safepoint_id?: SafepointId
  hotness_counter?: CounterId
}
```

May poll safepoint and update hotness.

Transfers control to loop target.

### 17.6 Switch

```text
Switch {
  discriminant: SlotId
  cases: List<SwitchCase>
  default_target: EirBlockId
}
```

Used for enum cases, pattern lowering, or dense branching.

### 17.7 Unwind

```text
Unwind {
  control: PendingControlRef
}
```

Transfers to structured unwinding machinery.

May be implemented by helper call in early VM.

### 17.8 Unreachable

If reached, signals InternalVMError.

Used after validation-proven impossible paths.

---

## 18. Fast Interpreter Loop

### 18.1 Conceptual Loop

```text
while true:
  block = current_function.blocks[current_block]
  while instruction_index < block.operations.len:
    op = block.operations[instruction_index]
    result = execute_op(op)
    if result == Continue:
      instruction_index += 1
      continue
    else:
      handle_nonlocal_result(result)
  terminator_result = execute_terminator(block.terminator)
  dispatch_terminator_result(terminator_result)
```

### 18.2 Interpreter Requirements

The interpreter must:

```text
use slots for locals
respect initialized state
update feedback where enabled
respect safepoints
call helpers through helper table
preserve source mapping for errors
propagate VmControl explicitly
not use Rust panic for language errors
```

### 18.3 Block Arguments

If EIR uses block parameters, jump arguments must be assigned before executing target block operations.

Assignments must be simultaneous if required to avoid clobbering.

### 18.4 Error Handling

If an operation returns `Raise`, control transfers to structured error propagation.

If an operation returns `InternalError`, execution aborts with VM diagnostic.

### 18.5 Deopt Handling

If an operation returns `Deopt`, the interpreter reconstructs lower-tier state or falls back to generic EIR path.

Early VM may reject optimized EIR containing deopt if deopt runtime is not implemented.

---

## 19. Operation Validation Additions

EIR validation must reject:

```text
operation reads unknown slot
operation writes unknown slot
operation writes non-writable slot
operation references unknown helper
operation references unknown call site
operation references unknown access site
operation references unknown safepoint
operation references unknown deopt point
operation has missing dest where result is required
operation has dest where no result allowed
type check references unknown TypeId
shape check references unknown ShapeId
field access references invalid FieldIndex
enum payload access references invalid PayloadIndex
guard failure has no valid target
terminator target block missing
branch condition not guaranteed or checked Bool
return outside function
raise operand not checked or known Error
```

---

## 20. Semantics Preservation Requirements

EIR operation semantics must preserve:

```text
left-to-right evaluation order
Bool-only conditions
no implicit coercion
checked integer overflow
division by zero errors
function defaults at call time
function declaration not hoisted
assignment target single evaluation
record fixed shape
enum closure
map insertion order
duplicate map-key replacement semantics
read-only view mutation rejection
structured unwinding
primary/suppressed error behavior
capability checks
module initialization order
source diagnostics
```

---

## 21. JIT Readiness

Every EIR operation must be classified by JIT lowering category:

```text
direct machine lowering
guarded fast path plus helper fallback
helper-only
interpreter-only until later tier
```

Round 1 does not assign all categories.

Future EIR operation semantics must include this classification.

---

## 22. Non-Goals

This document does not define:

```text
binary EIR encoding
public bytecode
concrete Cranelift lowering
concrete LLVM lowering
full GC protocol
complete deopt runtime
complete inline cache implementation
complete operation set
complete SIR-to-EIR lowering
```

---

## 23. Next Work

Next documents should define:

```text
SIR-to-RuntimePlan lowering
SIR-to-EIR lowering for expression/declaration nodes
EIR operation semantics round 2 for structured control and unwinding
fast interpreter concrete data structures
baseline JIT backend interface
runtime helper contracts

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-SIR-LOWERING-ROUND1.md -->


# Phase 3 · SIR to RuntimePlan / EIR Lowering · Round 1
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.6 lowering draft  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: SIR-to-RuntimePlan lowering rules, SIR-to-EIR lowering for declarations and expressions, slot allocation, shape/index lowering, call/access site creation, check insertion, source map preservation, lowering validation  
Out of scope: full structured-control lowering, full unwinding lowering, concrete JIT backend lowering, complete GC implementation, public bytecode

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



## 0. Round 1 Scope

This document defines the first lowering rules from frozen SIR into VM execution structures.

It covers:

1. lowering pipeline
2. lowering invariants
3. slot allocation
4. environment and binding lowering
5. type lowering
6. record shape lowering
7. enum shape lowering
8. module/function plan lowering
9. literal lowering
10. binding reference lowering
11. let/const lowering
12. assignment lowering
13. unary/binary/logical expression lowering
14. call lowering
15. access lowering
16. collection construction lowering
17. record/enum construction lowering
18. function construction lowering
19. format string lowering
20. type-check lowering
21. check insertion
22. call/access site creation
23. source map preservation
24. RuntimePlan validation
25. EIR validation
26. non-goals

This round intentionally does not lower:

```text
if
while
for
match
try/catch/finally
raise
return
break
continue
use
defer
assert
test
module import execution
structured unwinding
```

Those belong to later lowering rounds.

---

## 1. Lowering Pipeline

### 1.1 Pipeline

The lowering pipeline is:

```text
Validated SIR
  -> RuntimePlan generation
  -> EIR generation
  -> EIR validation
  -> fast interpreter / JIT-ready execution
```

### 1.2 Required Inputs

Lowering requires:

```text
IRUnit
NodeTable
BindingTable
ScopeTable
TypeTable
PatternTable
ControlRegionTable
ModuleInterfaceDescriptor
CapabilityTable
EffectTable
SourceTable
```

### 1.3 Output Artifacts

Lowering produces:

```text
RuntimePlan
EirModule
LoweringDiagnostics
```

### 1.4 Lowering Mode

Lowering modes:

```text
debug-lowering
checked-lowering
optimized-lowering
```

`debug-lowering` preserves maximal source structure and source spans.

`checked-lowering` is the default.

`optimized-lowering` may generate quickenable EIR and extra guard/deopt metadata.

All modes must preserve Phase 1 and Phase 2 semantics.

---

## 2. Lowering Invariants

Lowering must preserve:

```text
left-to-right evaluation order
binding identity
scope semantics
function default evaluation at call time
function declaration not hoisted
assignment target single evaluation
Bool-only condition semantics
no implicit coercion
checked integer overflow
division by zero errors
record fixed shape
enum closure
map insertion order
duplicate map key replacement semantics
read-only view mutation rejection
type contract checks
capability checks
source span mapping
module interface semantics
```

Lowering must not:

```text
perform textual lookup in hot paths
flatten source diagnostics away
turn SIR into public bytecode
expose EIR as package ABI
introduce CPython ABI assumptions
change error categories
change cleanup semantics
evaluate omitted defaults before call time
```

---

## 3. Slot Allocation

### 3.1 Slot Allocation Purpose

SIR uses `BindingId`.

Execution uses `SlotId`.

Lowering assigns each executable binding to a slot or cell-backed slot.

### 3.2 Slot Assignment Rule

Each runtime binding receives one of:

```text
direct value slot
cell slot
module slot
capture slot
builtin slot
```

### 3.3 Direct Value Slot

Use direct slot when:

```text
binding is local to function/block
binding is not captured mutably
binding is not exported
binding is not shared across module boundary
```

### 3.4 Cell Slot

Use cell slot when:

```text
binding is captured by closure and mutation is possible
binding is exported from module
binding may be observed by imported module during initialization
binding lifetime exceeds current frame
```

### 3.5 Module Slot

Top-level module bindings lower to module slots.

Exported module bindings must be cell-backed or otherwise observable through stable export table references.

### 3.6 Capture Slot

A capture slot references a binding cell or captured immutable value.

Mutable captures require cells.

Immutable captures may be copied only if no semantic observation is changed.

### 3.7 Temporary Slot

Expression lowering may allocate temporaries.

Temporaries:

```text
are not source-visible
must not appear in module interface descriptors
must have source-origin metadata where useful
may be reused when liveness allows
```

### 3.8 Slot Initialization

Declaration lowering must emit EIR that initializes slots at source execution points.

Functions are not hoisted.

A function slot is initialized when execution reaches the function declaration.

### 3.9 Slot Validation

Lowering must produce slot layout where:

```text
every BindingId used at runtime maps to a SlotId
every SlotId belongs to its function/module layout
every cell slot is initialized before cell read where required
every immutable binding has no invalid StoreCell/StoreSlot target
```

---

## 4. Environment and Scope Lowering

### 4.1 Scope to Slot Scope

SIR `ScopeId` maps to one or more execution environments.

For hot execution, scope lookup must be precomputed.

```text
ScopeId + BindingId -> SlotId
```

### 4.2 Block Scope

Block scope may lower to:

```text
slot range in current frame
nested environment object
debug-only scope map
```

Preferred hot path:

```text
slot range in current frame
```

### 4.3 Function Scope

Function scope lowers to a frame slot layout.

Parameters occupy parameter slots.

Locals occupy local slots.

Captures occupy capture slots or capture cells.

### 4.4 Module Scope

Module scope lowers to module slot table.

Module slot table supports:

```text
top-level declarations
imports
exports
module initialization state
circular import checks
```

### 4.5 Debug Scope

Debug tooling may reconstruct lexical scopes using source maps and slot maps.

Runtime execution must not require text-based lookup for ordinary variables.

---

## 5. Type Lowering

### 5.1 TypeDescriptor to RuntimeTypeDesc

Each SIR `TypeId` lowers to a `RuntimeTypeDesc`.

```text
TypeId -> RuntimeTypeDesc
```

### 5.2 Type Check Strategy

Lowering chooses a `TypeCheckStrategy`:

```text
Builtin -> ImmediateKindCheck
Record -> ShapeCheck
Enum -> ShapeCheck
Union -> UnionCheck
Optional -> OptionalCheck
Function -> FunctionSignatureCheck
Any -> AlwaysAccept
Never -> NeverAccept
Extension -> ExtensionCheck
```

### 5.3 Type Contract Check Insertion

Lowering must insert checks for:

```text
let/const declared type
function parameter type
function return type
record field type
enum payload type
explicit SIR TypeCheckNode
```

### 5.4 Contract Check Location

Contract checks must occur at the semantic boundary defined by SIR:

```text
binding initialization
parameter binding
return
field initialization
field assignment
enum construction
explicit check node
```

Lowering must not delay or advance checks in a way that changes error order.

---

## 6. Record Shape Lowering

### 6.1 Record Shape Creation

Each SIR record type lowers to a `RecordShape`.

```text
RecordId -> RecordShape
FieldId -> FieldIndex
```

### 6.2 Field Index Assignment

Field indices are assigned deterministically.

Recommended order:

```text
source declaration order
```

The chosen order is VM-internal but must be stable within a RuntimePlan.

### 6.3 Record Constructor Plan

A record constructor plan records:

```text
shape_id
required fields
default field expressions
field contract checks
field initializer order
```

Defaults remain evaluated at construction time.

### 6.4 Field Access Lowering

A statically resolved record field access lowers to:

```text
LoadField {
  receiver
  expected_shape
  field_index
  access_site_id
}
```

A statically resolved record field assignment lowers to:

```text
StoreField {
  receiver
  expected_shape
  field_index
  value
  access_site_id
}
```

### 6.5 Dynamic Field Rejection

Records have fixed shape.

Unknown dynamic field access must use fallback helper only to raise the correct error or support non-record receiver categories.

It must not add fields.

---

## 7. Enum Shape Lowering

### 7.1 Enum Shape Creation

Each SIR enum type lowers to an `EnumShape`.

```text
EnumId -> EnumShape
CaseId -> CaseIndex
Payload FieldId -> PayloadIndex
```

### 7.2 Enum Constructor Plan

An enum case constructor plan records:

```text
enum shape
case index
payload layout
payload contract checks
```

### 7.3 Enum Construction Lowering

Enum construction lowers to:

```text
ConstructEnumValue {
  dest
  shape_id
  case_index
  payload_values
}
```

### 7.4 Enum Pattern Lowering

Enum pattern checks lower to:

```text
check enum shape
check case index
load payload index
```

Full pattern lowering is completed in a later round.

---

## 8. ModulePlan Lowering

### 8.1 Module Body

The SIR module body lowers to a synthetic module initialization EIR function.

```text
ModuleBodyNode -> initialization_function
```

### 8.2 Top-Level Execution Order

The initialization function preserves top-level source order.

No declaration may be reordered across an import or effectful top-level expression unless proven semantically inert.

### 8.3 Import Lowering

This round creates `ImportPlanEntry` but does not fully lower import execution.

Import execution lowering is deferred to module/control lowering round.

### 8.4 Export Lowering

Each exported binding lowers to `ExportPlanEntry`.

Export entries reference module slots or module cells.

They do not copy values.

### 8.5 Module State

RuntimePlan reserves module state slot.

Module state transitions are implemented in later module execution lowering.

---

## 9. FunctionPlan Lowering

### 9.1 Function Declaration

A SIR function declaration lowers into:

```text
FunctionPlan
ConstructFunction EIR operation at declaration point
function binding slot initialization
```

Function bodies lower to separate EIR functions.

### 9.2 No Hoisting

Function declaration execution creates the function value when the declaration is reached.

Lowering must not initialize function binding at module/function entry unless source semantics prove equivalent.

### 9.3 Parameter Lowering

Each parameter binding lowers to a parameter slot.

Default expressions remain NodeId/EIR blocks evaluated at call time.

### 9.4 Return Contract

FunctionPlan records return type.

Actual return-check lowering is handled in return/control lowering round.

### 9.5 Closure Lowering

Captured bindings lower through `CaptureLayout`.

Mutable captures use cells.

Immutable captures may be optimized only after preserving closure semantics.

---

## 10. Literal Lowering

### 10.1 Nil/Bool/Int/Float

Immediate literals lower to `ConstantOp` or immediate EIR constants.

### 10.2 String

String literals lower to constant pool entries.

The VM may intern strings.

Interning must not change equality semantics.

### 10.3 Literal Source Mapping

Each literal operation should preserve source span for diagnostics, especially numeric overflow or invalid literal representation errors.

---

## 11. Binding Reference Lowering

### 11.1 Read Reference

A SIR `BindingReferenceNode` with `Read` lowers to:

```text
LoadSlot
LoadCell
LoadCapture
LoadModuleSlot
```

depending on slot storage.

### 11.2 WriteTarget Reference

A write target reference lowers to slot/cell target metadata.

The actual write is emitted by assignment lowering.

### 11.3 CallTarget Reference

A call target reference lowers to a value slot plus CallSite lowering.

### 11.4 TypeReference

Type references lower to runtime type descriptor slots or constants.

### 11.5 ModuleReference

Module references lower to module object slots.

### 11.6 Uninitialized Binding

If a binding can be read before initialization, generated EIR must preserve the uninitialized check.

---

## 12. Let/Const Lowering

### 12.1 LetDeclarationNode

Lowering:

1. lower initializer expression to value slot
2. insert type contract check if declared type exists
3. store into binding slot or cell
4. mark binding initialized

### 12.2 ConstDeclarationNode

Same as let, except destination mutability is immutable.

Const is shallow.

### 12.3 Declaration Destructuring

Declaration destructuring lowering is deferred to pattern lowering round.

This round may lower destructuring to helper call or reject unsupported lowering mode.

### 12.4 Initialization Order

Initializer is evaluated before binding becomes initialized.

If initializer raises, binding remains uninitialized.

---

## 13. Assignment Lowering

### 13.1 Binding Assignment

Lowering:

1. lower RHS value
2. resolve target BindingId to writable slot/cell
3. check mutability
4. check type contract if attached
5. store value

Target evaluation must occur once.

### 13.2 Field Assignment

Lowering:

1. lower receiver once to temporary slot
2. lower RHS value
3. emit `CheckReadonly`
4. emit shape check if statically known
5. emit field contract check if required
6. emit `StoreField`

### 13.3 Index Assignment

Lowering:

1. lower receiver once
2. lower index/key once
3. lower RHS value
4. emit readonly check
5. emit list/map-specific store or fallback helper

### 13.4 Augmented Assignment

Augmented assignment lowering must evaluate target once.

Recommended lowering:

```text
evaluate target address/reference once
load current value
evaluate RHS
perform binary op
store result to original target
```

The target address/reference must be represented explicitly in EIR or by a lowering-specific temporary descriptor.

---

## 14. Unary/Binary/Logical Lowering

### 14.1 Unary

SIR unary expressions lower to `UnaryOp`.

`not` emits Bool check or uses UnaryOp semantics that performs Bool check.

### 14.2 Binary

SIR binary expressions lower to `BinaryOp` plus required checks.

Arithmetic ops must preserve overflow/division semantics.

### 14.3 Equality and Identity

Equality lowers to `BinaryOp(Equal/NotEqual)`.

Identity lowers to `BinaryOp(Identity/NotIdentity)`.

### 14.4 Comparisons

Comparisons lower to Bool-producing operations.

Unsupported comparison paths call helper or raise `TypeError`.

### 14.5 Chained Comparisons

Chained comparisons lower to a sequence preserving single evaluation of operands.

Required structure:

```text
eval operand0 -> temp0
eval operand1 -> temp1
compare temp0 op0 temp1
if false -> result false
eval operand2 -> temp2
compare temp1 op1 temp2
...
```

Intermediate operands are evaluated exactly once.

### 14.6 Logical And/Or

Logical expressions lower to control-flow blocks.

They must short-circuit.

They must return Bool.

They must not lower to eager binary operations.

---

## 15. Call Lowering

### 15.1 Call Site Creation

Every SIR call expression creates a `CallSiteRecord`.

```text
CallExpressionNode -> CallSiteId
```

### 15.2 Evaluation Order

Call lowering preserves:

```text
callee first
positional arguments left-to-right
named arguments left-to-right
invoke after all argument evaluation
```

### 15.3 Lowered Shape

A call lowers to:

```text
lower callee -> callee slot
lower args -> argument slots
emit CallOp {
  call_site_id
  callee
  arguments
  fallback_helper
}
```

### 15.4 Defaults

Default argument evaluation occurs inside call handling when omitted.

Lowering must not evaluate defaults at function object construction.

### 15.5 Named Arguments

Named argument layout is recorded in `ArgumentLayout`.

Duplicate named arguments should already be SIR validation errors; lowering may still assert.

### 15.6 Call Feedback

CallOp references feedback and inline cache slots through CallSiteRecord.

---

## 16. Access Lowering

### 16.1 Access Site Creation

Each attribute/index/slice access that can become hot creates `AccessSiteRecord`.

```text
AttributeAccessNode -> AccessSiteId
IndexAccessNode -> AccessSiteId
SliceExpressionNode -> AccessSiteId
```

### 16.2 Attribute Read

If receiver type is statically known record:

```text
LoadField expected_shape field_index
```

Otherwise:

```text
AttributeRead fallback helper
```

### 16.3 Attribute Write

If receiver type is statically known record:

```text
StoreField expected_shape field_index
```

Otherwise fallback helper.

Unknown record fields remain invalid.

### 16.4 Method Access

Method access lowers to method-table lookup or specialized bound-method construction.

It should not use string dictionary lookup for known record methods.

### 16.5 Index Read

List and map index read lower to specialized EIR if receiver type is known.

Otherwise fallback helper.

### 16.6 Slice Read

List/string slicing lowers to `SliceRead` or helper.

Negative bounds and out-of-range behavior must match Phase 2 semantics.

---

## 17. Collection Construction Lowering

### 17.1 List Literal

List literal lowering:

```text
lower each element left-to-right
ConstructList
```

### 17.2 Map Literal

Map literal lowering:

```text
for each entry in source order:
  lower key
  check hashable
  lower value
ConstructMap
```

Duplicate key behavior:

```text
later value replaces earlier value
first insertion position preserved
```

### 17.3 Range

Range construction through core builtin may lower to helper or specialized Range construction.

---

## 18. Record Construction Lowering

### 18.1 RecordConstructionNode

Lowering:

```text
resolve RecordShape
lower field initializers in source order
lower omitted defaults at construction time
check required fields
check duplicate fields
check field contracts
ConstructRecord
```

### 18.2 Default Fields

Record field defaults are evaluated at construction time.

Lowering must preserve source-order and error-order semantics.

### 18.3 Field Contracts

Each field value is checked before record allocation commits or during construction before object publication.

If construction fails, no partially visible record escapes.

---

## 19. Enum Construction Lowering

### 19.1 EnumConstructionNode

Lowering:

```text
resolve EnumShape
resolve CaseIndex
lower payload expressions in source order
check payload contracts
ConstructEnumValue
```

### 19.2 No Dynamic Cases

Enum cases are closed.

Lowering must not generate dynamic case lookup for known enum case constructors except fallback error paths.

---

## 20. Function Construction Lowering

### 20.1 FunctionDeclarationNode

Lowering emits `ConstructFunction` at declaration execution point.

Then stores the function object into the function binding slot.

### 20.2 FunctionValueNode

A function value reference lowers to loading the function binding or constructing closure value if needed.

### 20.3 Capture Slots

Capture slots are collected from `CaptureLayout`.

Mutable captures use cell references.

### 20.4 Defaults

Parameter default NodeIds are retained for call-time lowering/execution.

They are not evaluated during function construction.

---

## 21. Format String Lowering

### 21.1 FormatStringNode

Lowering:

```text
create string builder or helper call
for each part left-to-right:
  text part -> constant string append
  expression part -> evaluate expression
  display convert
  append
produce String
```

### 21.2 Display Conversion

Display conversion is explicit through VM helper or builtin display protocol.

It is not implicit string coercion for arbitrary operations.

### 21.3 Error Order

Expression parts are evaluated left-to-right.

If expression evaluation raises, later parts are not evaluated.

---

## 22. Check Insertion

### 22.1 Required Checks

Lowering inserts EIR checks for:

```text
Bool conditions
type contracts
callability
arity
hashability
readonly mutation
capabilities
shape assumptions
integer overflow
division by zero
```

### 22.2 Check Placement

Checks must be placed before the operation whose semantic precondition they enforce.

A check must not be moved across side-effecting operations if that changes error order.

### 22.3 Guard vs Check

A `CheckOp` enforces language semantics.

A `GuardOp` enforces speculative optimization assumptions.

A failed check raises language error.

A failed guard falls back, deopts, calls helper, or raises only if the guard encodes a semantic precondition.

---

## 23. Source Map Preservation

### 23.1 SourceMap Requirement

Lowering must preserve mapping:

```text
EIR operation -> SIR NodeId -> SourceSpan
```

where source span exists.

### 23.2 Synthetic Operations

Synthetic operations inherit origin from the SIR node that required them.

Examples:

```text
contract checks
overflow checks
hashable checks
readonly checks
shape guards
helper calls
```

### 23.3 Diagnostic Rule

A runtime diagnostic from lowered EIR must report the source location of the original SIR/source construct, not merely the generated operation location.

---

## 24. RuntimePlan Validation

After RuntimePlan generation, validation must check:

```text
all runtime bindings have slots
all exported bindings have module cells or stable export slot references
all captures are represented
all functions have FunctionPlan
all records have RecordShape
all enums have EnumShape
all FieldId values used in EIR have FieldIndex
all CaseId values used in EIR have CaseIndex
all TypeId checks have RuntimeTypeDesc
all call expressions have CallSiteId
all access expressions have AccessSiteId where applicable
all synthetic module init functions exist
all source maps are structurally valid
```

---

## 25. EIR Validation

After EIR generation, validation must check:

```text
all slots referenced exist
all blocks terminate
all block targets exist
all checks reference valid TypeId/ShapeId
all calls reference valid CallSiteId
all accesses reference valid AccessSiteId
all helper references exist
all safepoints referenced exist
all deopt points referenced exist if guards use deopt
all writes have write-barrier path where heap mutation may occur
all source mappings are valid or explicitly synthetic
```

---

## 26. Lowering Failure

Lowering failure produces structured diagnostics.

Lowering must fail rather than generate semantically approximate EIR.

Failure examples:

```text
unsupported SIR node in current lowering mode
missing slot assignment
missing shape
missing helper
unsupported type contract
unrepresentable control region
unsupported pattern lowering
invalid source map
```

---

## 27. Compatibility

RuntimePlan and EIR are internal.

Lowering rules may evolve across VM versions.

However, any lowering from frozen SIR must preserve Phase 1 and Phase 2 semantics.

Lowering cache invalidates on:

```text
SIR digest change
RuntimePlan version change
EIR version change
VM version change
target profile change
helper table change
value layout profile change
GC profile change
capability environment change
dependency interface digest change
```

---

## 28. Non-Goals

This round does not define:

```text
structured-control lowering
unwinding lowering
import execution lowering
pattern lowering completeness
native backend lowering
Cranelift lowering
LLVM lowering
GC root enumeration details
public bytecode
```

---

## 29. Next Work

Next lowering round should define:

```text
block lowering
if lowering
while lowering
for lowering
return/break/continue lowering
raise lowering
try/catch/finally lowering
use/defer lowering
match/pattern lowering
assert/test lowering
module import execution lowering
structured unwinding lowering

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-SIR-LOWERING-ROUND1.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-CONTROL-LOWERING-ROUND2.md -->


# Phase 3 · Structured Control and Unwinding Lowering · Round 2
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.7 lowering draft  
Depends on: Phase 3 SIR to RuntimePlan / EIR Lowering Round 1 v0.6  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: lowering for blocks, if, while, for, return, break, continue, raise, try/catch/finally, use, defer, match/patterns, assert/test, module import execution, structured unwinding  
Out of scope: concrete Cranelift lowering, concrete GC implementation, full optimizer, public bytecode, native ABI

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



## 0. Round 2 Scope

This document completes the first VM lowering coverage for structured control and unwinding.

It defines lowering for:

1. block execution
2. if/elif/else
3. while loops
4. for loops
5. loop break/continue targets
6. return
7. raise
8. try/catch/finally
9. use
10. defer
11. match/case
12. pattern decision lowering
13. assert
14. test blocks
15. module import execution
16. module initialization control
17. structured unwinding
18. cleanup ordering
19. suppressed error handling
20. safepoints in control flow
21. deopt metadata for control flow
22. lowering validation

The guiding principle is:

```text
Preserve Phase 2 structured semantics while producing EIR suitable for fast interpretation and later JIT.
```

---

## 1. Control Lowering Overview

### 1.1 Source Structure

Phase 2 SIR preserves structured control:

```text
Block
If
While
For
Match
Try
Catch
Finally
Use
Defer
Return
Break
Continue
Raise
Assert
Test
```

### 1.2 Lowered Structure

Phase 3 lowers these constructs into:

```text
EIR blocks
EIR terminators
RegionPlan
RegionStack operations
RuntimeHelper calls
Safepoints
Deopt metadata
Control target maps
Cleanup plans
```

### 1.3 Required Preservation

Lowering must preserve:

```text
source execution order
Bool-only conditions
no truthiness
loop target identity
return target identity
structured unwinding
defer LIFO order
use close order
finally override rule
primary/suppressed error behavior
match case order
pattern binding scope
guard evaluation order
module import order
diagnostic source spans
```

---

## 2. RegionPlan Lowering

### 2.1 ControlRegionDescriptor to RegionPlan

Each SIR `ControlRegionDescriptor` lowers to a `RegionPlan`.

```text
ControlRegionId -> RegionPlan
```

### 2.2 RegionPlan

```text
RegionPlan {
  region_id: ControlRegionId
  kind: ControlRegionKind
  owner_node: NodeId
  parent?: ControlRegionId
  entry_block?: EirBlockId
  normal_exit_block?: EirBlockId
  unwind_entry_block?: EirBlockId
  cleanup_plan?: CleanupPlan
  source_span?: SourceSpanId
}
```

### 2.3 CleanupPlan

```text
CleanupPlan {
  defers: DeferStackPlan
  resources: ResourceCleanupPlan
  finally_block?: EirBlockId
  cleanup_order: CleanupOrder
}
```

### 2.4 CleanupOrder

The default cleanup order is:

```text
defers
resources
finally
```

For try/finally, the active try/catch block exits first, then finally executes.

Lowering may encode finally as an explicit EIR block or runtime helper target.

### 2.5 Region Stack Operations

Lowering may emit internal operations:

```text
PushRegion
PopRegion
RegisterDefer
RegisterResource
RunCleanup
BeginUnwind
ResumeUnwind
```

These may be represented as EIR ops or RuntimeHelperOp.

Early VM may implement them as helpers.

---

## 3. Pending Control Model

### 3.1 PendingControl

Structured control lowering uses a pending control value.

```text
PendingControl =
  | Normal
  | Return(Value)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
```

### 3.2 Pending Control Slot

Functions with structured control may reserve a hidden runtime slot:

```text
pending_control_slot: SlotId
```

This slot is not source-visible.

### 3.3 Lowering Rule

When a construct produces non-normal control, lowering writes pending control and jumps to the nearest relevant unwind block.

Example:

```text
return value
  -> write PendingControl::Return(value)
  -> jump function unwind entry
```

### 3.4 JIT/Deopt Rule

Pending control must be represented in deopt metadata.

Deoptimization must reconstruct:

```text
pending control kind
pending value or error
target region
active cleanup state
```

---

## 4. Block Lowering

### 4.1 BlockNode Lowering

A SIR block lowers to one or more EIR blocks.

Basic lowering:

```text
push block region if needed
lower each item in source order
if item completes non-normal -> jump unwind path
pop block region
jump normal exit
```

### 4.2 Block Scope

Block-local bindings lower to slots in the enclosing frame or block scope slot range.

If debug mode requires lexical reconstruction, source maps record block scope boundaries.

### 4.3 Empty Synthetic Blocks

Synthetic empty blocks lower to direct jump.

Source-originating empty blocks should already be rejected by Phase 2 validation.

### 4.4 Block Cleanup

If a block contains defers or resources, it owns cleanup state.

Exiting the block by:

```text
Normal
Return
Break
Continue
Raise
```

must execute block cleanup.

---

## 5. If Lowering

### 5.1 IfNode Lowering

An if node lowers to branch blocks.

Structure:

```text
entry
  evaluate condition0
  CheckBool
  Branch true -> then0, false -> next_condition
then0
  lower branch block
  jump if_exit
next_condition
  evaluate condition1
  CheckBool
  Branch true -> then1, false -> else_or_exit
...
else
  lower else block
if_exit
```

### 5.2 Bool Rule

Each condition must use `CheckBool`.

No truthiness.

### 5.3 Control Propagation

If any branch exits non-normally, it jumps into the relevant unwind path.

### 5.4 Source Mapping

Each condition check maps to the SIR condition source span.

Each branch block maps to the branch source span.

---

## 6. While Lowering

### 6.1 WhileNode Lowering

A while loop lowers to:

```text
loop_entry
  Safepoint? for loop entry
  evaluate condition
  CheckBool
  Branch true -> loop_body, false -> loop_exit

loop_body
  push loop/body regions as needed
  lower body
  on Normal -> loop_backedge
  on Continue(target loop) -> loop_backedge after cleanup
  on Break(target loop) -> loop_exit after cleanup
  on Return/Raise -> outer unwind

loop_backedge
  LoopBackedge terminator with safepoint and hotness counter
  jump loop_entry

loop_exit
```

### 6.2 Loop Safepoints

Loop backedges are safepoint candidates.

Lowering should attach:

```text
SafepointKind::LoopBackedge
hotness counter
live slot map
region state
```

### 6.3 Break/Continue Targeting

Break and continue target the nearest enclosing loop region unless Phase 2 introduces labels.

Current Phase 2 has no labels.

### 6.4 Loop Deopt State

Loop deopt metadata must include:

```text
current iteration state
live locals
active loop region
pending cleanup state
source span
```

---

## 7. For Lowering

### 7.1 ForNode Lowering

A for loop lowers to iterator-style EIR or helper-assisted iteration.

Structure:

```text
evaluate iterable
create iterator helper / iteration state
loop_entry
  poll next
  if done -> loop_exit
  bind iteration value
  lower body
  Normal/Continue -> loop_backedge
  Break -> loop_exit
  Return/Raise -> outer unwind
loop_backedge
  safepoint + hotness counter
  jump loop_entry
loop_exit
```

### 7.2 Iterable Categories

Required initial iterable categories:

```text
List
Map
Range
```

Map iteration yields keys in insertion order.

String iteration is not core.

### 7.3 Iterator State

Iterator state is stored in hidden runtime slots.

```text
iterable_slot
iterator_state_slot
current_value_slot
```

### 7.4 Iteration Binding

Each iteration writes the current value to the loop binding slot.

The binding is immutable per iteration.

Lowering may reuse the same physical slot across iterations if source semantics are preserved.

### 7.5 Pattern Destructuring in For

If future for-target patterns exist, they lower through pattern lowering.

If unsupported, lowering must reject with diagnostic.

---

## 8. Break and Continue Lowering

### 8.1 BreakNode

Lowering:

```text
write PendingControl::Break(loop_region)
jump current region unwind entry
```

### 8.2 ContinueNode

Lowering:

```text
write PendingControl::Continue(loop_region)
jump current region unwind entry
```

### 8.3 Cleanup Rule

Before reaching loop exit or loop backedge, all inner region cleanups must execute.

### 8.4 Target Validation

Lowering must reject:

```text
break without loop target
continue without loop target
target not ancestor loop
```

Phase 2 validation should already catch this; lowering rechecks.

---

## 9. Return Lowering

### 9.1 ReturnNode

Lowering:

```text
evaluate return value or nil
write PendingControl::Return(value)
jump function unwind entry
```

### 9.2 Return Contract

Return contract check may occur:

```text
before writing PendingControl::Return
or at function exit block
```

The chosen location must preserve error order.

Recommended:

```text
evaluate return expression
check return contract
write PendingControl::Return
unwind
```

If cleanup raises after return, cleanup error rules apply.

### 9.3 Function Exit

After all cleanup, function exit terminator returns the pending return value.

### 9.4 Top-Level Return

Top-level return is invalid and must not lower.

---

## 10. Raise Lowering

### 10.1 RaiseNode

Lowering:

```text
evaluate error expression
CheckType Error
write PendingControl::Raise(error)
jump current unwind entry
```

### 10.2 Non-Error Raise

If evaluated value is not Error, VM raises TypeError.

Lowering may represent this as:

```text
CheckType Error failure_code=TypeError
```

### 10.3 Raise Safepoint

Raise boundary is a safepoint candidate.

The safepoint map must include:

```text
error object
live locals
region stack
pending control state
```

---

## 11. Try/Catch/Finally Lowering

### 11.1 TryNode Structure

A try node lowers to:

```text
try_entry
try_body
catch_match
catch_body
finally_entry
try_exit
```

depending on available clauses.

### 11.2 Try Body

Try body executes normally.

If it raises and catch exists, control transfers to catch matching.

If it raises and no catch exists, pending raise continues to finally if present, then outward.

### 11.3 Catch Clause Lowering

Catch lowering:

```text
if pending control is Raise:
  bind error to catch binding
  evaluate catch guard if present
  CheckBool guard
  if guard true -> catch_body
  if guard false -> preserve original Raise
```

### 11.4 Catch Binding

Catch binding lowers to a local slot or cell as required by capture/export rules.

The caught error is initialized before guard evaluation.

### 11.5 Finally Lowering

Finally block executes on all exits:

```text
Normal
Return
Break
Continue
Raise
```

### 11.6 Finally Override

If finally completes normally, prior pending control resumes.

If finally produces Return/Break/Continue/Raise, it replaces prior pending control.

Lowering must encode this explicitly.

### 11.7 Finally Deopt Metadata

Finally deopt state must include:

```text
prior pending control
finally active state
live slots
active regions
source span
```

---

## 12. Use Lowering

### 12.1 UseNode Lowering

A use node lowers to:

```text
evaluate resource expression
bind resource
register resource cleanup in current/use region
lower body
on any exit -> run resource cleanup
```

### 12.2 Resource Registration

Resource registration records:

```text
resource slot
close method symbol
acquisition source span
closed flag
```

### 12.3 Close Rule

If acquisition succeeds, close must be called exactly once.

If acquisition fails, body does not execute and no close is registered.

### 12.4 Close Error Handling

If body exits normally and close raises, use exits with Raise.

If body raises and close raises, body error remains primary and close error is suppressed.

If body returns/breaks/continues and close raises, cleanup error becomes Raise unless Phase 2 amendment specifies otherwise.

### 12.5 Use Safepoint

Resource close is a helper call and safepoint candidate.

---

## 13. Defer Lowering

### 13.1 DeferNode Lowering

A defer node lowers to:

```text
evaluate callable
CheckCallable
CheckArity zero
register defer in current region
```

### 13.2 Defer Registration

Defer registration stores:

```text
callable value
registration source span
owning region
```

### 13.3 Defer Execution

Defers execute when owning region exits.

Order:

```text
last registered
first executed
```

### 13.4 Defer Error Handling

If defer raises while no error is pending, it becomes primary Raise.

If defer raises while Raise is pending, defer error is suppressed.

If defer raises while Return/Break/Continue is pending, lowering follows Phase 2 cleanup error rule.

### 13.5 Defer and JIT

JIT code must not inline away defer registration unless it proves identical cleanup behavior and deopt state.

---

## 14. Match and Pattern Lowering

### 14.1 MatchNode Lowering

A match lowers to an ordered decision sequence.

Structure:

```text
evaluate subject once
for each case in source order:
  attempt pattern
  if fail -> next case
  bind pattern variables
  if guard exists:
    evaluate guard
    CheckBool
    if false -> next case
  execute case body
  jump match_exit or propagate control
no match -> Normal
```

### 14.2 Subject Evaluation

Subject expression is evaluated exactly once.

The value is stored in a temporary slot.

### 14.3 Case Order

Case order is source order.

Lowering may build decision trees only when it preserves source-observable behavior.

### 14.4 Pattern Failure

Pattern failure inside match is branch control, not error.

Pattern failure in declaration destructuring remains `PatternMatchError`.

### 14.5 Pattern Binding Slots

Pattern bindings lower to case-scope slots.

Bindings are initialized only after successful pattern match.

### 14.6 Or-Pattern Lowering

Or-pattern alternatives must bind the same binding set.

Lowering may allocate shared binding slots.

Each alternative writes the same logical binding slots.

### 14.7 Guard Lowering

Guard executes after pattern bindings.

Guard must CheckBool.

Guard false proceeds to next case.

### 14.8 Pattern Operation Lowering

Pattern kinds lower as:

```text
Wildcard -> direct success
Literal -> equality check
Binding -> slot write
Record -> shape check + field subpatterns
Enum -> shape/case check + payload subpatterns
List -> length check + element subpatterns
Map -> key checks + value subpatterns
Or -> alternative branches
```

### 14.9 Decision Tree Rule

Optimized decision trees are allowed only if they preserve:

```text
subject single evaluation
case order where guards or side effects can observe order
binding scope
guard timing
error order
```

Conservative lowering is acceptable.

---

## 15. Assert Lowering

### 15.1 AssertNode

Lowering:

```text
evaluate condition
CheckBool
if true -> continue
if false:
  evaluate message if present
  construct AssertionError
  Raise
```

### 15.2 Message Evaluation

Message is evaluated only on assertion failure.

If message exists, it must be String.

### 15.3 Assertion Mode

Assertions are semantic in checked mode.

They must not be removed unless an explicit unchecked mode exists.

---

## 16. Test Lowering

### 16.1 TestNode

A test node lowers to a test function or test entry point.

Test nodes do not run during ordinary module initialization.

### 16.2 TestPlan

```text
TestPlan {
  test_name: String
  test_region: ControlRegionId
  test_function: EirFunctionId
  source_span?: SourceSpanId
}
```

### 16.3 Test Execution

A test runner invokes test functions explicitly.

Normal completion means success.

Raise or failed assertion means failure.

### 16.4 Test Isolation

Test isolation policy is host/test-runner defined.

The VM must at minimum preserve test source span and module association.

---

## 17. Module Import Execution Lowering

### 17.1 Import Declaration

Import execution lowering uses `ImportPlanEntry`.

Lowered steps:

```text
resolve module
load or retrieve module instance
if needed initialize module
check interface digest
bind imported module or export value
```

### 17.2 Source Order

Imports execute in source order as part of module initialization.

### 17.3 Named Import

Named import lowering:

```text
ensure provider initialized or safely initializing
check export exists
read provider export cell
bind local import slot
```

### 17.4 Whole Module Import

Whole-module import lowering binds the module object.

### 17.5 Circular Import

If imported value is an uninitialized export during a cycle, raise `ImportCycleError`.

### 17.6 Import Safepoint

Module import boundary is a safepoint candidate.

It may allocate, raise, and execute arbitrary module initialization.

---

## 18. Module Initialization Lowering

### 18.1 Initialization Function

Each module has synthetic initialization EIR function.

It handles:

```text
module state transition
top-level item execution
import execution
export table sealing
failure transition
```

### 18.2 Module State Transitions

Lowering emits or helpers implement:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
```

### 18.3 Failure Handling

If top-level execution raises, module state becomes Failed and stores initialization error.

### 18.4 Export Sealing

After successful initialization, export table is sealed.

Further mutation of export table is invalid.

---

## 19. Structured Unwinding Lowering

### 19.1 Unwind Entry

Each region that owns cleanup has an unwind entry block or helper target.

### 19.2 Unwind Algorithm

Lowered unwinding executes:

```text
while pending control not resolved:
  run current region defers in LIFO order
  run current region resource cleanup
  run finally block if attached
  pop region
  if pending control target reached:
    resolve control
  else continue outward
```

### 19.3 Cleanup Error Handling

Cleanup error handling follows Phase 2:

```text
pending Raise + cleanup Raise -> primary preserved, cleanup suppressed
pending Normal + cleanup Raise -> cleanup becomes primary Raise
pending Return/Break/Continue + cleanup Raise -> cleanup Raise supersedes non-error control unless amended
finally non-normal control -> overrides prior pending control
```

### 19.4 Suppressed Error Storage

Suppressed errors attach to primary Error object where supported.

If unsupported in bootstrap VM, the VM must still preserve diagnostics indicating suppressed cleanup error.

### 19.5 Unwind Helper

Early VM may implement unwinding through helper:

```text
RuntimeHelper::perform_unwind
```

Later EIR may inline parts of unwinding.

### 19.6 JIT Rule

JIT code may not skip unwinding.

Compiled frames must expose enough region/defer/resource state for unwinding helper or deopt.

---

## 20. Safepoints in Control Lowering

### 20.1 Required Control Safepoints

Control lowering must seed safepoints at:

```text
loop backedge
function call
runtime helper call
resource close
defer execution
module import
raise boundary
allocation in pattern/list/map/record/enum construction
```

### 20.2 Root Maps

Safepoint root maps must include:

```text
live slots
subject temporaries
pattern binding candidates
pending control value
error values
active region state
resource handles
defer callables
module import state
```

### 20.3 Deopt State

Deopt metadata for control constructs must include enough information to reconstruct:

```text
current source construct
active block
active loop
active match case
active try/catch/finally
pending control
cleanup progress
live values
```

---

## 21. Lowering Validation

Structured control lowering must reject:

```text
block region missing
if condition without Bool check
while condition without Bool check
loop without loop region
break target missing
continue target missing
return outside function
raise without Error check
try without valid region plan
catch binding without slot
finally without override handling
use without resource cleanup plan
defer without owning region
match without subject temp
pattern binding outside case scope
or-pattern with inconsistent binding layout
assert without Bool check
test executing during module initialization
import without ImportPlanEntry
module init without state transition handling
unwind path missing cleanup
cleanup path without suppressed error strategy
```

---

## 22. Compatibility

Control lowering may evolve.

But it must preserve Phase 1 and Phase 2 semantics.

Changing any of the following is a semantic change:

```text
condition Bool-only behavior
loop break/continue target
return cleanup order
raise propagation
finally override behavior
defer LIFO order
use close exactly-once rule
match case order
pattern binding scope
assert message evaluation timing
module import source order
circular import error behavior
suppressed error policy
```

---

## 23. Non-Goals

This document does not define:

```text
concrete machine-code lowering
Cranelift lowering
LLVM lowering
complete GC implementation
complete deopt runtime
complete inline cache machinery
public bytecode
native ABI
debugger protocol
```

---

## 24. Next Work

Next Phase 3 documents should define:

```text
runtime helper contracts
GC root enumeration concrete model
baseline JIT backend interface
EIR structured-control operation round 2 if needed
fast interpreter concrete data structures
Phase 3 audit pass

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-CONTROL-LOWERING-ROUND2.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-RUNTIME-HELPER-CONTRACTS.md -->


# Phase 3 · Runtime Helper Contracts
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.8 helper draft  
Depends on: Phase 3 Structured Control and Unwinding Lowering Round 2 v0.7  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: runtime helper ABI boundary, helper descriptors, helper families, GC/JIT/capability contracts, error/control return discipline, helper validation  
Out of scope: concrete native ABI, concrete Cranelift lowering, concrete LLVM lowering, full standard library, full FFI implementation

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

Runtime helpers are internal VM functions used by:

```text
EIR fast interpreter
baseline JIT
optimizing JIT
module initialization runtime
capability host boundary
GC-aware allocation paths
structured unwinding runtime
```

Runtime helpers are not:

```text
public native ABI
foreign extension ABI
CPython C API
Python extension boundary
stable plugin ABI
public bytecode helper set
```

Runtime helpers are internal, versioned, VM-controlled, and allowed to change across VM versions.

---

## 1. Purpose

Runtime helpers exist to keep EIR compact while preserving semantics.

They handle operations that are:

```text
complex
rare
effectful
allocation-heavy
capability-gated
error-heavy
unwinding-heavy
JIT-unfriendly
host-boundary-sensitive
```

Examples:

```text
generic call
generic access
map lookup
string slicing
record construction fallback
enum construction fallback
type contract check
pattern fallback
raise construction
structured unwind
defer execution
resource close
module import
capability check
allocation
write barrier
```

A helper is a slow path or semantic boundary, not an escape hatch for unspecified behavior.

---

## 2. Helper Boundary Rule

### 2.1 Internal ABI

Runtime helper ABI is VM-internal.

It may be called by:

```text
interpreter
JIT-generated code
VM builtin implementation
module initialization runtime
GC/runtime services
```

It must not be exposed as:

```text
native extension ABI
public package ABI
external compiler target ABI
user-callable function table
```

### 2.2 Versioning

Helper signatures are versioned by:

```text
VM version
RuntimeHelperTable version
target profile
value layout profile
GC profile
JIT backend profile
```

If helper ABI changes, compiled code and EIR caches that depend on it must be invalidated.

### 2.3 No Semantic Drift

Helper behavior must match Phase 1 and Phase 2 semantics.

Interpreter fast path and helper slow path must produce equivalent observable behavior.

---

## 3. Helper Descriptor

### 3.1 RuntimeHelperDescriptor

```text
RuntimeHelperDescriptor {
  helper_id: RuntimeHelperId
  name: String
  family: RuntimeHelperFamily
  signature: RuntimeHelperSignature
  may_allocate: Bool
  may_raise: Bool
  may_unwind: Bool
  is_safepoint: Bool
  requires_roots_visible: Bool
  required_capability?: CapabilityId
  effect?: EffectId
  gc_behavior: HelperGcBehavior
  jit_call_policy: HelperJitCallPolicy
  source_mapping_policy: HelperSourceMappingPolicy
}
```

### 3.2 RuntimeHelperFamily

```text
RuntimeHelperFamily =
  | Call
  | Access
  | Construction
  | TypeCheck
  | Pattern
  | Error
  | Unwind
  | Resource
  | Module
  | Capability
  | Allocation
  | WriteBarrier
  | Display
  | Numeric
  | Debug
```

### 3.3 RuntimeHelperSignature

```text
RuntimeHelperSignature {
  parameters: List<HelperParam>
  result: HelperResultType
  calling_convention: HelperCallingConvention
}
```

### 3.4 HelperParam

```text
HelperParam {
  name: String
  kind: HelperParamKind
  required: Bool
}
```

```text
HelperParamKind =
  | Value
  | SlotRef
  | FrameRef
  | VmRef
  | RuntimePlanRef
  | CallSiteId
  | AccessSiteId
  | ShapeId
  | TypeId
  | CapabilityId
  | SourceSpanId
  | PendingControlRef
  | RegionRef
  | ModuleId
  | HelperInternal
```

### 3.5 HelperResultType

```text
HelperResultType =
  | Value
  | VmControl
  | Unit
  | Bool
  | ObjectRef
  | ModuleRef
  | ErrorRef
  | HelperInternal
```

### 3.6 HelperCallingConvention

```text
HelperCallingConvention =
  | InterpreterDirect
  | JitRuntimeCall
  | HostBoundaryCall
  | GcRuntimeCall
  | InternalOnly
```

The calling convention is internal.

It is not public ABI.

---

## 4. Helper Return Discipline

### 4.1 HelperReturn

Conceptual result:

```text
HelperReturn =
  | Value(Value)
  | Control(VmControl)
  | Unit
  | Error(VmError)
```

### 4.2 Language Errors

Language-level errors return:

```text
Control(Raise(error))
```

Examples:

```text
TypeError
FieldError
ImportError
CapabilityError
AssertionError
PatternMatchError
```

### 4.3 VM Structural Failures

VM internal structural failures return:

```text
Error(VmError)
```

Examples:

```text
invalid helper ID
corrupt RuntimePlan
invalid SlotId
stale ObjectId
missing root map
broken frame map
```

### 4.4 Rust Panic Boundary

Rust panic is not a helper return mechanism.

A panic indicates implementation bug or unrecoverable host failure.

---

## 5. GC and Root Visibility Contract

### 5.1 Helper GC Declaration

Each helper declares:

```text
may_allocate
is_safepoint
requires_roots_visible
gc_behavior
```

### 5.2 HelperGcBehavior

```text
HelperGcBehavior =
  | NoAllocation
  | MayAllocateNoCollection
  | MayAllocateMayCollect
  | MayMoveObjects
  | GcInternal
```

### 5.3 Root Visibility

If helper may allocate or collect, all live roots must be visible before helper call.

Required visible roots:

```text
live slots
capture cells
module cells
pending control
active errors
region stack
defer callables
resource handles
host roots
JIT frame roots
```

### 5.4 Moving GC Compatibility

A helper that may move objects must not return stale raw object addresses.

All object references crossing helper boundary must use VM-managed handles or relocated references.

### 5.5 Helper Safepoint

A helper marked `is_safepoint` must have a valid `SafepointRecord`.

JIT calls to such helper must provide or reference correct stack/root maps.

---

## 6. JIT Helper Contract

### 6.1 JIT Call Rule

JIT code may call runtime helpers only through the VM-controlled helper table.

JIT code must not directly call arbitrary host/native pointers.

### 6.2 JitRuntimeCall Requirements

A helper callable from JIT must define:

```text
argument representation
return representation
clobbered registers abstractly
safepoint behavior
root visibility requirement
may_raise behavior
may_unwind behavior
deopt interaction
```

This is backend-internal metadata, not public ABI.

### 6.3 Deopt Interaction

If a helper can deopt, unwind, or raise, the call site must have:

```text
source span
frame map
root map
pending control map
region stack map
deopt point if speculative
```

### 6.4 Helper Inlining

A JIT may inline helper logic only if it preserves:

```text
error category
source mapping
capability checks
GC safepoints
write barriers
deopt metadata
unwinding behavior
```

Otherwise helper call must remain explicit.

---

## 7. Capability Contract

### 7.1 Capability Declaration

Any helper that performs effectful host access must declare:

```text
required_capability
effect
```

### 7.2 Capability Check Placement

Capability checks occur before effectful operation.

They may be:

```text
explicit CheckCapability EIR op
or helper-internal check
```

If helper-internal, the helper descriptor must declare required capability.

### 7.3 Capability Failure

Missing capability returns language error:

```text
CapabilityError
```

### 7.4 JIT Rule

JIT code must not bypass capability checks.

A capability-using helper must not be replaced with unchecked native call.

---

## 8. Helper Families

## 8.1 Call Helpers

### 8.1.1 `helper_generic_call`

Purpose:

```text
invoke value whose callable kind is not statically specialized
```

Inputs:

```text
callee: Value
arguments: ValueList
call_site_id: CallSiteId
frame: FrameRef
```

May:

```text
allocate
raise
unwind
be safepoint
update call-site feedback
```

Must preserve:

```text
argument evaluation order already completed by caller
arity checks
default argument call-time evaluation
parameter contract checks
return contract checks
call-site feedback updates
```

### 8.1.2 `helper_bind_method`

Purpose:

```text
construct bound method value
```

Must preserve receiver identity.

May allocate.

### 8.1.3 `helper_call_builtin`

Purpose:

```text
invoke builtin function through VM builtin table
```

Must enforce builtin signature and capability requirements.

---

## 8.2 Access Helpers

### 8.2.1 `helper_get_attribute`

Purpose:

```text
generic attribute read
```

Inputs:

```text
receiver
attribute symbol
access_site_id
```

Required behavior:

```text
record field lookup by shape when possible
module export lookup
method lookup
readonly view delegation
error on unknown field
no dynamic record field creation
```

### 8.2.2 `helper_set_attribute`

Purpose:

```text
generic attribute write
```

Must enforce:

```text
readonly rejection
fixed record shape
field mutability
field type contract
write barrier
```

### 8.2.3 `helper_index_read`

Purpose:

```text
generic index read
```

Supported categories:

```text
List[Int]
Map[Hashable]
```

String indexing remains non-core unless later amended.

### 8.2.4 `helper_index_write`

Purpose:

```text
generic index write
```

Must enforce:

```text
readonly rejection
list bounds
map hashability
write barrier
```

### 8.2.5 `helper_slice_read`

Purpose:

```text
list/string slicing
```

Must enforce:

```text
Int bounds
half-open slicing
negative bound errors
out-of-range errors unless Phase 2 amended
```

---

## 8.3 Construction Helpers

### 8.3.1 `helper_construct_record`

Purpose:

```text
fallback or generic record construction
```

Must enforce:

```text
fixed shape
required fields
default fields at construction time
unknown/duplicate field errors
field contract checks
no partially visible record escape on failure
```

### 8.3.2 `helper_construct_enum`

Purpose:

```text
fallback or generic enum value construction
```

Must enforce:

```text
closed enum cases
payload arity
payload contract checks
case identity
```

### 8.3.3 `helper_construct_map`

Purpose:

```text
construct map with hashability and duplicate-key semantics
```

Must preserve insertion order.

Duplicate key rule:

```text
later value replaces earlier value
first insertion position preserved
```

### 8.3.4 `helper_construct_error`

Purpose:

```text
construct language Error object
```

Must attach:

```text
error code
message
source span
stack trace if requested
suppressed error list
```

---

## 8.4 Type Check Helpers

### 8.4.1 `helper_check_type_contract`

Purpose:

```text
generic type contract check
```

Inputs:

```text
value
type_id
failure_code
source_span
```

Must return original value or raise `TypeContractError`.

### 8.4.2 `helper_check_callable`

Purpose:

```text
generic callable check
```

Must recognize:

```text
Function
BuiltinFunction
RecordConstructor
EnumCaseConstructor
BoundMethod
HostFunction
```

### 8.4.3 `helper_check_hashable`

Purpose:

```text
map key hashability check
```

Must not allow mutable/hash-unstable keys unless a future hash protocol explicitly permits them.

### 8.4.4 `helper_check_shape`

Purpose:

```text
generic shape check
```

Can fall back for non-record/non-enum categories depending on caller context.

---

## 8.5 Pattern Helpers

### 8.5.1 `helper_match_pattern`

Purpose:

```text
generic pattern fallback
```

Must support:

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

### 8.5.2 Pattern Failure Mode

Helper must distinguish:

```text
branch failure in match
PatternMatchError in destructuring declaration
```

### 8.5.3 Pattern Binding

Pattern bindings must be written only after successful match unless rollback semantics are explicit.

Recommended rule:

```text
write to temporary binding slots
commit to case binding slots only after full pattern success
```

---

## 8.6 Error Helpers

### 8.6.1 `helper_raise`

Purpose:

```text
convert Error value to pending Raise control
```

Must reject non-Error raise with `TypeError`.

### 8.6.2 `helper_attach_suppressed`

Purpose:

```text
attach cleanup error as suppressed error to primary error
```

If suppressed list unsupported in bootstrap runtime, helper must preserve diagnostic record.

### 8.6.3 `helper_assert_fail`

Purpose:

```text
construct AssertionError and raise
```

Must evaluate assertion message only when assertion fails.

---

## 8.7 Unwind Helpers

### 8.7.1 `helper_perform_unwind`

Purpose:

```text
execute structured unwinding according to RegionStack and PendingControl
```

Must preserve:

```text
defer LIFO order
resource close order
finally execution
finally override rule
primary/suppressed error behavior
loop target resolution
function return resolution
```

### 8.7.2 Inputs

```text
pending_control
frame
region_stack
source_span
```

### 8.7.3 Output

```text
VmControl
```

or updated pending control.

### 8.7.4 JIT Requirement

Compiled frames must expose region stack and root maps to unwind helper.

---

## 8.8 Resource Helpers

### 8.8.1 `helper_register_resource`

Purpose:

```text
register resource acquired by use
```

Must register cleanup only after successful acquisition.

### 8.8.2 `helper_close_resource`

Purpose:

```text
close resource exactly once
```

Must handle:

```text
Open -> Closing -> Closed
close raises
double close policy
suppressed cleanup error
capability-origin tracking
```

### 8.8.3 `helper_register_defer`

Purpose:

```text
register zero-argument deferred callable
```

Must check callable and arity.

### 8.8.4 `helper_execute_defer`

Purpose:

```text
execute one deferred callable
```

Must return Raise if callable raises.

---

## 8.9 Module Helpers

### 8.9.1 `helper_resolve_module`

Purpose:

```text
resolve module name through host-defined deterministic resolver
```

May require module-system capability if host chooses.

### 8.9.2 `helper_initialize_module`

Purpose:

```text
execute module initialization state machine
```

Must preserve:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
```

### 8.9.3 `helper_import_named`

Purpose:

```text
bind named export to local import slot
```

Must detect:

```text
missing export
interface mismatch
uninitialized export in cycle
```

### 8.9.4 `helper_import_module`

Purpose:

```text
bind module object to local import slot
```

### 8.9.5 `helper_seal_exports`

Purpose:

```text
seal module export table after successful initialization
```

---

## 8.10 Capability Helpers

### 8.10.1 `helper_check_capability`

Purpose:

```text
verify capability exists in current capability environment
```

Failure raises `CapabilityError`.

### 8.10.2 `helper_enter_host_call`

Purpose:

```text
prepare roots and capability state before host call
```

May be safepoint.

### 8.10.3 `helper_exit_host_call`

Purpose:

```text
normalize host return or host error into VM result
```

Must not leak host exception representation into language runtime.

---

## 8.11 Allocation Helpers

### 8.11.1 `helper_alloc_object`

Purpose:

```text
allocate generic heap object
```

Must:

```text
make roots visible
honor GC profile
return VM handle
not expose raw pointer
```

### 8.11.2 Specialized Allocation Helpers

Allowed helpers:

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

### 8.11.3 Allocation Safepoint

Allocation may be safepoint.

If allocation may trigger GC, root maps must be available.

---

## 8.12 Write Barrier Helpers

### 8.12.1 `helper_write_barrier`

Purpose:

```text
record heap reference mutation for future GC
```

Initial implementation may be no-op.

### 8.12.2 Barrier Sites

Barrier hook must exist for:

```text
record field write
list element write
map entry write
capture cell write
module binding write
resource state write where heap refs stored
```

### 8.12.3 Barrier Rule

Even if current GC does not need write barriers, mutation APIs must route through a barrier insertion point.

---

## 8.13 Display Helpers

### 8.13.1 `helper_display`

Purpose:

```text
convert value to display string
```

Used by:

```text
print
format string
debug
diagnostics
```

Must not create implicit coercion for ordinary operators.

### 8.13.2 Display Error

If display conversion can fail, failure must be language error or diagnostic error according to call site.

---

## 8.14 Numeric Helpers

### 8.14.1 `helper_numeric_binary`

Purpose:

```text
fallback numeric operation
```

Must enforce:

```text
no implicit coercion
overflow check
division by zero check
operator support
```

### 8.14.2 `helper_compare`

Purpose:

```text
generic comparison
```

Unsupported comparisons raise `TypeError`.

---

## 9. Helper Source Mapping

### 9.1 SourceSpan Input

Helpers that may raise must receive or reconstruct source span.

### 9.2 Diagnostic Mapping

Helper-generated diagnostics must map to:

```text
EIR op -> SIR node -> source span
```

### 9.3 Helper Internal Frames

Runtime helper frames may be hidden from user stack traces unless debug mode asks for VM internals.

Language stack trace must remain source-oriented.

---

## 10. Helper Validation

RuntimeHelperTable validation must reject:

```text
duplicate helper IDs
missing helper implementation
signature mismatch
helper marked no-allocate but allocation path exists
helper marked no-raise but can raise language error
helper marked non-safepoint but may collect
helper requires capability but descriptor omits it
JIT-callable helper without JIT call policy
GC-moving helper without handle-safe return policy
helper family mismatch
missing source mapping policy for may_raise helper
```

---

## 11. Helper Compatibility

Runtime helpers are VM-internal but cache-sensitive.

Changing helper signatures invalidates:

```text
EIR cache
RuntimePlan cache
JIT compiled code
safepoint maps
deopt metadata
runtime helper table digest
```

### 11.1 Helper Table Digest

Helper table digest includes:

```text
helper IDs
names
families
signatures
may_allocate
may_raise
may_unwind
is_safepoint
required capabilities
GC behavior
JIT call policy
```

### 11.2 Cache Rule

Compiled code must not run if helper table digest differs from the digest used when compiling.

---

## 12. Security and Capability Safety

Runtime helpers are a privilege boundary.

They must not:

```text
perform ambient host access
skip capability checks
expose raw object pointers
expose host exception objects
call arbitrary native code
mutate read-only values
bypass module interface validation
```

Any helper that crosses into host capability territory must be audited as host boundary code.

---

## 13. Bootstrap Policy

The initial VM may implement many helpers in Rust directly.

However:

```text
helper implementation detail must not become public ABI
Rc/RefCell inside helper is bootstrap-only
native pointer identity must not escape helper
helper names are internal descriptors, not user API
```

The bootstrap VM may implement unwinding, allocation, generic access, and pattern fallback through helpers for simplicity.

Later VM may inline or specialize them.

---

## 14. JIT Readiness Matrix

Each helper should be classified:

```text
JitLoweringClass =
  | AlwaysCallHelper
  | InlineFastPathWithHelperFallback
  | InlineAfterGuard
  | InterpreterOnly
  | ForbiddenInJit
```

Examples:

```text
generic call -> InlineFastPathWithHelperFallback
record field access fallback -> InlineAfterGuard
allocation -> AlwaysCallHelper or InlineWithGCProtocol
write barrier -> InlineAfterGuard or AlwaysCallHelper
perform_unwind -> AlwaysCallHelper initially
module import -> AlwaysCallHelper
capability check -> InlineAfterGuard or AlwaysCallHelper
```

This classification guides baseline JIT but does not change semantics.

---

## 15. Non-Goals

This document does not define:

```text
public native ABI
foreign extension ABI
CPython C API
concrete register convention
concrete Cranelift lowering
concrete LLVM lowering
full standard library
host resource implementations
debugger protocol
profiler format
```

---

## 16. Next Work

Next Phase 3 documents should define:

```text
GC root enumeration concrete model
baseline JIT backend interface
fast interpreter concrete data structures
runtime helper implementation plan
Phase 3 consistency audit

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-RUNTIME-HELPER-CONTRACTS.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md -->


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


<!-- END NORMATIVE DOCUMENT: PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-BASELINE-JIT-BACKEND-INTERFACE.md -->


# Phase 3 · Baseline JIT Backend Interface
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.10 JIT interface draft  
Depends on: Phase 3 GC Root Enumeration and Safepoint Model v0.9  
Depends on: Phase 3 Runtime Helper Contracts v0.8  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: baseline JIT backend abstraction, JIT context, compiled function model, code handle, helper-call ABI boundary, safepoint/stack-map/deopt emission, backend-independent lowering categories, cache invalidation, validation  
Out of scope: concrete Cranelift implementation, concrete LLVM implementation, optimizing JIT, register allocator design, native ABI, public bytecode

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

This document defines the first JIT backend interface for Phase 3.

It does not implement JIT.

It fixes the architecture so that a baseline JIT can later be implemented without rewriting the VM.

The core rule remains:

```text
JIT implementation: staged
JIT architecture: mandatory
```

The recommended first implementation target is a Cranelift-compatible baseline backend.

However:

```text
backend choice is not language semantics
backend ABI is VM-internal
backend lock-in is forbidden
EIR remains internal and non-public
```

---

## 1. Baseline JIT Purpose

The baseline JIT exists to reduce interpreter dispatch overhead and compile hot EIR functions quickly.

It is not the optimizing tier.

It should prioritize:

```text
fast compilation
semantic equivalence
stack-map correctness
helper-call correctness
safepoint correctness
deopt metadata preservation
capability-check preservation
GC compatibility
```

It may avoid complex optimizations.

The baseline JIT may still perform local fast-path lowering with guards and helper fallbacks.

---

## 2. JIT Tier Position

The execution tiers are:

```text
Tier 0: SIR correctness interpreter
Tier 1: RuntimePlan-driven interpreter
Tier 2: EIR fast interpreter
Tier 3: baseline JIT
Tier 4: optimizing JIT
```

This document defines Tier 3.

Tier 3 consumes:

```text
RuntimePlan
EIR
RuntimeHelperTable
SafepointTable
DeoptPointTable
FeedbackTable
GC target profile
Value layout profile
```

Tier 3 produces:

```text
CompiledFunction
StackMapTable
JitSafepointTable
JitDeoptTable
CodeMetadata
CacheMetadata
```

---

## 3. Backend Abstraction

### 3.1 JitBackend

Conceptual Rust-like interface:

```rust
pub trait JitBackend {
    fn backend_id(&self) -> JitBackendId;

    fn target_profile(&self) -> JitTargetProfile;

    fn supports(&self, input: &JitCompileInput) -> JitSupportReport;

    fn compile_function(
        &mut self,
        input: JitCompileInput,
        ctx: &mut JitContext,
    ) -> Result<CompiledFunction, JitCompileError>;

    fn invalidate(&mut self, reason: JitInvalidationReason);
}
```

This is conceptual.

The final Rust API may differ.

### 3.2 Backend Rule

A backend must compile from VM-internal EIR, not from source text or public bytecode.

A backend must not require EIR to become a public stable ABI.

### 3.3 Backend Kinds

```text
JitBackendKind =
  | CraneliftCompatibleBaseline
  | CustomBaseline
  | LlvmOrcOptimizing
  | WasmBackend
  | AotBackend
  | InterpreterAdapter
```

Only the baseline backend interface is in scope.

---

## 4. JitCompileInput

### 4.1 JitCompileInput

```text
JitCompileInput {
  eir_module: EirModuleRef
  eir_function: EirFunctionId
  runtime_plan: RuntimePlanRef
  function_plan: FunctionPlanRef
  helper_table: RuntimeHelperTableRef
  type_feedback: FeedbackTableRef
  shape_feedback: FeedbackTableRef
  inline_cache_state: InlineCacheTableRef
  safepoints: SafepointTableRef
  deopt_points: DeoptPointTableRef
  target_profile: JitTargetProfile
  compile_mode: BaselineCompileMode
}
```

### 4.2 BaselineCompileMode

```text
BaselineCompileMode =
  | Debug
  | Checked
  | Fast
```

`Debug` preserves maximal diagnostics and disables aggressive fast paths.

`Checked` is default.

`Fast` may emit more guarded fast paths but must preserve deopt/fallback correctness.

### 4.3 Input Validation

Before compilation, the backend must reject:

```text
invalid EIR
missing RuntimePlan
unsupported value layout
unsupported helper ABI
unsupported GC profile
missing safepoint data
missing stack-map requirements
unsupported operation family
missing deopt metadata for speculative guard
```

---

## 5. JitContext

### 5.1 JitContext

```text
JitContext {
  vm_version: Version
  runtime_plan_digest: Digest
  eir_digest: Digest
  helper_table_digest: Digest
  value_layout_profile: ValueLayoutProfile
  heap_profile: HeapProfile
  gc_profile: GcProfile
  target_profile: JitTargetProfile
  code_allocator: CodeAllocatorRef
  relocation_registry: RelocationRegistryRef
  safepoint_registry: SafepointRegistryRef
  stack_map_registry: StackMapRegistryRef
  deopt_registry: DeoptRegistryRef
  helper_call_registry: HelperCallRegistryRef
  diagnostics: DiagnosticSink
}
```

### 5.2 Context Rule

JitContext is VM-internal.

It may contain backend-specific state, but no backend-specific state may become language ABI.

### 5.3 Diagnostics

The JIT must emit structured diagnostics for:

```text
unsupported EIR op
invalid metadata
helper ABI mismatch
safepoint emission failure
stack map failure
deopt metadata failure
backend codegen failure
```

Compilation failure should fall back to EIR interpretation where safe.

---

## 6. JitTargetProfile

### 6.1 JitTargetProfile

```text
JitTargetProfile {
  architecture: TargetArchitecture
  operating_system?: TargetOperatingSystem
  pointer_width: UInt
  endianness: Endianness
  value_layout_profile: ValueLayoutProfile
  calling_convention_profile: CallingConventionProfile
  gc_profile: GcProfile
  code_model: CodeModel
  relocation_model: RelocationModel
  backend_feature_set: BackendFeatureSet
}
```

### 6.2 TargetArchitecture

```text
TargetArchitecture =
  | X86_64
  | AArch64
  | Wasm
  | InterpreterPseudoTarget
  | Other
```

### 6.3 Target Rule

Unsupported targets must fall back to interpreter.

The VM must not reinterpret semantics to satisfy a backend limitation.

---

## 7. EIR Lowering Categories

### 7.1 JitLoweringClass

Every EIR operation must eventually be classified:

```text
JitLoweringClass =
  | DirectLowering
  | GuardedFastPathWithHelperFallback
  | AlwaysCallHelper
  | DeoptRequired
  | InterpreterOnly
  | Forbidden
```

### 7.2 DirectLowering

Can be directly emitted as backend operations.

Examples:

```text
slot move
immediate constant load
simple branch
integer add with explicit overflow guard
shape comparison
case-index comparison
```

### 7.3 GuardedFastPathWithHelperFallback

Emits fast path plus fallback helper.

Examples:

```text
record field load
method read
monomorphic call
list index read
integer binary op
type descriptor check
```

### 7.4 AlwaysCallHelper

Always calls VM helper initially.

Examples:

```text
generic call
module import
perform_unwind
resource close
pattern fallback
map generic lookup
string slicing
allocation if GC protocol not inlined
```

### 7.5 DeoptRequired

May compile only if deopt metadata exists.

Examples:

```text
speculative shape-specialized call
speculative type-specialized arithmetic
inlined function body
elided check
```

### 7.6 InterpreterOnly

Backend does not compile this op.

Function remains interpreted or is split if supported.

### 7.7 Forbidden

The op must never appear in compiled code.

Examples:

```text
invalid VM-internal marker
debug-only assertion without runtime meaning
malformed synthetic op
```

---

## 8. CompiledFunction

### 8.1 CompiledFunction

```text
CompiledFunction {
  compiled_function_id: CompiledFunctionId
  source_eir_function: EirFunctionId
  function_id?: FunctionId
  code_handle: JitCodeHandle
  entry_points: CompiledEntryPoints
  stack_maps: StackMapTable
  safepoints: JitSafepointTable
  deopt_points: JitDeoptTable
  helper_calls: HelperCallSiteTable
  relocations: RelocationTable
  assumptions: AssumptionSet
  cache_key: JitCacheKey
  diagnostics?: DiagnosticTable
}
```

### 8.2 CompiledEntryPoints

```text
CompiledEntryPoints {
  normal_entry: CodeOffset
  checked_entry?: CodeOffset
  deopt_entry?: CodeOffset
  osr_entry_points?: List<OsrEntryPoint>
}
```

OSR is optional and deferred.

### 8.3 JitCodeHandle

```text
JitCodeHandle {
  code_id: CodeId
  memory_region: CodeMemoryRegion
  executable: Bool
  writable: Bool
  lifetime: CodeLifetime
}
```

### 8.4 Code Lifetime

```text
CodeLifetime =
  | UntilInvalidated
  | UntilModuleUnload
  | UntilVmShutdown
```

### 8.5 W^X Rule

If the platform supports it, code memory should not be writable and executable at the same time.

The spec does not require final code memory policy in Phase 3, but the backend interface must allow safe code memory management.

---

## 9. Runtime Helper Call ABI

### 9.1 Helper Call Site

```text
HelperCallSite {
  helper_id: RuntimeHelperId
  call_offset: CodeOffset
  safepoint_id?: SafepointId
  deopt_id?: DeoptId
  argument_map: HelperArgumentMap
  result_map: HelperResultMap
}
```

### 9.2 HelperArgumentMap

```text
HelperArgumentMap {
  arguments: List<HelperArgumentLocation>
}
```

```text
HelperArgumentLocation =
  | Register
  | StackSlot
  | Constant
  | RuntimeHandle
  | FrameRef
  | VmRef
```

### 9.3 Helper Result Handling

The JIT must handle helper returns:

```text
Value -> continue
VmControl::Raise -> enter unwind/deopt path
VmControl::Return/Break/Continue -> enter structured control path if helper can produce it
VmError -> abort compiled execution and report VM diagnostic
```

### 9.4 Helper Safepoint

If helper is safepoint, helper call site must have stack/root maps.

### 9.5 Helper ABI Rule

Helper ABI is internal.

Changing helper signature invalidates compiled code.

---

## 10. Safepoint and Stack Map Emission

### 10.1 JIT Safepoint Emission

For every compiled safepoint, backend emits:

```text
code offset
safepoint kind
live value locations
frame state
source span
deopt link if needed
```

### 10.2 StackMapTable

```text
StackMapTable {
  stack_maps: List<StackMap>
}
```

### 10.3 StackMap

```text
StackMap {
  stack_map_id: StackMapId
  code_offset: CodeOffset
  live_value_locations: List<ValueLocation>
  frame_state: FrameStateRef
  source_span?: SourceSpanId
}
```

### 10.4 ValueLocation

```text
ValueLocation =
  | Register
  | StackSlot
  | SpillSlot
  | RuntimeHandle
  | Immediate
  | Constant
```

Derived pointers are not recommended for baseline JIT.

If used, base object must be recorded.

### 10.5 Stack Map Correctness

If GC profile permits moving collection, every live heap reference must be updateable.

---

## 11. Deopt Emission

### 11.1 JitDeoptTable

```text
JitDeoptTable {
  deopt_points: List<JitDeoptRecord>
}
```

### 11.2 JitDeoptRecord

```text
JitDeoptRecord {
  deopt_id: DeoptId
  code_offset: CodeOffset
  source_eir_location: EirLocation
  frame_map: FrameMap
  local_slot_map: LocalSlotMap
  value_locations: List<ValueLocation>
  region_stack_state: RegionStackState
  pending_control_state?: PendingControlState
  resume_target: DeoptResumeTarget
}
```

### 11.3 DeoptResumeTarget

```text
DeoptResumeTarget =
  | EirInterpreterLocation
  | GenericEirFallback
  | FunctionEntry
  | UnwindHelper
```

### 11.4 Baseline Deopt Policy

Baseline JIT may minimize speculative deopt.

If a compiled operation uses no speculative assumption, it may not need deopt.

If it emits guard-based specialization, it must emit deopt or helper fallback.

### 11.5 Deopt Validation

JIT validation must reject compiled guards without valid failure paths.

---

## 12. Guard Lowering

### 12.1 Guard Lowering

EIR guards lower to backend checks.

Possible actions on failure:

```text
jump to helper fallback
jump to generic EIR fallback
deopt
raise language error
invalidate inline cache
```

### 12.2 Guard Types

Required guard support:

```text
type guard
shape guard
call target guard
capability guard
module state guard
readonly guard
overflow guard
division-by-zero guard
```

### 12.3 Semantic Guard vs Speculative Guard

Semantic guard failure raises language error.

Speculative guard failure falls back or deopts.

The backend must distinguish them.

---

## 13. Value Layout Interface

### 13.1 ValueLayoutProfile

JIT backend sees value representation only through `ValueLayoutProfile`.

It must not assume Rust enum layout.

### 13.2 Required Abstract Operations

Backend may request lowering hooks for:

```text
is_immediate
is_heap_ref
tag_of
unbox_int
box_int
unbox_float
box_float
load_obj_ref
compare_identity
```

### 13.3 Layout Non-Commitment

The VM may later change from:

```text
Rust enum
to tagged pointer
to NaN boxing
to compressed handles
```

without changing language semantics.

Such change invalidates compiled code.

---

## 14. GC Interface for JIT

### 14.1 JitGcInterface

```text
JitGcInterface {
  emit_safepoint
  emit_stack_map
  emit_write_barrier
  emit_allocation_call
  register_compiled_frame_layout
}
```

### 14.2 Allocation Lowering

Allocation may lower to:

```text
helper_alloc_object
specialized allocation helper
inline bump allocation with safepoint fallback
```

Baseline JIT may initially always call allocation helpers.

### 14.3 Write Barrier Lowering

Heap mutation must emit:

```text
inline barrier
or helper_write_barrier
```

If GC profile has no barrier need, barrier may lower to no-op.

The barrier call site must still exist in lowering logic.

### 14.4 Root Map Rule

Any compiled code that can reach allocation/helper safepoint must have valid root map.

---

## 15. Call Lowering

### 15.1 Generic Call

Generic call lowers to:

```text
helper_generic_call
```

initially.

### 15.2 Monomorphic Call Fast Path

If CallSite feedback is monomorphic:

```text
guard callee identity/function id
marshal arguments
call compiled function or interpreter entry
fallback helper on mismatch
```

### 15.3 Builtin Call

Builtin call may lower to helper call.

Inlining builtin is allowed only if it preserves:

```text
capability checks
error category
source mapping
safepoint behavior
```

### 15.4 Function Call Boundary

Function calls are safepoint candidates.

Compiled-to-compiled calls must preserve frame maps.

Compiled-to-interpreted calls must bridge argument and result representation.

---

## 16. Access Lowering

### 16.1 Record Field Read

Fast path:

```text
load receiver
guard shape
load field by FieldIndex
```

Fallback:

```text
helper_get_attribute
```

### 16.2 Record Field Write

Fast path:

```text
load receiver
guard shape
check readonly
check field mutability
check type contract if needed
write field
emit write barrier
```

Fallback:

```text
helper_set_attribute
```

### 16.3 Index Read/Write

List index access may use guarded fast path.

Map access usually uses helper until map layout is optimized.

String indexing is not core.

### 16.4 Method Read

Method read may use shape/method-table guard and construct bound method.

If construction allocates, root map is required.

---

## 17. Arithmetic Lowering

### 17.1 Int Arithmetic

Int arithmetic must check overflow if using fixed-width representation.

Overflow raises `NumericOverflowError`.

### 17.2 Division

Division must check zero divisor.

Zero divisor raises `DivisionByZeroError`.

### 17.3 Float Arithmetic

Float arithmetic follows VM float semantics.

### 17.4 Mixed Numeric Types

If mixed numeric promotion is not defined, backend must not invent one.

Unsupported combinations call helper or raise `TypeError`.

---

## 18. Control and Unwind Lowering

### 18.1 Return

Compiled return must:

```text
respect return contract
enter cleanup if active regions exist
return to caller only after cleanup
```

### 18.2 Raise

Compiled raise must enter unwind path or call unwind helper.

### 18.3 Break/Continue

Compiled break/continue must preserve target region and cleanup.

### 18.4 Try/Finally

Compiled code may not skip finally.

If lowering cannot compile correct finally semantics, function remains interpreted or calls helper.

### 18.5 Use/Defer

Compiled code may not skip resource close or defer execution.

Region/defer/resource state must be visible to unwinding helper.

---

## 19. Capability and Host Boundary

### 19.1 Capability Lowering

Capability checks may lower to:

```text
inline capability guard
or helper_check_capability
```

Missing capability raises `CapabilityError`.

### 19.2 Host Call

Host calls always go through helper/host boundary.

Compiled code must not directly call arbitrary host/native function pointers.

### 19.3 FFI

FFI remains out of scope.

Future FFI must use VM-controlled capability-gated boundary.

---

## 20. Code Cache

### 20.1 JitCacheKey

```text
JitCacheKey {
  eir_digest: Digest
  runtime_plan_digest: Digest
  helper_table_digest: Digest
  value_layout_profile_digest: Digest
  gc_profile_digest: Digest
  target_profile_digest: Digest
  backend_id: JitBackendId
  backend_version: Version
  compile_mode: BaselineCompileMode
}
```

### 20.2 Invalidation Reasons

Compiled code invalidates on:

```text
EIR change
RuntimePlan change
helper table change
value layout change
GC profile change
target profile change
backend version change
module interface incompatible change
capability environment incompatible change
shape invalidation if dynamic shapes later exist
deopt metadata schema change
stack map schema change
```

### 20.3 Cache Rule

Compiled code is discardable.

It is not package ABI.

It must not be loaded if cache key does not match current VM.

---

## 21. JIT Validation

### 21.1 JitValidationReport

```text
JitValidationReport {
  compiled_function_id: CompiledFunctionId
  valid: Bool
  diagnostics: List<Diagnostic>
}
```

### 21.2 Validation Checks

JIT validation must check:

```text
all safepoints have stack maps
all helper safepoints expose roots
all guards have valid failure path
all heap writes have barrier path
all may-raise calls have unwind path
all capability operations preserve checks
all compiled returns preserve cleanup
all deopt points have frame maps
all source spans map to SIR nodes
all helper IDs match helper table digest
all value layout assumptions match profile
```

### 21.3 Validation Failure

If validation fails, compiled function must not be installed.

VM falls back to EIR interpretation.

---

## 22. Installation and Dispatch

### 22.1 Installation

A compiled function may be installed only after:

```text
successful compilation
successful validation
cache key match
helper table match
GC profile match
target profile match
```

### 22.2 Dispatch

Function dispatch may choose:

```text
EIR interpreter
baseline compiled function
optimized compiled function
```

based on hotness, validity, and current runtime state.

### 22.3 Deoptimization Dispatch

On deopt:

```text
reconstruct frame
restore slots
restore region stack
restore pending control
resume EIR interpreter or unwind helper
```

---

## 23. Cranelift-Compatible Boundary

### 23.1 Recommended First Backend

A Cranelift-compatible backend is recommended for baseline JIT.

The VM-side interface should provide:

```text
EIR operation stream
value layout hooks
runtime helper signatures
safepoint emission requests
stack map emission requests
deopt metadata
relocation records
```

### 23.2 Boundary Rule

The spec must not depend on Cranelift-specific IR names or APIs.

Cranelift is an implementation target, not a semantic dependency.

### 23.3 Future LLVM Backend

LLVM ORC or another backend may be introduced later as optimizing or AOT backend.

It must consume the same VM semantic metadata.

---

## 24. Security and Safety

Compiled code must not:

```text
bypass capability checks
mutate read-only values
skip write barriers
call arbitrary native pointers
expose VM object layout
assume CPython ABI
assume public bytecode ABI
skip structured unwinding
hide language errors as VM internal errors
```

Any violation is a VM correctness bug.

---

## 25. Non-Goals

This document does not define:

```text
actual machine code generation
register allocation
instruction selection
concrete Cranelift API
concrete LLVM API
optimizing JIT
OSR implementation
native ABI
FFI
public bytecode
debugger protocol
profiler format
```

---

## 26. Next Work

Next Phase 3 documents should define:

```text
fast interpreter concrete data structures
runtime helper implementation plan
GC implementation staging plan
JIT lowering matrix per EIR operation
Phase 3 consistency audit

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-BASELINE-JIT-BACKEND-INTERFACE.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md -->


# Phase 3 · Fast Interpreter Concrete Data Structures
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.11 interpreter-structure draft  
Depends on: Phase 3 Baseline JIT Backend Interface v0.10  
Depends on: Phase 3 GC Root Enumeration and Safepoint Model v0.9  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: concrete fast-interpreter runtime structures, call stack, frames, slot arrays, region stack, instruction pointer, block dispatch, feedback/cache storage, helper transition, safepoint polling, interpreter/JIT shared metadata  
Out of scope: concrete Rust implementation, concrete byte encoding, concrete threaded-dispatch implementation, concrete Cranelift lowering, optimizing JIT, public bytecode

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

This document defines the concrete execution structures for the EIR fast interpreter.

The fast interpreter is the production non-JIT execution target.

It consumes:

```text
RuntimePlan
EIR
RuntimeHelperTable
SafepointTable
RootMap
FrameMap
FeedbackTable
InlineCacheTable
```

It must preserve the same execution metadata used by the baseline JIT.

The fast interpreter must not be a naive SIR-walk evaluator.

---

## 1. Design Goals

The fast interpreter must provide:

```text
low dispatch overhead
slot-based local access
preplanned field/case access
explicit region stack
explicit pending control
safepoint polling
root visibility
helper-call transition
feedback collection
inline-cache storage
JIT-compatible frame metadata
diagnostic source mapping
```

The fast interpreter must avoid:

```text
textual name lookup in hot path
HashMap locals in hot path
SIR table traversal in hot path
dynamic string field lookup for fixed records
Rc/RefCell proliferation in every hot operation
Rust panic for language control flow
hidden raw object pointers
```

---

## 2. FastInterpreter

### 2.1 FastInterpreter

```text
FastInterpreter {
  vm: VmRef
  runtime_plan: RuntimePlanRef
  eir_module: EirModuleRef
  call_stack: InterpreterCallStack
  dispatch_state: DispatchState
  feedback_store: FeedbackStore
  inline_cache_store: InlineCacheStore
  safepoint_poller: SafepointPoller
  helper_bridge: HelperBridge
  diagnostics: DiagnosticSink
}
```

### 2.2 VM Reference

`VmRef` gives controlled access to:

```text
heap
module environment
capability environment
runtime helper table
GC interface
diagnostic engine
host boundary
```

The interpreter must not bypass these VM services.

### 2.3 Interpreter State Rule

Interpreter state is VM-internal.

It must not be exposed as language ABI.

It must be inspectable by GC, diagnostics, and future debugger/profiler layers through controlled APIs.

---

## 3. DispatchState

### 3.1 DispatchState

```text
DispatchState {
  current_frame: FrameId
  current_function: EirFunctionId
  current_block: EirBlockId
  instruction_index: UInt
  pending_jump?: PendingJump
  current_source_span?: SourceSpanId
}
```

### 3.2 Instruction Pointer

The interpreter instruction pointer is:

```text
(EirFunctionId, EirBlockId, instruction_index)
```

It is not a public bytecode address.

### 3.3 EirLocation

```text
EirLocation {
  eir_function_id: EirFunctionId
  block_id: EirBlockId
  op_index?: UInt
  terminator: Bool
}
```

EirLocation is used by:

```text
diagnostics
safepoints
root maps
deopt metadata
profiling
debug mode
```

### 3.4 PendingJump

```text
PendingJump {
  target_block: EirBlockId
  arguments: List<SlotId>
}
```

PendingJump is used for block argument transfer.

---

## 4. InterpreterCallStack

### 4.1 InterpreterCallStack

```text
InterpreterCallStack {
  frames: Vec<InterpreterFrame>
  max_depth: UInt
}
```

### 4.2 Stack Depth

The VM must enforce a maximum logical call depth.

If exceeded, it raises:

```text
StackOverflowError
```

or equivalent VM runtime error category if Phase 1 has not yet named it.

### 4.3 Logical vs Host Stack

The interpreter may use Rust call stack or explicit trampoline internally.

But it must maintain a logical call stack for:

```text
diagnostics
GC roots
deoptimization
debugging
JIT transition
```

### 4.4 Stack Walking

The call stack must be walkable at safepoints.

Stack walking must expose:

```text
frame id
function id
module id
EIR location
source span
slot roots
region roots
pending control
```

---

## 5. InterpreterFrame

### 5.1 InterpreterFrame

```text
InterpreterFrame {
  frame_id: FrameId
  frame_kind: FrameKind
  module_id: ModuleId
  function_id?: FunctionId
  eir_function_id: EirFunctionId
  slots: SlotArray
  region_stack: RegionStack
  frame_map: FrameMapId
  return_target?: ReturnTarget
  caller_location?: EirLocation
  current_source_span?: SourceSpanId
  pending_control_slot?: SlotId
}
```

### 5.2 FrameKind

```text
FrameKind =
  | ModuleInit
  | Function
  | BuiltinAdapter
  | Test
  | HelperInternal
```

### 5.3 ModuleInit Frame

A module initialization frame executes synthetic module initialization EIR.

It cannot accept source-level return/break/continue.

### 5.4 Function Frame

A function frame owns:

```text
parameter slots
local slots
temporary slots
capture slots
region stack
return target
```

### 5.5 BuiltinAdapter Frame

A builtin adapter frame may be used when a builtin needs language-visible stack trace context.

Pure internal helper frames may be hidden from source stack traces.

### 5.6 Test Frame

A test frame is used by the test runner.

Test execution is not ordinary module initialization.

---

## 6. SlotArray

### 6.1 SlotArray

```text
SlotArray {
  slots: Vec<SlotState>
  layout: SlotLayoutRef
  initialized_bitmap?: BitSet
  heap_ref_bitmap?: BitSet
}
```

### 6.2 SlotState

```text
SlotState =
  | Uninitialized
  | Value(Value)
  | Cell(BindingCellRef)
  | RuntimeInternal(RuntimeValue)
```

### 6.3 Slot Access

Slot access is by `SlotId`.

No ordinary variable access uses text lookup.

### 6.4 Initialization Tracking

The interpreter may use:

```text
SlotState tags
initialized_bitmap
validation-derived initialization facts
```

Uninitialized source binding read raises `UninitializedBindingError`.

Uninitialized internal temporary read is VM bug or EIR validation failure.

### 6.5 Heap Reference Tracking

`heap_ref_bitmap` may accelerate root enumeration.

If value layout can distinguish heap refs cheaply, bitmap may be omitted.

Moving GC requires every live heap reference location to be updateable.

### 6.6 Slot Write Protocol

Slot writes must perform:

```text
mutability check where required
type contract check where attached
write barrier if heap reference mutation occurs
heap_ref tracking update
diagnostic source span preservation
```

### 6.7 Temporary Reuse

Temporaries may be reused only if liveness analysis proves no semantic effect.

Debug mode may disable aggressive reuse for better diagnostics.

---

## 7. BindingCell Runtime

### 7.1 BindingCellRef

```text
BindingCellRef {
  cell_id: CellId
}
```

### 7.2 BindingCell

```text
BindingCell {
  binding_id: BindingId
  state: BindingState
  mutability: BindingMutability
  type_contract?: TypeId
  owner: CellOwner
}
```

### 7.3 BindingState

```text
BindingState =
  | Uninitialized
  | Initialized(Value)
```

### 7.4 Cell Access

Cell read:

```text
check initialized
return value
```

Cell write:

```text
check mutability
check type contract
write value
run write barrier
```

### 7.5 Cell Root

Every live cell is traceable by GC.

Cells in closure captures or module exports may outlive frames.

---

## 8. RegionStack

### 8.1 RegionStack

```text
RegionStack {
  regions: Vec<RuntimeRegionFrame>
}
```

### 8.2 RuntimeRegionFrame

```text
RuntimeRegionFrame {
  region_id: ControlRegionId
  region_kind: ControlRegionKind
  owner_node: NodeId
  cleanup_state: CleanupState
  loop_target?: LoopTarget
  match_state?: MatchState
  try_state?: TryState
  source_span?: SourceSpanId
}
```

### 8.3 CleanupState

```text
CleanupState {
  defer_stack: Vec<DeferredCallable>
  resource_stack: Vec<ResourceCleanup>
  finally_state?: FinallyState
  cleanup_progress: CleanupProgress
}
```

### 8.4 CleanupProgress

```text
CleanupProgress =
  | NotStarted
  | RunningDefers
  | RunningResources
  | RunningFinally
  | Complete
```

CleanupProgress is required for deopt, diagnostics, and interruption safety.

### 8.5 LoopTarget

```text
LoopTarget {
  break_target: EirBlockId
  continue_target: EirBlockId
  loop_region: ControlRegionId
}
```

### 8.6 Region Stack Rule

All region stack entries must be visible to:

```text
unwind helper
GC root enumeration
deopt reconstruction
debug diagnostics
```

---

## 9. PendingControl Storage

### 9.1 PendingControlSlot

A frame may reserve a hidden slot for pending control.

```text
PendingControlSlot {
  slot_id: SlotId
}
```

### 9.2 PendingControlValue

```text
PendingControlValue =
  | Normal
  | Return(Value)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
```

### 9.3 PendingControl Rule

Pending control is not a source value.

It is VM-internal control state.

It must be traceable if it contains heap values.

### 9.4 Unwind Compatibility

Unwind helper reads and updates pending control.

Finally override writes new pending control.

Cleanup errors may replace or augment pending control according to Phase 2 semantics.

---

## 10. Block Dispatch

### 10.1 Block Execution

Block execution is:

```text
fetch current block
execute operations sequentially
execute terminator
dispatch terminator result
```

### 10.2 Operation Dispatch

Implementation options:

```text
match over EirOp
function pointer table
threaded dispatch where safe
quickened operation variants
```

The specification does not require one dispatch mechanism.

### 10.3 No Fallthrough

A block must explicitly terminate.

Interpreter must reject block fallthrough.

### 10.4 Block Arguments

Block argument transfer must be simultaneous.

Recommended implementation:

```text
copy jump argument values into temporary transfer buffer
then assign block parameter slots
```

This prevents clobbering.

### 10.5 Dispatch Diagnostics

On operation failure, current EirLocation and SourceSpan are captured.

---

## 11. Terminator Dispatch

### 11.1 TerminatorResult

```text
TerminatorResult =
  | NextBlock(EirBlockId)
  | Return(Value)
  | Raise(ErrorHandle)
  | Unwind(PendingControlValue)
  | Deopt(DeoptId)
  | Halt
```

### 11.2 NextBlock

Updates current block and resets instruction index.

### 11.3 Return

If active cleanup exists, return is converted to pending control and enters unwind.

If no active cleanup exists, frame returns to caller.

### 11.4 Raise

Raise enters unwind path.

If no handler exists, it propagates to caller.

### 11.5 Halt

Halt is valid only for VM-internal completion contexts.

Source program termination should be represented as module/test/function completion, not arbitrary halt.

---

## 12. FeedbackStore

### 12.1 FeedbackStore

```text
FeedbackStore {
  slots: Vec<FeedbackSlotRuntime>
}
```

### 12.2 FeedbackSlotRuntime

```text
FeedbackSlotRuntime {
  feedback_slot_id: FeedbackSlotId
  kind: FeedbackKind
  state: FeedbackState
  payload: FeedbackPayload
  update_count: UInt
}
```

### 12.3 FeedbackPayload

```text
FeedbackPayload =
  | Empty
  | TypeFeedbackPayload
  | ShapeFeedbackPayload
  | CallTargetFeedbackPayload
  | BranchFeedbackPayload
  | AllocationFeedbackPayload
  | ErrorFeedbackPayload
```

### 12.4 Feedback Update Rule

Feedback update must not change observable semantics.

Feedback may be disabled in deterministic testing mode if required.

### 12.5 Hotness Counters

Hotness counters may be stored as feedback payload or separate counter table.

Hotness events:

```text
function entry
loop backedge
call site execution
guard failure
allocation count
```

---

## 13. InlineCacheStore

### 13.1 InlineCacheStore

```text
InlineCacheStore {
  slots: Vec<InlineCacheRuntimeSlot>
}
```

### 13.2 InlineCacheRuntimeSlot

```text
InlineCacheRuntimeSlot {
  inline_cache_slot_id: InlineCacheSlotId
  state: InlineCacheState
  payload: InlineCacheRuntimePayload
  invalidation_epoch: UInt
}
```

### 13.3 Runtime Payloads

```text
InlineCacheRuntimePayload =
  | Empty
  | CallCacheRuntime
  | AttributeCacheRuntime
  | IndexCacheRuntime
  | TypeCheckCacheRuntime
```

### 13.4 Cache Epoch

Each cache slot has an invalidation epoch.

If global or module epoch changes, cache may become stale.

### 13.5 Cache Update Rule

Cache transitions:

```text
Uninitialized -> Monomorphic
Monomorphic -> Polymorphic
Polymorphic -> Megamorphic
any -> Disabled
```

Cache update must be atomic with respect to interpreter reentrancy if reentrancy is allowed.

---

## 14. Call Cache Runtime

### 14.1 CallCacheRuntime

```text
CallCacheRuntime {
  observed_targets: Vec<ObservedCallTarget>
  arity_shape: ArityShape
  miss_count: UInt
}
```

### 14.2 ObservedCallTarget

```text
ObservedCallTarget {
  callee_kind: CallableKind
  function_id?: FunctionId
  builtin_id?: BuiltinFunctionId
  shape_id?: ShapeId
  compiled_function_id?: CompiledFunctionId
}
```

### 14.3 Call Cache Use

A call cache may skip generic lookup only if guards pass.

Fallback helper is required.

### 14.4 Compiled Target

If compiled target exists, interpreter dispatch may call compiled function through VM call bridge.

The bridge must preserve frame/root/safepoint metadata.

---

## 15. Access Cache Runtime

### 15.1 AttributeCacheRuntime

```text
AttributeCacheRuntime {
  receiver_shape: ShapeId
  field_index?: FieldIndex
  method_index?: MethodIndex
  miss_count: UInt
}
```

### 15.2 IndexCacheRuntime

```text
IndexCacheRuntime {
  receiver_kind: RuntimeValueKind
  key_kind?: RuntimeValueKind
  strategy: IndexStrategy
  miss_count: UInt
}
```

### 15.3 IndexStrategy

```text
IndexStrategy =
  | ListIntIndex
  | MapHashLookup
  | SliceList
  | SliceString
  | GenericHelper
```

### 15.4 Access Cache Rule

Access cache must respect:

```text
readonly checks
shape checks
field mutability
type contracts
write barriers
error categories
```

---

## 16. SafepointPoller

### 16.1 SafepointPoller

```text
SafepointPoller {
  poll_epoch: UInt
  gc_requested: Bool
  interrupt_requested: Bool
  profiling_requested: Bool
  debug_trap_requested: Bool
}
```

### 16.2 Poll Sites

The interpreter polls at:

```text
loop backedge
function call
helper call
allocation
host call
module import
long-running builtin
debug-inserted poll
```

### 16.3 Poll Result

```text
PollResult =
  | Continue
  | RunGc
  | Interrupt
  | Cancel
  | DebugTrap
  | ProfileSample
```

### 16.4 Poll Rule

Polling must occur only where root maps can be constructed.

If no root map is available, poll must be delayed to next valid safepoint.

---

## 17. HelperBridge

### 17.1 HelperBridge

```text
HelperBridge {
  helper_table: RuntimeHelperTableRef
  current_helper_call?: HelperCallState
}
```

### 17.2 HelperCallState

```text
HelperCallState {
  helper_id: RuntimeHelperId
  arguments: Vec<Value>
  source_span?: SourceSpanId
  safepoint_id?: SafepointId
  roots_visible: Bool
}
```

### 17.3 Helper Transition

Before calling helper:

```text
validate helper descriptor
marshal arguments
make roots visible if required
perform capability check if required
enter helper call state
call helper
normalize result
exit helper call state
```

### 17.4 Helper Result Normalization

Helper returns are normalized to:

```text
Value
VmControl
VmError
```

Language errors become `VmControl::Raise`.

Structural VM errors become `VmError`.

---

## 18. Interpreter/JIT Bridge

### 18.1 Bridge Purpose

The interpreter may call compiled functions.

Compiled functions may deopt or call back into interpreter.

### 18.2 Interpreter to JIT

Interpreter-to-JIT call requires:

```text
argument representation conversion
frame setup or compiled frame bridge
root visibility at call boundary
safepoint metadata
return value normalization
error/control propagation
```

### 18.3 JIT to Interpreter

JIT-to-interpreter fallback requires:

```text
frame reconstruction
slot reconstruction
region stack reconstruction
pending control reconstruction
source location reconstruction
```

### 18.4 Bridge Rule

Interpreter and JIT share:

```text
RuntimePlan
EIR
FrameMap
RootMap
SafepointMap
Deopt metadata
RuntimeHelperTable
```

They must not implement divergent semantics.

---

## 19. Source Mapping Runtime

### 19.1 SourceLocationState

```text
SourceLocationState {
  current_eir_location: EirLocation
  current_node_id?: NodeId
  current_source_span?: SourceSpanId
}
```

### 19.2 Source Mapping Rule

Before executing an op that may raise or call helper, interpreter updates current source mapping.

### 19.3 Diagnostic Capture

On error, diagnostics capture:

```text
current source span
EIR location
SIR NodeId
frame stack
helper context if relevant
```

### 19.4 Helper Source Mapping

Helper calls receive source span or source mapping reference when they may raise.

---

## 20. Root Enumeration for Interpreter

### 20.1 InterpreterRootEnumerator

```text
InterpreterRootEnumerator {
  call_stack: InterpreterCallStackRef
  module_environment: ModuleEnvironmentRef
  helper_bridge: HelperBridgeRef
  host_roots: HostRootRegistryRef
}
```

### 20.2 Enumeration Steps

At safepoint:

1. enumerate frames
2. enumerate initialized live slots
3. enumerate cell/capture roots
4. enumerate region stack
5. enumerate pending control
6. enumerate helper arguments
7. enumerate module roots
8. enumerate host roots
9. emit RootSet

### 20.3 Live Slot Mode

Modes:

```text
AllInitializedSlots
LivenessDerivedSlots
DebugAllSlots
```

Bootstrap may use `AllInitializedSlots`.

Production should prefer `LivenessDerivedSlots`.

### 20.4 Root Update

If moving GC occurs, root enumerator must update locations.

Values must be written back to their slots/cells/root locations.

---

## 21. FrameMap Runtime Use

### 21.1 FrameMap Access

Each InterpreterFrame references FrameMapId.

FrameMap provides:

```text
visible bindings
slot mapping
source span mapping
region state mapping
deopt reconstruction metadata
```

### 21.2 Debug Inspection

Debug inspection uses FrameMap, not raw slot layout.

### 21.3 Stack Trace

Stack trace uses:

```text
function name
module name
source span
call site
```

Internal helper frames may be elided unless debug mode includes VM internals.

---

## 22. Operation Execution Contract

### 22.1 execute_op

Conceptual:

```text
execute_op(state, op) -> OpResult
```

### 22.2 OpResult

```text
OpResult =
  | Continue
  | Raise(ErrorHandle)
  | Deopt(DeoptId)
  | InternalError(VmError)
```

### 22.3 Contract

`execute_op` must:

```text
read only declared slots
write only declared dest slots
preserve source mapping
update feedback if needed
respect barrier hooks
respect helper descriptors
not modify instruction pointer except through defined protocol
```

---

## 23. Terminator Execution Contract

### 23.1 execute_terminator

```text
execute_terminator(state, terminator) -> TerminatorResult
```

### 23.2 Contract

`execute_terminator` must:

```text
check Bool for Branch
perform block argument transfer for Jump
mark loop backedge safepoint for LoopBackedge
enter unwind for Return/Raise/Break/Continue
reject invalid Unreachable
```

### 23.3 Terminator Source Mapping

Terminator diagnostics use terminator source span or owner node span.

---

## 24. Reentrancy and Host Calls

### 24.1 Reentrancy

The first VM may forbid reentrant execution except through explicitly supported host calls.

If reentrancy is allowed, interpreter state mutation must be protected.

### 24.2 Host Call State

Before host call:

```text
roots visible
capability checked
helper state entered
source mapping saved
```

After host call:

```text
result normalized
host errors converted
host roots released or transferred
state restored
```

### 24.3 Reentrancy Validation

If a host call reenters VM, nested call stack must be represented explicitly.

---

## 25. Interrupt and Cancellation

### 25.1 Cancellation

Cancellation may be delivered at safepoints.

Cancellation should become a VM-controlled interrupt/error category, not Rust panic.

### 25.2 Interrupt Safety

Interrupt cannot occur at arbitrary unsafe point.

It must occur only at safepoints or explicit poll points.

### 25.3 Cleanup on Cancellation

If cancellation behaves like Raise, structured unwinding must run.

If cancellation is host abort, VM must document cleanup guarantee.

---

## 26. Determinism Modes

### 26.1 Deterministic Execution Mode

A deterministic mode may disable:

```text
profile-dependent quickening
nondeterministic cache updates
adaptive JIT installation
timing-dependent optimization
```

Semantic results must remain same in all modes.

### 26.2 Test Mode

Test mode should prioritize:

```text
stable diagnostics
stable source mapping
clear stack traces
predictable assertion behavior
```

---

## 27. Validation

Fast interpreter structure validation must reject:

```text
frame without slot layout
frame without frame map
slot array size mismatch
invalid current block
instruction index out of range
block without terminator
region stack entry without RegionPlan
pending control slot missing where needed
helper call without descriptor
safepoint poll without root map
feedback slot kind mismatch
inline cache slot kind mismatch
JIT bridge without compatible metadata
source mapping missing for may-raise op where required
```

---

## 28. Compatibility

Changing any of the following invalidates RuntimePlan/EIR/JIT caches:

```text
SlotArray layout
Frame layout
RegionStack layout
InlineCache runtime payload layout
Feedback payload layout
SafepointPoller protocol
HelperBridge ABI
Interpreter/JIT bridge protocol
FrameMap runtime format
Root enumeration mode
```

These are VM-internal compatibility boundaries.

They are not public ABI.

---

## 29. Non-Goals

This document does not define:

```text
final Rust struct definitions
binary EIR encoding
computed-goto implementation
threaded-dispatch implementation
concrete GC implementation
concrete JIT backend implementation
debugger protocol
profiler format
native ABI
public bytecode
```

---

## 30. Next Work

Next Phase 3 documents should define:

```text
runtime helper implementation plan
GC implementation staging plan
JIT lowering matrix per EIR operation
fast interpreter implementation milestones
Phase 3 consistency audit

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md -->


---

<!-- BEGIN NORMATIVE DOCUMENT: PHASE-3-JIT-LOWERING-MATRIX.md -->


# Phase 3 · JIT Lowering Matrix per EIR Operation
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.14 JIT-lowering draft  
Depends on: Phase 3 GC Implementation Staging Plan v0.13  
Depends on: Phase 3 Baseline JIT Backend Interface v0.10  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: JIT lowering classes for EIR operations, helper fallback requirements, safepoint/root/barrier/deopt requirements, baseline JIT gating, validation matrix  
Out of scope: concrete Cranelift IR, concrete LLVM IR, final machine-code generation, register allocation, optimizing JIT, public bytecode

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

This document classifies EIR operations for baseline JIT lowering.

It does not implement a JIT backend.

It defines which EIR operations can be compiled directly, which need guards, which must call helpers, which require deopt metadata, and which must remain interpreted.

The purpose is to prevent future baseline JIT implementation from silently bypassing:

```text
GC roots
safepoints
write barriers
capability checks
structured unwinding
source diagnostics
deoptimization
```

---

## 1. Lowering Classes

### 1.1 JitLoweringClass

```text
JitLoweringClass =
  | DirectLowering
  | GuardedFastPathWithHelperFallback
  | AlwaysCallHelper
  | DeoptRequired
  | InterpreterOnly
  | Forbidden
```

### 1.2 DirectLowering

The backend may emit direct machine/backend operations.

Requirements:

```text
no helper required
no semantic fallback required
all failure cases are locally checked
root metadata emitted if safepoint possible
```

### 1.3 GuardedFastPathWithHelperFallback

The backend emits:

```text
guard
fast path
fallback helper or generic EIR fallback
```

Requirements:

```text
guard failure path
source span
helper call metadata if helper fallback
deopt metadata if deopt fallback
root map if helper may allocate/collect
```

### 1.4 AlwaysCallHelper

The backend must call a runtime helper.

Use when the operation is:

```text
semantic-heavy
host/capability-sensitive
unwinding-sensitive
allocation/GC-sensitive
generic or megamorphic
```

### 1.5 DeoptRequired

The backend may compile only if a valid deopt point exists.

Use when the operation relies on speculative assumptions that cannot be handled by simple helper fallback.

### 1.6 InterpreterOnly

The operation remains in EIR interpreter.

The function may:

```text
remain interpreted
be split if backend supports partial compilation
call interpreter bridge
```

### 1.7 Forbidden

The operation must never be compiled.

If present in JIT input, compilation fails.

---

## 2. Matrix Columns

Each operation family is classified with:

```text
lowering_class
helper_fallback
safepoint_required
root_map_required
barrier_required
deopt_required
capability_sensitive
may_raise
may_allocate
notes
```

### 2.1 `may_raise`

Means operation can produce language-level Raise.

### 2.2 `may_allocate`

Means operation can allocate or call helper that allocates.

### 2.3 `root_map_required`

Required when operation reaches safepoint, helper call, allocation, or deopt.

### 2.4 `barrier_required`

Required for heap reference writes.

### 2.5 `capability_sensitive`

Operation must preserve capability check.

---

## 3. Constant Operations

### 3.1 ConstantOp

```text
lowering_class: DirectLowering
helper_fallback: none
safepoint_required: no
root_map_required: no
barrier_required: no
deopt_required: no
capability_sensitive: no
may_raise: no, except invalid constant pool is VM error
may_allocate: no, if constants are preallocated/interned
```

If loading a heap-backed constant lazily allocates, it becomes:

```text
AlwaysCallHelper or GuardedFastPathWithHelperFallback
```

with allocation safepoint and root map.

### 3.2 String Constant

Preferred baseline rule:

```text
module initialization preallocates or interns string constants
JIT loads handle
```

Lazy allocation is allowed only through allocation helper.

---

## 4. Load Operations

### 4.1 LoadSlot

```text
lowering_class: DirectLowering
may_raise: only if uninitialized check is required
```

If source slot may be uninitialized, emit check.

Uninitialized source binding raises `UninitializedBindingError`.

Internal uninitialized temporary is VM error.

### 4.2 LoadCell

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: optional helper_load_cell
may_raise: yes
```

Fast path:

```text
load cell
check initialized
load value
```

Failure:

```text
UninitializedBindingError
```

### 4.3 LoadCapture

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
```

Direct if capture layout is stable and initialized facts hold.

Otherwise guard/check.

### 4.4 LoadModuleSlot

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_load_module_slot or helper_import_named
may_raise: yes
```

Must preserve:

```text
module state checks
ImportCycleError
UninitializedBindingError
```

### 4.5 LoadField

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_get_attribute
may_raise: yes
```

Fast path:

```text
load receiver
unwrap readonly view if read
guard record shape
load field index
```

Requires no allocation unless fallback creates diagnostics.

### 4.6 LoadEnumPayload

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: pattern helper or enum payload helper
may_raise: context-dependent
```

Fast path:

```text
guard enum shape
guard case index
load payload index
```

In match-pattern context, failure branches instead of raising.

---

## 5. Store Operations

### 5.1 StoreSlot

```text
lowering_class: DirectLowering
barrier_required: if slot stores into heap-reachable cell/module storage
```

Direct local temporary write generally needs no barrier.

Writing into a source-visible cell uses StoreCell rules.

### 5.2 StoreCell

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_store_cell or helper_write_barrier
may_raise: yes
barrier_required: yes if heap ref write possible
```

Fast path must check:

```text
mutability
type contract if attached
write barrier
```

### 5.3 StoreModuleSlot

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: module slot helper
may_raise: yes
barrier_required: yes
```

Must respect module initialization/export state.

### 5.4 StoreField

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_set_attribute
may_raise: yes
barrier_required: yes
```

Fast path checks:

```text
readonly
record shape
field mutability
field type contract
write barrier
```

### 5.5 StoreListIndex

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_index_write
may_raise: yes
barrier_required: yes
```

Fast path supported for:

```text
List[Int existing index]
```

### 5.6 StoreMapEntry

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_index_write or helper_construct_map path
may_raise: yes
barrier_required: yes
may_allocate: possible
```

Reason:

```text
hashing
key equality
entry insertion/replacement
order preservation
possible table resize
```

A later optimized map layout may introduce guarded fast path.

---

## 6. Unary Operations

### 6.1 Unary Plus

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_numeric_unary or helper_numeric_binary family
may_raise: yes
```

Direct lowering only after type is known numeric.

### 6.2 Unary Minus

```text
lowering_class: GuardedFastPathWithHelperFallback
may_raise: yes
```

Fast path:

```text
int neg with overflow check
float neg
```

Unsupported types fallback/raise `TypeError`.

### 6.3 Not

```text
lowering_class: DirectLowering
may_raise: yes if operand not proven Bool
```

Emit Bool check.

No truthiness.

---

## 7. Binary Operations

### 7.1 Int Arithmetic

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_numeric_binary
may_raise: yes
deopt_required: no if helper fallback exists
```

Fast path:

```text
guard both Int
perform checked op
overflow -> raise NumericOverflowError or helper
division by zero -> raise DivisionByZeroError
```

### 7.2 Float Arithmetic

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_numeric_binary
may_raise: yes for unsupported types/division rules
```

### 7.3 String Add

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_string_concat or helper_numeric_binary equivalent
may_allocate: yes
safepoint_required: yes
root_map_required: yes
```

Could later get specialized allocation fast path.

### 7.4 List Add

```text
lowering_class: AlwaysCallHelper initially
may_allocate: yes
root_map_required: yes
```

Only if list concatenation is included in language semantics.

### 7.5 Equality

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_compare
may_raise: generally no, unless equality protocol can raise later
```

Fast paths:

```text
immediate equality
object identity shortcut
shape-known enum/record structural path if defined
```

### 7.6 Identity

```text
lowering_class: DirectLowering
may_raise: no
```

Uses VM identity, not physical address.

### 7.7 Comparisons

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_compare
may_raise: yes
```

Unsupported comparisons raise `TypeError`.

### 7.8 Membership

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_membership
may_raise: yes
```

Reason:

```text
list scan
map key hash/equality
future protocol possibility
```

---

## 8. Logical Operations

### 8.1 Logical And/Or

Preferred lowering is control flow.

```text
lowering_class: DirectLowering for branch structure
may_raise: yes if Bool check fails
```

Rules:

```text
short-circuit preserved
Bool-only
result is Bool
no operand-return semantics
```

A single eager LogicalOp is forbidden unless operands are already evaluated according to lowered short-circuit structure.

---

## 9. Check Operations

### 9.1 CheckBool

```text
lowering_class: DirectLowering
may_raise: yes
```

Checks tag/kind equals Bool.

### 9.2 CheckType

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_type_contract
may_raise: yes
```

Fast path for:

```text
ImmediateKindCheck
ShapeCheck
OptionalCheck simple
```

Complex union/function/extension checks call helper.

### 9.3 CheckCallable

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_callable
may_raise: yes
```

### 9.4 CheckArity

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_check_arity or generic call helper
may_raise: yes
```

Can later specialize for monomorphic calls.

### 9.5 CheckHashable

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_hashable
may_raise: yes
```

### 9.6 CheckReadonly

```text
lowering_class: DirectLowering
may_raise: yes
```

### 9.7 CheckCapability

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_capability
capability_sensitive: yes
may_raise: yes
```

JIT must never eliminate capability check unless capability state is proven immutable and guard/deopt is present.

### 9.8 CheckShape

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
helper_fallback: helper_check_shape
may_raise: context-dependent
```

Direct shape equality is preferred.

### 9.9 CheckOverflow / CheckDivisionByZero

```text
lowering_class: DirectLowering
may_raise: yes
```

Semantic checks, not speculative guards.

---

## 10. Call Operations

### 10.1 Generic CallOp

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_generic_call
safepoint_required: yes
root_map_required: yes
may_raise: yes
may_allocate: possible
```

### 10.2 Monomorphic User Function Call

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_generic_call
safepoint_required: yes
root_map_required: yes
deopt_required: optional
```

Fast path:

```text
guard callee identity/function id
marshal args
call compiled/interpreter function entry
fallback on mismatch
```

### 10.3 Builtin Call

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_call_builtin
capability_sensitive: maybe
safepoint_required: descriptor-dependent
```

Inlining allowed only with descriptor-preserving checks.

### 10.4 Constructor Call

Record/enum constructor calls may lower as construction ops.

Generic constructor value call uses helper.

### 10.5 Method Call

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_generic_call or helper_bind_method
```

Must preserve receiver binding and source diagnostics.

### 10.6 HostFunction Call

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_enter_host_call + helper_exit_host_call
capability_sensitive: yes
safepoint_required: yes
root_map_required: yes
```

Compiled code must not directly call arbitrary host pointers.

---

## 11. Access Operations

### 11.1 AttributeRead

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_get_attribute
may_raise: yes
```

Fast path for known record shape.

### 11.2 AttributeWrite

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_set_attribute
may_raise: yes
barrier_required: yes
```

### 11.3 MethodRead

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_bind_method
may_allocate: possible
root_map_required: if allocation possible
```

### 11.4 IndexRead

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_index_read
may_raise: yes
```

Fast path for List[Int].

Map read helper initially.

### 11.5 IndexWrite

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_index_write
barrier_required: yes
may_raise: yes
```

Fast path for List[Int].

Map write helper initially.

### 11.6 SliceRead

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_slice_read
may_allocate: yes
safepoint_required: yes
root_map_required: yes
```

---

## 12. Construction Operations

### 12.1 ConstructList

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: allocation helper or construct_list helper
may_allocate: yes
safepoint_required: yes
root_map_required: yes
```

Later direct allocation fast path allowed.

### 12.2 ConstructMap

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_construct_map
may_allocate: yes
safepoint_required: yes
root_map_required: yes
may_raise: yes
```

Needs hashability, duplicate-key semantics, order preservation.

### 12.3 ConstructRecord

```text
lowering_class: GuardedFastPathWithHelperFallback or AlwaysCallHelper initially
helper_fallback: helper_construct_record
may_allocate: yes
safepoint_required: yes
root_map_required: yes
may_raise: yes
```

Direct fast path possible for known shape and prechecked fields.

### 12.4 ConstructEnumValue

```text
lowering_class: GuardedFastPathWithHelperFallback or AlwaysCallHelper initially
helper_fallback: helper_construct_enum
may_allocate: yes
root_map_required: yes
may_raise: yes
```

### 12.5 ConstructFunction

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: alloc_function/helper_construct_function
may_allocate: yes
root_map_required: yes
```

Must preserve declaration-time construction and capture semantics.

### 12.6 ConstructError

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_construct_error
may_allocate: yes
root_map_required: yes
```

---

## 13. Pattern Operations

### 13.1 PatternCheckLiteral

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern or helper_compare
may_raise: context-dependent
```

In match context, failure branches.

In destructuring context, failure raises.

### 13.2 PatternCheckRecordShape

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern
```

### 13.3 PatternCheckEnumCase

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern
```

### 13.4 PatternCheckListLength

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern
```

### 13.5 PatternCheckMapKey

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_match_pattern
```

Map key checks require hashing/equality.

### 13.6 PatternBind

```text
lowering_class: DirectLowering
barrier_required: if binding slot/cell write stores heap ref
```

Binding commit must preserve pattern success semantics.

### 13.7 PatternBranch

```text
lowering_class: DirectLowering
```

---

## 14. RuntimeHelperOp

### 14.1 RuntimeHelperOp

```text
lowering_class: AlwaysCallHelper
safepoint_required: descriptor-dependent
root_map_required: if helper may allocate/collect/raise/unwind
capability_sensitive: descriptor-dependent
```

The JIT must use helper descriptor exactly.

### 14.2 Helper That May Collect

Must have:

```text
SafepointRecord
RootMap
StackMap
HelperCallSite metadata
```

### 14.3 Helper That May Raise

Must have:

```text
source span
unwind path
FrameMap
```

### 14.4 Helper That May Unwind

Must have:

```text
region stack metadata
pending control metadata
deopt/unwind bridge
```

---

## 15. SafepointOp

### 15.1 SafepointOp

```text
lowering_class: DirectLowering
safepoint_required: yes
root_map_required: yes
```

Emits poll sequence or safepoint call.

### 15.2 Fast Poll

Loop safepoints may use fast epoch check.

Slow path calls VM safepoint handler.

### 15.3 GC Compatibility

If root map unavailable, JIT compilation must fail.

---

## 16. GuardOp

### 16.1 Semantic Guards

```text
lowering_class: DirectLowering
may_raise: yes
```

Examples:

```text
NonZeroDivisor
NoOverflow
NotReadOnly
```

Semantic guard failure raises language error.

### 16.2 Speculative Guards

```text
lowering_class: DeoptRequired or GuardedFastPathWithHelperFallback
deopt_required: if no helper fallback
```

Examples:

```text
IsCallTarget
HasShape for speculative optimization
IsType for specialized arithmetic
ModuleStateIs for import shortcut
```

### 16.3 Guard Failure

Every guard must have valid failure action:

```text
Fallback
Helper
Deopt
Raise
```

Compilation rejects missing failure path.

---

## 17. Terminators

### 17.1 Jump

```text
lowering_class: DirectLowering
```

Block arguments must be transferred without clobbering.

### 17.2 Branch

```text
lowering_class: DirectLowering
may_raise: yes if condition not proven Bool
```

Must preserve Bool-only condition.

### 17.3 Return

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_perform_unwind
may_raise: yes if cleanup raises
```

If no active cleanup and return contract already checked, direct return allowed.

### 17.4 Raise

```text
lowering_class: AlwaysCallHelper or GuardedFastPathWithHelperFallback
helper_fallback: helper_raise / helper_perform_unwind
may_raise: yes
```

### 17.5 LoopBackedge

```text
lowering_class: DirectLowering
safepoint_required: yes
root_map_required: yes
```

May update hotness counters.

### 17.6 Switch

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
```

Direct for dense enum case/pattern decisions.

Fallback for generic values.

### 17.7 Unwind

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_perform_unwind
may_raise: yes
may_unwind: yes
root_map_required: yes
```

### 17.8 Unreachable

```text
lowering_class: Forbidden
```

If reached in compiled code, report InternalVMError.

---

## 18. Structured Control Constructs

Although structured control lowers to EIR operations and terminators, the JIT must preserve construct-level invariants.

### 18.1 If

```text
lowering_class: DirectLowering over branches
```

All conditions require Bool check.

### 18.2 While

```text
lowering_class: DirectLowering with safepoint backedge
```

Loop backedge must expose roots.

### 18.3 For

```text
lowering_class: AlwaysCallHelper or GuardedFastPathWithHelperFallback initially
```

Iterator polling may call helper.

Map iteration order must be preserved.

### 18.4 Try/Catch/Finally

```text
lowering_class: AlwaysCallHelper for unwinding portions initially
```

JIT may compile body blocks but must call unwind/finally helper where needed.

### 18.5 Use/Defer

```text
lowering_class: AlwaysCallHelper for registration/execution initially
```

Compiled code must not remove cleanup paths.

### 18.6 Match

```text
lowering_class: DirectLowering for simple shape/case decision
lowering_class: AlwaysCallHelper for generic pattern fallback
```

Case order and guard timing must be preserved.

---

## 19. Module Operations

### 19.1 Import

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_resolve_module / helper_initialize_module / helper_import_named
safepoint_required: yes
root_map_required: yes
may_raise: yes
may_allocate: yes
```

### 19.2 Export Sealing

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_seal_exports
```

### 19.3 Module State Check

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: module helper
may_raise: yes
```

JIT may guard module state if safe.

---

## 20. Capability Operations

### 20.1 Capability Check

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_capability
capability_sensitive: yes
may_raise: yes
```

### 20.2 Host Boundary

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_enter_host_call / helper_exit_host_call
capability_sensitive: yes
safepoint_required: yes
root_map_required: yes
```

Compiled code cannot directly call host/native effectful functions.

---

## 21. Allocation Operations

### 21.1 Generic Allocation

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_alloc_object
safepoint_required: yes
root_map_required: yes
may_allocate: yes
```

### 21.2 Inline Allocation

Future:

```text
lowering_class: GuardedFastPathWithHelperFallback
```

Requirements:

```text
bump pointer fast path
allocation limit guard
safepoint fallback
root map
object initialization safety
write barrier if storing refs
```

Baseline JIT may defer inline allocation.

---

## 22. Write Barrier Operations

### 22.1 Write Barrier

```text
lowering_class: DirectLowering or AlwaysCallHelper
helper_fallback: helper_write_barrier
barrier_required: yes
```

If GC profile says no barrier needed, lowering may emit no-op.

The barrier site must still exist in lowering metadata.

### 22.2 Generational Profile

Under generational GC profile, JIT must emit actual barrier or call helper.

---

## 23. Source Mapping Requirements

Every compiled EIR op that may raise, call helper, deopt, or hit safepoint must map to:

```text
CodeOffset -> EirLocation -> SIR NodeId -> SourceSpan
```

Missing source map is a JIT validation error for may-raise operations.

---

## 24. Root Map Requirements

Root maps are required at:

```text
helper calls that may allocate/collect/raise/unwind
allocation points
loop backedge safepoints
function call safepoints
host call boundaries
deopt points
raise boundaries
```

If root map is missing, JIT compilation fails.

---

## 25. Deopt Requirements

Deopt metadata required for:

```text
speculative type specialization
speculative shape specialization
speculative call target specialization
inlined calls
elided checks
OSR entry/exit
compiled code side exit without helper fallback
```

Baseline JIT may avoid these features to reduce deopt burden.

---

## 26. Barrier Requirements

Write barrier required for:

```text
StoreField
StoreListIndex
StoreMapEntry
StoreCell when cell stores heap ref
StoreModuleSlot when module slot stores heap ref
ConstructRecord field initialization if object published before all stores
ConstructList/Map internal ref stores under generational/incremental profiles
Error suppressed list mutation
Resource metadata mutation
```

Bootstrap no-op barrier still requires call site.

---

## 27. Capability Requirements

Capability checks must be preserved for:

```text
host calls
module resolver if host marks it capability-gated
fs/net/process/env/random/clock helpers
future FFI
effectful builtins
```

JIT cannot fold capability check away unless capability environment immutability and guard/deopt are specified.

---

## 28. Lowering Matrix Summary

```text
EIR family              Baseline class
ConstantOp              Direct
LoadSlot                Direct
LoadCell                Guarded/Helper
LoadField               Guarded/Helper
StoreSlot               Direct
StoreField              Guarded/Helper + Barrier
StoreMapEntry           Helper
UnaryOp                 Guarded/Helper
BinaryOp                Guarded/Helper
LogicalOp               Direct control-flow
CheckBool               Direct
CheckType               Guarded/Helper
CheckCapability         Guarded/Helper
CallOp generic          Helper
CallOp monomorphic      Guarded/Helper
AccessOp record         Guarded/Helper
AccessOp map/string     Helper initially
ConstructList           Helper initially
ConstructMap            Helper
ConstructRecord         Guarded/Helper or Helper
ConstructEnum           Guarded/Helper or Helper
ConstructFunction       Helper
Pattern simple          Direct/Guarded
Pattern map/generic     Helper
RuntimeHelperOp         Helper
SafepointOp             Direct safepoint
GuardOp semantic        Direct
GuardOp speculative     Deopt/Helper
Jump                    Direct
Branch                  Direct
Return                  Direct only if no cleanup; otherwise Helper/Unwind
Raise                   Helper/Unwind
LoopBackedge            Direct + Safepoint
Switch                  Direct/Guarded
Unwind                  Helper initially
Unreachable             Forbidden
Import                  Helper
Host call               Helper
Allocation              Helper initially
Write barrier           Direct or Helper
```

---

## 29. JIT Validation Rules

JIT validation must reject:

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
compiled capability operation without check
compiled operation that assumes Rust Value enum layout
compiled operation that assumes object address identity
```

---

## 30. Implementation Staging

### 30.1 Stage J0 · Classification Only

Add JIT lowering class annotations to EIR op definitions.

### 30.2 Stage J1 · Validation

Implement validation that rejects unsupported JIT lowering cases.

### 30.3 Stage J2 · Direct Lowering Subset

Compile:

```text
ConstantOp
LoadSlot
StoreSlot
CheckBool
simple Branch
Jump
LoopBackedge poll skeleton
Identity comparison
```

### 30.4 Stage J3 · Helper Calls

Compile RuntimeHelperOp and generic helper call sequences with root maps.

### 30.5 Stage J4 · Guarded Record/Int Fast Paths

Compile:

```text
int arithmetic with guards
record field read/write with shape guard
list index read/write with guards
```

### 30.6 Stage J5 · Calls

Compile monomorphic call fast path with helper fallback.

### 30.7 Stage J6 · Structured Control Integration

Compile return/raise/unwind integration with helper_perform_unwind.

### 30.8 Stage J7 · GC Profile Integration

Enable JIT under collecting GC profile only after stack maps/root maps pass validation.

---

## 31. Non-Goals

This document does not define:

```text
Cranelift IR lowering
LLVM IR lowering
machine register allocation
code memory manager
OSR implementation
optimizing JIT
inline function optimization
escape analysis
native ABI
FFI
public bytecode
```

---

## 32. Next Work

Next Phase 3 documents should define:

```text
fast interpreter implementation milestones
Phase 3 consistency audit
Phase 3 freeze-readiness checklist

```


<!-- END NORMATIVE DOCUMENT: PHASE-3-JIT-LOWERING-MATRIX.md -->
