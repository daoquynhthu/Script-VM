# Repository Process

Document class: Non-normative implementation process  
Created: 2026-06-30  
Updated: 2026-07-10

## Working Layout

```text
AGENT.md                      workflow contract
PROGRESS.md                   append-only progress log
ISSUE.md                      append-only audit findings
docs/IMPLEMENTATION-STATUS.md live stage/WP snapshot (rewritable)
HANDOVER.md                   session handoff only (not live status)
ARCHITECTURE/                 frozen specification archive
PLAN/                         agent implementation plans
docs/agent-plan/              agent plan package + local-reference-map.md
docs/frozen-specs/            frozen spec routing index
crates/                       Rust workspace
tests/                        Stage 13 matrix inventory + READMEs
                              (executable suites primarily in crates/vm_tests)
scripts/                      check/test/validate helpers
.github/workflows/            CI
agent/                        handoffs, gate records, task logs
```

## Task Log Convention

- Record implementation changes only in `PROGRESS.md`.
- Record audit findings only in `ISSUE.md`.
- Refresh `docs/IMPLEMENTATION-STATUS.md` when stages/WP status or baseline commit change.
- Update `HANDOVER.md` only when ending a handoff session.
- Each progress entry cites work package ID, spec references, gates, and tests.

## ISSUE.md reading rule

Append-only history may contain multiple entries for one ID.  
**Effective status = last `Status:` for that ISSUE-ID.**

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

CI mirrors check + double test with `RUSTFLAGS=-D warnings` (see `.github/workflows/ci.yml`).

Report tests run, tests added, and tests not run in every handoff.

## Handoff Storage

Store session handoffs under:

```text
agent/handoffs/
```
