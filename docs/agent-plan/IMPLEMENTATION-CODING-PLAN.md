# Implementation Coding Plan

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications, `AGENT-MASTER-PLAN.md`, and `AGENT.md`  
Created: 2026-06-29 11:00:40

---

## 0. Purpose

This document is the concrete, step-by-step coding plan for Agent-led implementation.

It replaces any interpretation of the Agent plan as a merely abstract governance document.

The plan must be executable by a main Agent coordinating at most four subordinate Agents.

It specifies:

```text
workspace bootstrap
repository layout
crate creation order
module/file creation order
implementation sequence
test sequence
gate sequence
PROGRESS.md / ISSUE.md maintenance
handoff requirements
```

It does not specify VM semantics directly.

When implementation details require semantic content, this plan must cite frozen specification documents rather than restating or inventing formulas, schemas, or runtime rules.

---

## 1. Hard Boundary

The coding plan may prescribe:

```text
which directories to create
which files to create
which Rust crates/modules to create
which implementation units come first
which tests are required
which gates must pass
which documents to update
which Agent role may inspect a task
```

The coding plan must not prescribe:

```text
new RuntimePlan schema fields
new EIR operation semantics
new helper semantics
new runtime error semantics
new module semantics
new call semantics
new GC/JIT compatibility semantics
```

For those, cite frozen specifications.

---

## 2. Repository Bootstrap Target

The first implementation step is to create or align the repository to this target shape.

```text
/
  AGENT.md
  PROGRESS.md
  ISSUE.md

  docs/
    frozen-specs/
      phase-1/
      phase-2/
      phase-3/
    agent-plan/

  crates/
    sir/
    sir_validate/
    vm_core/
    vm_runtime/
    vm_eval/
    vm_diag/
    vm_host/
    vm_tests/
    vm_cli/

  tests/
    conformance/
    negative/
    diagnostics/
    regression/
    fixtures/

  agent/
    work-packages/
    handoffs/
    gate-records/
    audit-records/
    task-logs/

  scripts/
    check/
    test/
    validate/
```

If the existing repository already has a different layout, the Agent must not rewrite it blindly.

Instead:

```text
1. inspect existing layout
2. map existing directories to target roles
3. create only missing directories
4. record differences in ISSUE.md if they block execution
5. record created directories in PROGRESS.md
```

---

## 3. Bootstrap Files

Before coding, ensure these files exist at repository root:

```text
AGENT.md
PROGRESS.md
ISSUE.md
```

### 3.1 AGENT.md

`AGENT.md` must be copied or adapted from the Agent plan package.

It owns workflow constraints.

### 3.2 PROGRESS.md

Create only if absent.

Initial content:

```text
# PROGRESS.md

Document class: Append-only implementation progress log  
Rule: Only append change summaries. Do not rewrite old entries.
```

### 3.3 ISSUE.md

Create only if absent.

Initial content:

```text
# ISSUE.md

Document class: Append-only audit findings log  
Rule: Only append audit results. Do not rewrite old entries.
```

Do not fabricate history.

---

## 4. Frozen Specification Placement

The implementation repository must make frozen specifications reachable.

Preferred placement:

```text
docs/frozen-specs/
  phase-1/
  phase-2/
  phase-3/
```

Allowed alternatives:

```text
external archive path with stable relative reference
git submodule
read-only docs package
```

The Agent must record the chosen placement in `PROGRESS.md`.

If frozen specs are unavailable, G1 fails and implementation must stop.

---

## 5. Agent Plan Placement

Place the Agent implementation plan documents under:

```text
docs/agent-plan/
```

Required files:

```text
AGENT-MASTER-PLAN.md
AGENT-OPERATING-PROTOCOL.md
IMPLEMENTATION-CODING-PLAN.md
WORK-PACKAGE-INDEX.md
GATE-CHECKLIST.md
TRACEABILITY-MATRIX.md
RISK-REGISTER.md
HANDOFF-TEMPLATE.md
```

Repository-root `AGENT.md` may duplicate the operational subset because coding agents typically read root `AGENT.md` first.

---

## 6. Rust Workspace Bootstrap

Create or align root `Cargo.toml`.

Audit phrase:

```text
Cargo.toml workspace
```

Workspace members:

```text
crates/sir
crates/sir_validate
crates/vm_core
crates/vm_runtime
crates/vm_eval
crates/vm_diag
crates/vm_host
crates/vm_tests
crates/vm_cli
```

Recommended dependency direction:

```text
sir
  -> no internal dependencies

sir_validate
  -> sir
  -> vm_diag if diagnostics are centralized

vm_core
  -> sir
  -> vm_diag

vm_runtime
  -> vm_core
  -> vm_diag

vm_eval
  -> vm_core
  -> vm_runtime
  -> vm_diag

vm_host
  -> vm_core
  -> vm_runtime

vm_tests
  -> all internal crates needed for tests

vm_cli
  -> vm_eval
  -> vm_host
  -> vm_diag
```

Forbidden dependency direction:

```text
sir depends on vm_core
vm_core depends on vm_eval
vm_runtime depends on vm_eval
vm_diag depends on vm_eval
```

If existing crate names differ, create a mapping table before editing.

---

## 7. Stage 0 · Workspace Bootstrap

### 7.1 Goal

Create the repository and documentation scaffolding needed for controlled implementation.

### 7.2 Required Actions

```text
1. Create root AGENT.md if absent.
2. Create root PROGRESS.md if absent.
3. Create root ISSUE.md if absent.
4. Create docs/frozen-specs/ directories or stable references.
5. Create docs/agent-plan/ and copy Agent plan documents.
6. Create agent/ subdirectories.
7. Create scripts/check, scripts/test, scripts/validate.
8. Create or align Cargo workspace.
9. Create crate directories.
10. Run formatting/check command if available.
11. Append PROGRESS.md summary.
12. Append ISSUE.md entries only for audit findings.
```

### 7.3 Required Gates

```text
G0
G1
G2
G3
G4
G7
```

### 7.4 Tests

At bootstrap stage:

```text
cargo metadata
cargo check --workspace
```

If tests cannot run because code is empty, record that in PROGRESS.md.

---

## 8. Stage 1 · Specification Ingestion and Trace Setup

### 8.1 Goal

Make every implementation task traceable to frozen references.

### 8.2 Required Actions

```text
1. Verify Phase 1 frozen documents are reachable.
2. Verify Phase 2 frozen documents are reachable.
3. Verify Phase 3 frozen documents are reachable.
4. Verify Agent plan documents are reachable.
5. Create docs/agent-plan/local-reference-map.md if needed.
6. Add Phase 1 aliases needed for parser/source work.
7. Add Phase 2 aliases needed for SIR work.
8. Confirm Phase 3 aliases from AGENT-MASTER-PLAN.md.
9. Populate or update TRACEABILITY-MATRIX.md if implementation scope expands.
10. Append PROGRESS.md summary.
```

### 8.3 Stop Conditions

Stop if:

```text
frozen specs missing
freeze declaration missing
subsystem spec missing for current work
trace row missing and cannot be added without semantic judgment
```

Record missing references in ISSUE.md.

---

## 9. Stage 2 · Crate Skeleton and ID Types

### 9.1 Goal

Create compileable crate skeletons and low-level ID/type scaffolding.

### 9.2 Required Actions

```text
1. Create Cargo.toml for each crate.
2. Create lib.rs or main.rs.
3. In sir crate, create modules for SIR-facing IDs and source references.
4. In vm_core crate, create modules:
   - id
   - error
   - value
   - control
   - profile
   - cache
5. In vm_diag crate, create diagnostic structs and source-span references.
6. Add compile-only tests for crate imports.
7. Run cargo check --workspace.
8. Append PROGRESS.md.
```

### 9.3 Specification References

Use:

```text
PHASE-2-IR-SPEC.md
PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md
PHASE-3-EIR-SCHEMA-CLOSURE.md
PHASE-3-RUNTIME-ERROR-REGISTRY.md
PHASE-3-CONTROL-STATE-MODEL.md
PHASE-3-TARGET-PROFILE-SCHEMAS.md
```

Do not copy schemas blindly.

Implement fields only after checking the subsystem document.

---

## 10. Stage 3 · Runtime Error and Diagnostics

### 10.1 Goal

Implement error taxonomy and diagnostic foundation before executable runtime logic.

### 10.2 Required Actions

```text
1. Implement RuntimeErrorCode according to frozen registry.
2. Implement language Error object structure.
3. Implement VmStructuralError separately.
4. Implement source-span attachment support.
5. Implement stack-trace placeholder type if required by diagnostics.
6. Add tests for known error codes.
7. Add negative tests for invalid raise categories.
8. Run cargo test for error/diag crates.
9. Append PROGRESS.md.
10. Append ISSUE.md for missing registry coverage.
```

### 10.3 Specification References

Use:

```text
PHASE-3-RUNTIME-ERROR-REGISTRY.md
PHASE-3-VALIDATION-MATRIX.md
```

---

## 11. Stage 4 · RuntimePlan Data Model and Validator

### 11.1 Goal

Implement RuntimePlan structs and validation entry points before EIR execution.

### 11.2 Required Actions

```text
1. Create vm_core::runtime_plan module.
2. Implement plan identity/version fields.
3. Implement module plan table.
4. Implement function plan table.
5. Implement slot layout structures.
6. Implement type/shape plan structures.
7. Implement call/access site tables.
8. Implement safepoint/deopt seed table placeholders.
9. Implement helper requirement table references.
10. Implement capability gate plan placeholders.
11. Implement RuntimePlan validation entry point.
12. Add malformed plan tests.
13. Add missing table/unknown ID tests.
14. Run cargo test.
15. Append PROGRESS.md.
```

### 11.3 Specification References

Use:

```text
PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md
PHASE-3-VALIDATION-MATRIX.md
PHASE-3-CACHE-COMPATIBILITY-MATRIX.md
PHASE-3-TARGET-PROFILE-SCHEMAS.md
```

---

## 12. Stage 5 · EIR Data Model and Validator

### 12.1 Goal

Implement closed EIR representation and reject malformed EIR before interpreter work.

### 12.2 Required Actions

```text
1. Create vm_core::eir module.
2. Implement EirModule.
3. Implement EirFunction.
4. Implement EirBlock.
5. Implement EirOp variants from frozen EIR schema.
6. Implement EirTerminator variants from frozen EIR schema.
7. Implement block graph validation.
8. Implement op argument validation.
9. Implement helper reference validation.
10. Implement source-map requirement checks for raising ops.
11. Implement may-collect/root metadata checks where required.
12. Add negative tests.
13. Run cargo test.
14. Append PROGRESS.md.
```

### 12.3 Specification References

Use:

```text
PHASE-3-EIR-SCHEMA-CLOSURE.md
PHASE-3-RUNTIME-HELPER-REGISTRY.md
PHASE-3-GC-METADATA-OWNERSHIP.md
PHASE-3-VALIDATION-MATRIX.md
```

---

## 13. Stage 6 · Helper Registry

### 13.1 Goal

Implement helper descriptors and lookup before interpreter uses helper calls.

### 13.2 Required Actions

```text
1. Create vm_runtime::helpers module.
2. Implement HelperId and HelperDescriptor.
3. Implement helper family classification.
4. Implement may_allocate/may_raise/may_unwind/is_safepoint metadata.
5. Implement required capability/effect metadata.
6. Implement helper registry construction.
7. Implement duplicate ID/name rejection.
8. Implement descriptor/implementation consistency check placeholder.
9. Add tests for missing and duplicate helpers.
10. Append PROGRESS.md.
```

### 13.3 Specification References

Use:

```text
PHASE-3-RUNTIME-HELPER-REGISTRY.md
PHASE-3-RUNTIME-ERROR-REGISTRY.md
PHASE-3-VALIDATION-MATRIX.md
PHASE-3-HOST-BOUNDARY-CONTRACT.md
```

---

## 14. Stage 7 · Value, Heap, Frame, and Control Core

### 14.1 Goal

Implement the runtime substrate required by interpreter execution.

### 14.2 Required Actions

```text
1. Implement Value representation under selected bootstrap target profile.
2. Implement ObjRef or internal handle model.
3. Implement heap object enum or equivalent internal representation.
4. Implement ValueKey restrictions.
5. Implement String semantic helpers by reference to spec.
6. Implement ReadOnlyView object shell.
7. Implement Frame.
8. Implement SlotArray.
9. Implement slot initialization states.
10. Implement ControlState.
11. Implement PendingControl storage.
12. Implement RegionStack shell.
13. Add unit tests for value/key/slot/control behavior.
14. Append PROGRESS.md.
```

### 14.3 Specification References

Use:

```text
PHASE-3-VALUE-KEY-STRING-SEMANTICS.md
PHASE-3-READONLY-VIEW-SEMANTICS.md
PHASE-3-CONTROL-STATE-MODEL.md
PHASE-3-GC-METADATA-OWNERSHIP.md
PHASE-3-TARGET-PROFILE-SCHEMAS.md
```

---

## 15. Stage 8 · Structured Unwinding

### 15.1 Goal

Implement cleanup/unwind behavior before general interpreter execution becomes broad.

### 15.2 Required Actions

```text
1. Implement cleanup stack representation.
2. Implement defer record representation.
3. Implement resource cleanup record representation.
4. Implement finally action representation.
5. Implement perform_unwind entry point.
6. Implement pending-control update path.
7. Implement suppressed error attachment path.
8. Implement tests for return/raise through cleanup.
9. Implement tests for finally override.
10. Append PROGRESS.md.
```

### 15.3 Specification References

Use:

```text
PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md
PHASE-3-CONTROL-STATE-MODEL.md
PHASE-3-RUNTIME-ERROR-REGISTRY.md
PHASE-3-GC-METADATA-OWNERSHIP.md
```

---

## 16. Stage 9 · Module Runtime

### 16.1 Goal

Implement module state transitions and import/export mechanics before full program execution.

### 16.2 Required Actions

```text
1. Implement ModuleId/ModuleInstance storage.
2. Implement ModuleState.
3. Implement state transition validator.
4. Implement export table representation.
5. Implement export sealing.
6. Implement import resolution shell.
7. Implement failed module behavior.
8. Implement circular import checks.
9. Implement resolver capability boundary placeholder.
10. Add module tests.
11. Append PROGRESS.md.
```

### 16.3 Specification References

Use:

```text
PHASE-3-MODULE-RUNTIME-CONTRACT.md
PHASE-3-HOST-BOUNDARY-CONTRACT.md
PHASE-3-RUNTIME-ERROR-REGISTRY.md
PHASE-3-VALIDATION-MATRIX.md
```

---

## 17. Stage 10 · Call and Host Boundary

### 17.1 Goal

Implement call execution path and host boundary skeleton before broad interpreter dispatch.

### 17.2 Required Actions

```text
1. Implement CallFrameInput structure.
2. Implement callable category enum.
3. Implement callability check.
4. Implement arity and named-argument binding.
5. Implement default evaluation hook.
6. Implement parameter/return contract hook.
7. Implement builtin call descriptor path.
8. Implement HostFunctionWrapper shell.
9. Implement HostRootRegistry shell.
10. Implement host error normalization shell.
11. Add call negative tests.
12. Add host capability negative tests.
13. Append PROGRESS.md.
```

### 17.3 Specification References

Use:

```text
PHASE-3-CALL-EXECUTION-PROTOCOL.md
PHASE-3-HOST-BOUNDARY-CONTRACT.md
PHASE-3-RUNTIME-ERROR-REGISTRY.md
PHASE-3-VALIDATION-MATRIX.md
```

---

## 18. Stage 11 · GC Metadata and Cache Compatibility Hooks

### 18.1 Goal

Implement required metadata hooks before interpreter and future JIT/GC integration.

### 18.2 Required Actions

```text
1. Implement RootMap structure.
2. Implement FrameMap structure.
3. Implement SafepointRecord structure.
4. Implement root location enum.
5. Implement pending-control root metadata.
6. Implement cache key structures.
7. Implement digest input collection.
8. Implement stale cache rejection shell.
9. Add metadata validation tests.
10. Add cache key mismatch tests.
11. Append PROGRESS.md.
```

### 18.3 Specification References

Use:

```text
PHASE-3-GC-METADATA-OWNERSHIP.md
PHASE-3-CACHE-COMPATIBILITY-MATRIX.md
PHASE-3-TARGET-PROFILE-SCHEMAS.md
PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md
PHASE-3-EIR-SCHEMA-CLOSURE.md
```

---

## 19. Stage 12 · Fast Interpreter Minimal Execution

### 19.1 Goal

Implement minimal EIR interpreter execution path after validation, values, frames, helpers, unwinding, module runtime, call, and metadata hooks exist.

### 19.2 Required Actions

```text
1. Create vm_eval::interpreter module.
2. Implement InterpreterState.
3. Implement frame push/pop.
4. Implement block dispatch.
5. Implement ConstantOp handling.
6. Implement LoadOp/StoreOp handling.
7. Implement basic UnaryOp/BinaryOp shell according to helper policy.
8. Implement CheckOp dispatch to helper/validator path.
9. Implement RuntimeHelperOp dispatch.
10. Implement Jump/Branch/Return/Raise terminators.
11. Implement LoopBackedge safepoint hook.
12. Implement source diagnostics path.
13. Add minimal execution tests.
14. Add negative branch condition tests.
15. Add helper error propagation tests.
16. Append PROGRESS.md.
```

### 19.3 Specification References

Use:

```text
PHASE-3-EIR-SCHEMA-CLOSURE.md
PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md
PHASE-3-CONTROL-STATE-MODEL.md
PHASE-3-RUNTIME-HELPER-REGISTRY.md
PHASE-3-GC-METADATA-OWNERSHIP.md
PHASE-3-VALIDATION-MATRIX.md
```

---

## 20. Stage 13 · Conformance and Regression

### 20.1 Goal

Build a test suite that proves implementation alignment with the frozen baseline.

### 20.2 Required Actions

```text
1. Create tests/conformance.
2. Create tests/negative.
3. Create tests/diagnostics.
4. Create tests/regression.
5. Create tests/fixtures.
6. Map each test file to trace rows.
7. Add runtime error tests.
8. Add RuntimePlan validation tests.
9. Add EIR validation tests.
10. Add helper registry tests.
11. Add module runtime tests.
12. Add call tests.
13. Add ReadOnlyView tests.
14. Add host boundary tests.
15. Add cache compatibility tests.
16. Add interpreter minimal execution tests.
17. Run full test suite.
18. Append PROGRESS.md.
19. Append ISSUE.md for failures.
```

### 20.3 Specification References

Use:

```text
PHASE-3-VALIDATION-MATRIX.md
TRACEABILITY-MATRIX.md
WORK-PACKAGE-INDEX.md
```

---

## 21. Stage 14 · Integration Review

### 21.1 Goal

Review implementation as a coherent minimal VM candidate.

### 21.2 Required Actions

```text
1. Run G6 Integration Gate.
2. Run all tests.
3. Scan for public bytecode exposure.
4. Scan for RuntimePlan/EIR public API leakage.
5. Scan for unregistered helper usage.
6. Scan for unregistered error usage.
7. Scan for capability bypass.
8. Scan for host boundary bypass.
9. Scan for missing source diagnostics.
10. Scan for missing PROGRESS.md entries.
11. Scan for unrecorded audit findings.
12. Append ISSUE.md entries for findings.
13. Produce final handoff.
```

### 21.3 Specification References

Use:

```text
PHASE-3-FREEZE.md
PHASE-3-VALIDATION-MATRIX.md
PHASE-3-CACHE-COMPATIBILITY-MATRIX.md
PHASE-3-HOST-BOUNDARY-CONTRACT.md
PHASE-3-GC-METADATA-OWNERSHIP.md
```

---

## 22. Agent Dispatch Guidance by Stage

Default:

```text
main-only
```

Allowed maximums:

```text
Stage 0: main-only
Stage 1: main+1
Stage 2: main+1
Stage 3: main+1
Stage 4: main+2
Stage 5: main+2
Stage 6: main+2
Stage 7: main+1
Stage 8: main+2
Stage 9: main+2
Stage 10: main+2
Stage 11: main+2
Stage 12: main+3
Stage 13: main+3
Stage 14: main+3
```

`main+4` is reserved for exceptional audit review only.

---

## 23. Required Per-Stage Output

Every stage must produce:

```text
changed files
tests run or skipped with reason
PROGRESS.md entry
ISSUE.md entries if audit findings exist
handoff
gate status
```

No stage is complete without a handoff.

---

## 24. Completion Definition

The coding plan is complete when:

```text
workspace exists
frozen specs are reachable
Agent plan docs are installed
Rust workspace compiles
schema models compile
validators reject malformed inputs
runtime substrate exists
minimal interpreter executes validated EIR subset
conformance/negative/diagnostic tests exist
integration gate passes
PROGRESS.md and ISSUE.md have append-only history
```
