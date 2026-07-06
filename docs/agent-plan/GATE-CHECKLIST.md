# Gate Checklist

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications, `AGENT-MASTER-PLAN.md`, and `AGENT-OPERATING-PROTOCOL.md`  
Created: 2026-06-29 10:32:50

---

## 0. Purpose

This document defines executable gate checklists for Agent-led implementation planning and implementation work.

The gates are:

```text
G0 Scope Gate
G1 Spec Reference Gate
G2 Dependency Gate
G3 Design Gate
G4 Implementation Gate
G5 Validation Gate
G6 Integration Gate
G7 Handoff Gate
```

No work package may advance unless its required gates pass.

---

## 1. Gate Result States

Each gate has one result:

```text
PASS
PASS_WITH_NOTES
FAIL
DEFERRED
NOT_APPLICABLE
```

### 1.1 PASS

All required checks are satisfied.

### 1.2 PASS_WITH_NOTES

The gate passes, but a tracked caution exists.

Must record:

```text
note
risk if any
follow-up if any
```

### 1.3 FAIL

The work package cannot proceed.

Must record:

```text
failure reason
blocking issue
required correction
```

### 1.4 DEFERRED

Only allowed for checks that are not required at the current stage.

Must record:

```text
deferral reason
future gate where check becomes mandatory
```

### 1.5 NOT_APPLICABLE

Allowed only when the work package does not touch the relevant area.

Must record:

```text
why not applicable
```

---

## 2. G0 · Scope Gate

Purpose:

```text
ensure the task belongs to implementation planning or implementation work
```

Required checks:

```text
[ ] Task does not reopen frozen specification design.
[ ] Task does not introduce new language semantics.
[ ] Task does not introduce new VM semantics.
[ ] Task has a clear title and scope.
[ ] Task has explicit non-goals.
[ ] Task has an owner.
[ ] Task has defined outputs.
[ ] Task has a known work package ID or draft ID.
```

Automatic FAIL conditions:

```text
task modifies frozen specs without erratum process
task treats plan text as normative
task exposes RuntimePlan/EIR as public bytecode
task assumes CPython ABI compatibility
task lacks non-goals
```

Required output:

```text
G0 result
scope summary
non-goals
blocking issues if any
```

---

## 3. G1 · Spec Reference Gate

Purpose:

```text
ensure all work is traceable to frozen specifications
```

Required checks:

```text
[ ] Frozen spec references are listed.
[ ] Reference aliases resolve to concrete documents.
[ ] No requirement is unsupported by a frozen reference.
[ ] No large normative text is copied into the plan.
[ ] Any Phase 1/2 dependency is identified where relevant.
[ ] Any Phase 3 subsystem dependency is identified where relevant.
```

Automatic FAIL conditions:

```text
no frozen spec references
unresolved reference alias
implementation requirement with no source reference
subordinate Agent output without references
```

Required output:

```text
G1 result
reference list
missing references
traceability action if needed
```

---

## 4. G2 · Dependency Gate

Purpose:

```text
ensure the work package has its required inputs and does not violate sequencing
```

Required checks:

```text
[ ] Upstream work packages are complete or explicitly deferred.
[ ] Required documents exist.
[ ] Required data models exist or are in scope.
[ ] Required tests are known or planned.
[ ] Parallel tasks do not write shared state.
[ ] Dependencies are acyclic.
[ ] Deferred dependencies have clear fallback.
```

Automatic FAIL conditions:

```text
missing required input
unknown upstream dependency
parallel write conflict
cyclic dependency without resolution
```

Required output:

```text
G2 result
dependency list
missing inputs
sequencing constraints
```

---

## 5. G3 · Design Gate

Purpose:

```text
ensure implementation design is bounded and spec-traceable before coding
```

Required checks:

```text
[ ] Data structures are mapped to frozen references.
[ ] Interfaces are mapped to frozen references.
[ ] Error behavior is mapped to frozen references.
[ ] Validation behavior is mapped to frozen references.
[ ] Source diagnostics are considered where relevant.
[ ] Capability effects are considered where relevant.
[ ] Cache/profile impacts are considered where relevant.
[ ] JIT/GC hooks are considered where relevant.
[ ] Forbidden shortcuts are listed.
[ ] Test requirements are identified.
```

Automatic FAIL conditions:

```text
design weakens validation
design bypasses helper registry
design bypasses capability gate
design bypasses structured unwinding
design bypasses host boundary
design assumes object layout ABI
design lacks tests
```

Required output:

```text
G3 result
design summary
spec mapping
forbidden shortcuts
test obligations
```

---

## 6. G4 · Implementation Gate

Purpose:

```text
ensure a coding unit is ready to implement safely
```

Required checks:

```text
[ ] Implementation unit is small enough.
[ ] Expected changed files are listed.
[ ] Rollback path is known.
[ ] Test command is known.
[ ] Acceptance criteria are known.
[ ] Owner is known.
[ ] Subordinate Agent support, if any, is justified.
[ ] No unresolved gate failures remain from G0-G3.
```

Automatic FAIL conditions:

```text
unit too broad
unknown output files
unknown test command
no rollback path
unresolved G0-G3 failure
```

Required output:

```text
G4 result
implementation unit description
expected files
test command
rollback note
```

---

## 7. G5 · Validation Gate

Purpose:

```text
ensure implementation behavior is checked against frozen requirements
```

Required checks:

```text
[ ] Schema validation exists where relevant.
[ ] Semantic validation exists where relevant.
[ ] Negative tests exist where relevant.
[ ] Runtime error behavior is tested where relevant.
[ ] Source-span diagnostics are tested where relevant.
[ ] Capability checks are tested where relevant.
[ ] Cache compatibility checks are tested where relevant.
[ ] Host boundary behavior is tested where relevant.
[ ] Regression tests pass.
```

Automatic FAIL conditions:

```text
no negative tests for rejection behavior
runtime error code mismatch
missing source diagnostic where required
capability bypass
validation bypass
```

Required output:

```text
G5 result
tests run
tests missing
failures
coverage gaps
```

---

## 8. G6 · Integration Gate

Purpose:

```text
ensure the implementation unit can merge without cross-system regression
```

Required checks:

```text
[ ] No frozen-spec regression.
[ ] No unregistered helper.
[ ] No unregistered error code.
[ ] No unregistered cache/profile impact.
[ ] No unwinding bypass.
[ ] No capability bypass.
[ ] No host boundary bypass.
[ ] No object layout ABI leak.
[ ] No public bytecode leak.
[ ] Existing tests pass.
```

Automatic FAIL conditions:

```text
helper not in registry
error code not in registry
public RuntimePlan/EIR exposure
direct host pointer call
JIT/GC metadata bypass
unreviewed subordinate merge
```

Required output:

```text
G6 result
integration risks
regressions
merge decision
```

---

## 9. G7 · Handoff Gate

Purpose:

```text
ensure completed work can be understood and continued by another Agent
```

Required checks:

```text
[ ] Completed work is summarized.
[ ] Changed files are listed.
[ ] Frozen spec references are listed.
[ ] Gates run are listed.
[ ] Test results are listed.
[ ] Risks are recorded.
[ ] Open questions are recorded.
[ ] Next step is bounded.
[ ] Subordinate Agent outputs are reviewed if used.
```

Automatic FAIL conditions:

```text
no changed-file list
no spec-reference list
no gate result
no test result
unresolved risk hidden
subordinate output merged without review
```

Required output:

```text
handoff report
```

Use `HANDOFF-TEMPLATE.md`.

---

## 10. Gate Ordering

Default order:

```text
G0 -> G1 -> G2 -> G3 -> G4 -> G5 -> G6 -> G7
```

Some planning-only tasks may stop at:

```text
G0 -> G1 -> G2 -> G3 -> G7
```

Implementation tasks must pass:

```text
G0 through G7
```

unless a gate is explicitly marked `NOT_APPLICABLE`.

---

## 11. Gate Record Format

Each gate result should use:

```text
Gate:
Result:
Date:
Checked By:
Spec References:
Evidence:
Failures:
Risks:
Required Follow-Up:
```

---

## 12. Completion Criteria

This gate checklist is complete when every work package can identify:

```text
required gates
pass/fail criteria
evidence required
failure handling
handoff requirement
```

---

## 13. Concrete Artifact Gate Amendment

Added: 2026-06-29 11:00:40

G0-G4 must reject implementation work that lacks concrete artifacts.

### Workspace artifact checks

Audit phrase:

```text
workspace artifact checks
```

Before implementation begins, verify:

```text
[ ] root AGENT.md exists or will be created by WP-00
[ ] root PROGRESS.md exists or will be created by WP-00
[ ] root ISSUE.md exists or will be created by WP-00
[ ] docs/frozen-specs path exists or stable external spec path exists
[ ] docs/agent-plan path exists or will be created
[ ] crate/workspace layout is defined
[ ] test directory layout is defined
[ ] Agent handoff/audit/log directories are defined
```

### Coding artifact checks

Before a work package passes G4, verify:

```text
[ ] exact crate or module target is named
[ ] exact file creation or modification class is named
[ ] expected tests are named
[ ] expected command is named
[ ] PROGRESS.md update is required if files change
[ ] ISSUE.md update is required if audit findings exist
```

A task that only says `review`, `consider`, or `align` without concrete outputs fails G3/G4.

