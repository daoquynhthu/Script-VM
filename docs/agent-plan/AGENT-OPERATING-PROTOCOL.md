# Agent Operating Protocol

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications and `AGENT-MASTER-PLAN.md`  
Created: 2026-06-29 10:32:50

---

## 0. Purpose

This document defines the operating rules for the main Agent and subordinate Agents during implementation planning and execution.

It is designed for a team model with:

```text
1 main Agent
0–4 subordinate Agents
```

This document does not define VM semantics.

It defines:

```text
Agent responsibilities
dispatch rules
parallelism limits
token-cost controls
review and merge protocol
failure handling
handoff discipline
```

---

## 1. Authority

The authority order is:

```text
Frozen normative specifications
  > PHASE-3-FREEZE.md
  > AGENT-MASTER-PLAN.md
  > this operating protocol
  > work package notes
  > subordinate Agent output
```

If this document conflicts with `AGENT-MASTER-PLAN.md`, the master plan wins.

If any implementation plan conflicts with the frozen normative baseline, the frozen normative baseline wins.

---

## 2. Main Agent Responsibilities

The main Agent is responsible for:

```text
global context preservation
frozen specification reference discipline
work package decomposition
subordinate Agent dispatch
subordinate Agent result review
gate pass/fail decisions
risk classification
merge decisions
handoff report generation
plan document updates
```

The main Agent must not delegate:

```text
final architecture interpretation
final merge authority
frozen-spec conflict resolution
gate approval
freeze-baseline interpretation
```

The main Agent may ask for localized analysis, but remains responsible for correctness.

---

## 3. Subordinate Agent Responsibilities

Subordinate Agents are bounded workers.

They may perform:

```text
specification trace check
localized test matrix construction
localized implementation-option review
localized integration-risk review
localized documentation consistency scan
```

They must not:

```text
change frozen specifications
create new normative requirements
change work package scope
make final architecture decisions
approve gates
merge their own output
silently expand input scope
```

Every subordinate Agent output is advisory until reviewed by the main Agent.

---

## 4. Agent Roles

### 4.1 A1 · Spec Trace Agent

Primary function:

```text
verify that a task correctly references frozen specification documents
```

Allowed outputs:

```text
missing references
incorrect references
potential plan/spec mismatches
traceability suggestions
scope drift warnings
```

Forbidden outputs:

```text
new semantics
implementation architecture decisions
final gate approval
```

### 4.2 A2 · Runtime Design Agent

Primary function:

```text
review localized implementation structure against frozen references
```

Allowed outputs:

```text
data-structure dependency notes
interface boundary notes
implementation sequencing notes
localized risk notes
```

Forbidden outputs:

```text
global runtime architecture replacement
new EIR op semantics
new RuntimePlan fields
helper registry changes without trace reference
```

### 4.3 A3 · Validation/Test Agent

Primary function:

```text
derive test and validation obligations from frozen references
```

Allowed outputs:

```text
positive test requirements
negative test requirements
validation gate coverage
diagnostic coverage requirements
regression-risk notes
```

Forbidden outputs:

```text
weakening validation requirements
removing negative tests for convenience
treating tests as substitute for specification
```

### 4.4 A4 · Integration/Risk Agent

Primary function:

```text
identify cross-work-package risks and integration hazards
```

Allowed outputs:

```text
dependency risks
merge risks
token-cost risks
ordering risks
parallelism risks
```

Forbidden outputs:

```text
unbounded redesign
implementation shortcuts
gate approval
```

---

## 5. Dispatch Preconditions

A subordinate Agent task may be dispatched only if all are true:

```text
task has a unique Task ID
scope is bounded
frozen specification references are explicit
allowed inputs are listed
forbidden actions are listed
expected output is structured
validation criteria are objective
token budget is specified
stop conditions are specified
```

If any precondition is missing, the main Agent must not dispatch the task.

---

## 6. Dispatch Template

Every subordinate Agent dispatch must use this structure:

```text
Task ID:
Assigned Agent Role:
Scope:
Frozen Spec References:
Allowed Inputs:
Forbidden Actions:
Expected Output:
Validation Criteria:
Token Budget:
Stop Conditions:
Handoff Format:
```

### 6.1 Task ID

Format:

```text
WP-XX-A[role]-NN
```

Example:

```text
WP-06-A3-01
```

Meaning:

```text
WP-06 work package
A3 Validation/Test Agent
task 01
```

### 6.2 Scope

Scope must describe:

```text
what to inspect
what to produce
what not to inspect
```

Scope must be small enough for the main Agent to review.

### 6.3 Frozen Spec References

References must use either:

```text
reference aliases from AGENT-MASTER-PLAN.md
concrete frozen document names
```

A subordinate Agent must not operate from memory alone.

### 6.4 Forbidden Actions

Every dispatch must include task-specific forbidden actions.

Generic forbidden actions always include:

```text
do not modify frozen specs
do not create new semantics
do not widen scope
do not approve gates
do not merge output
```

### 6.5 Token Budget

Token budget must be expressed as:

```text
low
medium
high
```

Meaning:

```text
low     narrow scan or short matrix
medium  one localized subsystem
high    rare, cross-document check
```

High-budget subordinate tasks require explicit justification.

---

## 7. Parallelism Policy

### 7.1 Maximum

The maximum number of subordinate Agents is:

```text
4
```

### 7.2 Default

Default mode:

```text
main-only
```

or:

```text
main+1
```

### 7.3 Parallelism Approval Rule

Before using more than one subordinate Agent, the main Agent must verify:

```text
subtasks are independent
outputs do not require shared writes
each output has objective acceptance criteria
each subtask has separate spec references
merge conflicts are predictable
token cost is justified
```

### 7.4 Parallelism Denial Rule

The main Agent must deny parallelism if:

```text
the problem is architectural
the task requires final interpretation
the scope is unclear
the task is mostly writing rather than checking
the suboutputs cannot be independently validated
```

---

## 8. Review Protocol

Subordinate output must be reviewed in this order:

```text
1. Scope compliance check
2. Frozen spec reference check
3. Forbidden action check
4. Output structure check
5. Factual consistency check
6. Gate relevance check
7. Merge decision
```

Possible decisions:

```text
accept
accept with edits
request bounded revision
reject
defer
```

The main Agent must record the decision in the handoff or work package notes.

---

## 9. Merge Protocol

Subordinate output must not be merged directly.

Before merge, the main Agent must ensure:

```text
no new normative semantics
no conflict with frozen specs
no unverified assumptions
no unreferenced requirements
no unbounded task expansion
```

Merge result must identify:

```text
source Agent role
accepted content
rejected content
reason for rejection if any
affected work package
affected gates
```

---

## 10. Failure Handling

### 10.1 Scope Drift

If a subordinate Agent expands scope:

```text
reject expanded portion
retain only in-scope findings if useful
record scope drift risk
```

### 10.2 Spec Conflict

If subordinate output conflicts with frozen specs:

```text
reject conflicting portion
cite controlling frozen spec
record issue if recurring
```

### 10.3 Insufficient References

If output lacks references:

```text
do not merge
request reference-bound revision or reject
```

### 10.4 Excessive Token Cost

If a subordinate task consumes excessive budget:

```text
stop task
summarize partial useful findings
split remaining work into smaller task
```

---

## 11. Stop Conditions

A subordinate Agent must stop when:

```text
assigned scope is complete
a frozen spec contradiction is found
required input is missing
scope becomes ambiguous
token budget is reached
task would require creating new semantics
task would require final architecture judgment
```

Stop output must include:

```text
reason for stop
completed portion
unresolved portion
recommended next action
```

---

## 12. Main Agent Self-Checks

Before concluding any work package, the main Agent must ask:

```text
Did this task cite frozen specs?
Did it avoid copying normative text?
Did it avoid creating new semantics?
Did it pass the required gates?
Were subordinate outputs reviewed?
Were risks recorded?
Are next steps bounded?
```

A negative answer blocks completion.

---

## 13. Logging Requirements

Each work unit should produce a concise log entry:

```text
Task ID:
Date:
Agent Mode:
Inputs:
Frozen Spec References:
Outputs:
Gate Results:
Risks:
Decision:
Next Step:
```

Logs may be stored in a future task log document or work package section.

---

## 14. Completion Criteria

This operating protocol is complete when it can govern:

```text
main-only work
main+1 delegated review
main+2 independent review
main+3 high-risk cross-check
main+4 maximum parallel review
```

without allowing:

```text
uncontrolled scope expansion
unreviewed merge
specification drift
token waste
gate bypass
