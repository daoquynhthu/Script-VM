# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Current position

| Item | Status |
|------|--------|
| Active track | **T-P1 COMPLETE (v0)** |
| WP-L00..L05 | **COMPLETE** |
| **Next** | **T-P2 / WP-S00** — Phase 2 SIR materialization |
| T-P3B | ARCHIVED COMPLETE |
| T-DEMO | QUARANTINED |

---

## T-P1 acceptance (P1-A..F)

| Criterion | Status |
|-----------|--------|
| P1-A lexical §3–§6 | YES |
| P1-B grammar/AST v0 | YES (DEFER match/record/enum/…) |
| P1-C semantics binding/Bool/NFC | YES |
| P1-D diagnostics line/col | YES (`FrontendDiagnostic`) |
| P1-E GAP matrix | YES |
| P1-F AnalyzedModule API | YES (`check_module` / `analyze_source`) |

**Primary API:**

```rust
script_sema::check_module(source) -> AnalyzedModule
// .ok(), .module, .bindings, .diagnostics
```

---

## Residual DEFER (not T-P1 failure)

```text
match / record / enum / try / defer / use full forms
full type-contract runtime checking
import runtime loading
normative SIR + lowering (T-P2 / T-P3L)
```

---

## Next track

**T-P2:** AnalyzedModule → SIR (`SPEC-P2-*`), `sir_validate`, rewrite thin `script_lower` as needed.
