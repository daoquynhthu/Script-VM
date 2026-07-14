# Agent Implementation Plan Package

Updated: 2026-07-10

This package contains the Agent-team implementation planning framework for post-freeze Script VM implementation work.

This package is not just governance. It includes a concrete coding execution plan.

## Entry documents

```text
../../AGENT.md                         repository workflow contract
IMPLEMENTATION-CODING-PLAN.md          Stage 0–14 coding sequence
AGENT-MASTER-PLAN.md                   team / parallelism / WP model
WORK-PACKAGE-INDEX.md                  WP-00–WP-19 scope (Status fields maintained)
../../docs/IMPLEMENTATION-STATUS.md    live implementation snapshot (rewritable)
../../PROGRESS.md                      append-only change log
../../ISSUE.md                         append-only audit log
```

Use:

```text
IMPLEMENTATION-CODING-PLAN.md
```

for concrete repository layout, directory creation, crate order, module order, test order, and step-by-step implementation sequence.

Use:

```text
docs/IMPLEMENTATION-STATUS.md
```

for **current** stage/WP completion snapshot. Do not treat stale handoff notes as live status.

## Authority order

```text
Frozen normative specifications
  > PHASE-3-FREEZE.md
  > Agent implementation plan documents
  > AGENT.md
  > PROGRESS.md / ISSUE.md
```

## Document map

| File | Role |
|------|------|
| `IMPLEMENTATION-CODING-PLAN.md` | Stage order and required actions |
| `WORK-PACKAGE-INDEX.md` | WP identity, deps, gates, **Status** |
| `TRACEABILITY-MATRIX.md` | Spec ↔ implementation ↔ tests |
| `GATE-CHECKLIST.md` | G0–G7 criteria |
| `RISK-REGISTER.md` | Risks |
| `HANDOFF-TEMPLATE.md` | Handoff format |
| `local-reference-map.md` | Spec alias routing |
| `AGENT-OPERATING-PROTOCOL.md` | Main/sub-agent rules |

## Note on PLAN/

Repository also has `PLAN/` as a historical/canonical plan tree. Prefer **`docs/agent-plan/`** for day-to-day agent work; keep both WORK-PACKAGE Status fields aligned when updating.
