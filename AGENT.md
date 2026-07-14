# AGENT.md

Document class: Agent engineering alignment and workflow contract  
Normative status: Non-normative implementation constraint  
Authority: Subordinate to the frozen Phase 1–3 specifications and Agent implementation plan documents  
Created: 2026-06-29 10:42:08

---

## 0. Purpose

This document defines the engineering alignment rules and execution workflow for Agents working on the Script VM implementation.

It is intended to be read before every implementation or audit session.

It constrains:

```text
engineering boundaries
workflow order
main/sub-Agent coordination
spec-reference discipline
progress reporting
audit result recording
handoff behavior
```

It does not define VM semantics.

It must not be used to reinterpret frozen specifications.

---

## 1. Authority Order

Agents must obey this authority order:

```text
Frozen Phase 1–3 normative specifications
  > PHASE-3-FREEZE.md
  > Agent implementation plan documents
  > AGENT.md
  > PROGRESS.md
  > ISSUE.md
  > temporary Agent notes
```

If `AGENT.md` conflicts with a frozen specification, the frozen specification wins.

If `AGENT.md` conflicts with `AGENT-MASTER-PLAN.md`, `AGENT-MASTER-PLAN.md` wins.

If a local implementation note conflicts with `AGENT.md`, `AGENT.md` wins.

---

## 2. Frozen Specification Boundary

Agents must treat Phase 1–3 specifications as frozen.

Agents must not:

```text
modify frozen specifications
redefine VM semantics
add language semantics
add IR semantics
add RuntimePlan or EIR semantics
treat implementation convenience as specification authority
copy large blocks of normative text into implementation plans
expose RuntimePlan or EIR as public bytecode
introduce CPython ABI compatibility
introduce public native object layout ABI
```

Agents may:

```text
cite frozen specifications
map implementation work to frozen specifications
create implementation tasks
create tests
record audit results
record progress summaries
raise erratum candidates
```

If an implementation gap appears, Agents must classify it as one of:

```text
editorial issue
contradiction repair
specification erratum candidate
requires later phase
implementation misunderstanding
```

Agents must not silently patch the specification through code.

---

## 3. Required Companion Documents

Agents must maintain exactly two rolling project-state documents:

```text
PROGRESS.md
ISSUE.md
```

These documents have strict append-only behavior.

They are not specifications.

They are not replacements for tests, commits, or handoff reports.

---

## 4. PROGRESS.md Rule

`PROGRESS.md` records implementation progress.

It is append-only.

Agents must only add new entries.

Agents must not rewrite, reorder, compress, delete, or reinterpret old entries.

### 4.1 Allowed Content

`PROGRESS.md` may contain only change summaries.

Allowed:

```text
what changed
which files changed
which work package advanced
which tests were added or run
which gate was passed
which spec references were used
which risks were mitigated
```

Forbidden:

```text
audit findings
bug lists
speculation
design debate
long reasoning
unverified claims
normative requirements
```

### 4.2 Required Entry Format

Every entry must use:

```text
## YYYY-MM-DD HH:MM · <short title>

Work Package:
Agent Mode:
Changed Files:
Spec References:
Gates:
Tests:
Summary:
Next:
```

### 4.3 Minimal Example

```text
## 2026-06-29 10:00 · RuntimeErrorCode model scaffold

Work Package: WP-04
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/error.rs
Spec References:
  - SPEC-P3-ERRORS
  - SPEC-P3-VALID
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS_WITH_NOTES
Tests:
  - cargo test error_registry
Summary:
  Added RuntimeErrorCode enum scaffold and structural/language error separation.
Next:
  Add ErrorObj fields and negative tests.
```

---

## 5. ISSUE.md Rule

`ISSUE.md` records audit results only.

It is append-only.

Agents must only add new entries.

Agents must not rewrite, reorder, compress, delete, or reinterpret old entries.

### 5.1 Allowed Content

`ISSUE.md` may contain only audit findings.

Allowed:

```text
blockers
major issues
minor issues
info findings
failed gate results
spec-reference mismatches
test coverage gaps
regression findings
risk escalations
erratum candidates
```

Forbidden:

```text
ordinary progress summaries
implementation diary entries
unverified speculation
design brainstorming
task plans without audit result
```

### 5.2 Severity Levels

Use exactly:

```text
BLOCKER
MAJOR
MINOR
INFO
```

### 5.3 Status Values

Use exactly:

```text
OPEN
MITIGATED
RESOLVED
ACCEPTED
DEFERRED
REJECTED
```

### 5.4 Required Entry Format

Every entry must use:

```text
## ISSUE-YYYYMMDD-NNN · <short title>

Severity:
Status:
Work Package:
Detected By:
Spec References:
Affected Files:
Finding:
Evidence:
Required Action:
Gate Impact:
Resolution Notes:
```

### 5.5 Minimal Example

```text
## ISSUE-20260629-001 · Missing negative test for non-Error raise

Severity: MAJOR
Status: OPEN
Work Package: WP-04
Detected By: Main Agent
Spec References:
  - SPEC-P3-ERRORS
  - SPEC-P3-VALID
Affected Files:
  - crates/vm_core/src/error.rs
Finding:
  RuntimeErrorCode scaffold exists, but non-Error raise rejection is not tested.
Evidence:
  No negative test covers raise of non-Error value.
Required Action:
  Add rejection test before WP-04 can pass G5.
Gate Impact:
  G5 FAIL
Resolution Notes:
  Pending.
```

---

## 6. Separation Between PROGRESS.md and ISSUE.md

Agents must keep the two documents separate.

Use this rule:

```text
PROGRESS.md = what changed
ISSUE.md    = what failed audit or needs correction
```

If an entry contains both progress and an audit finding:

```text
write the change summary to PROGRESS.md
write the finding to ISSUE.md
cross-reference by issue ID
```

Do not combine them into one entry.

---

## 7. Session Start Workflow

At the start of every Agent session:

```text
1. Read AGENT.md.
2. Identify current work package.
3. Read relevant Agent plan document.
4. Identify frozen spec references.
5. Check whether PROGRESS.md exists.
6. Check whether ISSUE.md exists.
7. Create PROGRESS.md only if absent.
8. Create ISSUE.md only if absent.
9. Run G0 and G1 mentally before editing.
```

If `PROGRESS.md` or `ISSUE.md` is absent, create it with only a header and append-only policy.

Do not prefill fake history.

---

## 8. Work Execution Workflow

For implementation work:

```text
1. Select work package.
2. Confirm frozen spec references.
3. Confirm gates required.
4. Check existing ISSUE.md for blocking issues.
5. Make bounded changes.
6. Run relevant tests or state why not run.
7. Append change summary to PROGRESS.md.
8. Append audit findings to ISSUE.md only if findings exist.
9. Produce handoff.
```

For audit work:

```text
1. Select audit scope.
2. Confirm frozen spec references.
3. Inspect changed files.
4. Record findings in ISSUE.md.
5. Do not write progress unless files changed.
6. Produce audit handoff.
```

---

## 9. Gate Discipline

Agents must respect the gate system:

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

A work package cannot be marked complete unless required gates pass.

If a gate fails:

```text
record the finding in ISSUE.md
do not hide it in PROGRESS.md
do not mark the package complete
```

---

## 10. Main Agent / Sub-Agent Workflow

Default mode:

```text
main-only
```

Sub-Agents may be used only when justified by `AGENT-MASTER-PLAN.md` and `AGENT-OPERATING-PROTOCOL.md`.

Maximum subordinate Agents:

```text
4
```

Sub-Agent output must not be merged directly.

Main Agent must review:

```text
scope compliance
spec references
forbidden actions
output structure
gate relevance
merge safety
```

If Sub-Agent output creates an audit finding, write it to `ISSUE.md`.

If Sub-Agent output causes accepted implementation changes, summarize them in `PROGRESS.md`.

---

## 11. Engineering Alignment Rules

Agents must preserve these engineering constraints:

```text
source-first execution model
no public bytecode
RuntimePlan and EIR internal only
Rust implementation bias unless explicitly changed
validation before execution
negative tests for rejection behavior
host boundary capability gating
structured unwinding preservation
helper registry discipline
runtime error registry discipline
cache compatibility discipline
GC/JIT hooks preserved without prematurely implementing production GC/JIT
```

Agents must reject implementation shortcuts that violate frozen constraints.

---

## 12. Commit / Change Discipline

Each implementation change should be small enough to review.

Preferred unit:

```text
one work package subtask
one clear testable behavior
one gate movement
```

Avoid:

```text
large mixed commits
spec changes mixed with implementation
test changes without explanation
progress entries without changed files
issue entries without evidence
```

---

## 13. Test Discipline

Agents must prefer test-backed progress.

Every implementation handoff should state:

```text
tests run
tests added
tests not run
reason tests were not run
```

Negative tests are required for rejection behavior.

If a required negative test is missing, record it in `ISSUE.md`.

---

## 14. Audit Discipline

Audit results must be specific.

Each `ISSUE.md` entry must include:

```text
severity
status
work package
spec references
affected files
finding
evidence
required action
gate impact
```

Agents must not record vague issues such as:

```text
needs improvement
may be wrong
check later
probably incomplete
```

unless evidence and required action are included.

---

## 15. Erratum Discipline

If implementation exposes a possible frozen-spec defect, Agents must not edit frozen specs.

Instead, record an issue with:

```text
Severity: MAJOR or BLOCKER
Status: OPEN
Required Action: classify as erratum candidate
Gate Impact: relevant blocked gate
```

Then stop affected implementation path until the main Agent decides.

---

## 16. File Ownership

`AGENT.md` owns workflow constraints.

`PROGRESS.md` owns append-only progress summaries.

`ISSUE.md` owns append-only audit findings.

Agent plan documents own planning structure:

```text
AGENT-MASTER-PLAN.md
AGENT-OPERATING-PROTOCOL.md
WORK-PACKAGE-INDEX.md
GATE-CHECKLIST.md
TRACEABILITY-MATRIX.md
RISK-REGISTER.md
HANDOFF-TEMPLATE.md
```

Frozen specs own VM semantics.

---

## 17. Final Handoff Requirement

Every session must end with a concise handoff containing:

```text
work package
changed files
PROGRESS.md entry added or not
ISSUE.md entry added or not
tests run
gates affected
next bounded action
```

If no file changed, say so.

If no tests ran, say why.

If no issues were found, do not add empty issue entries.

---

## 18. Hard Stop Conditions

Agents must stop and report if:

```text
a task requires changing frozen specification semantics
a public bytecode or public IR ABI is requested
CPython ABI compatibility is introduced
RuntimePlan/EIR exposure is requested
host boundary bypass is required
capability bypass is required
structured unwinding cannot be preserved
required spec reference is missing
G0 or G1 fails
```

Hard stop findings must be recorded in `ISSUE.md`.

---

## 19. Minimal Bootstrap Files

If absent, create:

```text
PROGRESS.md
ISSUE.md
```

with only the following headers.

### PROGRESS.md bootstrap

```text
# PROGRESS.md

Document class: Append-only implementation progress log  
Rule: Only append change summaries. Do not rewrite old entries.
```

### ISSUE.md bootstrap

```text
# ISSUE.md

Document class: Append-only audit findings log  
Rule: Only append audit results. Do not rewrite old entries.
```

---

---

## 20. How to Use the Frozen Specification Documents

Agents must use the frozen specification documents as source-of-truth references, not as editable planning material.

### 20.1 Specification Usage Rule

Use frozen specifications to answer:

```text
what behavior is required
what behavior is forbidden
what validation must reject
what runtime error must be produced
what internal representation is constrained
what compatibility boundary must be preserved
```

Do not use frozen specifications to decide:

```text
task priority
Agent dispatch strategy
token budget
implementation order
handoff format
progress logging format
```

Those belong to Agent implementation plan documents.

### 20.2 Required Specification Reading Order

When starting a new implementation area, read in this order:

```text
1. PHASE-3-FREEZE.md
2. PHASE-3-VM-SPEC.md or PHASE-3-MINIMAL-VM.md
3. subsystem-specific frozen specification documents
4. PHASE-1 / PHASE-2 frozen documents if source language or SIR behavior is involved
```

Do not begin from memory.

Do not begin from implementation convenience.

Do not begin from the aggregate alone when a subsystem-specific document exists.

### 20.3 Aggregate vs Subsystem Rule

Aggregates:

```text
PHASE-3-VM-SPEC.md
PHASE-3-MINIMAL-VM.md
```

are frontdoors.

Subsystem documents are controlling references for detailed implementation behavior.

Examples:

```text
Runtime errors:
  use PHASE-3-RUNTIME-ERROR-REGISTRY.md

RuntimePlan:
  use PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md

EIR:
  use PHASE-3-EIR-SCHEMA-CLOSURE.md

Helpers:
  use PHASE-3-RUNTIME-HELPER-REGISTRY.md

Control state:
  use PHASE-3-CONTROL-STATE-MODEL.md

Unwinding:
  use PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md

Module runtime:
  use PHASE-3-MODULE-RUNTIME-CONTRACT.md

GC metadata:
  use PHASE-3-GC-METADATA-OWNERSHIP.md

Target/profile:
  use PHASE-3-TARGET-PROFILE-SCHEMAS.md

Value/string/key semantics:
  use PHASE-3-VALUE-KEY-STRING-SEMANTICS.md

Validation:
  use PHASE-3-VALIDATION-MATRIX.md

Cache compatibility:
  use PHASE-3-CACHE-COMPATIBILITY-MATRIX.md

Calls:
  use PHASE-3-CALL-EXECUTION-PROTOCOL.md

ReadOnlyView:
  use PHASE-3-READONLY-VIEW-SEMANTICS.md

Host boundary:
  use PHASE-3-HOST-BOUNDARY-CONTRACT.md
```

### 20.4 Specification Citation Rule

Every implementation task must cite frozen specifications using:

```text
concrete document names
or aliases from AGENT-MASTER-PLAN.md
```

Acceptable:

```text
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
```

Acceptable:

```text
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md
  - PHASE-3-VALIDATION-MATRIX.md
```

Not acceptable:

```text
Spec References:
  - the VM docs
  - the spec
  - previous discussion
  - according to memory
```

### 20.5 Specification Gap Rule

If required behavior cannot be located in frozen specifications:

```text
1. stop the affected implementation path
2. record an ISSUE.md entry
3. classify the gap
4. do not invent behavior
```

Allowed classifications:

```text
implementation misunderstanding
missing reference
editorial issue
contradiction repair candidate
specification erratum candidate
requires later phase
```

---

## 21. How to Use the Agent Plan Documents

Agent plan documents control execution workflow, not VM semantics.

### 21.1 Plan Document Roles

Use the plan documents as follows:

```text
AGENT-MASTER-PLAN.md
  overall authority, Agent team model, parallelism policy, work package model

AGENT-OPERATING-PROTOCOL.md
  main/sub-Agent dispatch rules, review rules, merge rules, stop conditions

WORK-PACKAGE-INDEX.md
  work package selection, dependencies, max Agent mode, package-level scope

GATE-CHECKLIST.md
  G0-G7 pass/fail criteria and evidence requirements

TRACEABILITY-MATRIX.md
  mapping from implementation items to frozen spec references and tests

RISK-REGISTER.md
  known risks, severity, mitigation, review gates

HANDOFF-TEMPLATE.md
  required output format for completion, failure, early stop, and merge review

AGENT.md
  repository-local workflow constraints and PROGRESS/ISSUE rules
```

### 21.2 Task Startup Procedure

For any new task, use this sequence:

```text
1. Read AGENT.md.
2. Identify the relevant work package in WORK-PACKAGE-INDEX.md.
3. Read the corresponding trace rows in TRACEABILITY-MATRIX.md.
4. Read known risks in RISK-REGISTER.md.
5. Identify required gates in GATE-CHECKLIST.md.
6. Read required frozen specification documents.
7. Decide Agent Mode under AGENT-MASTER-PLAN.md and AGENT-OPERATING-PROTOCOL.md.
8. Execute only the bounded task.
9. Record progress in PROGRESS.md if files changed.
10. Record audit findings in ISSUE.md if findings exist.
11. Produce handoff using HANDOFF-TEMPLATE.md.
```

### 21.3 Work Package Usage Rule

`WORK-PACKAGE-INDEX.md` answers:

```text
what task family this belongs to
what dependencies exist
what outputs are expected
what non-goals constrain the task
what gates apply
what tests are required
what max Agent mode is allowed
```

It does not answer:

```text
what the VM semantics are
what EIR op means
what RuntimePlan field means
what error code means
```

For those, use frozen specifications.

### 21.4 Traceability Usage Rule

`TRACEABILITY-MATRIX.md` must be used before coding.

It answers:

```text
which implementation item maps to which frozen specification
which work package owns the item
which gate checks the item
which tests are required
```

If no trace row exists for the implementation item:

```text
add or request a trace row before implementation
```

Do not proceed by assumption.

### 21.5 Gate Usage Rule

`GATE-CHECKLIST.md` must be used as an executable checklist.

At minimum:

```text
planning tasks require G0-G3 and G7
implementation tasks require G0-G7 unless explicitly NOT_APPLICABLE
audit tasks require G0, G1, relevant review gates, and G7
```

Gate failure must be written to `ISSUE.md`.

Gate passage with file changes must be summarized in `PROGRESS.md`.

### 21.6 Risk Register Usage Rule

Before starting a work package, scan `RISK-REGISTER.md` for:

```text
risks affecting that work package
BLOCKER risks
MAJOR risks
token-budget risks
Agent coordination risks
compatibility-boundary risks
```

If a risk is triggered:

```text
record an ISSUE.md entry
do not bury the risk in PROGRESS.md
```

### 21.7 Handoff Usage Rule

At the end of a task, use `HANDOFF-TEMPLATE.md`.

Do not invent a custom handoff format.

A valid handoff must include:

```text
work package
changed files
frozen spec references
gate results
tests run or not run
PROGRESS.md update status
ISSUE.md update status
risks
next bounded task
```

---

## 22. Required End-to-End Workflow

Every Agent session must follow this end-to-end workflow.

```text
1. Align:
   Read AGENT.md and identify current work package.

2. Route:
   Use WORK-PACKAGE-INDEX.md to locate scope, dependencies, non-goals, and max Agent mode.

3. Trace:
   Use TRACEABILITY-MATRIX.md to locate frozen specification references and test obligations.

4. Risk:
   Use RISK-REGISTER.md to identify known risks and blockers.

5. Specify:
   Read the actual frozen specification documents cited by the trace rows.

6. Gate:
   Apply GATE-CHECKLIST.md before implementation.

7. Execute:
   Make only bounded changes.

8. Test:
   Run relevant tests or record why tests were not run.

9. Record:
   Append change summaries only to PROGRESS.md.
   Append audit findings only to ISSUE.md.

10. Handoff:
   Use HANDOFF-TEMPLATE.md.
```

Skipping this sequence is allowed only for trivial documentation typos that do not affect implementation behavior.

Even then:

```text
PROGRESS.md must record the change if a file changed
ISSUE.md must record the finding if it came from audit
```

---

## 23. Document Misuse Rules

Agents must not misuse documents.

### 23.1 Frozen Specification Misuse

Forbidden:

```text
editing frozen specs during implementation
treating old temporary discussion as specification
using aggregate docs when subsystem docs are required
inventing behavior for an undocumented gap
```

### 23.2 Plan Document Misuse

Forbidden:

```text
treating plan documents as VM semantics
using work package text to override frozen specs
using risk entries as proof of behavior
using handoff text as test evidence
```

### 23.3 PROGRESS.md Misuse

Forbidden:

```text
adding audit findings
rewriting old entries
compressing history
removing failed attempts
```

### 23.4 ISSUE.md Misuse

Forbidden:

```text
adding ordinary progress summaries
removing resolved issues
rewriting history
recording vague findings without evidence
```

---

## 24. Quick Routing Table

Use this table when deciding which document to open.

| Need | Use |
|---|---|
| Check if a task is allowed | `AGENT.md`, `AGENT-MASTER-PLAN.md`, `GATE-CHECKLIST.md` |
| Choose a work package | `WORK-PACKAGE-INDEX.md` |
| Find required specs | `TRACEABILITY-MATRIX.md` |
| Understand VM behavior | frozen specification documents |
| Check known risks | `RISK-REGISTER.md` |
| Dispatch a sub-Agent | `AGENT-OPERATING-PROTOCOL.md` |
| Decide whether parallelism is allowed | `AGENT-MASTER-PLAN.md`, `AGENT-OPERATING-PROTOCOL.md` |
| Validate gate pass/fail | `GATE-CHECKLIST.md` |
| Record changed work | `PROGRESS.md` |
| Record audit finding | `ISSUE.md` |
| Read live Stage/WP snapshot | `docs/IMPLEMENTATION-STATUS.md` (rewritable; not PROGRESS) |
| End a task | `HANDOFF-TEMPLATE.md` |
| Session handoff only | `HANDOVER.md` (do not treat as live status between handoffs) |

ISSUE.md reading rule: for a given `ISSUE-YYYYMMDD-NNN`, the **last** `Status:` line for that ID is effective.

---

## 25. Patch Record

This section was added on 2026-06-29 10:48:11 to clarify how Agents must use the frozen specification documents and the Agent implementation plan document set.

2026-07-10: routing table extended for `docs/IMPLEMENTATION-STATUS.md` and ISSUE last-status rule (documentation sync; no semantic change).

## 26. Compliance Summary

An Agent is compliant with this document only if it:

```text
does not alter frozen semantics
uses frozen spec references
runs required gates
keeps PROGRESS.md append-only
keeps ISSUE.md append-only
separates progress from audit findings
reviews subordinate Agent output before merge
records tests and gate impact
stops on hard boundary violations
```

---

## 27. Concrete Coding Plan Usage

Added: 2026-06-29 11:00:40

Agents must not treat the plan package as merely abstract governance.

Before implementation, Agents must open:

```text
IMPLEMENTATION-CODING-PLAN.md
```

This file controls:

```text
workspace bootstrap
directory creation
root file creation
Rust workspace creation
crate creation order
module creation order
test directory creation
stage-by-stage coding sequence
per-stage gates
per-stage PROGRESS.md / ISSUE.md requirements
```

The Agent must execute from the coding plan unless a repository-specific constraint blocks it.

If blocked:

```text
1. stop the affected step
2. record an ISSUE.md audit finding
3. propose the smallest compatible adjustment
4. do not silently skip the step
```

`AGENT-MASTER-PLAN.md` and `AGENT-OPERATING-PROTOCOL.md` govern coordination.

`IMPLEMENTATION-CODING-PLAN.md` governs concrete execution sequence.

`WORK-PACKAGE-INDEX.md` governs work package identity and dependencies.

`TRACEABILITY-MATRIX.md` governs spec-to-task mapping.

`GATE-CHECKLIST.md` governs pass/fail criteria.

`RISK-REGISTER.md` governs risk review.

