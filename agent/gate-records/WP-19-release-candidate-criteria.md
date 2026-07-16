# WP-19 Release Candidate Criteria (implementation)

Document class: Non-normative  
Authority: Subordinate to PHASE-3-FREEZE and AGENT plan  
Scope: **Implementation candidate**, not public product release policy

## Bootstrap Phase 3 “minimal VM candidate” is ready when

```text
[x] Rust workspace compiles with -D warnings
[x] Validators reject malformed RuntimePlan / EIR
[x] Runtime substrate: heap, frames, unwind, modules, call, host hooks
[x] Helper registry: all 47 helpers via central dispatch
[x] Fast interpreter executes validated EIR subset (nested call, mid-block, init)
[x] WP-18 conformance matrix COMPLETE (traceable +/− tests)
[x] G6 integration gate PASS (scan + IG suite + full tests)
[x] CI runs check + test×2 + G6 scan on main
[x] PROGRESS.md / ISSUE.md append-only history present
[x] No public bytecode / CPython ABI / host extern "C" leak (scan)
```

## Explicitly NOT required for this RC

```text
Phase 1 source language front-end product path
Production GC / JIT
Exhaustive industrial language conformance
Public packaging / versioned ABI for RuntimePlan/EIR
```

## Post-RC work

Further product work starts from WP residual / Phase 1 traces (TR-GAP-*), not by reopening freeze.
