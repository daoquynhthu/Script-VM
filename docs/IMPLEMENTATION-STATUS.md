# Implementation Status Snapshot

Document class: Non-normative rolling status  
Updated: 2026-07-14 (Phase 1 WP-20..24 bootstrap)

---

## Pipeline

```text
source
  -> script_lex::lex
  -> script_parse::parse_module
  -> script_sema::analyze_module
  -> script_lower::compile_to_sir  =>  sir::IrUnit
  -> (next) SIR → RuntimePlan/EIR → vm_eval
```

| WP | Status | Crate / notes |
|----|--------|----------------|
| WP-20 | COMPLETE | process + TRACE |
| WP-21 | COMPLETE | `script_lex` |
| WP-22 | COMPLETE | `script_parse` (+ import/export/raise/assert, for/break/continue) |
| WP-23 | COMPLETE | `script_sema` |
| WP-24 | COMPLETE | `sir` IrUnit + `script_lower` |
| Next | WP-25 | SIR → RuntimePlan/EIR lowering into existing VM |

Phase 3 bootstrap VM (WP-00..19): **CLOSED**.

---

## Architecture books for WP-25

- `SPEC-P3-LOWERING` / SIR lowering coverage  
- `SPEC-P3-EIR`, `SPEC-P3-RTP`, `SPEC-P3-VALID`  
- keep `SPEC-P3-FREEZE` boundaries  

---

## Honest residuals

- Full SIR schema rounds (types, patterns, control regions filled)  
- Full Phase 1 grammar (match, record, enum, try, …)  
- End-to-end execute `fib` via VM  
