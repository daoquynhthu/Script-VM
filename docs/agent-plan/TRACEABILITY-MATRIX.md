# Traceability Matrix

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications and `AGENT-MASTER-PLAN.md`  
Created: 2026-06-29 10:36:55

---

## 0. Purpose

This document maps implementation work to frozen specification references, validation gates, and test obligations.

It does not copy normative text.

It does not define new semantics.

Every implementation work package must have at least one trace row before moving beyond G1.

---

## 1. Traceability Row Format

```text
Trace ID:
Implementation Item:
Work Package:
Frozen Spec References:
Required Implementation Behavior:
Validation Gates:
Test Requirement:
Status:
Notes:
```

Status values:

```text
DRAFT
MAPPED
GAP
VALIDATED
IMPLEMENTED
TESTED
DEFERRED
```

---

## 2. Reference Alias Rule

Trace rows should use aliases from:

```text
AGENT-MASTER-PLAN.md
```

If a Phase 1 or Phase 2 document is required, add a concrete document name until a stable alias is introduced.

---

## 3. Core Trace Rows

### TR-000 · Freeze Boundary

```text
Trace ID: TR-000
Implementation Item: Preserve frozen baseline boundary
Work Package: WP-00, WP-01, WP-19
Frozen Spec References:
  - SPEC-P3-FREEZE
Required Implementation Behavior:
  Implementation planning and implementation must not modify frozen normative semantics.
Validation Gates:
  - G0
  - G1
  - G6
  - G7
Test Requirement:
  Planning review must reject tasks that reopen specification design.
Status: MAPPED
Notes:
  This row controls all implementation work.
```

### TR-001 · Reference Alias Resolution

```text
Trace ID: TR-001
Implementation Item: Resolve frozen spec references
Work Package: WP-01, WP-02
Frozen Spec References:
  - SPEC-P3-FREEZE
  - SPEC-P3-VM
  - SPEC-P3-MIN
Required Implementation Behavior:
  Every work package must cite concrete frozen documents or valid aliases.
Validation Gates:
  - G1
Test Requirement:
  Alias resolution check.
Status: MAPPED
```

### TR-002 · Runtime Error Registry

```text
Trace ID: TR-002
Implementation Item: RuntimeErrorCode and ErrorObj model
Work Package: WP-04
Frozen Spec References:
  - SPEC-P3-ERRORS
  - SPEC-P3-VALID
Required Implementation Behavior:
  Implement language errors and VM structural errors as distinct categories.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Known error codes, non-Error raise rejection, structural error boundary tests.
Status: MAPPED
```

### TR-003 · RuntimePlan Schema

```text
Trace ID: TR-003
Implementation Item: RuntimePlan data model and validator
Work Package: WP-05
Frozen Spec References:
  - SPEC-P3-RTP
  - SPEC-P3-VALID
  - SPEC-P3-CACHE
  - SPEC-P3-PROFILE
Required Implementation Behavior:
  Implement RuntimePlan schema, ID resolution, target profile compatibility, and cache key participation.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Missing table rejection, unknown ID rejection, profile mismatch rejection.
Status: MAPPED
```

### TR-004 · EIR Schema

```text
Trace ID: TR-004
Implementation Item: EIR data model and validator
Work Package: WP-06
Frozen Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-VALID
  - SPEC-P3-HELPERS
  - SPEC-P3-GC-META
Required Implementation Behavior:
  Implement closed EIR op/terminator schema and reject unknown or malformed EIR.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Unknown op rejection, missing terminator rejection, invalid helper reference rejection.
Status: MAPPED
```

### TR-005 · Runtime Helper Registry

```text
Trace ID: TR-005
Implementation Item: Helper descriptors and helper table
Work Package: WP-07
Frozen Spec References:
  - SPEC-P3-HELPERS
  - SPEC-P3-ERRORS
  - SPEC-P3-VALID
  - SPEC-P3-CACHE
Required Implementation Behavior:
  Implement helper descriptors, lookup, validation, digest participation, and call boundary.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Duplicate helper rejection, missing helper rejection, may-raise/may-collect policy tests.
Status: MAPPED
```

### TR-006 · Value and ValueKey Semantics

```text
Trace ID: TR-006
Implementation Item: Value, ValueKey, string constraints
Work Package: WP-08
Frozen Spec References:
  - SPEC-P3-VALUES
  - SPEC-P3-PROFILE
  - SPEC-P3-ERRORS
Required Implementation Behavior:
  Implement hashability restrictions, map key rules, string length/slice behavior, and non-hashable rejection.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  NaN key rejection, mutable aggregate key rejection, string slice bounds tests.
Status: MAPPED
```

### TR-007 · ReadOnlyView Semantics

```text
Trace ID: TR-007
Implementation Item: ReadOnlyView runtime behavior
Work Package: WP-13
Frozen Spec References:
  - SPEC-P3-READONLY
  - SPEC-P3-VALUES
  - SPEC-P3-ERRORS
Required Implementation Behavior:
  Implement shallow readonly view, mutation rejection, read delegation, target rooting.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Field/index mutation through view rejected, original mutation reflected in view, non-hashable target remains non-hashable.
Status: MAPPED
```

### TR-008 · Frame / Slot / Control State

```text
Trace ID: TR-008
Implementation Item: Frame, SlotArray, PendingControl, ControlState
Work Package: WP-09
Frozen Spec References:
  - SPEC-P3-CONTROL
  - SPEC-P3-GC-META
  - SPEC-P3-RTP
Required Implementation Behavior:
  Implement canonical control-state layers, slot states, pending control rooting, and frame metadata links.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Uninitialized slot read rejection, pending control root visibility, invalid slot ID rejection.
Status: MAPPED
```

### TR-009 · Structured Unwinding

```text
Trace ID: TR-009
Implementation Item: Defer/resource/finally/catch unwinding
Work Package: WP-10
Frozen Spec References:
  - SPEC-P3-UNWIND
  - SPEC-P3-CONTROL
  - SPEC-P3-ERRORS
  - SPEC-P3-GC-META
Required Implementation Behavior:
  Implement canonical cleanup order, pending control updates, finally override, suppressed error handling, and deopt-visible cleanup state.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Return through finally, raise through defer, resource close raise, break/continue cleanup crossing.
Status: MAPPED
```

### TR-010 · Module Runtime

```text
Trace ID: TR-010
Implementation Item: Module initialization, import, export, and circular imports
Work Package: WP-11
Frozen Spec References:
  - SPEC-P3-MODULE
  - SPEC-P3-HOST
  - SPEC-P3-ERRORS
  - SPEC-P3-VALID
Required Implementation Behavior:
  Implement ModuleState transitions, source-order initialization, export sealing, circular export checks, resolver capability boundary.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Duplicate export rejection, uninitialized circular export rejection, failed module import behavior.
Status: MAPPED
```

### TR-011 · Call Execution Protocol

```text
Trace ID: TR-011
Implementation Item: User/builtin/constructor/host call execution
Work Package: WP-12
Frozen Spec References:
  - SPEC-P3-CALL
  - SPEC-P3-CONTROL
  - SPEC-P3-ERRORS
  - SPEC-P3-HOST
Required Implementation Behavior:
  Implement call evaluation order, argument binding, call-time defaults, parameter/return contracts, host call boundary.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Wrong arity, duplicate named argument, default raises, return contract failure, host capability failure.
Status: MAPPED
```

### TR-012 · Host Boundary

```text
Trace ID: TR-012
Implementation Item: HostFunctionWrapper, HostObjectWrapper, HostRootRegistry
Work Package: WP-14
Frozen Spec References:
  - SPEC-P3-HOST
  - SPEC-P3-CALL
  - SPEC-P3-ERRORS
  - SPEC-P3-PROFILE
  - SPEC-P3-CACHE
Required Implementation Behavior:
  Implement VM-controlled host call boundary, capability gating, root registration, error normalization, FFI deferred constraints.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Host call without capability rejected, host error normalized, retained VM value without root rejected.
Status: MAPPED
```

### TR-013 · GC Metadata

```text
Trace ID: TR-013
Implementation Item: RootMap, FrameMap, SafepointRecord
Work Package: WP-15
Frozen Spec References:
  - SPEC-P3-GC-META
  - SPEC-P3-PROFILE
  - SPEC-P3-VALID
Required Implementation Behavior:
  Implement canonical metadata ownership, root visibility, safepoint metadata, frame mapping, future moving-GC compatibility hooks.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Safepoint without RootMap rejection, RootMap unknown slot rejection, moving GC profile root policy checks.
Status: MAPPED
```

### TR-014 · Cache Compatibility

```text
Trace ID: TR-014
Implementation Item: RuntimePlan/EIR/helper/profile cache compatibility
Work Package: WP-16
Frozen Spec References:
  - SPEC-P3-CACHE
  - SPEC-P3-PROFILE
  - SPEC-P3-RTP
  - SPEC-P3-EIR
  - SPEC-P3-HELPERS
Required Implementation Behavior:
  Implement internal cache key components, stale cache rejection, digest participation, and non-public cache boundary.
Validation Gates:
  - G3
  - G5
  - G6
Test Requirement:
  Mismatched digest rejection, profile mismatch rejection, helper registry mismatch rejection, public-bytecode cache claim rejection.
Status: MAPPED
```

### TR-015 · Fast Interpreter Core

```text
Trace ID: TR-015
Implementation Item: EIR fast interpreter execution
Work Package: WP-17
Frozen Spec References:
  - SPEC-P3-EIR
  - SPEC-P3-RTP
  - SPEC-P3-CONTROL
  - SPEC-P3-HELPERS
  - SPEC-P3-GC-META
Required Implementation Behavior:
  Implement EIR execution through internal interpreter state, op handlers, terminator handlers, helper bridge, safepoint interaction, and source diagnostics.
Validation Gates:
  - G3
  - G4
  - G5
  - G6
Test Requirement:
  Literal execution, slot load/store, branch Bool check, helper call, raise/unwind path, module path.
Status: MAPPED
```

### TR-016 · Validation Matrix

```text
Trace ID: TR-016
Implementation Item: Unified validation pipeline
Work Package: WP-05, WP-06, WP-07, WP-15, WP-18
Frozen Spec References:
  - SPEC-P3-VALID
Required Implementation Behavior:
  Implement relevant P3 validation passes for each subsystem.
Validation Gates:
  - G5
  - G6
Test Requirement:
  Validation failure tests per subsystem.
Status: MAPPED
```

### TR-017 · Conformance Matrix

```text
Trace ID: TR-017
Implementation Item: Positive and negative conformance coverage
Work Package: WP-18
Frozen Spec References:
  - SPEC-P3-VALID
  - SPEC-P3-ERRORS
  - SPEC-P3-VALUES
  - SPEC-P3-UNWIND
  - SPEC-P3-MODULE
  - SPEC-P3-CALL
  - SPEC-P3-READONLY
  - SPEC-P3-HOST
Required Implementation Behavior:
  Map implemented behavior to conformance and regression tests.
Validation Gates:
  - G5
  - G6
  - G7
Test Requirement:
  Positive, negative, diagnostic, and regression matrix.
Status: COMPLETE (bootstrap Phase 3 — see tests/MATRIX.md; not full language product suite)
```

### TR-018 · Integration and Regression

```text
Trace ID: TR-018
Implementation Item: Cross-subsystem integration gates
Work Package: WP-19
Frozen Spec References:
  - SPEC-P3-FREEZE
  - SPEC-P3-VALID
  - SPEC-P3-CACHE
  - SPEC-P3-GC-META
  - SPEC-P3-HOST
Required Implementation Behavior:
  Prevent integration regressions, public ABI leaks, helper/error/cache unregistered behavior, host boundary bypass.
Validation Gates:
  - G6
  - G7
Test Requirement:
  Full regression run, integration gate checks, forbidden regression checks.
Status: COMPLETE (bootstrap Phase 3 — IG suite + G6 scan + CI; see agent/gate-records/WP-19-*)
```

---

## 4. Gap Rows

The following trace rows are intentionally marked for future expansion.

### TR-GAP-001 · Phase 1 Language References

```text
Trace ID: TR-GAP-001
Implementation Item: Phase 1 source language semantics
Work Package: WP-02, WP-18
Frozen Spec References:
  - PHASE-1-LANGUAGE-SPEC.md
  - PHASE-1-LANGUAGE-DESIGN.md
Required Implementation Behavior:
  Add detailed trace rows when parser/source-level implementation begins.
Validation Gates:
  - G1
  - G3
Status: GAP
```

### TR-GAP-002 · Phase 2 SIR References

```text
Trace ID: TR-GAP-002
Implementation Item: Phase 2 SIR schema and validation references
Work Package: WP-02, WP-03, WP-18
Frozen Spec References:
  - PHASE-2-IR-SPEC.md
  - PHASE-2-FREEZE.md
Required Implementation Behavior:
  Add detailed trace rows when SIR ingestion/lowering implementation begins.
Validation Gates:
  - G1
  - G3
Status: GAP
```

---

## 5. Traceability Completion Criteria

This matrix is sufficient for implementation planning when:

```text
all work packages have at least one trace row
all trace rows cite frozen references
all trace rows identify gates
all trace rows identify test obligations or mark not applicable
gaps are explicit
```

This initial version satisfies work-package-level traceability.

Detailed implementation-level trace rows should be added as code tasks are created.

---

## 6. Concrete Coding-Plan Trace Rows

Added: 2026-06-29 11:00:40

### TR-019 · Workspace Bootstrap

```text
Trace ID: TR-019
Implementation Item: Repository workspace and root control files
Work Package: WP-00
Frozen Spec References:
  - SPEC-P3-FREEZE
Required Implementation Behavior:
  Create or align repository directories and root control files without modifying frozen specifications.
Validation Gates:
  - G0
  - G1
  - G2
  - G3
  - G4
  - G7
Test Requirement:
  cargo metadata and cargo check --workspace if Rust workspace has been created.
Status: MAPPED
```

### TR-020 · Concrete Coding Sequence

```text
Trace ID: TR-020
Implementation Item: Stage-by-stage implementation order
Work Package: WP-00 through WP-19
Frozen Spec References:
  - SPEC-P3-FREEZE
Required Implementation Behavior:
  Follow IMPLEMENTATION-CODING-PLAN.md for execution sequence while citing subsystem specs for semantics.
Validation Gates:
  - G0
  - G1
  - G2
  - G3
  - G4
  - G7
Test Requirement:
  Each stage records required commands and handoff.
Status: MAPPED
```

