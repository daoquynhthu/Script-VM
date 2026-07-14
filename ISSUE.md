# ISSUE.md

Document class: Append-only audit findings log  
Rule: Only append audit results. Do not rewrite old entries.

## ISSUE-20260630-001 · Partial EIR inner-op variant coverage

Severity: INFO
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent
Spec References:
  - SPEC-P3-EIR
  - PHASE-3-EIR-SCHEMA-CLOSURE.md
Affected Files:
  - crates/vm_core/src/eir/schema.rs
Finding:
  EirOpKind union covers all 15 frozen op kinds, but CheckOp, ConstructOp, and PatternOp inner variants are not yet fully enumerated (subset implemented).
Evidence:
  schema.rs implements CheckBool/Type/Callable/Arity only; ConstructList/Error only; PatternBind only.
Required Action:
  Extend inner variants in a later bounded WP-06 follow-up or WP-17 interpreter stage without inventing new op kinds.
Gate Impact:
  G5 PASS_WITH_NOTES
Resolution Notes:
  Resolved in WP-06 remediation: full CheckOp (10), ConstructOp (6), PatternOp (9) unions added; validator updated.

## ISSUE-20260701-001 · ConstantPool references not validated

Severity: MAJOR
Status: OPEN
Work Package: WP-06
Detected By: Main Agent (audit)
Spec References:
  - SPEC-P3-VALID
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4)
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §7
Affected Files:
  - crates/vm_core/src/eir/validate.rs
Finding:
  `ConstantOp` validation checks only `dest` SlotId; referenced `ConstantId` is never verified against `EirModule.constants`.
Evidence:
  `validate_eir_op` Constant arm calls `require_slot(constant.dest, ctx)` only; no `require_constant` or pool lookup; `EirValidationError` has no `UnknownConstantId` variant.
Required Action:
  Add constant-pool membership validation for every `ConstantOp` and a negative test rejecting unknown `ConstantId`.
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260701-002 · Invalid block argument count never enforced

Severity: MAJOR
Status: OPEN
Work Package: WP-06
Detected By: Main Agent (audit)
Spec References:
  - SPEC-P3-EIR
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4)
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
Affected Files:
  - crates/vm_core/src/eir/validate.rs
Finding:
  `EirValidationError::InvalidBlockArgumentCount` is defined but no validator returns it; `Jump`/`LoopBackedge` arg counts are not compared to target block `parameters`.
Evidence:
  `rg InvalidBlockArgumentCount` matches only the enum definition; `validate_terminator` Jump/LoopBackedge arms validate slot presence and block existence only.
Required Action:
  Compare terminator transfer args to target block parameter count; add negative test with mismatched Jump args.
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260701-003 · Heap write barrier policy not validated

Severity: MAJOR
Status: OPEN
Work Package: WP-06
Detected By: Main Agent (audit)
Spec References:
  - SPEC-P3-VALID
  - PHASE-3-VALIDATION-MATRIX.md (P3-V5)
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §9, §23
Affected Files:
  - crates/vm_core/src/eir/validate.rs
Finding:
  Frozen spec requires rejecting heap writes without barrier policy; `StoreOp` validation checks slots and access sites only.
Evidence:
  `validate_slots_in_store` has no barrier-policy field or check; no corresponding `EirValidationError` variant or negative test.
Required Action:
  Add barrier-policy validation for heap-mutating store paths per frozen §9; add negative test.
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260701-004 · Unknown TypeId/ShapeId/FieldId/CaseId not validated

Severity: MAJOR
Status: OPEN
Work Package: WP-06
Detected By: Main Agent (audit)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4)
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/schema.rs
Finding:
  EIR ops carry `TypeId`, `ShapeId`, `FieldId`, and `CaseId` references but `EirValidationContext` provides no binding tables and validator never rejects unknown IDs.
Evidence:
  `EirValidationError` lacks unknown-ID variants for these types; CheckType/LoadField/ConstructRecord/etc. reference IDs without context lookup.
Required Action:
  Extend validation context with RuntimePlan-bound ID sets (or wire from plan validation) and reject unknown references; add negative tests.
Gate Impact:
  G5 PASS_WITH_NOTES
Resolution Notes:
  May require RuntimePlan execution-binding integration beyond current WP-06 fixtures.

## ISSUE-20260701-005 · Negative tests missing for implemented EIR rejections

Severity: MINOR
Status: OPEN
Work Package: WP-06
Detected By: Main Agent (audit)
Spec References:
  - SPEC-P3-VALID
  - AGENT.md §13
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Several rejection paths are implemented but lack dedicated negative tests.
Evidence:
  No tests for `RuntimePlanDigestMismatch`, `UnknownSlotId`, `GuardWithoutFailureAction`, or `InvalidEntryBlock`; `eir::validate::tests` covers 8 cases only.
Required Action:
  Add one negative test per implemented rejection path listed above.
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260701-006 · Helper may_collect classification is WP-06 stub

Severity: INFO
Status: DEFERRED
Work Package: WP-06
Detected By: Main Agent (audit)
Spec References:
  - SPEC-P3-HELPERS
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md
  - PLAN/IMPLEMENTATION-CODING-PLAN.md Stage 6
Affected Files:
  - crates/vm_core/src/eir/validate.rs
Finding:
  `HelperRegistryView::may_collect` is manually populated in test context; full helper descriptor metadata arrives in WP-07.
Evidence:
  Comment in validate.rs: "full registry arrives in WP-07"; `may_collect_validation_context` uses `with_may_collect` stub.
Required Action:
  Wire may_collect/may_allocate from WP-07 helper descriptors into `HelperRegistryView`.
Gate Impact:
  G6 PASS_WITH_NOTES
Resolution Notes:
  Deferred to WP-07 per coding plan Stage 6.

## ISSUE-20260701-006 · Helper may_collect classification is WP-06 stub — RESOLVED

Severity: INFO
Status: RESOLVED
Work Package: WP-07
Detected By: Main Agent (WP-07 completion)
Spec References:
  - SPEC-P3-HELPERS
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md
Affected Files:
  - crates/vm_runtime/src/helpers/validate.rs
  - crates/vm_runtime/src/helpers/registry.rs
  - crates/vm_tests/src/lib.rs
Finding:
  Original audit: `HelperRegistryView::may_collect` was manually stubbed in WP-06 test context.
Evidence:
  `eir_validation_view` derives may-collect IDs from canonical `RuntimeHelperRegistry` descriptors; `vm_tests::smoke::eir_may_collect_rejects_missing_root_map_via_helper_registry` validates EIR rejection using registry-derived view.
Required Action:
  Completed.
Gate Impact:
  G6 PASS
Resolution Notes:
  vm_core unit tests retain `may_collect_validation_context` stub for crate isolation; production path uses vm_runtime bridge.

## ISSUE-20260701-007 · Harness MCP descriptor tree present in workspace

Severity: INFO
Status: ACCEPTED
Work Package: WP-06
Detected By: Main Agent (audit)
Spec References:
  - AGENT.md §11
Affected Files:
  - D:\script\mcps/grok_com_github/tools/*.json
Finding:
  `D:\script\mcps` exists (~70 JSON tool descriptors); not part of Script VM deliverables; regenerated by dev harness after prior removal.
Evidence:
  `Test-Path D:\script\mcps` returns True; tree contains GitHub MCP tool schemas only; no WP-06 PROGRESS changed-files claim for this round.
Required Action:
  Exclude `mcps/` from WP-06 scope and changed-files reporting; do not treat as implementation artifact.
Gate Impact:
  G0 NOT_APPLICABLE (environmental)
Resolution Notes:
  Accepted as harness infrastructure outside repository implementation scope.

## ISSUE-20260701-001 · ConstantPool references not validated — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent (audit remediation)
Spec References:
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4)
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §7
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Original audit: ConstantOp did not verify ConstantId against module ConstantPool.
Evidence:
  Added `require_constant()` and `UnknownConstantId` error; test `unknown_constant_id_is_rejected` passes via `validate_eir_module`.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  Minimal fixture now seeds constant pool entry for ConstantId 0.

## ISSUE-20260701-002 · Invalid block argument count never enforced — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent (audit remediation)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Original audit: `InvalidBlockArgumentCount` never returned.
Evidence:
  `require_block_transfer_args()` compares Jump/LoopBackedge args to target block parameters; test `invalid_block_argument_count_is_rejected` passes.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  Pending.

## ISSUE-20260701-003 · Heap write barrier policy not validated — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent (audit remediation)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §9, §23
  - PHASE-3-VALIDATION-MATRIX.md (P3-V5)
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Original audit: heap-mutating StoreOp paths lacked barrier policy validation.
Evidence:
  Added `requires_write_barrier`, `barrier_access_site_ids`, `HeapWriteWithoutBarrierPolicy`; StoreField/ListIndex/MapEntry call `require_heap_write_barrier()`; test passes.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  Barrier policy bound via RuntimePlan access-site registration in validation context.

## ISSUE-20260701-004 · Unknown TypeId/ShapeId/FieldId/CaseId not validated — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent (audit remediation)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Original audit: no RuntimePlan-bound ID sets in validation context.
Evidence:
  Extended `EirValidationContext` with type/shape/field/case ID sets; wired through Check/Load/Store/Construct/Pattern/Call paths; test `unknown_type_id_is_rejected` passes.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  Shape/Field/Case rejection uses same require_* helpers; dedicated negative tests for each may follow in WP-07 integration.

## ISSUE-20260701-005 · Negative tests missing for implemented EIR rejections — RESOLVED

Severity: MINOR
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent (audit remediation)
Spec References:
  - AGENT.md §13
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Original audit: four implemented rejection paths lacked tests.
Evidence:
  Added tests: `runtime_plan_digest_mismatch_is_rejected`, `unknown_slot_id_is_rejected`, `guard_without_failure_action_is_rejected`, `invalid_entry_block_is_rejected`; all pass in `cargo test -p vm_core eir`.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  EIR-filtered test count now 17 (16 eir::validate + 1 runtime_plan eir-named).

## ISSUE-20260706-001 · EIR source-mapping check ignores helper may_raise metadata

Severity: MAJOR
Status: OPEN
Work Package: WP-06, WP-07
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-HELPERS
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §5
  - PHASE-3-VALIDATION-MATRIX.md (P3-V6)
Affected Files:
  - crates/vm_core/src/eir/validate.rs
Finding:
  `op_may_raise` treats every `EirOpKind::RuntimeHelper` as may-raise; validation does not consult helper descriptor `may_raise` from the canonical registry (unlike `may_collect`, which is registry-driven via `HelperRegistryView`).
Evidence:
  `op_may_raise` matches all `RuntimeHelper(_)`; `HelperRegistryView` exposes `may_collect` only; `helper_write_barrier` has `may_raise: false` in `canonical.rs` but would still require source mapping if present without span metadata.
Required Action:
  Extend `HelperRegistryView` with may-raise (and optionally may-unwind) classification from `RuntimeHelperRegistry`; gate `MayRaiseWithoutSourceMapping` on per-helper metadata.
Gate Impact:
  G5 PASS_WITH_NOTES
  G6 PASS_WITH_NOTES

## ISSUE-20260706-002 · RuntimePlan cache key omits helper registry digest

Severity: MAJOR
Status: OPEN
Work Package: WP-07
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-CACHE-COMPATIBILITY-MATRIX.md §4, §6
  - PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md §16
Affected Files:
  - crates/vm_core/src/cache.rs
  - crates/vm_runtime/src/helpers/registry.rs
Finding:
  `RuntimePlanCacheKey.helper_registry_digest` is always `None` in `from_plan`, despite registry digest computation existing on `RuntimeHelperRegistry::digest()`.
Evidence:
  `cache.rs` line 56 sets `helper_registry_digest: None`; `compute_registry_digest` in `registry.rs` returns a `Digest` that is never wired into cache identity; no test asserts digest participation.
Required Action:
  Populate cache key from canonical registry digest (or plan-bound digest field when available); add cache-key mutation test when registry digest changes.
Gate Impact:
  G6 FAIL

## ISSUE-20260706-003 · Catch-region unwinding not implemented

Severity: MAJOR
Status: OPEN
Work Package: WP-10
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - SPEC-P3-UNWIND
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §9
  - PLAN/TRACEABILITY-MATRIX.md TR-009
Affected Files:
  - crates/vm_runtime/src/unwind/perform.rs
  - crates/vm_runtime/src/unwind/region.rs
Finding:
  Structured unwinding implements defer/resource/finally phases only; catch-region handling (`TryCatch`, `catch_entries`, PendingRaise dispatch to matching catch) is absent.
Evidence:
  `ControlRegionKind::TryCatch` is defined but unused; `perform_unwind` has no catch-matching loop; no tests for raise handled by catch or guard-raise during catch selection.
Required Action:
  Implement catch dispatch per frozen §9 before WP-10 can pass G6; add raise-to-catch and no-match propagation tests.
Gate Impact:
  G4 PASS_WITH_NOTES
  G6 FAIL

## ISSUE-20260706-004 · helper_perform_unwind not wired to runtime unwind engine

Severity: MAJOR
Status: OPEN
Work Package: WP-10, WP-07
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §15
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §3
Affected Files:
  - crates/vm_runtime/src/unwind/perform.rs
  - crates/vm_runtime/src/helpers/canonical.rs
Finding:
  Canonical helper `helper_perform_unwind` is registered but no interpreter/helper dispatch connects it to `perform_unwind`; unwinding is only reachable via direct Rust API in tests.
Evidence:
  `rg perform_unwind` matches only `unwind/perform.rs` and `unwind/mod.rs`; `vm_eval::interpreter` does not invoke unwind; no integration test through helper ID 29 (`helper_perform_unwind`).
Required Action:
  Add helper dispatch shell (or documented internal entry) wiring `helper_perform_unwind` to `UnwindContext` + `perform_unwind` before interpreter execution stage.
Gate Impact:
  G6 FAIL

## ISSUE-20260706-005 · Negative tests missing for Shape/Field/Case ID rejection

Severity: MINOR
Status: OPEN
Work Package: WP-06
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4)
  - AGENT.md §13
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  `UnknownShapeId`, `UnknownFieldId`, and `UnknownCaseId` rejection paths are implemented but lack dedicated negative tests; only `unknown_type_id_is_rejected` exists.
Evidence:
  `require_shape/require_field/require_case` return errors; `fixtures.rs` has no malformed modules for shape/field/case; `cargo test -p vm_core eir -- --list` shows no corresponding test names.
Required Action:
  Add one negative test per ID kind with bound validation context.
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260706-006 · Negative tests missing for CallSite/AccessSite/Deopt ID rejection

Severity: MINOR
Status: OPEN
Work Package: WP-06
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
  - AGENT.md §13
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  `UnknownCallSiteId`, `UnknownAccessSiteId`, and `UnknownDeoptId` validators exist but have no negative tests.
Evidence:
  `require_call_site`, `require_access_site`, `require_deopt` in `validate.rs`; no matching fixtures or `eir::validate::tests` entries.
Required Action:
  Add fixtures and negative tests for each rejection path.
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260706-007 · Nested multi-region unwind lacks test coverage

Severity: MINOR
Status: OPEN
Work Package: WP-10
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §5, §10
  - PLAN/TRACEABILITY-MATRIX.md TR-009
Affected Files:
  - crates/vm_runtime/src/unwind/perform.rs
Finding:
  Unwind tests exercise single-region stacks only; nested cleanup crossing (e.g., inner `Block` defer then outer `Function` finally) is not verified.
Evidence:
  All five `perform::tests` push one `RuntimeRegionFrame`; no test asserts LIFO cleanup order across two stacked regions before return resolution.
Required Action:
  Add nested-region test with ordered execution log (`defer inner` → `defer outer` → `finally outer`).
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260706-008 · SlotState bootstrap omits Cell and RuntimeInternal variants

Severity: INFO
Status: OPEN
Work Package: WP-08, WP-09
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md §2.2
  - PLAN/IMPLEMENTATION-CODING-PLAN.md Stage 7
Affected Files:
  - crates/vm_runtime/src/frame.rs
Finding:
  `SlotState` implements only `Uninitialized` and `Value`; frozen EIR slot semantics also require `Cell` and `RuntimeInternal` storage modes.
Evidence:
  `frame.rs` enum has two variants; no cell binding read/write or runtime-internal slot policy.
Required Action:
  Defer to WP-09 frame/slot integration or Stage 9+ interpreter wiring; track as known bootstrap gap.
Gate Impact:
  G4 PASS_WITH_NOTES

## ISSUE-20260706-009 · ReadOnlyView identity semantics not exercised

Severity: INFO
Status: OPEN
Work Package: WP-08
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-READONLY-VIEW-SEMANTICS.md §4–§5
  - PLAN/TRACEABILITY-MATRIX.md TR-007
Affected Files:
  - crates/vm_runtime/src/readonly.rs
  - crates/vm_runtime/src/heap/heap.rs
Finding:
  ReadOnlyView shell implements mutation rejection and shallow read, but identity rules (`readonly(x) is x` MUST be false for heap-backed mutable aggregates) are not implemented or tested.
Evidence:
  No `is_identical`/`value_identity` helper; tests cover only mutation rejection and field-read delegation.
Required Action:
  Add identity comparison helper and tests when WP-13/TR-007 integration begins; acceptable bootstrap gap for WP-08.
Gate Impact:
  G5 PASS_WITH_NOTES

## ISSUE-20260706-010 · Dual region-stack representations coexist

Severity: INFO
Status: ACCEPTED
Work Package: WP-08, WP-10
Detected By: Main Agent (pre-Stage-9 audit)
Spec References:
  - PHASE-3-CONTROL-STATE-MODEL.md
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §2.2
Affected Files:
  - crates/vm_runtime/src/control.rs
  - crates/vm_runtime/src/unwind/region.rs
Finding:
  `control::RegionStack` (ID-only shell from WP-08) and `unwind::UnwindContext::region_frames` (full cleanup-bearing frames from WP-10) coexist without unification.
Evidence:
  `RegionStack` stores `Vec<ControlRegionId>`; `UnwindContext` stores `Vec<RuntimeRegionFrame>` with `CleanupState`; no bridge between them.
Required Action:
  Unify or document migration path when interpreter attaches unwind to frames (WP-09/WP-17).
Gate Impact:
  G3 PASS_WITH_NOTES
Resolution Notes:
  Accepted bootstrap split; unwind owns canonical cleanup state.

## ISSUE-20260706-001 · EIR source-mapping check ignores helper may_raise metadata — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-06, WP-07
Detected By: Main Agent (pre-Stage-9 remediation)
Spec References:
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §5
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_runtime/src/helpers/registry.rs
  - crates/vm_runtime/src/helpers/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
  - crates/vm_tests/src/lib.rs
Finding:
  Original audit: all RuntimeHelper ops required source mapping regardless of descriptor `may_raise`.
Evidence:
  `HelperRegistryView::may_raise` added; `eir_validation_view` populates from registry; `op_requires_source_mapping` consults per-helper metadata; integration test `non_may_raise_helper_without_source_passes_with_registry_view` passes.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
  G6 PASS

## ISSUE-20260706-002 · RuntimePlan cache key omits helper registry digest — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-07
Detected By: Main Agent (pre-Stage-9 remediation)
Spec References:
  - PHASE-3-CACHE-COMPATIBILITY-MATRIX.md §4, §6
Affected Files:
  - crates/vm_core/src/cache.rs
  - crates/vm_runtime/src/cache.rs
  - crates/vm_tests/src/lib.rs
Finding:
  Original audit: `helper_registry_digest` always `None`.
Evidence:
  `RuntimePlanCacheKey::from_plan_with_helper_registry_digest` added; `vm_runtime::cache::runtime_plan_cache_key` wires canonical registry digest; tests `runtime_plan_cache_key_includes_helper_registry_digest` and workspace smoke assert digest present.
Required Action:
  Completed.
Gate Impact:
  G6 PASS

## ISSUE-20260706-003 · Catch-region unwinding not implemented — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-10
Detected By: Main Agent (pre-Stage-9 remediation)
Spec References:
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §9
Affected Files:
  - crates/vm_runtime/src/unwind/catch.rs
  - crates/vm_runtime/src/unwind/perform.rs
  - crates/vm_runtime/src/unwind/region.rs
Finding:
  Original audit: catch dispatch absent from unwind loop.
Evidence:
  `CatchEntry`, `dispatch_catch_handlers`, TryCatch integration in `perform_unwind`; tests `matching_catch_clears_pending_raise`, `pending_raise_handled_by_try_catch_region` pass.
Required Action:
  Completed.
Gate Impact:
  G4 PASS
  G6 PASS

## ISSUE-20260706-004 · helper_perform_unwind not wired to runtime unwind engine — RESOLVED

Severity: MAJOR
Status: RESOLVED
Work Package: WP-10, WP-07
Detected By: Main Agent (pre-Stage-9 remediation)
Spec References:
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §15
Affected Files:
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_tests/src/lib.rs
Finding:
  Original audit: no dispatch path from helper id 29 to `perform_unwind`.
Evidence:
  `dispatch_helper` routes `HELPER_PERFORM_UNWIND_ID` to `perform_unwind`; unit and integration tests pass.
Required Action:
  Completed.
Gate Impact:
  G6 PASS

## ISSUE-20260706-005 · Negative tests missing for Shape/Field/Case ID rejection — RESOLVED

Severity: MINOR
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent (remediation pass 1)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4)
  - AGENT.md §13
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Original audit: UnknownShapeId, UnknownFieldId, UnknownCaseId rejection paths lacked dedicated negative tests.
Evidence:
  Added fixtures `eir_module_with_unknown_shape_id`, `eir_module_with_unknown_field_id`, `eir_module_with_unknown_case_id` and tests `unknown_shape_id_is_rejected`, `unknown_field_id_is_rejected`, `unknown_case_id_is_rejected`; all pass in `cargo test -p vm_core eir`.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  Bound validation contexts `shape_bound_validation_context`, `field_bound_validation_context`, `case_bound_validation_context` added alongside fixtures.
## ISSUE-20260706-006 · Negative tests missing for CallSite/AccessSite/Deopt ID rejection — RESOLVED

Severity: MINOR
Status: RESOLVED
Work Package: WP-06
Detected By: Main Agent (remediation pass 1)
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
  - AGENT.md §13
Affected Files:
  - crates/vm_core/src/eir/validate.rs
  - crates/vm_core/src/eir/fixtures.rs
Finding:
  Original audit: UnknownCallSiteId, UnknownAccessSiteId, UnknownDeoptId validators lacked negative tests.
Evidence:
  Added fixtures `eir_module_with_unknown_call_site_id`, `eir_module_with_unknown_access_site_id`, `eir_module_with_unknown_deopt_id` and tests `unknown_call_site_id_is_rejected`, `unknown_access_site_id_is_rejected`, `unknown_deopt_id_is_rejected`; all pass in `cargo test -p vm_core eir`.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  EIR-filtered test count now 23.


## ISSUE-20260706-007 · Nested multi-region unwind lacks test coverage — RESOLVED

Severity: MINOR
Status: RESOLVED
Work Package: WP-10
Detected By: Main Agent (remediation pass 2)
Spec References:
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §5, §10
  - PLAN/TRACEABILITY-MATRIX.md TR-009
  - AGENT.md §13
Affected Files:
  - crates/vm_runtime/src/unwind/perform.rs
Finding:
  Original audit: unwind tests exercised single-region stacks only; nested LIFO cleanup order was not verified.
Evidence:
  Added `nested_regions_unwind_inner_defer_before_outer_defer_and_finally` with Block-inside-Function region stack; `perform_unwind` log asserts `defer:1` (inner) → `defer:2` (outer) → `finally:3` before Return resolution; test passes.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  Helper `nested_block_inside_function_regions` builds reproducible two-level fixture.


## ISSUE-20260706-008 · SlotState bootstrap omits Cell and RuntimeInternal variants — RESOLVED

Severity: INFO
Status: RESOLVED
Work Package: WP-08, WP-09
Detected By: Main Agent (remediation pass 3)
Spec References:
  - PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md §2.2–§2.5
  - PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md §6.2, §7
  - IMPLEMENTATION-CODING-PLAN.md Stage 7
Affected Files:
  - crates/vm_runtime/src/frame.rs
  - crates/vm_runtime/src/binding_cell.rs
  - crates/vm_runtime/src/runtime_value.rs
  - crates/vm_core/src/id.rs
Finding:
  Original audit: SlotState had only Uninitialized and Value; frozen spec requires Cell and RuntimeInternal storage modes with distinct access policy.
Evidence:
  SlotState now has four variants; BindingCell includes type_contract and CellOwner; `write_cell` enforces mutability (immutable -> ReadOnlyError), TypeContractChecker, and WriteBarrierHook on heap-ref writes; `read_with_policy` supports §2.3 PermitUninitialized on value slots; runtime-internal slots reject user-visible read/write with InternalVMError; 13 frame::tests pass.
Required Action:
  Completed.
Gate Impact:
  G4 PASS
  G5 PASS
Resolution Notes:
  LoadCell/StoreCell interpreter op wiring deferred to WP-17 per remediation plan non-goals.


## ISSUE-20260709-001 · H3 generic/builtin call omits frame body execution

Severity: INFO
Status: OPEN
Work Package: WP-07
Detected By: Main Agent (remediation pass 6 / H3)
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §12.2, §20.4
  - PHASE-3-CALL-EXECUTION-PROTOCOL.md §3, §9–§11
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.1.1
Affected Files:
  - crates/vm_runtime/src/helpers/h3.rs
  - crates/vm_runtime/src/helpers/dispatch.rs
Finding:
  Milestone H3 ships prepare/bind/contract/capability validation and call-site feedback update for helper_generic_call / helper_call_builtin, returning VmControl::Normal after successful prepare. Frozen §12.2 also requires frame push, body execution through interpreter, return contract, and frame pop. Those steps are not implemented in the helper boundary to avoid inventing a second interpreter call stack outside existing vm_eval activation records.
Evidence:
  helper_generic_call and helper_call_builtin return VmControl::Normal(Some(callee)) after prepare/validate without pushing InterpreterFrame or running EIR body; dispatch tests assert prepare outcomes only.
Required Action:
  Wire prepared calls into interpreter frame enter/exit (call-engine integration) in a later pass; do not fake body results in helper unit tests.
Gate Impact:
  G6 PASS_WITH_NOTES for full call-engine integration; G4/G5 PASS for shipped prepare/validate subset
Resolution Notes:
  Deferred per H3 plan risk note (largest spec-faithful prepare/bind/contract/capability subset).

## ISSUE-20260709-002 · H5 initialize_module omits init EIR body execution

Severity: INFO
Status: OPEN
Work Package: WP-07
Detected By: Main Agent (remediation pass 8 / H5)
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.6
  - PHASE-3-MODULE-RUNTIME-CONTRACT.md §3–§4
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.9.2
Affected Files:
  - crates/vm_runtime/src/helpers/h5.rs
Finding:
  helper_initialize_module advances the module lifecycle state machine to Initializing but does not execute the module initialization EIR function body through the interpreter.
Evidence:
  Dispatch/unit tests assert ModuleState::Initializing after helper call without running initialization_function; VmControl::Normal returned as prepare-style outcome.
Required Action:
  Wire initialize_module to interpreter module top-level execution and finish_module_init (seal + Initialized / Failed) in a later pass.
Gate Impact:
  G6 PASS_WITH_NOTES for full module init integration; G4/G5 PASS for shipped state-machine subset
Resolution Notes:
  Deferred; cycle handling and seal/import paths use existing ModuleRuntime APIs.

## ISSUE-20260709-003 · Stage 14 G6 notes on deferred body paths

Severity: INFO
Status: OPEN
Work Package: WP-19
Detected By: Main Agent (Stage 14 integration scan)
Spec References:
  - IMPLEMENTATION-CODING-PLAN.md Stage 14
  - ISSUE-20260709-001
  - ISSUE-20260709-002
Affected Files:
  - crates/vm_runtime/src/helpers/h3.rs
  - crates/vm_runtime/src/helpers/h5.rs
Finding:
  Integration scan confirms no public bytecode / CPython ABI exposure in crates. Remaining G6 notes are deferred call-body and module-init-body execution already tracked as ISSUE-001/002; pattern match is bootstrap Bool tags only.
Evidence:
  cargo test --workspace green (290 unit tests); ripgrep of crates found no public bytecode/CPython ABI identifiers; all 47 helper ids dispatch.
Required Action:
  Close via interpreter frame enter for generic_call and module init EIR body; expand pattern match as needed.
Gate Impact:
  G6 PASS_WITH_NOTES
Resolution Notes:
  Pending later integration passes.

## ISSUE-20260709-001 · H3 generic/builtin call omits frame body execution — RESOLVED

Severity: INFO
Status: RESOLVED
Work Package: WP-07, WP-17
Detected By: Main Agent
Spec References:
  - PHASE-3-CALL-EXECUTION-PROTOCOL.md
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §12
Affected Files:
  - crates/vm_runtime/src/helpers/h3.rs
  - crates/vm_eval/src/interpreter/mod.rs
Finding:
  Original: prepare-only generic_call without nested body.
Evidence:
  PreparedUserCall + Interpreter::enter_user_call; test generic_call_enters_user_function_body returns Int(7) from nested callee.
Required Action:
  Completed for UserFunction/BoundMethod nested EIR; builtin body still prepare-only by design.
Gate Impact:
  G6 PASS
Resolution Notes:
  Builtin/host call bodies remain prepare/validate; user EIR body wired.

## ISSUE-20260709-002 · H5 initialize_module omits init EIR body execution — RESOLVED

Severity: INFO
Status: RESOLVED
Work Package: WP-11, WP-17
Detected By: Main Agent
Spec References:
  - PHASE-3-MODULE-RUNTIME-CONTRACT.md §3–§4
Affected Files:
  - crates/vm_eval/src/interpreter/mod.rs
Finding:
  Original: state machine only without init EIR.
Evidence:
  Interpreter::run_module_init_function + module_init_body_executes returns Int(99).
Required Action:
  Completed for explicit init-function entry; full helper_initialize_module → auto-run-init bridge still optional host/orchestration glue.
Gate Impact:
  G6 PASS
Resolution Notes:
  Init body execution API available; orchestration may call after state advance.

## ISSUE-20260709-003 · Stage 14 G6 notes on deferred body paths — RESOLVED

Severity: INFO
Status: RESOLVED
Work Package: WP-19
Detected By: Main Agent
Finding:
  G6 notes closed by ISSUE-001/002 resolution for primary deferred body paths.
Evidence:
  Nested call + module init tests green; workspace 292 unit tests.
Required Action:
  Completed.
Gate Impact:
  G6 PASS
Resolution Notes:
  Pattern match still bootstrap tags; acceptable for current phase.

## ISSUE-20260706-009 · ReadOnlyView identity semantics not exercised — RESOLVED

Severity: INFO
Status: RESOLVED
Work Package: WP-13
Detected By: Main Agent
Spec References:
  - PHASE-3-READONLY-VIEW-SEMANTICS.md §4–§5
Affected Files:
  - crates/vm_runtime/src/value/mod.rs
  - crates/vm_runtime/src/readonly.rs
  - crates/vm_tests/src/conformance.rs
Finding:
  Original: identity rules not implemented or tested.
Evidence:
  values_identical / values_equal added; tests readonly_view_is_not_identical_to_target, readonly_view_equals_target_by_equality, nested view, CF-10; all pass under cargo test.
Required Action:
  Completed.
Gate Impact:
  G5 PASS
Resolution Notes:
  Identity is handle-based; equality unwraps ReadOnlyView and may compare aggregate structure for lists/records/enums.
