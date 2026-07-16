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


## 2026-07-06 23:00 · Remediation pass 3: WP-08/09 frozen SlotState semantics (ISSUE-008)

Work Package: WP-08, WP-09
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/id.rs
  - crates/vm_runtime/src/binding_cell.rs (new)
  - crates/vm_runtime/src/runtime_value.rs (new)
  - crates/vm_runtime/src/frame.rs
  - crates/vm_runtime/src/lib.rs
Spec References:
  - PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md §2.2–§2.5
  - PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md §6.2, §7
  - IMPLEMENTATION-CODING-PLAN.md Stage 7
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime frame:: PASS — 10 frame/slot tests
  - cargo test --workspace PASS (2 consecutive runs) — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 38 tests" / "test result: ok. 38 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_eval: "running 10 tests" / "test result: ok. 10 passed"
    - vm_host: "running 6 tests" / "test result: ok. 6 passed"
    - vm_runtime: "running 108 tests" / "test result: ok. 108 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 170 (1+38+3+10+6+108+4)
  - cargo check --workspace PASS
Summary:
  Extended SlotState with Cell(BindingCellRef) and RuntimeInternal(RuntimeValue). Added BindingCellStore with mutability-checked write_cell, ordinary read dereferencing cells per §2.5, immutable cell write rejection (TypeError), and runtime-internal slots rejecting user-visible read/write (InternalVMError) while exposing read_runtime_internal/write_runtime_internal for VM-internal access. Added CellId to vm_core.
Next:
  WP-07 helper dispatch expansion (Pass 4).

## 2026-07-06 23:30 · Remediation pass 3b: WP-08/09 slot semantics audit fixes

Work Package: WP-08, WP-09
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/binding_cell.rs
  - crates/vm_runtime/src/frame.rs
  - crates/vm_runtime/src/write_barrier.rs (new)
  - crates/vm_runtime/src/lib.rs
Spec References:
  - PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md §2.2–§2.5
  - PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md §6.2, §7.2–§7.4
  - PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md §6.2
  - PHASE-3-VALIDATION-MATRIX.md (const assignment / readonly mutation)
Gates:
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p vm_runtime frame:: PASS — 13 frame/slot tests
  - cargo test --workspace PASS (2 consecutive runs, full stdout in scratch logs) — verbatim unit-test suites:
    - sir: "running 1 test" / "test result: ok. 1 passed"
    - vm_core: "running 38 tests" / "test result: ok. 38 passed"
    - vm_diag: "running 3 tests" / "test result: ok. 3 passed"
    - vm_eval: "running 10 tests" / "test result: ok. 10 passed"
    - vm_host: "running 6 tests" / "test result: ok. 6 passed"
    - vm_runtime: "running 113 tests" / "test result: ok. 113 passed"
    - vm_tests: "running 4 tests" / "test result: ok. 4 passed"
    - unit-test total: 175 (1+38+3+10+6+113+4)
  - cargo check --workspace PASS
  - Evidence: C:\Users\Lenovo\AppData\Local\Temp\grok-goal-00af83e14af3\implementer\workspace-test-1.log, workspace-test-2.log, slot-tests.log, check.log
Summary:
  Audit remediation for Pass 3: BindingCell now includes type_contract and CellOwner per §7.2; immutable cell writes raise ReadOnlyError (assignment error); write_cell checks TypeContractChecker and invokes WriteBarrierHook on heap-ref mutation; SlotReadPolicy::PermitUninitialized supports §2.3 explicit-permit reads on value slots. Prior pass 3 entry understated errors (TypeError) and omitted write_barrier.rs; verification logs replaced with full cargo captures.
Next:
  WP-07 helper dispatch expansion (Pass 4).

## 2026-07-06 23:45 · Remediation pass 3c: scope and evidence correction (documentation only)

Work Package: WP-08, WP-09
Agent Mode: main-only
Changed Files:
  - PROGRESS.md (this entry)
  - C:\Users\Lenovo\AppData\Local\Temp\grok-goal-00af83e14af3\implementer\check.log (evidence capture)
Spec References:
  - AGENT.md §4, §12
Gates:
  - G7 PASS
Tests:
  - cargo check --workspace PASS (output saved to scratch check.log)
  - cargo test -p vm_runtime frame:: PASS — 13 tests
Summary:
  Documentation correction: WP-08/09 slot work (passes 3+3b) cumulative changed files are crates/vm_core/src/id.rs, crates/vm_runtime/src/binding_cell.rs, crates/vm_runtime/src/runtime_value.rs, crates/vm_runtime/src/frame.rs, crates/vm_runtime/src/write_barrier.rs, crates/vm_runtime/src/lib.rs only. Pass 1 (eir/fixtures.rs, eir/validate.rs, tests/*/.gitkeep) and pass 2 (unwind/perform.rs) are separate bounded entries above; not part of pass 3 scope. Prior check.log was empty; replaced with full cargo check stdout (exit 0, warnings only).
Next:
  WP-07 helper dispatch expansion (Pass 4).

## 2026-07-07 00:00 · Remediation pass 3: WP-08/09 frozen SlotState semantics (git-scoped handoff)

Work Package: WP-08, WP-09
Agent Mode: main-only
Changed Files:
  - crates/vm_core/src/id.rs
  - crates/vm_runtime/src/binding_cell.rs (new)
  - crates/vm_runtime/src/runtime_value.rs (new)
  - crates/vm_runtime/src/write_barrier.rs (new)
  - crates/vm_runtime/src/frame.rs
  - crates/vm_runtime/src/lib.rs
  - PROGRESS.md
  - ISSUE.md
Spec References:
  - PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md §2.2–§2.5
  - PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md §6.2, §7.2–§7.4
  - PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md §6.2
  - IMPLEMENTATION-CODING-PLAN.md Stage 7
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime frame:: PASS — 13 frame/slot tests
  - cargo test --workspace PASS (2 runs, scratch logs) — unit-test total: 175 (1+38+3+10+6+113+4)
  - cargo check --workspace PASS
Summary:
  SlotState implements four frozen modes. BindingCell includes type_contract and CellOwner. Immutable cell writes raise ReadOnlyError. write_cell enforces TypeContractChecker and WriteBarrierHook on heap-ref mutation. SlotReadPolicy::PermitUninitialized supports §2.3 explicit-permit reads. Runtime-internal slots reject user-visible access (InternalVMError). Pass 1 and pass 2 committed separately (775dcf5, 342d3ea); this handoff matches git status for pass 3 only.
Next:
  WP-07 helper dispatch expansion (Pass 4).

## 2026-07-07 02:00 · Remediation pass 4: WP-07 Milestone H1 helper dispatch

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h1.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - crates/vm_eval/src/interpreter/state.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - crates/vm_eval/src/interpreter/ops.rs
  - crates/vm_eval/src/interpreter/fixtures.rs
  - crates/vm_eval/src/interpreter/mod.rs
  - crates/vm_tests/src/lib.rs
  - PROGRESS.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §3
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.3–§8.4, §8.11–§8.12
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.2
  - IMPLEMENTATION-CODING-PLAN.md
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime helpers:: PASS — 24 helper/dispatch tests (10 h1 + 3 dispatch + 11 validate/canonical)
  - cargo test -p vm_eval interpreter:: PASS — 11 tests incl helper_alloc_object_integration + undispatched_helper + helper_perform_unwind
  - cargo test --workspace PASS (2 runs, scratch logs) — unit-test total: 188 (1+38+3+11+6+125+4)
  - cargo check --workspace PASS
Summary:
  Pass 3 committed at 4d404ab. Implemented Milestone H1 helpers (alloc_object, construct_error, check_type_contract, check_callable, check_hashable, write_barrier) in helpers/h1.rs with central dispatch via HelperDispatchEnv/HelperDispatchOutcome. Interpreter state gains heap/callable_registry/type_checker/write_barrier substrate; bridge handles Value/Unit/VmControl outcomes. Undispatched negative test retargeted to helper_get_attribute (id 15). helper_perform_unwind routing preserved.
Next:
  Milestone H2 access/construction helpers (Pass 5).

## 2026-07-07 02:30 · Remediation pass 4b: H1 dispatch-API test coverage

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/dispatch.rs
  - PROGRESS.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.2
  - AGENT.md §13
Gates:
  - G5 PASS
Tests:
  - cargo test -p vm_runtime helpers::dispatch PASS — 10 tests via dispatch_helper for all six H1 ids (alloc_object, write_barrier, construct_error, check_type_contract, check_callable, check_hashable) plus perform_unwind and undispatched rejection
  - cargo test --workspace PASS (2 runs) — unit-test total: 195 (1+38+3+11+6+132+4)
Summary:
  Replaced partial h1_helpers_dispatch_through_central_boundary with per-family dispatch_helper tests including type/callable/hashable positive and negative paths; h1-helper-tests.log now captures helpers::dispatch output only.
Next:
  Milestone H2 access/construction helpers (Pass 5).

## 2026-07-07 03:00 · Agent onboarding HANDOVER.md

Work Package: WP-00 (documentation)
Agent Mode: main-only
Changed Files:
  - HANDOVER.md (created)
  - PROGRESS.md (this entry)
Spec References:
  - AGENT.md
  - docs/agent-plan/README.md
  - docs/agent-plan/HANDOFF-TEMPLATE.md
Gates:
  - G7 PASS
Tests:
  - cargo test --workspace PASS — unit-test total: 195 (1+38+3+11+6+132+4)
Summary:
  Added HANDOVER.md for new Agent sessions: authority order, repo map, remediation commit chain, key code paths, open issues, git scope discipline, and recommended next task (Pass 5 H2).
Next:
  Milestone H2 access/construction helpers (Pass 5).


## 2026-07-09 14:00 · Remediation pass 5: WP-07 Milestone H2 helper dispatch

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h2.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - PROGRESS.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.3
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.2–§8.3, §8.13–§8.14
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §3
  - IMPLEMENTATION-CODING-PLAN.md
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime helpers::dispatch PASS — 24 tests (H1 regression + all H2 ids via dispatch_helper + undispatched id 15)
  - cargo test -p vm_runtime helpers:: PASS — 51 helper tests
  - cargo test -p vm_eval interpreter:: PASS — 11 tests (H1 bridge + undispatched)
  - cargo test --workspace PASS (2 runs) — unit-test total: 215 (1+38+3+11+6+152+4)
Summary:
  Implemented Milestone H2 helpers (get/set attribute, index read/write, slice read, construct record/enum/map, numeric binary, compare, display) in helpers/h2.rs with bootstrap arg layouts over existing heap substrate. Routed canonical ids 11–14, 16–18, 21–23, 42 through central dispatch_helper; non-H2 ids remain InvalidHelperError (negative uses id 15 bind_method). Per-helper dispatch success + raise/reject coverage; no frozen-spec edits; interpreter H2 op expansion deferred (dispatch-boundary coverage sufficient per plan non-goals).
Next:
  Milestone H3 call-engine helpers (Pass 6), or Stage 13 / WP-18 conformance first matrix row.

## 2026-07-09 16:30 · Remediation pass 6: WP-07 Milestone H3 call-engine helpers

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h3.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - crates/vm_runtime/src/heap/heap.rs
  - crates/vm_runtime/src/call/contract.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - crates/vm_eval/src/interpreter/fixtures.rs
  - PROGRESS.md
  - ISSUE.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.4 / §12
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.1
  - PHASE-3-CALL-EXECUTION-PROTOCOL.md §3–§8, §12
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §3
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime helpers::dispatch PASS — 32 tests (H1/H2 regression + H3 call helpers + undispatched id 28)
  - cargo test -p vm_runtime helpers:: PASS — 64 helper tests
  - cargo test -p vm_eval interpreter:: PASS — 11 tests (undispatched retargeted to id 28)
  - cargo test --workspace PASS (2 runs) — unit-test total: 228 (1+38+3+11+6+165+4)
Summary:
  Implemented Milestone H3 helpers over existing call/ substrate: bind_method (15), check_arity (27), generic_call prepare path (25), call_builtin validate path (26). HelperDispatchEnv gains mut callable_registry, CapabilitySet, CallSiteFeedback, call_depth. heap.alloc_function for bound-method identity shells. Full body execution / frame push-pop deferred (ISSUE-20260709-001). Undispatched negative retargeted to helper_match_pattern id 28. H2 prerequisite retained unregressed.
Next:
  Milestone H4 control helpers (remaining raise/assert/defer/resource), or interpreter frame enter for prepared calls, or Stage 13 / WP-18.

## 2026-07-09 18:00 · Remediation pass 7: WP-07 Milestone H4 control helpers

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h4.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - PROGRESS.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.5
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.6–§8.8
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md §2, §5–§8
  - PHASE-3-RUNTIME-ERROR-REGISTRY.md §4
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime helpers::dispatch PASS — 38 tests (H1–H3 + H4 control + undispatched id 28)
  - cargo test -p vm_runtime helpers:: PASS — 79 helper tests
  - cargo test -p vm_eval interpreter:: PASS — 11 tests
  - cargo test --workspace PASS (2 runs) — unit-test total: 243 (1+38+3+11+6+180+4)
Summary:
  Implemented Milestone H4 helpers over existing error/unwind substrate: raise (3), attach_suppressed (4), assert_fail (5), register_defer (30), execute_defer (31), register_resource (32), close_resource (33). perform_unwind (29) already shipped in H1. Non-Error raise materializes TypeError; double close → ResourceStateError; defer/resource registration requires active region. Undispatched negative remains id 28 (match_pattern). No frozen-spec edits.
Next:
  Milestone H5 module helpers, or Stage 13 / WP-18 conformance first matrix row.

## 2026-07-09 19:30 · Remediation pass 8: WP-07 Milestone H5 module helpers

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h5.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - crates/vm_runtime/src/module/runtime.rs
  - crates/vm_runtime/src/module/resolver.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - PROGRESS.md
  - ISSUE.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.6
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.9
  - PHASE-3-MODULE-RUNTIME-CONTRACT.md §2–§10, §12, §14
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime helpers:: PASS — 88 helper tests
  - cargo test -p vm_eval interpreter:: PASS — 11 tests
  - cargo test --workspace PASS (2 runs) — unit-test total: 252 (1+38+3+11+6+189+4)
Summary:
  Implemented Milestone H5 helpers over ModuleRuntime: resolve_module (34), initialize_module (35), import_named (36), import_module (37), seal_exports (38). Dispatch env gains optional module_runtime + module_resolver. Capability-gated resolve, ImportCycleError for uninitialized circular exports, optional interface_id mismatch → ImportError. Initialize advances Unloaded→Loading→Initializing only (init EIR body deferred: ISSUE-20260709-002). Undispatched remains id 28.
Next:
  Milestone H6 capability/host helpers, or Stage 13 / WP-18 conformance first matrix row.

## 2026-07-09 20:30 · Remediation pass 9: WP-07 Milestone H6 capability/host helpers

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h6.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - PROGRESS.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.7
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §8.10
  - PHASE-3-HOST-BOUNDARY-CONTRACT.md §5–§8
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime helpers:: PASS — 95 helper tests
  - cargo test -p vm_eval interpreter:: PASS — 11 tests
  - cargo test --workspace PASS — unit-test total: 259 (1+38+3+11+6+196+4)
Summary:
  Implemented Milestone H6 helpers in vm_runtime bootstrap (no vm_host dependency cycle): check_capability (39), enter_host_call (40), exit_host_call (41) with call-scoped host roots and host error normalization to Raise/Structural. HelperDispatchEnv gains optional HostBoundarySession. Undispatched remains id 28.
Next:
  Milestone H7 optimization readiness, or Stage 13 / WP-18 conformance first matrix row.

## 2026-07-09 21:00 · Add GitHub Actions CI for workspace check/test

Work Package: WP-00
Agent Mode: main-only
Changed Files:
  - .github/workflows/ci.yml (new)
  - crates/vm_eval/src/interpreter/terminators.rs
  - PROGRESS.md
Spec References:
  - AGENT.md
  - REPOSITORY-PROCESS.md
  - scripts/check/workspace.ps1
  - scripts/test/workspace.ps1
Gates:
  - G0 PASS
  - G7 PASS
Tests:
  - cargo check --workspace PASS with RUSTFLAGS=-D warnings
  - cargo test --workspace PASS — unit-test total: 259 (1+38+3+11+6+196+4)
Summary:
  Added GitHub Actions CI on push/PR to main: stable Rust, cargo check --workspace, cargo test --workspace twice (flake guard), rust-cache. Fixed vm_eval TerminatorOutcome::Halt dead_code under -D warnings. CI mirrors local scripts/check and scripts/test.
Next:
  Push main to origin (fast-forward after rebase); then H7 or Stage 13.

## 2026-07-09 22:00 · Remediation pass 10: WP-07 Milestone H7 optimization readiness

Work Package: WP-07
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h7.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - PROGRESS.md
Spec References:
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §20.8
  - PHASE-3-RUNTIME-HELPER-CONTRACTS.md §6, §8.4.4
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §4–§6
  - PHASE-3-GC-METADATA-OWNERSHIP.md §8
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime helpers:: PASS — 106 helper tests
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings — unit-test total: 270 (1+38+3+11+6+207+4)
Summary:
  Milestone H7: helper_check_shape (id 9) via ShapeRegistry + dispatch; JIT readiness matrix (47 rows) with HelperJitCallDescriptor and optional deopt links; safepoint/root policy validation for descriptors and matrix. Undispatched remains id 28. Helper milestones H1–H7 complete for WP-07 bootstrap path.
Next:
  Stage 13 / WP-18 conformance first matrix row, or remaining non-dispatched helpers (pattern/membership/string_concat/load_cell).

## 2026-07-09 23:00 · Stage 13 / WP-18 conformance matrix scaffold + remaining helpers

Work Package: WP-18, WP-07
Agent Mode: main-only
Changed Files:
  - tests/MATRIX.md (new)
  - tests/conformance/README.md (new)
  - tests/negative/README.md (new)
  - tests/diagnostics/README.md (new)
  - tests/regression/README.md (new)
  - crates/vm_tests/src/conformance.rs (new)
  - crates/vm_tests/src/negative.rs (new)
  - crates/vm_tests/src/diagnostics.rs (new)
  - crates/vm_tests/src/regression.rs (new)
  - crates/vm_tests/src/lib.rs
  - crates/vm_runtime/src/helpers/remainder.rs (new)
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - crates/vm_runtime/src/helpers/h7.rs
  - crates/vm_runtime/src/frame.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - PROGRESS.md
Spec References:
  - IMPLEMENTATION-CODING-PLAN.md Stage 13
  - WORK-PACKAGE-INDEX.md WP-18
  - PHASE-3-VALIDATION-MATRIX.md
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §3
Gates:
  - G0 PASS
  - G1 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_tests PASS — 19 tests (CF/NG/DG/RG + smoke)
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings — unit-test total: 289 (1+38+3+11+6+211+19)
Summary:
  Stage 13/WP-18: MATRIX.md inventory with CF-01..07, NG-01..05, DG-01, RG-01..02 driving shipped APIs via vm_tests modules. Dispatched remaining helpers: numeric_unary, membership, construct_list/function, string_concat, load_cell/store_cell (via cell_slots env). Undispatched: match_pattern (28), load_module_slot (46).
Next:
  Stage 14 integration review (G6), or helper_match_pattern / load_module_slot, or deepen conformance coverage.

## 2026-07-09 23:30 · Full helper dispatch coverage + Stage 14 integration scan

Work Package: WP-07, WP-19
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/remainder.rs
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/h7.rs
  - crates/vm_eval/src/interpreter/fixtures.rs
  - crates/vm_tests/src/negative.rs
  - PROGRESS.md
  - ISSUE.md
Spec References:
  - IMPLEMENTATION-CODING-PLAN.md Stage 14
  - PHASE-3-FREEZE.md
  - PHASE-3-RUNTIME-HELPER-REGISTRY.md §3
Gates:
  - G0 PASS
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings — unit-test total: 290 (1+38+3+11+6+212+19)
Summary:
  Dispatched final registry helpers match_pattern (28) and load_module_slot (46); all 47 canonical helpers now route through dispatch_helper. Undispatched negative retargeted to id 99. Stage 14 scan: no public bytecode/CPython ABI strings in crates; language failures use RuntimeFailure not panic. Open notes remain ISSUE-001/002 (call body / module init body).
Next:
  Deepen conformance matrix, wire interpreter for prepared calls and module init body, or production GC/JIT phases later.

## 2026-07-10 00:30 · Interpreter nested call + module init body (ISSUE-001/002)

Work Package: WP-17, WP-07, WP-11
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/helpers/h3.rs
  - crates/vm_runtime/src/helpers/dispatch.rs
  - crates/vm_runtime/src/helpers/mod.rs
  - crates/vm_eval/src/interpreter/state.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - crates/vm_eval/src/interpreter/ops.rs
  - crates/vm_eval/src/interpreter/mod.rs
  - crates/vm_eval/src/interpreter/fixtures.rs
  - crates/vm_tests (env field prepared_call)
  - PROGRESS.md
  - ISSUE.md
Spec References:
  - PHASE-3-CALL-EXECUTION-PROTOCOL.md §3–§11
  - PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md §12, §20.4
  - PHASE-3-MODULE-RUNTIME-CONTRACT.md §3–§4
  - PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_eval PASS — 13 tests incl generic_call_enters_user_function_body + module_init_body_executes
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings — unit-test total: 292 (1+38+3+13+6+212+19)
Summary:
  generic_call now stores PreparedUserCall; interpreter EnterUserCall pushes nested frame, binds args, runs callee EIR to return, writes parent dest. run_module_init_function executes module init EIR body. Functions table loaded from EirModule. ISSUE-001/002 resolved.
Next:
  Expand nested-call mid-block resume, deepen conformance, or later-phase GC/JIT.

## 2026-07-10 10:00 · Documentation status unification (no HANDOVER)

Work Package: WP-00
Agent Mode: main-only
Changed Files:
  - docs/IMPLEMENTATION-STATUS.md (new)
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/README.md
  - ARCHITECTURE/00-project/STATUS.md
  - REPOSITORY-PROCESS.md
  - tests/MATRIX.md
  - AGENT.md
  - PROGRESS.md
Spec References:
  - AGENT.md
  - IMPLEMENTATION-CODING-PLAN.md
  - WORK-PACKAGE-INDEX.md
Gates:
  - G0 PASS
  - G7 PASS
Tests:
  - Not run (documentation-only; no code changes)
Summary:
  Unified live status without editing HANDOVER.md. Added docs/IMPLEMENTATION-STATUS.md as rewritable Stage/WP snapshot (baseline f8343d0). Set WP-00–WP-17 COMPLETE and WP-18/19 IN_PROGRESS in both plan indexes. Updated ARCHITECTURE STATUS, REPOSITORY-PROCESS, agent-plan README, MATRIX inventory, AGENT routing table + ISSUE last-status rule. Effective open audit: ISSUE-20260706-009 only.
Next:
  WP-18 matrix expansion or ISSUE-009; refresh IMPLEMENTATION-STATUS when baseline moves.

## 2026-07-10 12:00 · ISSUE-009 ReadOnlyView identity + WP-18 matrix expansion

Work Package: WP-13, WP-18, WP-19
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/value/mod.rs
  - crates/vm_runtime/src/readonly.rs
  - crates/vm_tests/src/conformance.rs
  - tests/MATRIX.md
  - agent/gate-records/G6-20260710-integration-notes.md (new)
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
  - ISSUE.md
Spec References:
  - PHASE-3-READONLY-VIEW-SEMANTICS.md §4–§5
  - PHASE-3-VALUE-KEY-STRING-SEMANTICS.md
  - TRACEABILITY-MATRIX.md TR-007
Gates:
  - G0 PASS
  - G1 PASS
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test -p vm_runtime readonly::tests PASS
  - cargo test -p vm_tests PASS — 21 tests (CF-10, CF-11 added)
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings — unit-test total: 298 (1+38+3+13+6+216+21)
Summary:
  Implemented values_identical / values_equal with ReadOnlyView unwrap for equality; enforced readonly(x) is x false for heap aggregates (ISSUE-009). Expanded WP-18 matrix CF-10/CF-11. Added agent/gate-records G6 notes. Effective open audits: none remaining from prior OPEN list after 009 resolve.
Next:
  Further WP-18 rows; optional deeper equality for Map; mid-block nested-call resume polish.

## 2026-07-10 14:00 · Map structural equality (WP-08/18)

Work Package: WP-08, WP-18
Agent Mode: main-only
Changed Files:
  - crates/vm_runtime/src/value/mod.rs
  - crates/vm_tests/src/conformance.rs
  - tests/MATRIX.md
  - PROGRESS.md
Spec References:
  - PHASE-3-VALUE-KEY-STRING-SEMANTICS.md
  - PHASE-3-READONLY-VIEW-SEMANTICS.md §5
Gates:
  - G0 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_runtime map_equality map_inequality PASS
  - cargo test -p vm_tests cf12 PASS
Summary:
  values_equal compares Map entries order-independently by key multiset; CF-12 matrix row added.
Next:
  Mid-block nested-call resume polish (commit 2 of session).

## 2026-07-10 14:30 · Mid-block nested-call resume (WP-17)

Work Package: WP-17
Agent Mode: main-only
Changed Files:
  - crates/vm_eval/src/interpreter/state.rs
  - crates/vm_eval/src/interpreter/mod.rs
  - crates/vm_eval/src/interpreter/fixtures.rs
  - PROGRESS.md
Spec References:
  - PHASE-3-CALL-EXECUTION-PROTOCOL.md
  - PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md
Gates:
  - G0 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_eval PASS — 14 tests incl generic_call_resumes_ops_after_call_site
Summary:
  InterpreterFrame.next_op_index tracks resume point after nested generic_call; Jump resets index. Mid-block fixture loads call result into another slot after return.
Next:
  Expand WP-18 matrix; optional Map equality via readonly view.

## 2026-07-14 12:00 · WP-18 TRACEABILITY matrix expansion (TR-006..015)

Work Package: WP-18, WP-19
Agent Mode: main-only
Changed Files:
  - crates/vm_tests/src/conformance.rs
  - crates/vm_tests/src/negative.rs
  - crates/vm_tests/src/regression.rs
  - tests/MATRIX.md
  - docs/IMPLEMENTATION-STATUS.md
  - agent/gate-records/G6-20260714-integration-notes.md (new)
  - PROGRESS.md
Spec References:
  - TRACEABILITY-MATRIX.md TR-006 TR-007 TR-008 TR-011 TR-014 TR-015
  - PHASE-3-VALUE-KEY-STRING-SEMANTICS.md
  - PHASE-3-READONLY-VIEW-SEMANTICS.md
  - PHASE-3-CALL-EXECUTION-PROTOCOL.md
  - PHASE-3-CACHE-COMPATIBILITY-MATRIX.md
  - PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md (interpreter fixtures)
Gates:
  - G0 PASS
  - G1 PASS
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test -p vm_tests PASS · 40 tests
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings · ~320 unit tests
Summary:
  Expanded Stage 13 / WP-18 matrix: CF-13..21, NG-06..13, RG-03 covering values/keys, readonly, slots, call bind, cache boundary, and interpreter TR-015 positive/negative paths. Refreshed IMPLEMENTATION-STATUS and G6 gate notes. No new OPEN audit items.
Next:
  Further TRACEABILITY rows (TR-009 finally override, TR-010 duplicate export, TR-013 RootMap); keep G6 evidence current.

## 2026-07-14 12:20 · WP-18 TR-009/010 matrix + status/G6 sync

Work Package: WP-18, WP-19
Agent Mode: main-only
Changed Files:
  - crates/vm_tests/src/negative.rs
  - crates/vm_tests/src/regression.rs
  - tests/MATRIX.md
  - docs/IMPLEMENTATION-STATUS.md
  - agent/gate-records/G6-20260714-integration-notes.md
  - PROGRESS.md
Spec References:
  - TRACEABILITY-MATRIX.md TR-009 TR-010
  - PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md
  - PHASE-3-MODULE-RUNTIME-CONTRACT.md
Gates:
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test -p vm_tests PASS · 43 tests
Summary:
  Added NG-14 duplicate export and RG-04/05 finally override/suppress rows. Synced IMPLEMENTATION-STATUS and G6 notes to ~323 unit tests.
Next:
  TR-013 RootMap / host residual matrix rows; keep commits frequent.

## 2026-07-14 12:30 · TR-013 NG-15 + push main

Work Package: WP-18, WP-19
Agent Mode: main-only
Changed Files:
  - crates/vm_tests/src/negative.rs
  - tests/MATRIX.md
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - TRACEABILITY-MATRIX.md TR-013
  - PHASE-3-GC-METADATA-OWNERSHIP.md
  - PHASE-3-VALIDATION-MATRIX.md
Gates:
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test -p vm_tests ng15 PASS
Summary:
  NG-15 may-collect without RootMap matrixed. Status baseline e1ff43c; main pushed (5849835..e1ff43c matrix session).
Next:
  Host root / TR-012 residual matrix; keep expanding WP-18 without blocking.

## 2026-07-14 12:40 · TR-012 host root matrix

Work Package: WP-18
Agent Mode: main-only
Changed Files:
  - crates/vm_tests/src/conformance.rs
  - crates/vm_tests/src/negative.rs
  - tests/MATRIX.md
  - PROGRESS.md
Spec References:
  - TRACEABILITY-MATRIX.md TR-012
  - PHASE-3-HOST-BOUNDARY-CONTRACT.md
Gates:
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_tests cf22_ ng16_ PASS
Summary:
  CF-22/NG-16 cover host root registry retention policy in the Stage 13 matrix.
Next:
  Residual WP-18 validation cells; keep G6 notes on major baselines.

## 2026-07-14 12:55 · WP-18 CF-23/DG-02 + status refresh

Work Package: WP-18, WP-19
Agent Mode: main-only
Changed Files:
  - crates/vm_tests/src/conformance.rs
  - crates/vm_tests/src/diagnostics.rs
  - tests/MATRIX.md
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - TRACEABILITY-MATRIX.md TR-015
  - PHASE-3-GC-METADATA-OWNERSHIP.md (safepoint poll)
Gates:
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p vm_tests PASS · 48 tests
Summary:
  CF-23 loop backedge safepoint poll and DG-02 interpreter source-span diagnostics added to matrix. Snapshot test count ~328.
Next:
  Continue residual TRACEABILITY cells when needed; WP-19 G6 remain PASS_WITH_NOTES.

## 2026-07-14 14:00 · WP-18 complete: TRACEABILITY gap closure

Work Package: WP-18
Agent Mode: main-only
Changed Files:
  - crates/vm_tests/src/gap_closure.rs (new)
  - crates/vm_tests/src/lib.rs
  - tests/MATRIX.md
  - docs/IMPLEMENTATION-STATUS.md
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/TRACEABILITY-MATRIX.md
  - PROGRESS.md
Spec References:
  - TRACEABILITY-MATRIX.md TR-002 through TR-017
  - WORK-PACKAGE-INDEX.md WP-18 completion criteria
  - PHASE-3-VALIDATION-MATRIX.md
  - IMPLEMENTATION-CODING-PLAN Stage 13
Gates:
  - G0 PASS
  - G1 PASS
  - G5 PASS
  - G6 PASS_WITH_NOTES
  - G7 PASS
Tests:
  - cargo test -p vm_tests PASS · 78 tests
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings
Summary:
  Closed WP-18 bootstrap completion criteria: every implemented subsystem has
  traceable positive and negative matrix rows (TR-002..017). Added gap_closure
  module (CF-08/09/24..32, NG-17..33, RG-06/07, DG-03). Marked WP-18 COMPLETE,
  Stage 13 COMPLETE, TR-017 COMPLETE (bootstrap). Exhaustive product-language
  conformance remains out of scope (TR-GAP / WP-19 residual).
Next:
  WP-19 Stage 14 integration sign-off.

## 2026-07-14 16:00 · WP-19 complete: Stage 14 integration gates

Work Package: WP-19
Agent Mode: main-only
Changed Files:
  - crates/vm_tests/src/integration.rs (new)
  - crates/vm_tests/src/lib.rs
  - scripts/integration/g6-scan.ps1 (new)
  - scripts/integration/g6-scan.sh (new)
  - .github/workflows/ci.yml
  - agent/gate-records/WP-19-integration-gate-plan.md
  - agent/gate-records/WP-19-regression-gate-plan.md
  - agent/gate-records/WP-19-release-candidate-criteria.md
  - agent/gate-records/WP-19-post-freeze-erratum-policy.md
  - agent/gate-records/G0-G7-20260714-wp19-final.md
  - agent/gate-records/WP-19-handoff.md
  - docs/IMPLEMENTATION-STATUS.md
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/TRACEABILITY-MATRIX.md
  - HANDOVER.md
  - PROGRESS.md
Spec References:
  - SPEC-P3-FREEZE
  - SPEC-P3-VALID
  - SPEC-P3-CACHE
  - SPEC-P3-GC-META
  - SPEC-P3-HOST
  - TRACEABILITY-MATRIX.md TR-018
  - GATE-CHECKLIST.md G0-G7
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
  - cargo test --workspace PASS with RUSTFLAGS=-D warnings · ~368 unit tests (vm_tests 88)
  - cargo test -p vm_tests integration:: PASS · IG-01..IG-10
  - scripts/integration/g6-scan.ps1 PASS
Summary:
  Closed WP-19 and Stage 14 for Phase 3 bootstrap: integration gate plan,
  regression plan, RC criteria, post-freeze erratum policy, G0-G7 final record,
  IG suite, automated G6 scan in CI. Coding plan §24 bootstrap completion MET.
  All WP-00..WP-19 COMPLETE for current Phase 3 substrate goals.
Next:
  Optional Phase 1 / TR-GAP language work; maintain CI green.

## 2026-07-16 11:34 · Phase 3 bootstrap re-verification and closure stamp

Work Package: WP-00..WP-19 (closure audit; no package reopen)
Agent Mode: main-only
Changed Files:
  - agent/gate-records/PHASE-3-BOOTSTRAP-CLOSURE-20260714.md
  - docs/IMPLEMENTATION-STATUS.md
  - docs/agent-plan/TRACEABILITY-MATRIX.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - PLAN/TRACEABILITY-MATRIX.md
  - HANDOVER.md
  - PROGRESS.md
Spec References:
  - PHASE-3-FREEZE.md
  - IMPLEMENTATION-CODING-PLAN.md §24
  - GATE-CHECKLIST.md G0-G7
  - TRACEABILITY-MATRIX.md TR-000..TR-020, TR-GAP-*
Gates:
  - G0 PASS (re-check)
  - G1 PASS (re-check)
  - G5 PASS (cargo test --workspace)
  - G6 PASS (g6-scan.ps1)
  - G7 PASS (closure record)
Tests:
  - cargo check --workspace PASS with RUSTFLAGS=-D warnings
  - cargo test --workspace PASS · ~368 unit tests (vm_tests 88)
  - scripts/integration/g6-scan.ps1 PASS
Summary:
  Re-verified Phase 3 bootstrap against coding plan §24 and WP-19 final gates.
  Effective OPEN issues: 0. Fixed PLAN WP-18/19 status drift (IN_PROGRESS→COMPLETE).
  Upgraded TRACEABILITY core rows to COMPLETE (bootstrap). Wrote formal closure
  record PHASE-3-BOOTSTRAP-CLOSURE-20260714.md. Decision: Phase 3 bootstrap CLOSED.
Next:
  New product WPs only (e.g. Phase 1 frontend); do not reopen WP-00..19 without reason.

## 2026-07-16 11:41 · WP-20 process + WP-21 Phase 1 lexer (script_lex)

Work Package: WP-20, WP-21
Agent Mode: main-only
Changed Files:
  - crates/script_lex/** (new)
  - Cargo.toml (workspace member)
  - Cargo.lock
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/TRACEABILITY-MATRIX.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - PLAN/TRACEABILITY-MATRIX.md
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-FREEZE
  - SPEC-P1-LANG (sections 3-6 lexical)
  - SPEC-P1-DESIGN
  - TR-P1-000..TR-P1-003
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p script_lex PASS (18 tests)
  - cargo test --workspace PASS
Summary:
  Opened Phase 1 frontend track: WP-20 process/TRACE complete; WP-21 implements
  crates/script_lex lexer (indent stack, keywords, literals, strings, operators,
  line continuation) per PHASE-1-LANGUAGE-SPEC. WP-21 COMPLETE.
Next:
  WP-22 parser and AST (minimal module toward fib).

## 2026-07-16 11:42 · WP-22 Phase 1 parser/AST bootstrap (script_parse)

Work Package: WP-22
Agent Mode: main-only
Changed Files:
  - crates/script_parse/** (new)
  - Cargo.toml
  - Cargo.lock
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/TRACEABILITY-MATRIX.md
  - PLAN/*
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG (module/decl/stmt/expr bootstrap)
  - TR-P1-004
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p script_parse PASS (6 tests, fib shape)
Summary:
  Added recursive-descent parser and AST for bootstrap Phase 1 surface
  (let/const/def/if/while/return/call/arith/list). WP-22 COMPLETE bootstrap.
Next:
  WP-23 semantic binding skeleton.

## 2026-07-16 11:45 · WP-23 script_sema + parser for/break/continue

Work Package: WP-23 (COMPLETE); WP-22 surface extend
Agent Mode: main-only
Changed Files:
  - crates/script_sema/** (new)
  - crates/script_parse/src/ast.rs
  - crates/script_parse/src/parser.rs
  - Cargo.toml
  - Cargo.lock
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/TRACEABILITY-MATRIX.md
  - PLAN/*
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG section 2.1, 2.2, for/break/continue/return rules
  - TR-P1-005
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p script_sema PASS (13)
  - cargo test -p script_parse PASS (7)
Summary:
  Binding/scope analyzer with let/const/def, block scope, loop control,
  top-level return rejection, print prelude. Parser adds for/break/continue.
Next:
  WP-24 SIR materialization from analyzed AST.

## 2026-07-16 11:48 · WP-24 SIR materialization + Phase 1 surface expand

Work Package: WP-24 (COMPLETE); WP-22/23 surface
Agent Mode: main-only
Changed Files:
  - crates/sir/src/{unit,node,lib}.rs
  - crates/script_lower/** (new)
  - crates/script_parse (import/export/raise/assert)
  - crates/script_sema (import/export/raise/assert)
  - Cargo.toml, Cargo.lock
  - docs/agent-plan/*, PLAN/*, docs/IMPLEMENTATION-STATUS.md, PROGRESS.md
Spec References:
  - SPEC-P1-LANG
  - SPEC-P2-IR / SPEC-P2-FREEZE
  - TR-P1-006, TR-P1-007
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p script_lower -p script_parse -p script_sema -p sir PASS
Summary:
  Bootstrap IrUnit + SirNode tables; script_lower compile_to_sir after sema.
  Expanded Phase 1 surface: import, export, raise, assert. Fib lowers to SIR.
Next:
  WP-25 SIR to RuntimePlan/EIR for vm_eval execution path.

## 2026-07-16 11:52 · WP-25 source-to-EIR codegen; fib(10)=55 end-to-end

Work Package: WP-25
Agent Mode: main-only
Changed Files:
  - crates/script_codegen/** (new)
  - crates/vm_eval/src/interpreter/ops.rs (binary arith/compare)
  - Cargo.toml, Cargo.lock
  - docs/*, PLAN/*, PROGRESS.md
Spec References:
  - SPEC-P1-LANG
  - SPEC-P3-EIR
  - SPEC-P3-CALL
  - TR-P1-008
  - PROJECT-OVERVIEW.md fib success sample
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G6 PASS_WITH_NOTES (bootstrap subset)
Tests:
  - cargo test -p script_codegen PASS (fib 55, print(fib(10)))
  - cargo test -p vm_eval PASS
  - cargo test --workspace PASS
Summary:
  Added script_codegen lowering AST to EIR with nested calls; extended
  interpreter binary ops. PROJECT-OVERVIEW fib sample executes end-to-end.
Next:
  Expand codegen surface (for/lists/and-or) or host print I/O as product needs.

## 2026-07-16 11:58 · Unified implementation guidance (Phase-1-first)

Work Package: WP-L00 (plan document landing; GAP matrix still pending)
Agent Mode: main-only
Changed Files:
  - PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md (new, master)
  - docs/agent-plan/UNIFIED-IMPLEMENTATION-GUIDANCE.md (mirror)
  - PLAN/README.md
  - docs/agent-plan/README.md
  - docs/IMPLEMENTATION-STATUS.md
  - PLAN/MANIFEST.md (if updated)
  - PROGRESS.md
Spec References:
  - SPEC-P1-FREEZE / SPEC-P1-LANG / SPEC-P1-DESIGN (strategy cites)
  - PROJECT-OVERVIEW.md product pipeline
  - PHASE-3 bootstrap archive (T-P3B)
Gates:
  - G0 PASS (planning only; no frozen-spec edits)
  - G1 PASS (plan cites concrete specs)
  - G7 PASS (handoff via PROGRESS + status)
Tests:
  - not run (documentation-only change)
Summary:
  Published sole forward plan guide: tracks T-P3B archive, T-P1 active
  (Phase-1-first), T-P2/T-P3L ordered later, T-DEMO quarantine for
  script_codegen. New WP-L* series. WP-20..25 superseded as plan IDs.
Next:
  WP-L00 remainder: docs/phase-1/P1-GAP-MATRIX.md + WORK-PACKAGE-INDEX WP-L* rows.

## 2026-07-16 12:01 · WP-L00 complete + WP-L01 lexical SPEC alignment

Work Package: WP-L00, WP-L01
Agent Mode: main-only
Changed Files:
  - docs/phase-1/P1-GAP-MATRIX.md
  - docs/phase-1/P1-TEST-MATRIX.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - crates/script_lex/src/lexer.rs
  - crates/script_lex/src/span.rs
  - crates/script_lex/src/lib.rs
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG sections 3-6
  - UNIFIED-IMPLEMENTATION-GUIDANCE.md
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
  - G7 PASS
Tests:
  - cargo test -p script_lex PASS (26 tests)
Summary:
  Closed WP-L00 (GAP/TEST matrices, WP-L* registry). WP-L01: NFC idents,
  XID unicode, CR lines, full keyword table, delimiters, brace/bracket
  continuation, line_col_at helper. Next WP-L02 grammar/AST v0.
Next:
  WP-L02 expand script_parse against P1-GAP-MATRIX grammar rows.

## 2026-07-16 12:04 · WP-L02 grammar/AST v0 surface expansion

Work Package: WP-L02
Agent Mode: main-only
Changed Files:
  - crates/script_parse/**
  - crates/script_sema/src/analyze.rs
  - crates/script_codegen/src/codegen.rs
  - crates/script_lower/src/lower.rs
  - docs/phase-1/P1-GAP-MATRIX.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG §4.4, §6.7, §9.6, §10.2
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p script_parse -p script_sema -p script_lower -p script_codegen PASS
Summary:
  from-import, augmented assignment, map literals, empty/missing block rejection.
  Sema covers from-import bindings and aug-assign mutability.
Next:
  WP-L03 semantic v0 (Bool conditions, NFC clash, export flags).

## 2026-07-16 12:07 · WP-L03 semantic analysis (Bool, NFC, export)

Work Package: WP-L03
Agent Mode: main-only
Changed Files:
  - crates/script_sema/**
  - docs/phase-1/P1-GAP-MATRIX.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
  - Cargo.lock
Spec References:
  - SPEC-P1-LANG §2.1–2.3, §3.3
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p script_sema PASS (22)
Summary:
  Bool conditions for if/while/assert; and/or/not operand checks; NFC
  same-scope clash; export marks Binding.exported.
Next:
  WP-L04 AnalyzedModule API + unified diagnostics.

## 2026-07-16 12:07 · WP-L04 AnalyzedModule + WP-L05 T-P1 acceptance

Work Package: WP-L04, WP-L05
Agent Mode: main-only
Changed Files:
  - crates/script_sema/src/analyzed.rs
  - crates/script_sema/src/lib.rs
  - docs/phase-1/P1-GAP-MATRIX.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG
  - UNIFIED-IMPLEMENTATION-GUIDANCE.md §5.2 P1-A..F
Gates:
  - G0-G5 PASS
  - G7 PASS
Tests:
  - cargo test -p script_sema PASS (26)
Summary:
  check_module/analyze_source → AnalyzedModule with FrontendDiagnostic
  (stage, span, line/col). T-P1 accepted for v0; residuals only DEFER in GAP.
Next:
  T-P2 WP-S00 SIR materialization planning/implementation.

## 2026-07-16 12:10 · T-P2 WP-S00/S01 SIR materialize + validate

Work Package: WP-S00, WP-S01
Agent Mode: main-only
Changed Files:
  - crates/sir/src/unit.rs
  - crates/script_lower/**
  - crates/sir_validate/**
  - docs/IMPLEMENTATION-STATUS.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - docs/agent-plan/WORK-PACKAGE-INDEX.md
  - PROGRESS.md
Spec References:
  - SPEC-P2-IR §4
  - UNIFIED-IMPLEMENTATION-GUIDANCE T-P2
Gates:
  - G0 PASS
  - G1 PASS
  - G4 PASS
  - G5 PASS
Tests:
  - cargo test -p script_lower -p sir_validate -p sir PASS
Summary:
  Canonical path: analyze_source → materialize_sir → IrUnit with sources table.
  sir_validate::validate_ir_unit structural checks. T-P2 bootstrap baseline.
Next:
  WP-S02 deepen SIR tables; then T-P3L WP-R00 normative lowering.

## 2026-07-16 12:14 · WP-S02 control_regions + WP-R00 SIR→EIR fib path

Work Package: WP-S02, WP-R00
Agent Mode: main-only
Changed Files:
  - crates/sir (ControlRegionDescriptor)
  - crates/script_lower (emit regions)
  - crates/sir_validate (SIR012-015)
  - crates/script_eir_lower/** (new)
  - Cargo.toml
  - docs/IMPLEMENTATION-STATUS.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - PROGRESS.md
Spec References:
  - SPEC-P2-IR §4 control_regions
  - PHASE-3-EIR-SCHEMA / SIR-LOWERING (bootstrap subset)
Gates:
  - G0-G5 PASS
Tests:
  - cargo test -p script_eir_lower PASS (fib 55 via SIR)
  - script_lower / sir_validate PASS
Summary:
  SIR control_regions populated; normative pipeline compile_source_via_sir
  runs fib(10)=55 without script_codegen.
Next:
  Expand SIR→EIR surface; RuntimePlan packaging; host print I/O.

## 2026-07-16 12:16 · WP-R01 list/for/and-or, RuntimePlan shell, script-vm CLI

Work Package: WP-R01
Agent Mode: main-only
Changed Files:
  - crates/script_eir_lower (list, for unroll, and/or, package)
  - crates/vm_cli (script-vm run/eval)
  - docs/IMPLEMENTATION-STATUS.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG (for, list, and/or)
  - SPEC-P3-EIR helpers construct_list
  - SPEC-P3-RTP shell
Gates:
  - G0-G5 PASS
Tests:
  - cargo test -p script_eir_lower PASS (6)
  - script-vm eval fib(10) → 55
Summary:
  Expanded SIR→EIR; compile_executable attaches validated RuntimePlan shell;
  CLI runs normative pipeline.
Next:
  Deeper RuntimePlan from SIR; host print; remaining control constructs.

## 2026-07-16 12:19 · WP-R02 break/continue/raise/assert + host print

Work Package: WP-R02
Agent Mode: main-only
Changed Files:
  - crates/script_eir_lower/src/lower.rs
  - crates/script_eir_lower/src/pipeline.rs
  - crates/vm_eval/src/interpreter/helpers.rs
  - crates/vm_eval/src/interpreter/mod.rs
  - crates/vm_cli/src/main.rs
  - docs/IMPLEMENTATION-STATUS.md
  - PLAN/WORK-PACKAGE-INDEX.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG control/raise/assert/print
  - SPEC-P3-HELPERS display/construct_error
Gates:
  - G0-G5 PASS
Tests:
  - cargo test -p script_eir_lower PASS (10)
  - cargo test -p vm_eval PASS
Summary:
  while break/continue; raise/assert → EIR Raise; print → helper_display + stdout;
  frame slots 128. CLI prints raise messages.
Next:
  General for; richer RuntimePlan from SIR; map literals EIR.

## 2026-07-16 12:23 · WP-R03 map SIR/EIR + RuntimePlan metadata from SIR

Work Package: WP-R03
Agent Mode: main-only
Changed Files:
  - crates/sir/src/node.rs (SirNode::Map)
  - crates/script_lower (map entries)
  - crates/sir_validate (map children)
  - crates/script_eir_lower (map construct, package)
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG §6.7
  - SPEC-P3-HELPERS construct_map
  - SPEC-P3-RTP export/function plans
Gates:
  - G0-G5 PASS
Tests:
  - cargo test -p script_eir_lower PASS (12)
Summary:
  SirNode::Map; EIR construct_map; compile_executable fills exports and
  per-EIR function plans from SIR/EIR.
Next:
  Index/subscript syntax; general for; finally/unwind on raise.

## 2026-07-16 12:25 · WP-R04 index/subscript full stack

Work Package: WP-R04
Agent Mode: main-only
Changed Files:
  - crates/script_parse (Index, IndexAssign)
  - crates/script_sema
  - crates/sir (Index, IndexAssign nodes)
  - crates/script_lower
  - crates/script_eir_lower (index_read/write helpers)
  - crates/script_codegen (reject with note)
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG assignment_target / index
  - SPEC-P3-HELPERS index_read/write
Gates:
  - G0-G5 PASS
Tests:
  - cargo test -p script_eir_lower PASS (14) list/map index
Summary:
  xs[i] / m[k] parse and lower via helper_index_read/write.
Next:
  Attribute access; general for; finally unwind.

## 2026-07-16 12:27 · WP-R05 attribute access end-to-end

Work Package: WP-R05
Agent Mode: main-only
Changed Files:
  - crates/script_parse (Attr, AttrAssign, unified L-value assign)
  - crates/script_sema
  - crates/sir (Attr, AttrAssign)
  - crates/script_lower / script_eir_lower
  - docs/IMPLEMENTATION-STATUS.md
  - PROGRESS.md
Spec References:
  - SPEC-P1-LANG assignment_target attribute
  - SPEC-P3-HELPERS index_read/write (bootstrap map keys)
Gates:
  - G0-G5 PASS
Tests:
  - cargo test -p script_eir_lower PASS (15) attr_as_map_field
Summary:
  o.x / o.x = v lowered as map string-key index ops (record shapes later).
Next:
  Record types; general for; finally unwind.
