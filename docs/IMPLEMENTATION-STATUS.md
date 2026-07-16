# Implementation Status Snapshot

Document class: Non-normative rolling status (rewritable)  
Updated: 2026-07-16  

**Plan authority:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md` (sole forward guide)

---

## 0. Current position

| Item | Status |
|------|--------|
| **Forward plan** | Unified guide ACTIVE; Phase-1-first |
| **Active track** | **T-P1** Language frontend |
| **Current WP** | **WP-L00** (plan landing + P1-GAP-MATRIX) — next to execute |
| T-P3B (WP-00..19, Stage 0–14) | **ARCHIVED COMPLETE** (bootstrap VM) |
| WP-20..25 | **SUPERSEDED** as plan IDs (prototype/demo assets remain in tree) |
| T-DEMO (`script_codegen`) | **QUARANTINED** — not normative completion evidence |
| T-P2 / T-P3L | Not started (blocked on T-P1) |

---

## 1. How to read state

| Need | Source |
|------|--------|
| What to do next | `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md` §12 |
| Semantics | `ARCHITECTURE/**` frozen specs |
| What changed | `PROGRESS.md` |
| Audit | `ISSUE.md` (last-status-wins) |
| This snapshot | rewrite freely |

---

## 2. Capability honesty

**Have**

- Phase 3 bootstrap VM substrate + EIR interpreter (fixtures / validated EIR)
- Frontend prototypes: `script_lex`, `script_parse`, `script_sema`
- Thin SIR experiment: `script_lower`
- Demo path: `script_codegen` can run fib via AST→EIR (not normative lowering)

**Do not claim**

- Full Phase 1 language frontend complete per SPEC-P1 acceptance (P1-A..F)
- Full Phase 2 SIR materialization
- Normative SIR → RuntimePlan/EIR lowering complete
- Product v0 on the **architecture** pipeline complete

---

## 3. Next actions (only)

1. **WP-L00:** create `docs/phase-1/P1-GAP-MATRIX.md`; register WP-L* in index; mark WP-20..25 SUPERSEDED  
2. **WP-L01:** lexical alignment to SPEC-P1  
3. Then L2→L5 per unified guide  

---

## 4. Architecture books for T-P1

1. `ARCHITECTURE/01-phase-1/freeze/PHASE-1-FREEZE.md`  
2. `ARCHITECTURE/01-phase-1/normative/PHASE-1-LANGUAGE-SPEC.md`  
3. `ARCHITECTURE/01-phase-1/normative/PHASE-1-LANGUAGE-DESIGN.md`  
