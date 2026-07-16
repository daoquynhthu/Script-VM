# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Position

| Track | Status |
|-------|--------|
| T-P1 | COMPLETE |
| T-P2 | S00–S02 COMPLETE |
| T-P3L | **R00–R03 COMPLETE** (bootstrap) |
| CLI | `script-vm` |

---

## Pipeline

```text
source → AnalyzedModule → IrUnit (+ Map, control_regions)
      → validate_ir_unit → EIR → Interpreter
      → optional RuntimePlan (exports + function_plans from SIR/EIR)
```

```rust
script_eir_lower::compile_source_via_sir(src, "main")?;
script_eir_lower::compile_executable(src, "main")?; // + plan shell
```

### R03

- `SirNode::Map` / `HELPER_CONSTRUCT_MAP`
- `ExecutableUnit.sir` retained; plan digests + export list + EIR function plans

### Residual

- `m[k]` index syntax in parser  
- for over variables  
- finally / structured unwind on raise  
- full RuntimePlan generation (not fixture scaffold)  

---

## Next

WP-R04: index/subscript AST+EIR; or CLI sample scripts + docs.
