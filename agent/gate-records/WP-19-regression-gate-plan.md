# WP-19 Regression Gate Plan

Document class: Non-normative regression plan  
Work Package: WP-19  
Status: **EXECUTED** (2026-07-14)

## Suites

| Suite | Command | Purpose |
|-------|---------|---------|
| Full workspace | `cargo test --workspace` | Unit + integration |
| Full workspace (flake) | second `cargo test --workspace` in CI | Non-determinism guard |
| Negative / conformance | `cargo test -p vm_tests` | WP-18 matrix (CF/NG/DG/RG) + gap_closure |
| Integration | `cargo test -p vm_tests integration::` | TR-018 IG-01..10 |
| G6 scan | `scripts/integration/g6-scan.*` | Forbidden boundary regressions |

## Cross-subsystem regression topics

1. **Validation → execute**: plan + EIR validate then interpreter return (IG-01)
2. **Helper registry ↔ cache digest**: stable digest (IG-02, IG-09)
3. **Host / capability**: missing capability fails (IG-06, IG-07)
4. **Cache freeze boundary**: public bytecode claim rejected (IG-03)
5. **Profile**: fingerprint mismatch rejects reuse (IG-04)
6. **Error layers**: language vs structural counts (IG-08)
7. **Coherence after registry use**: interpreter still runs (IG-10)

## Cache / profile / host residual

Covered by matrix rows CF-18, NG-11/12/19/28 and IG-02..07/09.

## Pass criterion

All suites green under `RUSTFLAGS=-D warnings`; G6 scan exit 0.
