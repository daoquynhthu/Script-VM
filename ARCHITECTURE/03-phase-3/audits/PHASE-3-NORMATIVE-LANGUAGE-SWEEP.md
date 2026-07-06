# Phase 3 · Normative Language Sweep

Document class: Administrative audit  
Planning status: This document records the S2-R4 sweep for unmarked planning-style language in normative documents. It is not itself a normative specification.

Created: 2026-06-29 09:33:23

---

## 0. Purpose

This document completes second-stage repair item:

```text
S2-R4: Sweep old normative docs for unmarked planning language.
```

It addresses residual finding:

```text
S2-M04: Existing earlier normative documents may still contain unmarked planning language.
```

---

## 1. Repair Action

The sweep did not rewrite large semantic sections.

Instead, it applied a uniform normative interpretation hook to normative documents:

```text
This document is interpreted under PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md.
```

The hook states that unmarked planning-style words are interpreted as one of:

```text
MUST / MUST NOT / SHOULD / MAY
BOOTSTRAP allowance
RECOMMENDED implementation option
DEFERRED design area
NON-NORMATIVE NOTE
```

according to local context.

This avoids accidental semantic drift from mechanical rewriting.

---

## 2. Patched Documents

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

## 3. Planning-Language Scan

The scan looked for:

```text
bootstrap
recommended
staged / stage
first implementation / initial implementation
later / future
milestone
plan / planning
```

| Document | Count | Terms |
|---|---:|---|
| `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md` | 38 | bootstrap:6, recommended:2, staged:2, first implementation:2, later:8, milestone:5, plan:13 |
| `PHASE-3-RUNTIME-ERROR-REGISTRY.md` | 13 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:5, milestone:1, plan:3 |
| `PHASE-3-EIR-SCHEMA-CLOSURE.md` | 11 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:3 |
| `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md` | 13 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:5 |
| `PHASE-3-RUNTIME-HELPER-REGISTRY.md` | 11 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:3 |
| `PHASE-3-CONTROL-STATE-MODEL.md` | 11 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:3 |
| `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` | 15 | bootstrap:2, recommended:1, staged:1, first implementation:1, later:6, milestone:1, plan:3 |
| `PHASE-3-MODULE-RUNTIME-CONTRACT.md` | 14 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:5, milestone:1, plan:4 |
| `PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md` | 15 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:4, milestone:1, plan:6 |
| `PHASE-3-GC-METADATA-OWNERSHIP.md` | 11 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:3 |
| `PHASE-3-TARGET-PROFILE-SCHEMAS.md` | 11 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:3 |
| `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md` | 17 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:9, milestone:1, plan:3 |
| `PHASE-3-VALIDATION-MATRIX.md` | 12 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:4 |
| `PHASE-3-CACHE-COMPATIBILITY-MATRIX.md` | 11 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:3, milestone:1, plan:3 |
| `PHASE-3-CALL-EXECUTION-PROTOCOL.md` | 16 | bootstrap:1, recommended:1, staged:2, first implementation:1, later:6, milestone:1, plan:4 |
| `PHASE-3-READONLY-VIEW-SEMANTICS.md` | 13 | bootstrap:1, recommended:1, staged:2, first implementation:1, later:4, milestone:1, plan:3 |
| `PHASE-3-HOST-BOUNDARY-CONTRACT.md` | 15 | bootstrap:1, recommended:1, staged:2, first implementation:1, later:6, milestone:1, plan:3 |
| `PHASE-3-VM-FRAMEWORK.md` | 39 | bootstrap:2, recommended:5, staged:13, first implementation:1, later:11, milestone:1, plan:6 |
| `PHASE-3-VM-RUNTIME-ROUND1.md` | 37 | bootstrap:2, recommended:6, staged:1, first implementation:2, later:17, milestone:1, plan:8 |
| `PHASE-3-PERFORMANCE-ARCHITECTURE.md` | 40 | bootstrap:7, recommended:5, staged:3, first implementation:1, later:11, milestone:1, plan:12 |
| `PHASE-3-RUNTIMEPLAN-EIR-FRAMEWORK.md` | 16 | bootstrap:1, recommended:1, staged:1, first implementation:2, later:4, milestone:1, plan:6 |
| `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` | 15 | bootstrap:1, recommended:1, staged:1, first implementation:1, later:7, milestone:1, plan:3 |
| `PHASE-3-SIR-LOWERING-ROUND1.md` | 23 | bootstrap:1, recommended:3, staged:1, first implementation:1, later:8, milestone:1, plan:8 |
| `PHASE-3-CONTROL-LOWERING-ROUND2.md` | 19 | bootstrap:2, recommended:2, staged:1, first implementation:1, later:6, milestone:1, plan:6 |
| `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` | 23 | bootstrap:5, recommended:2, staged:1, first implementation:2, later:8, milestone:1, plan:4 |
| `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md` | 28 | bootstrap:11, recommended:1, staged:2, first implementation:1, later:7, milestone:1, plan:5 |
| `PHASE-3-BASELINE-JIT-BACKEND-INTERFACE.md` | 25 | bootstrap:1, recommended:5, staged:2, first implementation:2, later:9, milestone:1, plan:5 |
| `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md` | 17 | bootstrap:2, recommended:2, staged:1, first implementation:1, later:4, milestone:2, plan:5 |
| `PHASE-3-JIT-LOWERING-MATRIX.md` | 31 | plan:4, bootstrap:2, recommended:1, staged:9, first implementation:1, later:12, milestone:2 |

---

## 4. Interpretation Result

Planning-style language inside normative documents is now controlled by:

```text
PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md
```

and by local canonical repair documents.

If an older document uses a schema or term differently from a newer canonical repair document, the newer canonical repair document owns the repaired term or schema.

---

## 5. Remaining Risk

This sweep does not guarantee that every old sentence is perfectly phrased.

It does establish a normative interpretation rule sufficient for freeze-readiness review.

A future editorial cleanup MAY rewrite older documents for readability, but it is not required to interpret the normative model.

---

## 6. Audit Tracking

This document completes:

```text
S2-R4
```

It addresses:

```text
S2-M04
```
