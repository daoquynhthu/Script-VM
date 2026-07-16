# Agent Implementation Plan Package

Updated: 2026-07-16

This package contains the Agent-team implementation planning framework for Script VM work after Phase 1–3 **specification freeze**.

---

## 唯一前向入口（必读）

```text
PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md
```

**Sole forward-looking plan guide.** Defines tracks, Phase-1-first strategy, work-package series `WP-L*` / `WP-S*` / `WP-R*` / `WP-V*`, and disposition of legacy WP-00..25 / demo codegen.

Mirror (must match master):

```text
docs/agent-plan/UNIFIED-IMPLEMENTATION-GUIDANCE.md
```

---

## Supporting documents

| File | Role |
|------|------|
| `AGENT.md` (repo root) | Workflow, PROGRESS/ISSUE, hard stops |
| `AGENT-MASTER-PLAN.md` | Agent team model, gates concepts, aliases |
| `AGENT-OPERATING-PROTOCOL.md` | Dispatch / merge rules |
| `GATE-CHECKLIST.md` | G0–G7 criteria |
| `HANDOFF-TEMPLATE.md` | Handoff format |
| `TRACEABILITY-MATRIX.md` | Spec ↔ implementation rows |
| `RISK-REGISTER.md` | Risks |
| `IMPLEMENTATION-CODING-PLAN.md` | **Archive:** Phase 3 bootstrap Stage 0–14 (executed) |
| `WORK-PACKAGE-INDEX.md` | WP registry (legacy + superseded notes; extend per unified guide) |
| `MANIFEST.md` | Package inventory |

Live implementation snapshot (rewritable):

```text
docs/IMPLEMENTATION-STATUS.md
```

---

## Authority order

```text
Frozen normative specifications (ARCHITECTURE/)
  > PHASE-*-FREEZE.md
  > UNIFIED-IMPLEMENTATION-GUIDANCE.md   ← forward plan
  > Agent governance plans (MASTER / PROTOCOL / GATES)
  > AGENT.md
  > PROGRESS.md / ISSUE.md
  > IMPLEMENTATION-STATUS / HANDOVER
```

These documents are **non-normative**. They must not redefine frozen VM / language / IR semantics.

---

## Historical note

WP-00..19 and Stage 0–14 delivered **Phase 3 bootstrap VM substrate** (complete for that scope).  
WP-20..25 were experimental product-path spikes; they are **superseded** as plan IDs by the unified guide (§8).
