# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Current position

| Track | Status |
|-------|--------|
| T-P3B | ARCHIVED COMPLETE |
| T-P1 frontend v0 | COMPLETE (WP-L00..L05) |
| T-P2 SIR | **WP-S00..S02 COMPLETE** (bootstrap) |
| T-P3L lowering | **WP-R00 COMPLETE** (SIR→EIR bootstrap) |
| T-DEMO `script_codegen` | QUARANTINED (not acceptance) |

---

## Normative pipeline (working)

```text
source
  → script_sema::check_module
  → script_lower::materialize_sir
  → sir_validate::validate_ir_unit   (+ control_regions SIR012+)
  → script_eir_lower::lower_sir_to_eir / compile_source_via_sir
  → vm_eval::Interpreter
```

**Milestone:** `fib(10) → 55` and `print(fib(10)) → 55` on this path  
(crate: `script_eir_lower` pipeline tests).

```rust
let prog = script_eir_lower::compile_source_via_sir(src, "main")?;
// install callables → run_module
```

---

## Residual (honest)

- Full PHASE-3-SIR-LOWERING / CONTROL-LOWERING-ROUND2 coverage  
- RuntimePlan packaging + cache digests for product units  
- for/list/and-or/raise full EIR lower  
- Real host `print` I/O  
- script_codegen remains demo-only  

---

## Next

1. Expand SIR→EIR surface (for, lists, short-circuit)  
2. Emit minimal RuntimePlan alongside EIR  
3. Wire `vm_cli` to `compile_source_via_sir`  
