# Implementation Status Snapshot

Document class: Non-normative rolling status  
Authority: Subordinate to frozen specs, `AGENT.md`, and plan package  
Rule: This file **may be rewritten** as a snapshot. It is **not** a substitute for append-only `PROGRESS.md` / `ISSUE.md`.  
**Do not use `HANDOVER.md` as live status** (update only at session handoff).

Updated: 2026-07-14 (Phase 3 bootstrap **re-verification / closure stamp**)  
Baseline tip at re-check: `77973b7` (closure-doc commits may advance)  
Workspace unit tests (approx.): **368** (sir 1 + vm_core 38 + vm_diag 3 + vm_eval 14 + vm_host 6 + vm_runtime 218 + vm_tests 88)

---

## 0. Phase 3 bootstrap closure

| Item | Status |
|------|--------|
| Agent coding plan Stages 0–14 | **COMPLETE** |
| WP-00 .. WP-19 | **COMPLETE** (bootstrap / substrate) |
| Coding plan §24 definition | **MET** |
| Effective OPEN audit issues | **0** |
| G0–G7 | **PASS** |
| G6 automated scan | **PASS** |
| Closure evidence | `agent/gate-records/PHASE-3-BOOTSTRAP-CLOSURE-20260714.md` |

**Decision: Phase 3 bootstrap minimal VM candidate is CLOSED.**

Not closed as product: Phase 1 language frontend, production GC/JIT, industrial full-language suite (TR-GAP / freeze deferred).

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
| Bootstrap closure evidence | `agent/gate-records/PHASE-3-BOOTSTRAP-CLOSURE-20260714.md` |
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
| 0–12 | Bootstrap through fast interpreter | **COMPLETE** |
| 13 | Conformance and regression | **COMPLETE** (WP-18) |
| 14 | Integration review | **COMPLETE** (WP-19) |

**Coding plan §24 completion (Phase 3 bootstrap substrate): MET.**

---

## 3. Work packages (WP-00–WP-19)

| WP | Title | Status | Notes |
|----|--------|--------|-------|
| WP-00 .. WP-17 | Substrate packages | COMPLETE | Bootstrap / substrate goals |
| WP-18 | Conformance test matrix | **COMPLETE** | `tests/MATRIX.md` + `vm_tests` |
| WP-19 | Integration and regression gates | **COMPLETE** | G6 scan, IG suite, CI, gate records |

All WP-00–WP-19 are **COMPLETE** for current Phase 3 bootstrap goals.  
`PLAN/WORK-PACKAGE-INDEX.md` mirrored to match `docs/agent-plan/` (re-check fixed WP-18/19 drift).

---

## 4. Effective open audit items

No effective OPEN blockers (last-status-wins on `ISSUE.md`).

| Status | Count |
|--------|-------|
| OPEN | 0 |
| RESOLVED | 19 |
| ACCEPTED | 2 (harness `mcps/`; dual region-stack bootstrap) |

---

## 5. Implementation capability (honest)

**In place (minimal VM candidate)**

- Full Rust workspace; validators; runtime substrate; 47-helper dispatch  
- Interpreter subset (nested call, mid-block resume, module init)  
- WP-18 matrix TR-002..017; WP-19 TR-018 IG + G6 automation  
- CI: check + test×2 + `scripts/integration/g6-scan.sh`  

**Scaffold / residual (accepted for bootstrap)**

- `vm_cli` binary prints scaffold message only (crate present; product CLI deferred)  
- Dual control vs unwind region stacks (ISSUE-20260706-010 ACCEPTED)  

**Not in place**

- Phase 1 source language product path (TR-GAP-001)  
- Phase 2 SIR depth (TR-GAP-002)  
- Production GC / JIT  
- Full language product packaging  

---

## 6. Re-verification evidence (this session)

```text
cargo check --workspace   PASS  (RUSTFLAGS=-D warnings)
cargo test --workspace    PASS  (~368 unit tests; vm_tests 88)
g6-scan.ps1               PASS
ISSUE last-status-wins    OPEN=0
PLAN vs docs WP-18/19     synced COMPLETE
TRACEABILITY core rows    COMPLETE (bootstrap Phase 3); GAP rows remain GAP
```

---

## 7. Recommended next work

1. **Do not reopen** WP-00..19 without documented reason.  
2. Product/Phase 1 language work when prioritized (**new** WPs / TR-GAP rows).  
3. Keep CI green; expand substrate only with new TRACEABILITY + matrix rows.  

---

## 8. Related doc sync policy

| File | Policy |
|------|--------|
| `PROGRESS.md` | Append only on real changes |
| `ISSUE.md` | Append only findings / resolutions |
| This file | Rewrite freely as snapshot |
| `HANDOVER.md` | **Only at handoff** |
| Frozen normative docs | Never edit for progress tracking |
