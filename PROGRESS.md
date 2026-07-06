# PROGRESS.md

Document class: Append-only implementation progress log  
Rule: Only append change summaries. Do not rewrite old entries.

## 2026-06-29 12:00 · WP-00 Agent and repository process setup

Work Package: WP-00
Agent Mode: main-only
Changed Files:
  - D:\script\REPOSITORY-PROCESS.md (created)
  - D:\script\PROGRESS.md (created)
  - D:\script\ISSUE.md (created)
Spec References:
  - SPEC-P3-FREEZE
  - AGENT-MASTER-PLAN.md
  - AGENT-OPERATING-PROTOCOL.md
  - GATE-CHECKLIST.md
  - HANDOFF-TEMPLATE.md
Gates:
  - G0 PASS
  - G1 PASS
  - G2 PASS
  - G3 PASS
  - G7 PASS
Tests:
  - Not applicable (process setup, no code)
Summary:
  Created REPOSITORY-PROCESS.md defining repository working layout, task log convention, branch/commit discipline, test command reporting format, and handoff storage convention. Created PROGRESS.md and ISSUE.md companion documents.
Next:
  WP-01 Frozen spec reference ingestion.

## 2026-06-30 10:00 · Stage 0 workspace bootstrap completion

Work Package: WP-00
Agent Mode: main-only
Changed Files:
  - docs/frozen-specs/README.md (created)
  - docs/frozen-specs/phase-1/INDEX.md (created)
  - docs/frozen-specs/phase-2/INDEX.md (created)
  - docs/frozen-specs/phase-3/INDEX.md (created)
  - agent/work-packages/.gitkeep (created)
  - agent/handoffs/.gitkeep (created)
  - agent/gate-records/.gitkeep (created)
  - agent/audit-records/.gitkeep (created)
  - agent/task-logs/.gitkeep (created)
  - scripts/check/workspace.ps1 (created)
  - scripts/test/workspace.ps1 (created)
  - scripts/validate/aliases.ps1 (created)
  - REPOSITORY-PROCESS.md (created)
Spec References:
  - IMPLEMENTATION-CODING-PLAN.md (Stage 0)
  - SPEC-P3-FREEZE
Gates:
  - G0 PASS
  - G1 PASS
  - G2 PASS
  - G3 PASS
  - G4 PASS
  - G7 PASS
Tests:
  - cargo metadata --no-deps PASS
  - cargo check --workspace PASS (after Stage 2 skeleton)
Summary:
  Completed remaining Stage 0 scaffolding: frozen-spec routing index, agent handoff directories, check/test/validate scripts, and REPOSITORY-PROCESS.md (remediating WP-00 missing file).
Next:
  WP-01 frozen spec reference ingestion.

## 2026-06-30 10:15 · WP-01 frozen spec reference ingestion

Work Package: WP-01
Agent Mode: main-only
Changed Files:
  - docs/agent-plan/local-reference-map.md (created)
Spec References:
  - SPEC-P1-FREEZE
  - SPEC-P1-LANG
  - SPEC-P2-FREEZE
  - SPEC-P2-IR
  - SPEC-P3-FREEZE
  - SPEC-P3-VM
  - SPEC-P3-MIN
  - AGENT-MASTER-PLAN.md
Gates:
  - G0 PASS
  - G1 PASS
  - G2 PASS
  - G3 PASS
  - G7 PASS
Tests:
  - scripts/validate/aliases.ps1 PASS (all mapped ARCHITECTURE paths exist)
Summary:
  Added Phase 1/2/3 alias map and early work-package subsystem routing. Verified frozen documents are reachable under ARCHITECTURE/.
Next:
  Stage 2 crate skeleton and ID types (WP-03).

## 2026-06-30 10:30 · Stage 2 crate skeleton and ID types

Work Package: WP-03
Agent Mode: main-only
Changed Files:
  - crates/sir/src/id.rs (created)
  - crates/sir/src/source.rs (created)
  - crates/vm_diag/src/source_span.rs (created)
  - crates/vm_diag/src/diagnostic.rs (created)
  - crates/vm_core/src/id.rs (created)
  - crates/vm_core/src/error.rs (created)
  - crates/vm_core/src/value.rs (created)
  - crates/vm_core/src/control.rs (created)
  - crates/vm_core/src/profile.rs (created)
  - crates/vm_core/src/cache.rs (created)
  - crates/vm_core/src/lib.rs (updated)
  - crates/vm_runtime/src/runtime_plan.rs (created)
  - crates/vm_runtime/src/eir.rs (created)
  - crates/vm_runtime/src/helpers.rs (created)
  - crates/sir_validate/src/validate.rs (created)
  - crates/vm_eval/src/interpreter.rs (created)
  - crates/vm_host/src/host_function.rs (created)
  - crates/vm_host/src/host_object.rs (created)
  - crates/vm_host/src/host_root.rs (created)
  - crates/vm_tests/src/lib.rs (updated)
  - crates/*/Cargo.toml (dependencies wired)
Spec References:
  - SPEC-P2-IR
  - SPEC-P3-RTP
  - SPEC-P3-EIR
  - SPEC-P3-ERRORS
  - SPEC-P3-CONTROL
  - SPEC-P3-PROFILE
  - SPEC-P3-VALID
  - SPEC-P3-HELPERS
  - SPEC-P3-HOST
Gates:
  - G0 PASS
  - G1 PASS
  - G2 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo check --workspace PASS
  - cargo test --workspace PASS (3 unit tests, 1 integration smoke test)
Summary:
  Implemented compileable crate skeletons with Phase 2 SIR IDs, Phase 3 runtime IDs, control/value/error/profile/cache scaffolds, RuntimePlan/EIR/helper/host/interpreter stubs, and workspace dependency wiring.
Next:
  Stage 3 runtime error registry and diagnostics (WP-04).

## 2026-06-30 11:00 · Stage 3 runtime error registry and diagnostics

Work Package: WP-04
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/error/mod.rs (created)
  - crates/vm_core/src/error/registry.rs (created)
  - crates/vm_core/src/error/language.rs (created)
  - crates/vm_core/src/error/structural.rs (created)
  - crates/vm_core/src/error/raise.rs (created)
  - crates/vm_core/src/error.rs (removed, replaced by error/ module)
  - crates/vm_core/src/value.rs (updated: Value::Error variant)
  - crates/vm_diag/src/source_span.rs (updated: SourceSpanId)
  - crates/vm_diag/src/diagnostic.rs (updated: DiagnosticError, StackTrace, ErrorSourceMapping)
  - crates/vm_diag/src/lib.rs (updated: diagnostic tests)
Spec References:
  - SPEC-P3-ERRORS
  - SPEC-P3-VALID
  - PHASE-3-RUNTIME-ERROR-REGISTRY.md
Gates:
  - G0 PASS
  - G1 PASS
  - G2 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test --workspace PASS (14 tests)
  - vm_core error registry tests (19 language + 10 structural codes)
  - vm_core non-Error raise rejection -> TypeError
  - vm_core structural error non-catchability
  - vm_core cleanup suppressed-error attachment
  - vm_diag source-span and stack-trace tests
Summary:
  Implemented frozen runtime error registry with full RuntimeErrorCode and VmStructuralErrorCode sets, ErrorObj/ErrorStore, VmError separation, raise validation, and diagnostic source-span/stack-trace foundation.
Next:
  Stage 4 RuntimePlan data model and validator (WP-05).

## 2026-06-30 12:00 · Stage 4 RuntimePlan data model and validator

Work Package: WP-05
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/digest.rs (created)
  - crates/vm_core/src/runtime_plan/mod.rs (created)
  - crates/vm_core/src/runtime_plan/schema.rs (created)
  - crates/vm_core/src/runtime_plan/validate.rs (created)
  - crates/vm_core/src/runtime_plan/fixtures.rs (created)
  - crates/vm_core/src/profile.rs (updated: full RuntimeTargetProfile)
  - crates/vm_core/src/cache.rs (updated: RuntimePlanCacheKey)
  - crates/vm_core/src/id.rs (updated: SlotLayoutId, ShapeId, FrameMapId, etc.)
  - crates/vm_core/src/lib.rs (updated)
  - crates/vm_runtime/src/runtime_plan.rs (updated: re-export vm_core)
  - crates/vm_eval/src/interpreter.rs (updated import path)
  - crates/vm_tests/src/lib.rs (updated smoke test)
Spec References:
  - SPEC-P3-RTP
  - SPEC-P3-VALID
  - SPEC-P3-CACHE
  - SPEC-P3-PROFILE
  - PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md
Gates:
  - G0 PASS
  - G1 PASS
  - G2 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS (20 tests)
  - minimal_valid_plan validation pass
  - unresolved ModuleId rejection
  - function without entry EIR rejection
  - record shape missing field index rejection
  - cache profile mismatch rejection
  - RuntimePlanCacheKey identity and digest sensitivity
Summary:
  Implemented closed RuntimePlan schema in vm_core with module/function/slot/type/shape/call/access/safepoint/deopt/helper/capability tables, validation entry point aligned to frozen rejection rules, and cache key construction.
Next:
  Stage 5 EIR data model and validator (WP-06).

## 2026-06-30 13:00 · Stage 5 EIR data model and validator

Work Package: WP-06
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/eir/mod.rs (created)
  - crates/vm_core/src/eir/schema.rs (created)
  - crates/vm_core/src/eir/validate.rs (created)
  - crates/vm_core/src/eir/fixtures.rs (created)
  - crates/vm_core/src/id.rs (updated: EirOpId, ConstantId)
  - crates/vm_core/src/lib.rs (updated)
  - crates/vm_runtime/src/eir.rs (updated: re-export vm_core)
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
  - SPEC-P3-HELPERS
  - PHASE-3-EIR-SCHEMA-CLOSURE.md
Gates:
  - G0 PASS
  - G1 PASS
  - G2 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test --workspace PASS (26 tests)
  - cargo test -p vm_core eir PASS (6 EIR validation tests)
  - cargo check --workspace PASS
Summary:
  Implemented closed EIR schema (module/function/block/op/terminator unions), ingest op/terminator kind validation, EIR module validator with block graph/helper/source-mapping rejection rules, and test fixtures.
Next:
  Stage 6 helper registry (WP-07).

## 2026-06-30 14:00 · WP-06 remediation after verification audit

Work Package: WP-06
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/eir/schema.rs (updated: full Check/Construct/Pattern unions, UnknownKind ingest, Optional terminator, DebugOp source_span)
  - crates/vm_core/src/eir/validate.rs (updated: LogicalOp block graph, BlockWithoutTerminator, may-collect root-map checks)
  - crates/vm_core/src/eir/fixtures.rs (updated: module-level unknown op, missing terminator, may-collect fixtures)
  - crates/vm_core/src/id.rs (updated: RootMapId)
  - D:\script\mcps/ (removed out-of-scope MCP artifact tree)
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
  - PHASE-3-EIR-SCHEMA-CLOSURE.md
  - PHASE-3-GC-METADATA-OWNERSHIP.md
Gates:
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS (29 tests: vm_core 25, vm_diag 3, sir 1, vm_tests 1)
  - cargo test -p vm_core eir PASS (10 eir-filtered tests)
  - cargo check --workspace PASS
Summary:
  Remediated WP-06 audit gaps: completed frozen inner-op unions, module-level unknown-op rejection via UnknownKind, LogicalOp block validation, BlockWithoutTerminator and may-collect/root-map validation, removed accidental mcps/ tree.
Next:
  Stage 6 helper registry (WP-07).

## 2026-07-01 10:00 · WP-06 ingest-layer remediation (schema closure)

Work Package: WP-06
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/eir/schema.rs (updated: required terminator, RootMap/SafepointRecord on module, no UnknownKind in closed union)
  - crates/vm_core/src/eir/ingest.rs (created: pre-resolution ingest types and validate_eir_module_ingest)
  - crates/vm_core/src/eir/validate.rs (updated: module-level root-map validation, ingest-based negative tests)
  - crates/vm_core/src/eir/fixtures.rs (updated: closed-schema valid fixtures, ingest fixtures for malformed cases)
  - crates/vm_core/src/eir/mod.rs (updated: export ingest module)
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
  - PHASE-3-EIR-SCHEMA-CLOSURE.md
  - PHASE-3-GC-METADATA-OWNERSHIP.md
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS (30 tests: vm_core 25, vm_diag 3, sir 1, vm_tests 1)
  - cargo test -p vm_core eir PASS (10 eir-filtered tests)
  - cargo check --workspace PASS
Summary:
  Remediated WP-06 schema-closure gaps: removed UnknownKind from closed EirOpKind union, restored required block terminator, added RootMap/SafepointRecord module tables with GC metadata validation, and introduced ingest layer for unknown-op/missing-terminator/unknown-terminator rejection tests.
Next:
  Stage 6 helper registry (WP-07).

## 2026-07-01 11:30 · WP-06 wire pipeline restructure (scope + single entry point)

Work Package: WP-06
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/eir/wire.rs (created: pub(crate) wire types, private tag validators)
  - crates/vm_core/src/eir/resolve.rs (created: wire-to-closed-schema resolution)
  - crates/vm_core/src/eir/ingest.rs (deleted)
  - crates/vm_core/src/eir/validate.rs (updated: EirModuleInput, single validate_eir_module, HelperRegistryView::may_collect)
  - crates/vm_core/src/eir/fixtures.rs (updated: wire fixtures, module safepoint/root-map metadata for may-collect)
  - crates/vm_core/src/eir/mod.rs (updated: private wire/resolve modules, no public ingest exports)
  - PROGRESS.md (this entry)
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
  - PHASE-3-EIR-SCHEMA-CLOSURE.md
  - PHASE-3-GC-METADATA-OWNERSHIP.md
Gates:
  - G0 PASS (scope: eir/** + PROGRESS only; D:\script\mcps removed before edits)
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 24 tests" / "test result: ok. 24 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_tests: "running 1 test" / "test result: ok. 1 passed"
    - unit-test total: 29 (1+24+3+1)
  - cargo test -p vm_core eir PASS — verbatim:
    - "running 9 tests" / "test result: ok. 9 passed"
  - cargo check --workspace PASS
Summary:
  Restructured WP-06 per strategist plan: removed out-of-scope mcps/ tree; collapsed ingest into private wire→resolve→semantic pipeline; single public validate_eir_module(EirModuleInput, ctx) entry point; may-collect classification via HelperRegistryView::may_collect and module SafepointRecord metadata; all negative tests route through validate_eir_module (Wire or Resolved). Note: earlier WP-06 PROGRESS entries cited inaccurate aggregate test totals (26/29/30); this entry corrects with verbatim log lines.
Next:
  Stage 6 helper registry (WP-07).

## 2026-07-01 14:00 · WP-06 ISSUE audit remediation (001–005)

Work Package: WP-06
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/eir/validate.rs (updated: ConstantPool, block args, barrier, Type/Shape/Field/Case ID validation, 8 new negative tests)
  - crates/vm_core/src/eir/fixtures.rs (updated: constant pool in minimal module, malformed fixtures for each ISSUE)
Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §7, §9, §23
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4, P3-V5)
Gates:
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_core eir PASS — "running 17 tests" / "test result: ok. 17 passed"
  - cargo test --workspace PASS — vm_core "running 32 tests" / unit total 37 (32+3+1+1)
Summary:
  Remediated ISSUE-20260701-001 through 005: constant-pool membership, Jump/LoopBackedge block-argument count, heap-write barrier policy via barrier_access_site_ids, RuntimePlan-bound Type/Shape/Field/Case ID sets, and negative tests for digest mismatch, unknown slot, guard failure, invalid entry block.
Next:
  Stage 6 helper registry (WP-07).

## 2026-07-06 10:00 · WP-07 Helper registry and dispatch

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/mod.rs (new)
  - crates/vm_runtime/src/helpers/schema.rs (new)
  - crates/vm_runtime/src/helpers/canonical.rs (new)
  - crates/vm_runtime/src/helpers/registry.rs (new)
  - crates/vm_runtime/src/helpers/validate.rs (new)
  - crates/vm_runtime/src/helpers.rs (removed)
  - crates/vm_core/src/eir/validate.rs (updated: HelperRegistryView doc)
  - crates/vm_tests/src/lib.rs (updated: EIR/registry integration test)
Spec References:
  - SPEC-P3-HELPERS
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md
  - PHASE-3-VALIDATION-MATRIX.md (P3-V6)
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime PASS — "running 10 tests" / "test result: ok. 10 passed"
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_runtime: "running 10 tests" / "test result: ok. 10 passed"
    - vm_tests: "running 2 tests" / "test result: ok. 2 passed"
    - unit-test total: 48 (1+32+3+10+2)
Summary:
  Implemented frozen canonical helper registry (47 descriptors), build/lookup/digest, policy validation (duplicate id/name, may_collect roots, may_raise source mapping, JIT-callable policy, capability metadata), implementation-consistency placeholder, and `eir_validation_view` bridge to EIR `HelperRegistryView`. Integration test confirms may-collect rejection uses registry-derived classification (ISSUE-20260701-006).
Next:
  Stage 7 value/heap/frame/control core (WP-08).

## 2026-07-06 12:00 · WP-08 Value / heap / object reference model

Work Package: WP-08
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/value/mod.rs (new)
  - crates/vm_runtime/src/value/key.rs (new)
  - crates/vm_runtime/src/value/string.rs (new)
  - crates/vm_runtime/src/heap/mod.rs (new)
  - crates/vm_runtime/src/heap/obj_ref.rs (new)
  - crates/vm_runtime/src/heap/object.rs (new)
  - crates/vm_runtime/src/heap/heap.rs (new)
  - crates/vm_runtime/src/readonly.rs (new)
  - crates/vm_runtime/src/frame.rs (new)
  - crates/vm_runtime/src/control.rs (new)
  - crates/vm_runtime/src/runtime_error.rs (new)
  - crates/vm_runtime/src/lib.rs (updated: module exports)
  - crates/vm_core/src/value.rs (updated: doc)
Spec References:
  - SPEC-P3-VALUES
  - SPEC-P3-PROFILE
  - SPEC-P3-GC-META
  - SPEC-P3-READONLY
  - SPEC-P3-CONTROL
  - PHASE-3-VALUE-KEY-STRING-SEMANTICS.md
  - PHASE-3-READONLY-VIEW-SEMANTICS.md
  - PHASE-3-CONTROL-STATE-MODEL.md
  - PHASE-3-VM-RUNTIME-ROUND1.md
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime PASS — "running 32 tests" / "test result: ok. 32 passed"
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_runtime: "running 32 tests" / "test result: ok. 32 passed"
    - vm_tests: "running 2 tests" / "test result: ok. 2 passed"
    - unit-test total: 70 (1+32+3+32+2)
Summary:
  Implemented runtime substrate: ValueKey hashability (NaN/non-hashable rejection), string scalar len/slice, generational ObjRef/Heap with List/Map/Record/ReadOnlyView/EnumValue objects, ReadOnlyView mutation rejection and shallow read delegation, Frame/SlotArray with Uninitialized/Value states, PendingControl/VmControl/RegionStack shells with ControlState mapping.
Next:
  Stage 8 structured unwinding (WP-10) or WP-09 frame/control integration per plan.

## 2026-07-06 14:00 · WP-10 Structured unwinding implementation

Work Package: WP-10
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/unwind/mod.rs (new)
  - crates/vm_runtime/src/unwind/cleanup.rs (new)
  - crates/vm_runtime/src/unwind/region.rs (new)
  - crates/vm_runtime/src/unwind/combine.rs (new)
  - crates/vm_runtime/src/unwind/perform.rs (new)
  - crates/vm_runtime/src/lib.rs (updated: unwind module export)
Spec References:
  - SPEC-P3-UNWIND
  - SPEC-P3-CONTROL
  - SPEC-P3-ERRORS
  - SPEC-P3-GC-META
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md
  - PHASE-3-CONTROL-STATE-MODEL.md
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime unwind PASS — "running 11 tests" / "test result: ok. 11 passed"
  - cargo test -p vm_runtime PASS — "running 43 tests" / "test result: ok. 43 passed"
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_runtime: "running 43 tests" / "test result: ok. 43 passed"
    - vm_tests: "running 2 tests" / "test result: ok. 2 passed"
    - unit-test total: 81 (1+32+3+43+2)
Summary:
  Implemented structured unwinding: CleanupState (defer/resource/finally stacks, CleanupProgress), RuntimeRegionFrame/UnwindContext, combine_cleanup_result and finally_override with suppressed-error attachment, perform_unwind loop with UnwindExecutor trait. Tests cover return-through-finally, defer/resource raise combination, break cleanup crossing, and finally override.
Next:
  Stage 9 module runtime (WP-11).

## 2026-07-06 16:00 · Pre-Stage-9 MAJOR audit remediation (001–004) + git bootstrap

Work Package: WP-06, WP-07, WP-10
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/eir/validate.rs (HelperRegistryView::may_raise, per-helper source mapping)
  - crates/vm_core/src/cache.rs (from_plan_with_helper_registry_digest)
  - crates/vm_core/src/eir/fixtures.rs (write_barrier fixture)
  - crates/vm_runtime/src/cache.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs (new)
  - crates/vm_runtime/src/helpers/registry.rs (may_raise_helper_ids)
  - crates/vm_runtime/src/helpers/validate.rs (eir_validation_view may_raise)
  - crates/vm_runtime/src/unwind/catch.rs (new)
  - crates/vm_runtime/src/unwind/perform.rs (catch dispatch, helper trait extension)
  - crates/vm_runtime/src/unwind/region.rs (catch_entries)
  - crates/vm_tests/src/lib.rs (integration tests)
  - .gitignore (new)
Spec References:
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §5
  - PHASE-3-CACHE-COMPATIBILITY-MATRIX.md §4, §6
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §9, §15
Gates:
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_runtime: "running 50 tests" / "test result: ok. 50 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 90 (1+32+3+50+4)
Summary:
  Remediated ISSUE-20260706-001 through 004: registry-driven may_raise for EIR source mapping, helper registry digest in cache key, catch-region dispatch in perform_unwind, helper_perform_unwind dispatch shell. Initialized git repository and committed workspace snapshot.
Next:
  Stage 9 module runtime (WP-11).

## 2026-07-06 18:30 · Stage 9 module runtime (WP-11)

Work Package: WP-11
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/module/mod.rs (new)
  - crates/vm_runtime/src/module/state.rs (new)
  - crates/vm_runtime/src/module/export.rs (new)
  - crates/vm_runtime/src/module/instance.rs (new)
  - crates/vm_runtime/src/module/registry.rs (new)
  - crates/vm_runtime/src/module/import.rs (new)
  - crates/vm_runtime/src/module/resolver.rs (new)
  - crates/vm_runtime/src/module/validate.rs (new)
  - crates/vm_runtime/src/module/runtime.rs (new)
  - crates/vm_runtime/src/lib.rs
Spec References:
  - PHASE-3-MODULE-RUNTIME-CONTRACT.md
  - PHASE-3-HOST-BOUNDARY-CONTRACT.md §11
  - PHASE-3-RUNTIME-ERROR-REGISTRY.md
  - PHASE-3-VALIDATION-MATRIX.md P3-V7
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_runtime: "running 69 tests" / "test result: ok. 69 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 109 (1+32+3+69+4)
Summary:
  Implemented module runtime substrate: ModuleState transition validator with explicit-retry policy, ModuleInstance/ModuleRegistry storage, ExportTable with sealing and duplicate rejection, named/whole import resolution with circular-export and failed-module checks, capability-gated resolver shell, top-level control rejection, ModuleRuntime orchestration, and canonical module helper id constants.
Next:
  Stage 10 call execution and host boundary (WP-12).

## 2026-07-06 19:00 · Stage 10 call execution and host boundary (WP-12)

Work Package: WP-12
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/call/mod.rs (new)
  - crates/vm_runtime/src/call/input.rs (new)
  - crates/vm_runtime/src/call/callable.rs (new)
  - crates/vm_runtime/src/call/bind.rs (new)
  - crates/vm_runtime/src/call/default.rs (new)
  - crates/vm_runtime/src/call/contract.rs (new)
  - crates/vm_runtime/src/call/builtin.rs (new)
  - crates/vm_runtime/src/call/runtime.rs (new)
  - crates/vm_runtime/src/lib.rs
  - crates/vm_host/src/call.rs (new)
  - crates/vm_host/src/error.rs (new)
  - crates/vm_host/src/host_function.rs
  - crates/vm_host/src/host_object.rs
  - crates/vm_host/src/host_root.rs
  - crates/vm_host/src/lib.rs
  - crates/vm_host/Cargo.toml
Spec References:
  - PHASE-3-CALL-EXECUTION-PROTOCOL.md
  - PHASE-3-HOST-BOUNDARY-CONTRACT.md
  - PHASE-3-RUNTIME-ERROR-REGISTRY.md
  - PHASE-3-VALIDATION-MATRIX.md
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_host: "running 6 tests" / "test result: ok. 6 passed"
    - vm_runtime: "running 84 tests" / "test result: ok. 84 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 130 (1+32+3+6+84+4)
Summary:
  Implemented call execution substrate: CallFrameInput, CallableTarget/registry, positional/named argument binding with arity errors, call-time default evaluation hook, parameter/return contract checks, builtin descriptor validation, CallRuntime orchestration, and call helper id constants. Expanded vm_host with HostFunctionWrapper/Descriptor, HostObjectWrapper, HostRootRegistry, host error normalization, and capability-gated execute_host_call protocol shell.
Next:
  Stage 11 GC metadata and cache compatibility hooks (WP-15).

## 2026-07-06 20:00 · Stage 11 GC metadata and cache compatibility hooks (WP-15, WP-16)

Work Package: WP-15, WP-16
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/gc/mod.rs (new)
  - crates/vm_runtime/src/gc/root_location.rs (new)
  - crates/vm_runtime/src/gc/root_map.rs (new)
  - crates/vm_runtime/src/gc/frame_map.rs (new)
  - crates/vm_runtime/src/gc/safepoint.rs (new)
  - crates/vm_runtime/src/gc/pending_control.rs (new)
  - crates/vm_runtime/src/gc/profile.rs (new)
  - crates/vm_runtime/src/gc/validate.rs (new)
  - crates/vm_runtime/src/cache_compat.rs (new)
  - crates/vm_runtime/src/lib.rs
Spec References:
  - PHASE-3-GC-METADATA-OWNERSHIP.md
  - PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md
  - PHASE-3-CACHE-COMPATIBILITY-MATRIX.md
  - PHASE-3-TARGET-PROFILE-SCHEMAS.md
  - PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_host: "running 6 tests" / "test result: ok. 6 passed"
    - vm_runtime: "running 100 tests" / "test result: ok. 100 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 146 (1+32+3+6+100+4)
Summary:
  Implemented GC metadata substrate: RootLocation enum, runtime RootMap/FrameMap/SafepointRecord with owner/location metadata, pending-control root visibility, GcProfile moving-GC policy checks, and metadata validation (safepoint/root-map/frame-map). Added cache compatibility layer: EirCacheKey, GcMetadataCacheKey, DigestInputSet, InternalCacheStore stale rejection, profile/helper digest mismatch rejection, and public-bytecode cache boundary guard.
Next:
  Stage 12 fast interpreter minimal execution (WP-17).

## 2026-07-06 21:00 · Stage 12 fast interpreter minimal execution (WP-17)

Work Package: WP-17
Agent Mode: main-only
Changed Files:
  - crates/vm_eval/src/interpreter/mod.rs (new)
  - crates/vm_eval/src/interpreter/state.rs (new)
  - crates/vm_eval/src/interpreter/ops.rs (new)
  - crates/vm_eval/src/interpreter/terminators.rs (new)
  - crates/vm_eval/src/interpreter/helpers.rs (new)
  - crates/vm_eval/src/interpreter/diagnostics.rs (new)
  - crates/vm_eval/src/interpreter/error.rs (new)
  - crates/vm_eval/src/interpreter/fixtures.rs (new)
  - crates/vm_eval/src/lib.rs
  - crates/vm_eval/src/interpreter.rs (removed)
  - crates/vm_tests/src/lib.rs
Spec References:
  - PHASE-3-EIR-SCHEMA-CLOSURE.md
  - PHASE-3-CONTROL-STATE-MODEL.md
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md
  - PHASE-3-GC-METADATA-OWNERSHIP.md
  - PHASE-3-VALIDATION-MATRIX.md
Gates:
  - G0 PASS
  - G1 PASS
  - G3 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 32 tests" / "test result: ok. 32 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_eval: "running 10 tests" / "test result: ok. 10 passed"
    - vm_host: "running 6 tests" / "test result: ok. 6 passed"
    - vm_runtime: "running 100 tests" / "test result: ok. 100 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 156 (1+32+3+10+6+100+4)
Summary:
  Implemented minimal EIR fast interpreter: InterpreterState with frame push/pop, block dispatch loop, Constant/Load/Store/Unary/Binary/Check/RuntimeHelper/Safepoint op handlers, Jump/Branch/Return/Raise/LoopBackedge terminators, helper bridge to perform_unwind, source span diagnostics, and safepoint poll hook. Tests cover literal execution, slot copy, branch bool check (positive/negative), binary add, loop backedge safepoint, undispatched helper error propagation, raise terminator, and helper_perform_unwind integration.
Next:
  Stage 13 conformance and regression (WP-18).

## 2026-07-06 22:00 · Remediation pass 1: Stage 0 tests dirs + WP-06 negative tests

Work Package: WP-00, WP-06
Agent Mode: main-only
Changed Files:
  - tests/conformance/.gitkeep (created)
  - tests/negative/.gitkeep (created)
  - tests/diagnostics/.gitkeep (created)
  - tests/regression/.gitkeep (created)
  - tests/fixtures/.gitkeep (created)
  - crates/vm_core/src/eir/fixtures.rs
  - crates/vm_core/src/eir/validate.rs
Spec References:
  - IMPLEMENTATION-CODING-PLAN.md (Stage 0 §7, Stage 5 §12)
  - PHASE-3-EIR-SCHEMA-CLOSURE.md §23
  - PHASE-3-VALIDATION-MATRIX.md (P3-V4)
  - AGENT.md §13
Gates:
  - G0 PASS
  - G1 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 38 tests" / "test result: ok. 38 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_eval: "running 10 tests" / "test result: ok. 10 passed"
    - vm_host: "running 6 tests" / "test result: ok. 6 passed"
    - vm_runtime: "running 100 tests" / "test result: ok. 100 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 162 (1+38+3+10+6+100+4)
  - cargo test -p vm_core eir PASS — 23 eir-filtered tests
Summary:
  Remediation pass 1 closes earliest plan gaps: created missing Stage 0 tests/ directory scaffold (conformance, negative, diagnostics, regression, fixtures). Closed WP-06 ISSUE-005/006 by adding six negative fixtures and tests for UnknownShapeId, UnknownFieldId, UnknownCaseId, UnknownCallSiteId, UnknownAccessSiteId, and UnknownDeoptId rejection paths.
Next:
  WP-10 nested multi-region unwind test (ISSUE-007); then WP-08/09 SlotState Cell/RuntimeInternal (ISSUE-008).


## 2026-07-06 22:30 · Remediation pass 2: WP-10 nested unwind test coverage

Work Package: WP-10
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/unwind/perform.rs
Spec References:
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §5, §10
  - PLAN/TRACEABILITY-MATRIX.md TR-009
  - AGENT.md §13
Gates:
  - G0 PASS
  - G1 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime nested_regions_unwind PASS
  - cargo test --workspace PASS — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 38 tests" / "test result: ok. 38 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_eval: "running 10 tests" / "test result: ok. 10 passed"
    - vm_host: "running 6 tests" / "test result: ok. 6 passed"
    - vm_runtime: "running 101 tests" / "test result: ok. 101 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 163 (1+38+3+10+6+101+4)
Summary:
  Closed ISSUE-007 by adding `nested_regions_unwind_inner_defer_before_outer_defer_and_finally` test: inner Block defer runs before outer Function defer and finally when pending Return unwinds a two-region stack; execution log asserts `defer:1` → `defer:2` → `finally:3` and empty region stack on resolution.
Next:
  WP-08/09 SlotState Cell/RuntimeInternal variants (ISSUE-008).

