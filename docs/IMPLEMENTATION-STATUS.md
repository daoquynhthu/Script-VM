# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Current position

| Track | Status |
|-------|--------|
| T-P1 frontend | COMPLETE |
| T-P2 SIR | WP-S00..S02 COMPLETE (bootstrap) |
| T-P3L | **WP-R00 + WP-R01 COMPLETE** (bootstrap) |
| CLI | **`script-vm`** run/eval via SIR→EIR |
| T-DEMO codegen | quarantined |

---

## Run

```powershell
cargo run -p vm_cli -- eval "fib source..."
cargo run -p vm_cli -- run path\to\file.script
```

Pipeline:

```text
source → check_module → materialize_sir → validate_ir_unit
      → lower_sir_to_eir → Interpreter
```

Also: `compile_executable` → EIR + validated RuntimePlan shell.

---

## WP-R01 additions

- List literals → `HELPER_CONSTRUCT_LIST`
- `for x in [..]:` unrolled (list-literal only)
- `and` / `or` short-circuit via branch
- CLI prints return value (`fib(10)` → `55`)

---

## Next

- RuntimePlan fields derived from SIR (not fixture shell only)
- General for-over-variable (needs len/iter helpers)
- break/continue/raise EIR
- Host print to stdout
