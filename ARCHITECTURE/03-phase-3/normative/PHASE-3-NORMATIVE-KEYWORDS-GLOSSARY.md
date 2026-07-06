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
