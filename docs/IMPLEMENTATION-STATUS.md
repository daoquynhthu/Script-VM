# Implementation Status Snapshot

Document class: Non-normative rolling status  
Authority: Subordinate to frozen specs, `AGENT.md`, and plan package  
Rule: This file **may be rewritten** as a snapshot. It is **not** a substitute for append-only `PROGRESS.md` / `ISSUE.md`.  
**Do not use `HANDOVER.md` as live status** (update only at session handoff).

Updated: 2026-07-14  
Baseline commit: `0b21869`  
Workspace unit tests (approx.): **358** (sir 1 + vm_core 38 + vm_diag 3 + vm_eval 14 + vm_host 6 + vm_runtime 218 + vm_tests 78)

---

## 1. How to read project state

| Need | Source of truth |
|------|-----------------|
| What changed (history) | `PROGRESS.md` (append-only) |
| Audit findings | `ISSUE.md` — **last entry with the same ISSUE-ID wins** |
| Coding sequence | `docs/agent-plan/IMPLEMENTATION-CODING-PLAN.md` (Stage 0–14) |
| WP identity / scope | `docs/agent-plan/WORK-PACKAGE-INDEX.md` |
| Live implementation snapshot | **This file** |
| Session handoff | `HANDOVER.md` (only when handing off) |
| Semantics | Frozen specs under `ARCHITECTURE/` / `docs/frozen-specs/` |

Authority order (unchanged):

```text
Frozen Phase 1–3 specs
  > PHASE-3-FREEZE
  > Agent plan docs (docs/agent-plan/, PLAN/)
  > AGENT.md
  > PROGRESS.md / ISSUE.md
  > this snapshot / HANDOVER / notes
```

---

## 2. Coding stages (IMPLEMENTATION-CODING-PLAN)

| Stage | Title | Status |
|-------|--------|--------|
| 0 | Workspace bootstrap | **COMPLETE** |
| 1 | Spec ingestion / trace setup | **COMPLETE** |
| 2 | Crate skeleton and ID types | **COMPLETE** |
| 3 | Runtime error and diagnostics | **COMPLETE** |
| 4 | RuntimePlan model and validator | **COMPLETE** |
| 5 | EIR model and validator | **COMPLETE** |
| 6 | Helper registry | **COMPLETE** (extended well beyond initial scaffold) |
| 7 | Value, heap, frame, control | **COMPLETE** |
| 8 | Structured unwinding | **COMPLETE** |
| 9 | Module runtime | **COMPLETE** (bootstrap) |
| 10 | Call and host boundary | **COMPLETE** (bootstrap) |
| 11 | GC metadata and cache hooks | **COMPLETE** (hooks, not production GC) |
| 12 | Fast interpreter minimal execution | **COMPLETE** (+ nested user-call body, module init, mid-block resume) |
| 13 | Conformance and regression | **COMPLETE** (WP-18 bootstrap matrix; see `tests/MATRIX.md`) |
| 14 | Integration review | **IN_PROGRESS** (CI + G6 notes; not final sign-off) |

Plan completion definition (coding plan §24) is **not fully met**: full source→AST→IR→VM product pipeline remains later (Stage 14 / WP-19 residual).

---

## 3. Work packages (WP-00–WP-19)

Statuses use values from `WORK-PACKAGE-INDEX.md` §2.  
“COMPLETE” means **current Phase 3 implementation goals for that package** (bootstrap / substrate), not industrial production.

| WP | Title | Status | Notes |
|----|--------|--------|-------|
| WP-00 | Agent and repository process | COMPLETE | AGENT/PROGRESS/ISSUE, scripts, CI |
| WP-01 | Frozen spec reference ingestion | COMPLETE | ARCHITECTURE + frozen-specs indexes |
| WP-02 | Traceability matrix | COMPLETE | Matrix exists; keep updating as scope grows |
| WP-03 | ID and schema skeleton | COMPLETE | |
| WP-04 | Runtime error registry | COMPLETE | |
| WP-05 | RuntimePlan model/validation | COMPLETE | |
| WP-06 | EIR model/validation | COMPLETE | Negative ID tests remediated |
| WP-07 | Helper registry and dispatch | COMPLETE | All **47** registry helpers via `dispatch_helper` |
| WP-08 | Value / heap / object refs | COMPLETE | Map structural equality; NaN key rules |
| WP-09 | Frame / slot / control | COMPLETE | Four SlotState modes |
| WP-10 | Structured unwinding | COMPLETE | Nested region + finally/suppress matrixed |
| WP-11 | Module runtime | COMPLETE | Bootstrap; init body via interpreter API |
| WP-12 | Call execution protocol | COMPLETE | Prepare + nested user EIR body for generic_call |
| WP-13 | ReadOnlyView | COMPLETE | Identity/equality + mutation reflection matrixed |
| WP-14 | Host boundary skeleton | COMPLETE | vm_host + H6 helpers + host root matrix |
| WP-15 | GC metadata structures | COMPLETE | Metadata/hooks + RootMap policy matrix |
| WP-16 | Cache compatibility checks | COMPLETE | Digest / public-claim / profile matrix |
| WP-17 | Fast interpreter core | COMPLETE | Stage 12 + nested call/init + mid-block resume |
| WP-18 | Conformance test matrix | **COMPLETE** | `tests/MATRIX.md` + `vm_tests` (78); TR-002..017 covered for bootstrap |
| WP-19 | Integration and regression gates | **IN_PROGRESS** | CI + `agent/gate-records/`; Stage 14 final sign-off open |

---

## 4. Effective open audit items

Computed by **last** `Status:` for each `ISSUE-YYYYMMDD-NNN` in `ISSUE.md` (2026-07-14):

| ID | Effective status | Topic |
|----|------------------|--------|
| ISSUE-20260706-009 | RESOLVED | ReadOnlyView identity semantics exercised |
| ISSUE-20260706-010 | ACCEPTED | Dual region-stack bootstrap |
| ISSUE-20260701-007 | ACCEPTED | Harness MCP tree |
| ISSUE-20260709-001..003 | RESOLVED | Call body / module init / Stage 14 notes |
| Most 20260630 / 20260701 / 20260706-001..008 | RESOLVED | See ISSUE.md trailing entries |

**No effective OPEN blockers** for bootstrap Phase 3 substrate.  
**Do not** treat earlier OPEN lines for the same ID as current.

---

## 5. Implementation capability (honest)

**In place**

- Rust workspace: sir, sir_validate, vm_core, vm_runtime, vm_eval, vm_diag, vm_host, vm_tests, vm_cli  
- EIR / RuntimePlan validation with negatives  
- Runtime: heap, frames/cells, unwind, modules, call substrate, host boundary hooks  
- Helper central dispatch for full canonical table  
- Value equality: immediates, lists/records, maps, ReadOnlyView unwrap  
- Interpreter: literals, slots, branch, binary, loop safepoint, helpers, nested user call, mid-block resume, module init  
- WP-18 bootstrap conformance matrix (positive / negative / diagnostic / regression) mapped to TR-002..TR-017  
- CI: `.github/workflows/ci.yml` (`check` + `test` ×2, `-D warnings`)

**Not in place / bootstrap only**

- Source language front-end pipeline (Phase 1 product path)  
- Production GC / JIT  
- Full language product conformance (beyond Phase 3 substrate matrix)  
- Full pattern-match / full builtin body execution  
- Stage 14 / WP-19 final integration sign-off  

---

## 6. Recommended next work (from plan)

1. **WP-19**: Stage 14 integration review — formal G6 sign-off checklist, public-bytecode / ABI scans, final gate evidence  
2. Keep matrix green under CI; expand only when new substrate lands  

---

## 7. Related doc sync policy

| File | Policy |
|------|--------|
| `PROGRESS.md` | Append only on real changes |
| `ISSUE.md` | Append only findings / resolutions |
| This file | Rewrite freely as snapshot |
| `HANDOVER.md` | **Only at handoff** (not part of routine status sync) |
| Frozen normative docs | Never edit for progress tracking |
