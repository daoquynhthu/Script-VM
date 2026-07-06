# Phase 3 · Freeze Declaration

Document class: Freeze declaration  
Freeze status: This document declares the Phase 3 normative VM specification baseline frozen. It does not introduce new VM semantics beyond the frozen normative document set.

Created: 2026-06-29 09:35:11

---

## 0. Declaration

Phase 3 is hereby frozen as:

```text
Version: 1.0 frozen normative baseline
Status: Frozen
Phase: Phase 3 · Minimal VM Design
Scope: Normative VM specification
```

This marks the end of the overall Phase 3 specification design stage.

From this point forward, Phase 3 is no longer in:

```text
normative discovery
normative repair
normative consolidation
freeze-candidate review
```

It is now in:

```text
frozen normative baseline
```

Implementation planning and implementation work may proceed against this frozen baseline, but implementation plans remain non-normative.

---

## 1. Freeze Basis

This freeze is based on:

```text
PHASE-3-FINAL-FREEZE-READINESS-AUDIT.md
```

Final audit result:

```text
Freeze candidate: yes
Automatically frozen: no
Requires explicit freeze declaration: yes
```

This document supplies that explicit freeze declaration.

---

## 2. Frozen Normative Document Set

The following documents constitute the Phase 3 frozen normative baseline.

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

Aggregates:

```text
PHASE-3-VM-SPEC.md
PHASE-3-MINIMAL-VM.md
```

These aggregates are generated normative frontdoors over the frozen normative document set.

---

## 3. Excluded Non-Normative Documents

Implementation plans are excluded from the frozen normative baseline.

- `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md`
- `PHASE-3-GC-IMPLEMENTATION-STAGING-PLAN.md`
- `PHASE-3-FAST-INTERPRETER-IMPLEMENTATION-MILESTONES.md`

The implementation plan aggregate is also excluded:

```text
PHASE-3-IMPLEMENTATION-PLAN.md
```

It may guide project execution, but it does not define VM semantics.

---

## 4. Administrative and Audit Documents

The following documents record history, classification, audits, status, or workspace structure.

They are not semantic specifications.

- `PHASE-3-CHANGELOG.md`
- `STATUS.md`
- `PHASE-3-DOCUMENT-MANIFEST.md`
- `PHASE-3-NORMATIVE-CONSISTENCY-AUDIT.md`
- `PHASE-3-AUDIT-REPAIR-LOG.md`
- `PHASE-3-SECOND-NORMATIVE-AUDIT.md`
- `PHASE-3-NORMATIVE-LANGUAGE-SWEEP.md`
- `PHASE-3-FINAL-FREEZE-READINESS-AUDIT.md`
- `WORKSPACE-INDEX.md`

This freeze declaration itself is authoritative only for:

```text
freeze status
included normative document set
excluded non-normative document set
post-freeze change policy
```

It does not supersede the normative VM semantics in the frozen document set.

---

## 5. Compatibility Boundaries Frozen

The following compatibility boundaries are frozen:

```text
No public bytecode.
No public EIR.
No public RuntimePlan.
No package ABI based on internal VM IR.
No native ABI based on VM object layout.
No CPython C API compatibility.
No CPython ABI compatibility.
No Python wheel compatibility.
No PyObject layout compatibility.
No CPython-style refcount architecture as VM architecture.
No GC finalizer-based resource cleanup requirement.
No production SIR-walk execution path.
```

Internal representations remain:

```text
VM-versioned
target-profile-versioned
discardable
cacheable only under internal compatibility rules
not user-facing artifacts
```

---

## 6. Mandatory Frozen Architecture Commitments

The following architecture commitments are frozen:

```text
source-first language execution model
RuntimePlan / EIR internal execution pipeline
slot-based execution model
fixed-shape record access through FieldId / FieldIndex
closed enum access through CaseId / CaseIndex
canonical runtime error registry
canonical helper registry
canonical control-state model
canonical structured unwinding algorithm
canonical module runtime contract
canonical SIR lowering coverage matrix
canonical GC metadata ownership model
canonical target/runtime profile schemas
canonical ValueKey and string semantics
canonical validation matrix
canonical cache compatibility matrix
canonical call execution protocol
canonical ReadOnlyView semantics
canonical host boundary contract
```

---

## 7. Deferred Areas

The following areas remain deferred and are not required for the Phase 3 minimal VM implementation:

```text
production optimizing JIT
production moving/compacting GC
incremental or concurrent GC
public debugger protocol
public profiler protocol
FFI
native extension ABI
async/await
threads
generators/yield
public bytecode distribution
package manager semantics
full standard library implementation
```

Deferred does not mean unconstrained.

Deferred areas must preserve the frozen compatibility boundaries.

---

## 8. Post-Freeze Change Policy

After this freeze, Phase 3 normative documents may be changed only under one of these categories.

### 8.1 Editorial Change

Allowed without reopening Phase 3 if it:

```text
fixes spelling
fixes formatting
clarifies wording without changing semantics
updates cross-reference formatting
regenerates aggregates without semantic change
```

### 8.2 Contradiction Repair

Allowed only with explicit record if it:

```text
resolves direct contradiction inside frozen documents
does not introduce a new feature
does not alter intended semantics beyond contradiction removal
```

Must update:

```text
PHASE-3-CHANGELOG.md
STATUS.md
```

### 8.3 Specification Erratum

Allowed only with explicit erratum record if it:

```text
fixes a discovered semantic gap required for implementation correctness
preserves existing frozen architecture commitments
does not reopen rejected compatibility boundaries
```

Errata must be tracked separately.

### 8.4 Reopening Change

Requires reopening Phase 3 or creating a later phase if it:

```text
adds public bytecode
adds public IR ABI
adds native object layout ABI
adds CPython ABI compatibility
changes RuntimePlan/EIR semantics incompatibly
changes control/unwinding semantics
changes module runtime semantics
changes ValueKey/string semantics incompatibly
changes GC/JIT compatibility boundaries
```

---

## 9. Implementation Permission

Implementation may proceed after this freeze.

Implementation work must treat the frozen normative documents as the source of truth.

Implementation plans may be revised freely as non-normative project documents, provided they do not contradict the frozen normative baseline.

---

## 10. Freeze Completion Statement

Phase 3 normative specification design is complete.

The project may now move from:

```text
specification design
```

to:

```text
implementation preparation
implementation planning
prototype implementation
conformance testing
```

under the frozen Phase 3 baseline.
