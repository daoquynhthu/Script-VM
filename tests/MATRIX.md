# Conformance / Regression Test Matrix (Stage 13 / WP-18)

Document class: Non-normative test inventory  
Rule: Maps in-repo tests to frozen specs and work packages. Does not redefine semantics.  
Updated: 2026-07-14  
WP status: **COMPLETE** (bootstrap Phase 3 — every implemented subsystem has traceable +/− tests)

## Scope note

WP-18 completion means **traceable positive and negative coverage for implemented Phase 3 substrate**, not an exhaustive industrial language conformance suite. Deferred:

```text
Phase 1 source-language product path (TR-GAP-001)
full validation-matrix cells beyond bootstrap fixtures
production GC / JIT / full pattern-match bodies
```

Wire-format EIR tags (`UnknownOpKind`, missing terminator) remain covered by `cargo test -p vm_core eir::validate` unit tests; matrix NG-20..22 cover resolved-module validation failures.

## Layout

| Directory | Purpose | Owning tests |
|-----------|---------|----------------|
| `tests/conformance/` | Positive baseline behaviors | `vm_tests::conformance` + `gap_closure` CF |
| `tests/negative/` | Rejection / raise paths | `vm_tests::negative` + `gap_closure` NG |
| `tests/diagnostics/` | Source-span / error attachment | `vm_tests::diagnostics` + `gap_closure` DG |
| `tests/regression/` | Unwind / fixed paths | `vm_tests::regression` + `gap_closure` RG |
| `tests/fixtures/` | Shared fixture notes | crate fixtures under `vm_core` / `vm_eval` |

Run:

```text
cargo test -p vm_tests
cargo test --workspace
```

Live stage/WP snapshot: `docs/IMPLEMENTATION-STATUS.md`.

## TRACEABILITY coverage (TR-002..TR-017)

| Trace | Work package | Matrix / test evidence |
|-------|--------------|------------------------|
| TR-002 Errors | WP-04 | CF-27, DG-01, NG-17, NG-33 |
| TR-003 RuntimePlan | WP-05 | CF-02, NG-18, NG-19 |
| TR-004 EIR | WP-06 | CF-01, NG-01, NG-20..22 (+ vm_core wire units) |
| TR-005 Helpers | WP-07 | CF-04, CF-11, CF-26, CF-30, NG-02, NG-24 |
| TR-006 Values | WP-08 | CF-12, CF-14, CF-16, NG-05..07 |
| TR-007 ReadOnlyView | WP-13 | CF-07, CF-10, CF-13, CF-15, NG-32 |
| TR-008 Control/slots | WP-09 | NG-08, NG-25, CF-32, RG-02 |
| TR-009 Unwind | WP-10 | RG-01, RG-04..07 |
| TR-010 Module | WP-11 | CF-05, CF-09, NG-04, NG-14, NG-31 |
| TR-011 Call | WP-12 | CF-08, CF-17, CF-28, NG-09/10, NG-26 |
| TR-012 Host | WP-14 | CF-06, CF-22, CF-31, NG-03, NG-16, NG-27, DG-03 |
| TR-013 GC meta | WP-15 | CF-23, CF-29, CF-32, NG-15, NG-29/30 |
| TR-014 Cache | WP-16 | CF-18, NG-11/12, NG-19, NG-28 |
| TR-015 Interpreter | WP-17 | CF-03, CF-08/09, CF-19..21, CF-23..26, NG-13, RG-03, DG-02 |
| TR-016 Validation | multi | CF-01/02, NG-01, NG-15, NG-18..22 |
| TR-017 Conformance | WP-18 | this matrix + `vm_tests` (this document) |

## Trace rows (inventory)

| Test ID | Category | Spec / WP | Shipped entry under test |
|---------|----------|-----------|---------------------------|
| CF-01 | conformance | SPEC-P3-VALID, WP-06 | `validate_eir_module` minimal module |
| CF-02 | conformance | SPEC-P3-VALID, WP-05 | `validate_runtime_plan` minimal plan |
| CF-03 | conformance | Stage 12, WP-17 | `Interpreter::run_module` return constant |
| CF-04 | conformance | SPEC-P3-HELPERS, WP-07 | `dispatch_helper` check_shape true path |
| CF-05 | conformance | SPEC-P3-MODULE, WP-11 | module resolve + initialize state |
| CF-06 | conformance | SPEC-P3-HOST, WP-12 | capability grant + host enter/exit |
| CF-07 | conformance | SPEC-P3-READONLY, WP-13 | readonly mutation rejection |
| CF-08 | conformance | SPEC-P3-CALL TR-011, WP-17 | nested `generic_call` user body |
| CF-09 | conformance | SPEC-P3-MODULE TR-010, WP-17 | `run_module_init_function` |
| CF-10 | conformance | SPEC-P3-READONLY, WP-13 | `readonly(x) is x` false; `==` may true |
| CF-11 | conformance | SPEC-P3-HELPERS, WP-07 | registry size 47; id 99 rejected |
| CF-12 | conformance | SPEC-P3-VALUES, WP-08 | map structural equality (order-independent) |
| CF-13 | conformance | SPEC-P3-READONLY / VALUES | map equality through ReadOnlyView |
| CF-14 | conformance | SPEC-P3-VALUES TR-006 | string scalar len + in-bounds slice |
| CF-15 | conformance | SPEC-P3-READONLY TR-007 | original mutation visible via view read |
| CF-16 | conformance | SPEC-P3-VALUES | float NaN `==` true under values_equal |
| CF-17 | conformance | SPEC-P3-CALL TR-011 | bind_arguments positional + pending default |
| CF-18 | conformance | SPEC-P3-CACHE TR-014 | digest includes helper registry |
| CF-19 | conformance | SPEC-P3-EIR TR-015 | interpreter branch true path |
| CF-20 | conformance | SPEC-P3-EIR TR-015 | interpreter binary add |
| CF-21 | conformance | SPEC-P3-EIR TR-015 | interpreter raise terminator |
| CF-22 | conformance | SPEC-P3-HOST TR-012 | call-scoped host roots cleared |
| CF-23 | conformance | SPEC-P3-EIR / GC-META | loop backedge safepoint poll count |
| CF-24 | conformance | SPEC-P3-CALL TR-015 | mid-block resume after nested call |
| CF-25 | conformance | SPEC-P3-EIR TR-015 | slot load/store round-trip |
| CF-26 | conformance | SPEC-P3-HELPERS TR-015 | helper_alloc_object returns ObjectRef |
| CF-27 | conformance | SPEC-P3-ERRORS TR-002 | language/structural registry sets |
| CF-28 | conformance | SPEC-P3-CALL TR-011 | return contract success |
| CF-29 | conformance | SPEC-P3-GC-META TR-013 | moving GC accepts updateable RootMap |
| CF-30 | conformance | SPEC-P3-HELPERS TR-005 | may_raise / may_collect policy sets |
| CF-31 | conformance | SPEC-P3-HOST TR-012 | host call with capability succeeds |
| CF-32 | conformance | SPEC-P3-CONTROL / GC-META | pending control heap root visible |
| NG-01 | negative | SPEC-P3-VALID, WP-06 | unknown shape id rejected |
| NG-02 | negative | SPEC-P3-HELPERS, WP-07 | out-of-range helper id |
| NG-03 | negative | SPEC-P3-HOST, WP-12 | missing capability CapabilityError |
| NG-04 | negative | SPEC-P3-MODULE, WP-11 | import cycle ImportCycleError |
| NG-05 | negative | SPEC-P3-VALUES, WP-08 | non-hashable map key TypeError |
| NG-06 | negative | SPEC-P3-VALUES TR-006 | NaN map key TypeError |
| NG-07 | negative | SPEC-P3-VALUES TR-006 | string slice bounds IndexError |
| NG-08 | negative | SPEC-P3-CONTROL TR-008 | uninitialized slot read |
| NG-09 | negative | SPEC-P3-CALL TR-011 | wrong arity ArityError |
| NG-10 | negative | SPEC-P3-CALL TR-011 | duplicate named argument |
| NG-11 | negative | SPEC-P3-CACHE TR-014 | public bytecode cache claim rejected |
| NG-12 | negative | SPEC-P3-CACHE TR-014 | helper registry digest mismatch |
| NG-13 | negative | SPEC-P3-EIR TR-015 | branch non-bool raises |
| NG-14 | negative | SPEC-P3-MODULE TR-010 | duplicate export name rejected |
| NG-15 | negative | SPEC-P3-GC-META TR-013 | may-collect without RootMap |
| NG-16 | negative | SPEC-P3-HOST TR-012 | retained VM value without host root |
| NG-17 | negative | SPEC-P3-ERRORS TR-002 | non-Error raise rejected |
| NG-18 | negative | SPEC-P3-RTP TR-003 | RuntimePlan unresolved module |
| NG-19 | negative | SPEC-P3-RTP / CACHE | RuntimePlan cache profile mismatch |
| NG-20 | negative | SPEC-P3-EIR TR-004 | invalid block graph |
| NG-21 | negative | SPEC-P3-EIR TR-004 | unknown constant id |
| NG-22 | negative | SPEC-P3-EIR / HELPERS | unknown runtime helper id |
| NG-24 | negative | SPEC-P3-HELPERS TR-005 | duplicate helper id at registry build |
| NG-25 | negative | SPEC-P3-CONTROL TR-008 | invalid slot id |
| NG-26 | negative | SPEC-P3-CALL TR-011 | return contract failure |
| NG-27 | negative | SPEC-P3-HOST TR-012 | host call without capability |
| NG-28 | negative | SPEC-P3-CACHE TR-014 | profile fingerprint mismatch |
| NG-29 | negative | SPEC-P3-GC-META TR-013 | moving GC non-updateable RootMap |
| NG-30 | negative | SPEC-P3-GC-META TR-013 | RootMap unknown slot |
| NG-31 | negative | SPEC-P3-MODULE TR-010 | failed module import |
| NG-32 | negative | SPEC-P3-READONLY TR-007 | readonly list view non-hashable key |
| NG-33 | negative | SPEC-P3-ERRORS TR-002 | structural not catchable as language |
| DG-01 | diagnostics | SPEC-P3-ERRORS, WP-04 | construct_error code/message/span |
| DG-02 | diagnostics | SPEC-P3-EIR TR-015 | interpreter last_source_span |
| DG-03 | diagnostics | SPEC-P3-HOST TR-012 | host error normalized to raise |
| RG-01 | regression | SPEC-P3-UNWIND, WP-10 | nested region LIFO defer order |
| RG-02 | regression | SPEC-P3-VALUES, WP-08/09 | immutable cell ReadOnlyError |
| RG-03 | regression | SPEC-P3-CALL / WP-17 | interpreter minimal module still returns |
| RG-04 | regression | SPEC-P3-UNWIND TR-009 | finally raise overrides pending return |
| RG-05 | regression | SPEC-P3-UNWIND TR-009 | finally raise suppresses prior raise |
| RG-06 | regression | SPEC-P3-UNWIND TR-009 | defer raise suppressed under pending raise |
| RG-07 | regression | SPEC-P3-UNWIND TR-009 | finally Normal preserves pending return |

Executable modules: `conformance`, `negative`, `diagnostics`, `regression`, `gap_closure` under `cargo test -p vm_tests`.
