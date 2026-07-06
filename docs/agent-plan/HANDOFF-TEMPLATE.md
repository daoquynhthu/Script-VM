# Handoff Template

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications, `AGENT-MASTER-PLAN.md`, and `AGENT-OPERATING-PROTOCOL.md`  
Created: 2026-06-29 10:32:50

---

## 0. Purpose

This document defines mandatory handoff formats for completed Agent work.

A handoff is required when:

```text
a work package is completed
a subordinate Agent returns output
a gate fails
a task is stopped early
a task is handed to another Agent
```

The handoff must make work auditable, reviewable, and continuable.

---

## 1. Main Agent Work Package Handoff

Use this format when the main Agent completes or pauses a work package.

```text
# Work Package Handoff

Task ID:
Work Package:
Date:
Agent Mode:
Owner:

## 1. Summary

Completed:

Paused / Blocked:

Reason:

## 2. Frozen Spec References Used

- ...

## 3. Inputs

- ...

## 4. Outputs

Changed / Created files:

- ...

Generated artifacts:

- ...

## 5. Gate Results

G0 Scope Gate:
G1 Spec Reference Gate:
G2 Dependency Gate:
G3 Design Gate:
G4 Implementation Gate:
G5 Validation Gate:
G6 Integration Gate:
G7 Handoff Gate:

## 6. Tests

Tests run:

Tests added:

Tests missing:

Known failures:

## 7. Risks

New risks:

Updated risks:

Resolved risks:

## 8. Subordinate Agent Use

Subordinate Agents used:

If yes:

- Agent role:
- Task ID:
- Output accepted:
- Output rejected:
- Review notes:

## 9. Open Questions

- ...

## 10. Next Step

Recommended next bounded task:

Required gate before next task:

## 11. Completion Decision

Decision:

```text
complete
complete_with_notes
blocked
deferred
rejected
```
```

---

## 2. Subordinate Agent Handoff

Use this format for every subordinate Agent output.

```text
# Subordinate Agent Handoff

Task ID:
Assigned Agent Role:
Date:
Token Budget:
Stop Condition Reached:

## 1. Scope Restatement

Assigned scope:

What was inspected:

What was not inspected:

## 2. Frozen Spec References

- ...

## 3. Findings

Finding 1:

```text
type:
severity:
description:
spec reference:
evidence:
recommendation:
```

Finding 2:

...

## 4. Output Requested by Main Agent

Expected output:

Actual output:

Missing output:

## 5. Forbidden Action Compliance

Confirmed:

```text
did not modify frozen specs
did not create new semantics
did not widen scope
did not approve gates
did not merge output
```

Exceptions:

- ...

## 6. Risks

- ...

## 7. Stop Reason

```text
scope complete
input missing
contradiction found
scope ambiguous
token budget reached
requires main Agent judgment
```

## 8. Recommendation

Recommended next action:

```text
accept
accept_with_edits
request_revision
reject
defer
```
```

---

## 3. Gate Failure Handoff

Use this format when a gate fails.

```text
# Gate Failure Handoff

Task ID:
Gate:
Date:
Checked By:

## 1. Failure Summary

Failed check:

Why it failed:

Blocking severity:

```text
blocker
major
minor
info
```

## 2. Evidence

- ...

## 3. Frozen Spec References

- ...

## 4. Required Correction

Required action:

Owner:

Expected output:

## 5. Retry Conditions

Gate may be retried when:

- ...

## 6. Parallelism Restriction

Until resolved:

```text
main-only
main+1 allowed
parallelism forbidden
```
```

---

## 4. Early Stop Handoff

Use this format when a task stops before completion.

```text
# Early Stop Handoff

Task ID:
Date:
Agent:
Stop Reason:

## 1. Completed Portion

- ...

## 2. Uncompleted Portion

- ...

## 3. Why Work Stopped

- ...

## 4. Frozen Spec References Involved

- ...

## 5. Risks Created

- ...

## 6. Recommended Next Step

- ...
```

---

## 5. Merge Review Handoff

Use this format when the main Agent reviews subordinate output.

```text
# Merge Review Handoff

Subordinate Task ID:
Main Work Package:
Date:
Reviewed By:

## 1. Output Summary

- ...

## 2. Accepted Content

- ...

## 3. Rejected Content

- ...

## 4. Rejection Reasons

- ...

## 5. Spec Conflicts Found

- ...

## 6. Scope Drift Found

- ...

## 7. Merge Decision

```text
accept
accept_with_edits
request_revision
reject
defer
```

## 8. Follow-Up

- ...
```

---

## 6. Minimum Handoff Requirements

Every handoff must include:

```text
Task ID
date
Agent role or owner
frozen spec references
outputs
gate status or stop reason
risks
next step
```

A handoff is invalid if it lacks:

```text
spec references
changed/output file list
decision
next bounded task or stop reason
```

---

## 7. Completion Criteria

This template is complete when it can cover:

```text
normal completion
subordinate output
gate failure
early stop
merge review
```
