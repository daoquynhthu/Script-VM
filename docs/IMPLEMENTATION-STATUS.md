# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Current position

| Item | Status |
|------|--------|
| Active track | **T-P1** |
| WP-L00 | **COMPLETE** (guide + GAP matrix + WP-L* registry) |
| WP-L01 | **COMPLETE** (lexical SPEC §3–§6 + tests) |
| **Next** | **WP-L02** grammar/AST v0 surface |
| T-P3B | ARCHIVED COMPLETE |
| T-DEMO (`script_codegen`) | QUARANTINED |

---

## T-P1 progress

| Stage | Status |
|-------|--------|
| L0 plan + gap baseline | COMPLETE |
| L1 lexical | COMPLETE (`script_lex` 26 tests; `P1-TEST-MATRIX` LX-01..24) |
| L2 grammar/AST | NEXT |
| L3 sema | pending |
| L4 diagnostics API | pending |
| L5 acceptance | pending |

---

## Key docs

```text
PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md
docs/phase-1/P1-GAP-MATRIX.md
docs/phase-1/P1-TEST-MATRIX.md
```

---

## Honesty

- Frontend lexical layer aligned for v0 with explicit PARTIAL/DEFER in GAP matrix.  
- Normative SIR→EIR pipeline **not** claimed complete.  
- Demo codegen **not** used as acceptance.  
