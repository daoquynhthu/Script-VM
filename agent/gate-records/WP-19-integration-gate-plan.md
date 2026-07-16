# WP-19 Integration Gate Plan

Document class: Non-normative gate plan  
Work Package: WP-19  
Status: **EXECUTED** (2026-07-14)

## Goal

Treat the Phase 3 bootstrap implementation as a **coherent minimal VM candidate** under G6.

## Full gate run order

```text
G0 Scope → G1 Spec refs → G2 Dependencies → G3 Design → G4 Implementation
  → G5 Validation → G6 Integration → G7 Handoff
```

## G6 required checks (GATE-CHECKLIST)

| Check | How verified |
|-------|----------------|
| No frozen-spec regression | Specs not edited; code cites frozen docs only |
| No unregistered helper | Canonical registry 47 + `dispatch_helper` rejects id 99 (IG-05) |
| No unregistered error code | RuntimeErrorCode / VmStructuralErrorCode registries (IG-08) |
| No unregistered cache/profile impact | digest + profile mismatch rejection (IG-02/04/09) |
| No unwinding bypass | WP-18 RG-01/04..07 + unit tests |
| No capability bypass | IG-06 helper + IG-07 host |
| No host boundary bypass | `execute_host_call` path; no `extern "C"` in runtime/eval/host scan |
| No object layout ABI leak | No public native layout ABI; heap internal |
| No public bytecode leak | `reject_public_bytecode_cache_claim` + G6 scan |
| Existing tests pass | `cargo test --workspace` `-D warnings` |

## Automation

| Tool | Role |
|------|------|
| `scripts/integration/g6-scan.ps1` | Local Windows scan |
| `scripts/integration/g6-scan.sh` | CI Unix scan |
| `cargo test -p vm_tests integration::` | IG-01..IG-10 regression |
| `.github/workflows/ci.yml` | check + test×2 + G6 scan |

## Merge decision (bootstrap)

**APPROVE** Phase 3 substrate as minimal VM candidate for further language front-end work.  
Not a product release of a full source language.
