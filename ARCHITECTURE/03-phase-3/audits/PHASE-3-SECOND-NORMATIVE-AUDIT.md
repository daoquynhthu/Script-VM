# Phase 3 · Second Normative Audit

Document class: Administrative audit  
Planning status: This document records the second audit after repairs R1-R14 and second-stage consolidation progress. It is not itself a normative specification.

Updated: 2026-06-29 09:33:23

## Second-Stage Repair Progress

```text
S2-R1 complete
S2-R2 complete
S2-R3 complete
S2-R4 complete
S2-R5 complete
```

Final freeze-readiness result is recorded in:

```text
PHASE-3-FINAL-FREEZE-READINESS-AUDIT.md
```

---

## 0. Audit Verdict

Phase 3 is **substantially closer to normative closure**, but **not yet freeze-approved**.

Current state:

```text
Original blocker set B-01 through B-08: addressed
Normative/plan separation: maintained
Core schema closure: improved
Validation/cache closure: added
Remaining risk level: major residual issues
Freeze-approved: no
```

This audit does not claim final freeze.

It records the state after the first repair pass.

---

## 1. Completed Repair Coverage

```text
R1  Normative keyword policy and terminology glossary
R2  Runtime error registry
R3  EIR operation schema closure
R4  RuntimePlan schema closure
R5  Runtime helper registry
R6  Unified control-state model
R7  Structured unwinding algorithm
R8  Module runtime contract
R9  SIR lowering coverage matrix
R10 GC metadata ownership
R11 Target/runtime profile schemas
R12 ValueKey and string runtime semantics
R13 Unified validation matrix
R14 Cache compatibility matrix
```

---

## 2. Original Blocker Status

| Blocker | Status | Repair |
|---|---|---|
| B-01 EIR instruction schema not closed | addressed | R3 |
| B-02 RuntimePlan schema framework-level | addressed | R4 |
| B-03 helper names lack registry | addressed | R5 |
| B-04 error taxonomy not centralized | addressed | R2 |
| B-05 control-state model not unified | addressed | R6 |
| B-06 unwinding algorithm not closed | addressed | R7 |
| B-07 module runtime contract not closed | addressed | R8 |
| B-08 SIR lowering coverage incomplete | addressed | R9 |

No original blocker remains open.

---

## 3. Remaining Major Residuals

## S2-M01 · Call execution protocol still deserves canonical extraction

Severity: MAJOR  
Related original finding: M-08

The current repair set closes many call-related pieces through:

```text
EIR schema
RuntimePlan schema
helper registry
validation matrix
```

However, call execution remains distributed across multiple documents.

A freeze-ready spec should still add a canonical call execution protocol covering:

```text
callee evaluation
positional/named argument evaluation
default argument evaluation at call time
parameter binding
closure capture
builtin call
host call
return contract
call-site feedback
error/source-span behavior
```

Recommended repair:

```text
Add PHASE-3-CALL-EXECUTION-PROTOCOL.md
```

---

## S2-M02 · ReadOnlyView semantics still need a canonical document

Severity: MAJOR  
Related original finding: M-09

ReadOnlyView rules exist in several places, but the current repair set does not centralize:

```text
identity
equality
delegated read
mutation through view
mutation through original object
shallow vs deep behavior
JIT assumptions
helper behavior
```

Recommended repair:

```text
Add PHASE-3-READONLY-VIEW-SEMANTICS.md
```

---

## S2-M03 · Host boundary / FFI deferred status should be sharpened

Severity: MAJOR  
Related original finding: M-13

The spec repeatedly rejects public native ABI and defers FFI, while still allowing host functions, module resolver, host roots, and capability-gated effects.

The status is directionally correct but needs one canonical host boundary document.

Recommended repair:

```text
Add PHASE-3-HOST-BOUNDARY-CONTRACT.md
```

It should define:

```text
host function wrapper
host object wrapper
host root registry
host error normalization
capability gates
FFI deferred boundary
prohibition on raw VM object pointer retention
```

---

## S2-M04 · Existing earlier normative documents may still contain unmarked planning language

Severity: MAJOR  
Related original findings: M-01, M-12, m-05

R1 defines the keyword policy, but older normative documents were not exhaustively rewritten.

Before freeze, scan and mark:

```text
recommended
bootstrap
staged
first implementation
later
milestone
plan
```

as one of:

```text
MUST / SHOULD / MAY
BOOTSTRAP
RECOMMENDED implementation option
DEFERRED
NON-NORMATIVE NOTE
```

---

## S2-M05 · Second-order consistency check needed after adding S2-M01-S2-M03

Severity: MAJOR

Adding call/readonly/host documents may affect:

```text
EIR schema
helper registry
validation matrix
cache compatibility matrix
target profile schemas
runtime error registry
```

A final freeze audit must be run after those residual repairs.

---

## 4. Minor Residuals

## S2-m01 · Document volume is now high

The organized mirror helps, but the spec would benefit from a short normative index that groups documents by semantic subsystem.

Current `WORKSPACE-INDEX.md` is administrative, not normative.

## S2-m02 · Aggregate size may become hard to review

`PHASE-3-VM-SPEC.md` is now a large aggregate.

This is acceptable as a generated aggregate, but future review should focus on subsystem documents first.

## S2-m03 · Some old sections may duplicate newly canonical schemas

Older documents may still define earlier versions of terms now owned by repair documents.

The precedence rule handles conflicts, but cleanup would improve readability.

---

## 5. Freeze Readiness

Phase 3 is not freeze-approved until:

```text
S2-M01 repaired or explicitly deferred
S2-M02 repaired or explicitly deferred
S2-M03 repaired or explicitly deferred
S2-M04 repaired or accepted with documented precedence
final audit passes
```

However, the original blocker class is resolved.

The project is now in:

```text
normative consolidation phase
```

not:

```text
initial architecture discovery
implementation planning
```

---

## 6. Recommended Next Repair Batch

```text
S2-R1: Add canonical call execution protocol.
S2-R2: Add canonical ReadOnlyView semantics.
S2-R3: Add canonical host boundary contract.
S2-R4: Sweep old normative docs for unmarked planning language.
S2-R5: Final freeze-readiness audit.
```

---

## 7. Audit Conclusion

The Phase 3 specification has crossed the main structural gap:

```text
closed core schema
closed helper/error/control/module/unwind coverage
closed validation/cache profile direction
```

But freeze would still be premature until the remaining major residuals are handled.
