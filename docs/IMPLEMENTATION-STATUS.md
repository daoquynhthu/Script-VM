# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Current position

| Item | Status |
|------|--------|
| Active track | **T-P1** |
| WP-L00..L03 | **COMPLETE** |
| **Next** | **WP-L04** diagnostics + `AnalyzedModule` API |
| T-P3B | ARCHIVED COMPLETE |
| T-DEMO | QUARANTINED |

---

## T-P1 stages

| Stage | Status |
|-------|--------|
| L0 | COMPLETE |
| L1 lexical | COMPLETE |
| L2 grammar/AST | COMPLETE |
| L3 sema | **COMPLETE** (Bool §2.3, NFC §3.3, export flag) |
| L4 diagnostics API | NEXT |
| L5 acceptance | pending |

---

## script_sema highlights (L3)

- Conditions: `if` / `while` / `assert` must be Bool (static reject of non-Bool literals/arithmetic)
- `and` / `or` / `not` operand Bool checks
- Binding names NFC-normalized; same-scope NFC clash = duplicate
- `export` → `Binding.exported == true`
- Tests: **22**

---

## Docs

```text
PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md
docs/phase-1/P1-GAP-MATRIX.md
docs/phase-1/P1-TEST-MATRIX.md
```
