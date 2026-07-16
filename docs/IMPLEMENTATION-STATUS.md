# Implementation Status Snapshot

Updated: 2026-07-14 (Phase 1 WP-20..25 bootstrap; **fib end-to-end**)

---

## Pipeline (working)

```text
source
  -> script_lex
  -> script_parse
  -> script_sema
  -> script_lower (IrUnit)          [optional SIR materialization]
  -> script_codegen (EirModule)     [WP-25]
  -> vm_eval::Interpreter
```

**Milestone:** `fib(10)` → **55** via `script_codegen::compile_source` + interpreter.

| WP | Status |
|----|--------|
| WP-00..19 Phase 3 VM | CLOSED |
| WP-20..23 lex/parse/sema | COMPLETE |
| WP-24 SIR IrUnit | COMPLETE |
| WP-25 source→EIR execute | **COMPLETE** |

---

## Next (optional product depth)

- for / break / continue / lists / short-circuit and-or in codegen  
- match / record / enum / try  
- fuller SIR schema rounds  
- RuntimePlan packaging + cache digests for production  
- real `print` host I/O  

---

## Architecture books

- Phase 1: `SPEC-P1-LANG`  
- Phase 2: `SPEC-P2-IR` (SIR depth)  
- Phase 3: `SPEC-P3-EIR`, `SPEC-P3-CALL`, `SPEC-P3-LOWERING`  
