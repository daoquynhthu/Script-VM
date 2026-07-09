# Conformance / Regression Test Matrix (Stage 13 / WP-18)

Document class: Non-normative test inventory  
Rule: Maps in-repo tests to frozen specs and work packages. Does not redefine semantics.

## Layout

| Directory | Purpose | Owning tests |
|-----------|---------|----------------|
| `tests/conformance/` | Positive baseline behaviors | `vm_tests::conformance` |
| `tests/negative/` | Rejection / raise paths | `vm_tests::negative` |
| `tests/diagnostics/` | Source-span / error attachment | `vm_tests::diagnostics` |
| `tests/regression/` | Previously fixed bugs / nested paths | `vm_tests::regression` |
| `tests/fixtures/` | Shared fixture notes | see crate fixtures |

Run:

```text
cargo test -p vm_tests
cargo test --workspace
```

## Trace rows (initial matrix)

| Test ID | Category | Spec / WP | Shipped entry under test |
|---------|----------|-----------|---------------------------|
| CF-01 | conformance | SPEC-P3-VALID, WP-06 | `validate_eir_module` minimal module |
| CF-02 | conformance | SPEC-P3-VALID, WP-05 | `validate_runtime_plan` minimal plan |
| CF-03 | conformance | Stage 12, WP-17 | `Interpreter::run_module` return constant |
| CF-04 | conformance | SPEC-P3-HELPERS, WP-07 | `dispatch_helper` check_shape true path |
| CF-05 | conformance | SPEC-P3-MODULE, WP-11 | module resolve + initialize state |
| CF-06 | conformance | SPEC-P3-HOST, WP-12 | capability grant + host enter/exit |
| CF-07 | conformance | SPEC-P3-READONLY, WP-13 | readonly mutation rejection |
| NG-01 | negative | SPEC-P3-VALID, WP-06 | unknown shape id rejected |
| NG-02 | negative | SPEC-P3-HELPERS, WP-07 | undispatched helper InvalidHelperError |
| NG-03 | negative | SPEC-P3-HOST, WP-12 | missing capability CapabilityError |
| NG-04 | negative | SPEC-P3-MODULE, WP-11 | import cycle ImportCycleError |
| NG-05 | negative | SPEC-P3-VALUES, WP-08 | non-hashable map key TypeError |
| DG-01 | diagnostics | SPEC-P3-ERRORS, WP-04 | construct_error stores code/message |
| RG-01 | regression | SPEC-P3-UNWIND, WP-10 | nested region LIFO defer order |
| RG-02 | regression | SPEC-P3-VALUES, WP-08/09 | immutable cell ReadOnlyError |
