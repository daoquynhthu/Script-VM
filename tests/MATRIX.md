# Conformance / Regression Test Matrix (Stage 13 / WP-18)

Document class: Non-normative test inventory  
Rule: Maps in-repo tests to frozen specs and work packages. Does not redefine semantics.  
Updated: 2026-07-14  
WP status: **IN_PROGRESS** (expanded TRACEABILITY rows; not full validation matrix)

## Layout

| Directory | Purpose | Owning tests |
|-----------|---------|----------------|
| `tests/conformance/` | Positive baseline behaviors | `vm_tests::conformance` |
| `tests/negative/` | Rejection / raise paths | `vm_tests::negative` |
| `tests/diagnostics/` | Source-span / error attachment | `vm_tests::diagnostics` |
| `tests/regression/` | Previously fixed bugs / nested paths | `vm_tests::regression` |
| `tests/fixtures/` | Shared fixture notes | crate fixtures under `vm_core` / `vm_eval` |

Run:

```text
cargo test -p vm_tests
cargo test --workspace
```

Live stage/WP snapshot: `docs/IMPLEMENTATION-STATUS.md`.

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
| CF-08 | conformance | SPEC-P3-CALL, WP-17 | nested `generic_call` user body (`vm_eval` unit) |
| CF-09 | conformance | SPEC-P3-MODULE, WP-17 | `run_module_init_function` (`vm_eval` unit) |
| CF-10 | conformance | SPEC-P3-READONLY, WP-13 | `readonly(x) is x` false; `==` may true |
| CF-11 | conformance | SPEC-P3-HELPERS, WP-07 | registry size 47; id 99 rejected |
| CF-12 | conformance | SPEC-P3-VALUES, WP-08 | map structural equality (order-independent) |
| CF-13 | conformance | SPEC-P3-READONLY / VALUES, WP-13/08 | map equality through ReadOnlyView |
| CF-14 | conformance | SPEC-P3-VALUES TR-006, WP-08 | string scalar len + in-bounds slice |
| CF-15 | conformance | SPEC-P3-READONLY TR-007, WP-13 | original mutation visible via view read |
| CF-16 | conformance | SPEC-P3-VALUES, WP-08 | float NaN `==` true under values_equal |
| CF-17 | conformance | SPEC-P3-CALL TR-011, WP-12 | bind_arguments positional + pending default |
| CF-18 | conformance | SPEC-P3-CACHE TR-014, WP-16 | digest includes helper registry; internal cache ok |
| CF-19 | conformance | SPEC-P3-EIR TR-015, WP-17 | interpreter branch true path |
| CF-20 | conformance | SPEC-P3-EIR TR-015, WP-17 | interpreter binary add |
| CF-21 | conformance | SPEC-P3-EIR TR-015, WP-17 | interpreter raise terminator |
| NG-01 | negative | SPEC-P3-VALID, WP-06 | unknown shape id rejected |
| NG-02 | negative | SPEC-P3-HELPERS, WP-07 | out-of-range helper id → InvalidHelperError (id 99) |
| NG-03 | negative | SPEC-P3-HOST, WP-12 | missing capability CapabilityError |
| NG-04 | negative | SPEC-P3-MODULE, WP-11 | import cycle ImportCycleError |
| NG-05 | negative | SPEC-P3-VALUES, WP-08 | non-hashable map key TypeError |
| NG-06 | negative | SPEC-P3-VALUES TR-006, WP-08 | NaN map key TypeError |
| NG-07 | negative | SPEC-P3-VALUES TR-006, WP-08 | string slice bounds IndexError |
| NG-08 | negative | SPEC-P3-CONTROL TR-008, WP-09 | uninitialized slot read UninitializedBindingError |
| NG-09 | negative | SPEC-P3-CALL TR-011, WP-12 | wrong arity ArityError |
| NG-10 | negative | SPEC-P3-CALL TR-011, WP-12 | duplicate named argument ArityError |
| NG-11 | negative | SPEC-P3-CACHE TR-014, WP-16 | public bytecode cache claim rejected |
| NG-12 | negative | SPEC-P3-CACHE TR-014, WP-16 | helper registry digest mismatch rejected |
| NG-13 | negative | SPEC-P3-EIR TR-015, WP-17 | branch non-bool raises |
| NG-14 | negative | SPEC-P3-MODULE TR-010, WP-11 | duplicate export name rejected |
| NG-15 | negative | SPEC-P3-GC-META TR-013, WP-15 | may-collect without RootMap rejected |
| DG-01 | diagnostics | SPEC-P3-ERRORS, WP-04 | construct_error stores code/message/span |
| RG-01 | regression | SPEC-P3-UNWIND, WP-10 | nested region LIFO defer order |
| RG-02 | regression | SPEC-P3-VALUES, WP-08/09 | immutable cell ReadOnlyError |
| RG-03 | regression | SPEC-P3-CALL / WP-17 | interpreter minimal module still returns |
| RG-04 | regression | SPEC-P3-UNWIND TR-009, WP-10 | finally raise overrides pending return |
| RG-05 | regression | SPEC-P3-UNWIND TR-009, WP-10 | finally raise suppresses prior raise |

Note: CF-08/CF-09 and mid-block resume live under `cargo test -p vm_eval interpreter::` rather than `vm_tests`.
