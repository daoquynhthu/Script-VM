# Repository Process

Document class: Non-normative implementation process  
Created: 2026-06-30

## Working Layout

```text
AGENT.md              workflow contract
PROGRESS.md           append-only progress log
ISSUE.md              append-only audit findings
ARCHITECTURE/         frozen specification archive
PLAN/                 agent implementation plans (canonical)
docs/agent-plan/      agent plan mirror + local-reference-map.md
docs/frozen-specs/    frozen spec routing index
crates/               Rust workspace
tests/                conformance and regression tests (planned)
scripts/              check/test/validate helpers
agent/                handoffs, gate records, task logs
```

## Task Log Convention

- Record implementation changes only in `PROGRESS.md`.
- Record audit findings only in `ISSUE.md`.
- Each progress entry cites work package ID, spec references, gates, and tests.

## Branch / Commit Discipline

- Prefer one work-package subtask per change set.
- Do not mix specification edits with implementation.
- Do not rewrite frozen specifications through code.

## Test Command Reporting

```text
cargo metadata
cargo check --workspace
cargo test --workspace
```

Report tests run, tests added, and tests not run in every handoff.

## Handoff Storage

Store session handoffs under:

```text
agent/handoffs/
```