# Risk Register

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications and `AGENT-MASTER-PLAN.md`  
Created: 2026-06-29 10:36:55

---

## 0. Purpose

This document tracks risks for Agent-led implementation planning and implementation.

It covers:

```text
specification-reference risks
implementation risks
integration risks
testing risks
Agent coordination risks
token-budget risks
```

It does not define VM semantics.

---

## 1. Risk Severity

```text
BLOCKER
MAJOR
MINOR
INFO
```

### 1.1 BLOCKER

Prevents safe progress.

Requires immediate correction.

### 1.2 MAJOR

Can cause serious implementation drift, incorrect behavior, or integration failure.

Must be mitigated before affected package completes.

### 1.3 MINOR

Local issue or process weakness.

Should be tracked and cleaned up.

### 1.4 INFO

Observation without immediate action requirement.

---

## 2. Risk Status

```text
OPEN
MITIGATED
ACCEPTED
DEFERRED
CLOSED
```

### OPEN

Risk is active.

### MITIGATED

Controls are in place, but risk should still be watched.

### ACCEPTED

Risk is known and accepted for a stated reason.

### DEFERRED

Risk is tied to deferred work.

### CLOSED

Risk no longer applies.

---

## 3. Risk Record Format

```text
Risk ID:
Area:
Severity:
Status:
Description:
Spec References:
Affected Work Packages:
Mitigation:
Owner:
Review Gate:
Notes:
```

---

## 4. Active Risks

### RISK-001 · Specification Drift

```text
Risk ID: RISK-001
Area: specification reference discipline
Severity: BLOCKER
Status: OPEN
Description:
  Agent output may accidentally reinterpret or extend frozen specifications.
Spec References:
  - SPEC-P3-FREEZE
  - SPEC-P3-KEYWORDS
Affected Work Packages:
  - all
Mitigation:
  Require G0 and G1 before all work.
  Require frozen spec references in every work package.
  Reject unreferenced requirements.
Owner:
  Main Agent
Review Gate:
  G0
  G1
  G6
Notes:
  This is the primary risk after specification freeze.
```

### RISK-002 · Plan Text Treated as Normative

```text
Risk ID: RISK-002
Area: document authority
Severity: MAJOR
Status: OPEN
Description:
  Implementation plan language may be mistaken for VM semantics.
Spec References:
  - SPEC-P3-FREEZE
Affected Work Packages:
  - all
Mitigation:
  Mark all Agent plan documents as non-normative.
  Use authority order in every major plan document.
Owner:
  Main Agent
Review Gate:
  G0
  G1
Notes:
  Especially relevant when work packages summarize behavior.
```

### RISK-003 · Overuse of Subordinate Agents

```text
Risk ID: RISK-003
Area: Agent coordination / token budget
Severity: MAJOR
Status: OPEN
Description:
  Excessive parallelism may waste token budget and create inconsistent outputs.
Spec References:
  - AGENT-MASTER-PLAN.md
  - AGENT-OPERATING-PROTOCOL.md
Affected Work Packages:
  - all packages using main+2 or higher
Mitigation:
  Default to main-only or main+1.
  Require written parallelism justification.
  Limit max subordinate Agents to 4.
Owner:
  Main Agent
Review Gate:
  G2
  G7
Notes:
  Parallelism should be exceptional, not decorative.
```

### RISK-004 · RuntimePlan Schema Drift

```text
Risk ID: RISK-004
Area: RuntimePlan implementation
Severity: MAJOR
Status: OPEN
Description:
  Implementation may omit required RuntimePlan fields or cache-key components.
Spec References:
  - SPEC-P3-RTP
  - SPEC-P3-CACHE
  - SPEC-P3-VALID
Affected Work Packages:
  - WP-05
  - WP-16
  - WP-17
Mitigation:
  Use trace rows TR-003 and TR-014.
  Require schema validation tests.
Owner:
  Main Agent
Review Gate:
  G3
  G5
  G6
```

### RISK-005 · EIR Schema Drift

```text
Risk ID: RISK-005
Area: EIR implementation
Severity: MAJOR
Status: OPEN
Description:
  Implementation may add unregistered op kinds or weaken EIR validation.
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
Affected Work Packages:
  - WP-06
  - WP-17
Mitigation:
  Reject unknown ops and terminators.
  Maintain closed op/terminator set.
Owner:
  Main Agent
Review Gate:
  G3
  G5
  G6
```

### RISK-006 · Helper Registry Mismatch

```text
Risk ID: RISK-006
Area: runtime helpers
Severity: MAJOR
Status: OPEN
Description:
  Helper implementation may diverge from helper descriptors or omit safety metadata.
Spec References:
  - SPEC-P3-HELPERS
  - SPEC-P3-VALID
  - SPEC-P3-CACHE
Affected Work Packages:
  - WP-07
  - WP-17
Mitigation:
  Require descriptor/implementation validation.
  Require helper registry digest participation.
Owner:
  Main Agent
Review Gate:
  G5
  G6
```

### RISK-007 · LanguageError / VmStructuralError Confusion

```text
Risk ID: RISK-007
Area: runtime error model
Severity: MAJOR
Status: OPEN
Description:
  Implementation may expose structural VM failures as catchable language errors.
Spec References:
  - SPEC-P3-ERRORS
Affected Work Packages:
  - WP-04
  - WP-07
  - WP-17
Mitigation:
  Implement separate error categories.
  Add tests for non-catchable structural failures.
Owner:
  Main Agent
Review Gate:
  G3
  G5
```

### RISK-008 · Structured Unwinding Bugs

```text
Risk ID: RISK-008
Area: control flow / cleanup
Severity: BLOCKER
Status: OPEN
Description:
  Return, raise, break, continue, defer, resource close, or finally may bypass canonical cleanup behavior.
Spec References:
  - SPEC-P3-UNWIND
  - SPEC-P3-CONTROL
Affected Work Packages:
  - WP-09
  - WP-10
  - WP-17
Mitigation:
  Implement cleanup state explicitly.
  Test return/raise/break/continue through cleanup.
Owner:
  Main Agent
Review Gate:
  G5
  G6
```

### RISK-009 · Module Circular Import Ambiguity

```text
Risk ID: RISK-009
Area: module runtime
Severity: MAJOR
Status: OPEN
Description:
  Implementation may mishandle partially initialized exports or failed module state.
Spec References:
  - SPEC-P3-MODULE
  - SPEC-P3-ERRORS
Affected Work Packages:
  - WP-11
  - WP-17
Mitigation:
  Enforce ModuleState transitions.
  Add circular import tests.
Owner:
  Main Agent
Review Gate:
  G5
  G6
```

### RISK-010 · Source-Span Diagnostics Under-Coverage

```text
Risk ID: RISK-010
Area: diagnostics
Severity: MAJOR
Status: OPEN
Description:
  Errors may lose source mapping across RuntimePlan/EIR/helper/interpreter paths.
Spec References:
  - SPEC-P3-ERRORS
  - SPEC-P3-EIR
  - SPEC-P3-VALID
Affected Work Packages:
  - WP-04
  - WP-05
  - WP-06
  - WP-07
  - WP-17
  - WP-18
Mitigation:
  Require source-span checks in validation and conformance tests.
Owner:
  Main Agent
Review Gate:
  G5
```

### RISK-011 · Capability or Host Boundary Bypass

```text
Risk ID: RISK-011
Area: host boundary / capabilities
Severity: BLOCKER
Status: OPEN
Description:
  Host calls or effectful operations may bypass capability checks or host boundary wrappers.
Spec References:
  - SPEC-P3-HOST
  - SPEC-P3-CALL
  - SPEC-P3-PROFILE
Affected Work Packages:
  - WP-12
  - WP-14
  - WP-17
Mitigation:
  Route host calls through helper/host boundary.
  Test missing capability rejection.
Owner:
  Main Agent
Review Gate:
  G5
  G6
```

### RISK-012 · Cache Compatibility Omission

```text
Risk ID: RISK-012
Area: cache compatibility
Severity: MAJOR
Status: OPEN
Description:
  Cache keys may omit helper, profile, source, or schema components.
Spec References:
  - SPEC-P3-CACHE
  - SPEC-P3-PROFILE
Affected Work Packages:
  - WP-05
  - WP-06
  - WP-07
  - WP-16
Mitigation:
  Implement explicit cache key component checks.
  Test stale cache rejection.
Owner:
  Main Agent
Review Gate:
  G5
  G6
```

### RISK-013 · Public Bytecode Leak

```text
Risk ID: RISK-013
Area: compatibility boundary
Severity: BLOCKER
Status: OPEN
Description:
  Internal RuntimePlan/EIR/cache artifacts may be accidentally treated as public bytecode or package ABI.
Spec References:
  - SPEC-P3-FREEZE
  - SPEC-P3-CACHE
Affected Work Packages:
  - WP-05
  - WP-06
  - WP-16
  - WP-17
Mitigation:
  Mark artifacts internal and discardable.
  Reject public-bytecode cache claims.
Owner:
  Main Agent
Review Gate:
  G0
  G6
```

### RISK-014 · Object Layout ABI Leak

```text
Risk ID: RISK-014
Area: runtime value / host / JIT boundary
Severity: BLOCKER
Status: OPEN
Description:
  Implementation may expose VM object layout or raw object pointers to host/JIT/public APIs.
Spec References:
  - SPEC-P3-FREEZE
  - SPEC-P3-HOST
  - SPEC-P3-PROFILE
Affected Work Packages:
  - WP-08
  - WP-14
  - WP-15
  - WP-17
Mitigation:
  Use ObjRef/handle model.
  Forbid raw pointer retention without HostRoot.
Owner:
  Main Agent
Review Gate:
  G3
  G6
```

### RISK-015 · ReadOnlyView Misinterpreted as Deep Freeze

```text
Risk ID: RISK-015
Area: readonly semantics
Severity: MAJOR
Status: OPEN
Description:
  Implementation may treat ReadOnlyView as deep immutable/frozen copy or as hashability wrapper.
Spec References:
  - SPEC-P3-READONLY
  - SPEC-P3-VALUES
Affected Work Packages:
  - WP-08
  - WP-13
  - WP-17
Mitigation:
  Implement shallow view semantics.
  Test mutation through original object is reflected.
Owner:
  Main Agent
Review Gate:
  G5
```

### RISK-016 · Conformance Test Gaps

```text
Risk ID: RISK-016
Area: testing
Severity: MAJOR
Status: OPEN
Description:
  Implementation may pass happy-path tests but miss negative and diagnostic behavior.
Spec References:
  - SPEC-P3-VALID
Affected Work Packages:
  - WP-18
  - WP-19
Mitigation:
  Require positive and negative matrix.
  Require source diagnostic tests.
Owner:
  Main Agent
Review Gate:
  G5
  G6
```

### RISK-017 · Fast Interpreter Scope Explosion

```text
Risk ID: RISK-017
Area: interpreter implementation
Severity: MAJOR
Status: OPEN
Description:
  WP-17 may become too broad and absorb unresolved earlier work.
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-RTP
  - SPEC-P3-HELPERS
Affected Work Packages:
  - WP-17
Mitigation:
  Start only after dependencies pass gates.
  Split op families if necessary.
Owner:
  Main Agent
Review Gate:
  G2
  G4
```

### RISK-018 · Implementation Before Traceability

```text
Risk ID: RISK-018
Area: process
Severity: MAJOR
Status: OPEN
Description:
  Coding may begin before traceability and gates are established.
Spec References:
  - AGENT-MASTER-PLAN.md
  - GATE-CHECKLIST.md
Affected Work Packages:
  - all
Mitigation:
  Require WP-00 through WP-03 before implementation packages.
Owner:
  Main Agent
Review Gate:
  G0
  G1
  G2
```

---

## 5. Initial Risk Priority

Highest priority risks:

```text
RISK-001 specification drift
RISK-008 structured unwinding bugs
RISK-011 capability or host boundary bypass
RISK-013 public bytecode leak
RISK-014 object layout ABI leak
```

These are blockers if triggered.

---

## 6. Risk Review Cadence

Risk review should occur:

```text
at G2 dependency review
at G3 design review
at G6 integration review
before any main+3 or main+4 parallel task
before WP-17 fast interpreter core begins
before WP-19 integration review
```

---

## 7. Risk Closure Criteria

A risk may close only if:

```text
affected work packages are complete
relevant tests exist
gate evidence exists
no open dependent risk remains
```

A risk may be mitigated but remain open if future work can reintroduce it.

---

## 8. Completion Criteria

This risk register is complete for initial planning when:

```text
all major subsystems have risk coverage
all blocker compatibility boundaries are tracked
Agent coordination risks are tracked
token-budget risk is tracked
testing gaps are tracked
```

This initial version satisfies those criteria.

---

## 9. Concrete Planning Risk Amendments

Added: 2026-06-29 11:00:40

### RISK-019 · Abstract Plan Without Coding Steps

```text
Risk ID: RISK-019
Area: implementation planning
Severity: BLOCKER
Status: OPEN
Description:
  Agent plan may remain at governance level and fail to provide concrete coding steps, directories, crates, files, tests, and execution order.
Spec References:
  - AGENT-MASTER-PLAN.md
  - IMPLEMENTATION-CODING-PLAN.md
Affected Work Packages:
  - WP-00
  - WP-01
  - all implementation packages
Mitigation:
  Use IMPLEMENTATION-CODING-PLAN.md as the concrete execution sequence.
  Require G3/G4 artifact checks.
Owner:
  Main Agent
Review Gate:
  G3
  G4
```

### RISK-020 · Workspace Bootstrap Omitted

```text
Risk ID: RISK-020
Area: repository setup
Severity: BLOCKER
Status: OPEN
Description:
  Agent may start coding before creating root control files, docs placement, crate directories, tests, and Agent record directories.
Spec References:
  - IMPLEMENTATION-CODING-PLAN.md
Affected Work Packages:
  - WP-00
Mitigation:
  WP-00 must create or align workspace directories and root files before implementation packages start.
Owner:
  Main Agent
Review Gate:
  G0
  G2
  G4
```

