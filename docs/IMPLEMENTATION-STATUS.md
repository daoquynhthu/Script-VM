# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Current position

| Track | Status |
|-------|--------|
| T-P3B Phase 3 bootstrap | ARCHIVED COMPLETE |
| **T-P1** language frontend v0 | **COMPLETE** (WP-L00..L05) |
| **T-P2** SIR | **IN PROGRESS** — WP-S00/S01 COMPLETE |
| T-P3L normative lowering | not started |
| T-DEMO codegen | QUARANTINED |

**Next:** WP-S02 (SIR depth) or start **WP-R00** lowering plan against CONTROL-LOWERING-ROUND2 (prefer S02 first if types/regions needed).

---

## Pipeline (honest)

```text
source
  → script_sema::check_module / analyze_source   [T-P1]
  → script_lower::materialize_sir / compile_to_sir [T-P2 S00]
  → sir_validate::validate_ir_unit                 [T-P2 S01]
  → (next) SIR → RuntimePlan/EIR                   [T-P3L]
  → vm_eval
```

Demo `script_codegen` still exists but is **not** the plan path.

---

## T-P2 S00/S01 deliverables

- `IrUnit.sources` required table populated  
- `interface_exports` + `exports`  
- `materialize_sir(AnalyzedModule)`  
- `validate_ir_unit` codes SIR001–SIR011  

---

## Primary APIs

```rust
let a = script_sema::check_module(src);
let unit = script_lower::materialize_sir(&a, "main")?;
let v = sir_validate::validate_ir_unit(&unit);
assert!(v.is_valid());
```
