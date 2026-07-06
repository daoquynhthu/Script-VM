# Phase 3 · GC Metadata Ownership

Document class: Normative specification  
Normative status: This document defines canonical ownership and projection rules for RootMap, FrameMap, SafepointRecord, StackMap, and DeoptPoint metadata.

Created: 2026-06-29 09:26:37

---

## Normative Interpretation

This document is interpreted under `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md`.

Unmarked planning-style words such as `bootstrap`, `recommended`, `staged`, `first implementation`, `later`, `plan`, or `milestone` do not create implementation-plan status inside this normative document. They are interpreted as one of:

```text
MUST / MUST NOT / SHOULD / MAY
BOOTSTRAP allowance
RECOMMENDED implementation option
DEFERRED design area
NON-NORMATIVE NOTE
```

according to their local context and the normative keyword policy.

If this document conflicts with a later canonical repair document, the later canonical repair document owns the repaired term or schema.



## 0. Purpose

This document repairs audit item:

```text
R10: Canonicalize RootMap/FrameMap/SafepointRecord ownership.
```

It addresses major finding:

```text
M-03: RootMap / FrameMap / SafepointRecord definitions are duplicated across documents.
```

This document is the canonical cross-document ownership map for GC/JIT/interpreter metadata.

---

## 1. Canonical Ownership Rule

Each metadata family has one canonical owner.

Other documents MAY define projections, uses, or lowering references, but MUST NOT redefine the canonical schema incompatibly.

| Metadata | Canonical owner | Secondary users |
|---|---|---|
| `RootMap` | GC Metadata Ownership + GC Safepoint Root Model | interpreter, JIT, helpers |
| `FrameMap` | GC Metadata Ownership + GC Safepoint Root Model | interpreter, diagnostics, deopt |
| `SafepointRecord` | GC Metadata Ownership + GC Safepoint Root Model | interpreter, JIT, helpers |
| `StackMap` | Baseline JIT Backend Interface under GC metadata constraints | GC, deopt, compiled code |
| `DeoptPoint` | RuntimePlan Schema Closure + JIT projection | interpreter, JIT, diagnostics |
| `RegionStackState` | Structured Unwinding Algorithm + Control State Model | GC, JIT, deopt |
| `PendingControlState` | Control State Model | GC, unwind, JIT, deopt |

---

## 2. RootMap

### 2.1 Canonical Schema

```text
RootMap {
  root_map_id: RootMapId
  owner: RootMapOwner
  safepoint_id?: SafepointId
  frame_map_id?: FrameMapId
  roots: List<RootLocation>
  source_span?: SourceSpanId
}
```

### 2.2 RootMapOwner

```text
RootMapOwner =
  | InterpreterFrame
  | EirFunction
  | RuntimeHelper
  | JitCompiledFunction
  | HostBoundary
  | ModuleInitialization
```

### 2.3 RootLocation

```text
RootLocation =
  | SlotRootLocation
  | CellRootLocation
  | ModuleRootLocation
  | ConstantRootLocation
  | RegionRootLocation
  | PendingControlRootLocation
  | ErrorRootLocation
  | HelperArgRootLocation
  | HostRootLocation
  | JitRootLocation
```

### 2.4 RootMap Requirements

A RootMap MUST be available at every safepoint where GC may run.

A RootMap MUST be updateable if moving GC is enabled.

A RootMap MUST NOT rely on conservative scanning under moving GC profile.

---

## 3. FrameMap

### 3.1 Canonical Schema

```text
FrameMap {
  frame_map_id: FrameMapId
  owner_function: EirFunctionId
  source_function?: FunctionId
  module_id: ModuleId
  slot_layout: SlotLayoutId
  visible_bindings: List<VisibleBinding>
  region_state_schema: RegionStateSchema
  source_span?: SourceSpanId
}
```

### 3.2 VisibleBinding

```text
VisibleBinding {
  binding_id: BindingId
  slot_id: SlotId
  visibility: BindingVisibility
  value_kind_hint?: RuntimeValueKind
  source_span?: SourceSpanId
}
```

### 3.3 FrameMap Uses

FrameMap supports:

```text
source stack trace
debug inspection
deopt reconstruction
GC root enumeration
error diagnostics
interpreter/JIT bridge
```

FrameMap is internal metadata and not public ABI.

---

## 4. SafepointRecord

### 4.1 Canonical Schema

```text
SafepointRecord {
  safepoint_id: SafepointId
  kind: SafepointKind
  owner: SafepointOwner
  location: SafepointLocation
  root_map: RootMapId
  frame_map?: FrameMapId
  deopt_id?: DeoptId
  source_span?: SourceSpanId
}
```

### 4.2 SafepointKind

```text
SafepointKind =
  | FunctionCall
  | LoopBackedge
  | Allocation
  | HostCall
  | HelperCall
  | RaiseBoundary
  | ImportBoundary
  | DeoptExit
  | DebugPoll
```

### 4.3 SafepointOwner

```text
SafepointOwner =
  | Interpreter
  | EirFunction
  | RuntimeHelper
  | JitCompiledFunction
  | HostCall
```

### 4.4 Safepoint Requirements

A SafepointRecord MUST link to RootMap when GC can run.

A SafepointRecord MUST link to FrameMap when deopt, stack trace, or debugging may inspect the frame.

---

## 5. StackMap Projection

StackMap is JIT-specific projection of RootMap and FrameMap.

```text
StackMap {
  stack_map_id: StackMapId
  compiled_function_id: CompiledFunctionId
  code_offset: CodeOffset
  live_value_locations: List<ValueLocation>
  frame_state: FrameStateRef
  source_span?: SourceSpanId
}
```

StackMap MUST be sufficient to reconstruct/update live heap references at compiled safepoints.

StackMap MUST NOT replace RootMap as canonical cross-tier root metadata.

---

## 6. DeoptPoint Projection

DeoptPoint canonical semantic seed belongs to RuntimePlan.

JIT deopt records project that seed to code offsets.

```text
JitDeoptRecord {
  deopt_id: DeoptId
  code_offset: CodeOffset
  source_eir_location: EirLocation
  frame_map: FrameMapId
  root_map?: RootMapId
  region_stack_state: RegionStackState
  pending_control_state?: PendingControlState
  resume_target: DeoptResumeTarget
}
```

Deopt metadata MUST preserve enough state to resume in EIR interpreter or unwind helper.

---

## 7. Ownership Constraints

Secondary documents MUST follow these rules:

```text
RuntimePlan documents may reference FrameMapId/RootMapId/SafepointId but MUST NOT redefine their schema.
EIR documents may attach safepoint/root/frame references but MUST NOT redefine canonical GC metadata.
JIT documents may define StackMap projection but MUST preserve RootMap/FrameMap semantics.
Interpreter documents may define runtime storage but MUST preserve FrameMap/RootMap visibility.
Helper documents may define helper safepoint behavior but MUST reference SafepointRecord.
```

---

## 8. Validation

GC metadata validation MUST reject:

```text
safepoint without RootMap when GC may run
JIT safepoint without StackMap
FrameMap referencing unknown SlotLayout
RootMap referencing unknown SlotId
RootMap non-updateable under moving GC profile
DeoptPoint without FrameMap
helper may_collect without SafepointRecord
compiled helper call without stack/root metadata
duplicate incompatible schema definitions
```

---

## 9. Audit Tracking

This document completes:

```text
R10
```

It addresses:

```text
M-03
```

It supports:

```text
R13
R14
```
