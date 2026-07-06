# Phase 3 · Unified Control State Model

Document class: Normative specification  
Normative status: This document defines the canonical control-state model for Phase 3 VM execution, EIR, helpers, unwinding, interpreter, and JIT.

Created: 2026-06-29 09:21:35

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
R6: Unify control-state model.
```

It resolves blocker:

```text
B-05: PendingControl / VmControl / TerminatorResult are not fully unified.
```

---

## 1. Canonical Control Layers

Phase 3 has these control layers:

```text
ExpressionResult
OpResult
TerminatorResult
PendingControl
VmControl
HelperReturn
FrameExit
```

All layers MUST map to the canonical `ControlState`.

---

## 2. ControlState

```text
ControlState =
  | Normal(Value?)
  | Return(Value?)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
  | Halt
  | Deopt(DeoptId)
  | VmError(VmError)
```

### 2.1 Normal

`Normal` means ordinary execution.

For expression context, `Normal` usually carries a value.

For statement context, `Normal` may carry no value.

### 2.2 Return

`Return` carries optional return value.

If cleanup regions are active, Return MUST first become PendingControl and enter unwinding.

### 2.3 Break / Continue

Break and Continue MUST carry a `ControlRegionId`.

Bare break/continue states are forbidden in canonical Phase 3 control state.

### 2.4 Raise

Raise carries `ErrorHandle`.

Non-Error raise is converted to:

```text
TypeError
```

### 2.5 Halt

Halt is VM-internal.

Source programs MUST NOT observe arbitrary Halt.

### 2.6 Deopt

Deopt is execution-tier transition, not language control.

Deopt MUST reconstruct frame/slot/region/pending-control state.

### 2.7 VmError

VmError is structural failure, not language Error.

---

## 3. ExpressionResult

```text
ExpressionResult =
  | Value(Value)
  | Raise(ErrorHandle)
  | VmError(VmError)
```

Expression evaluation MUST NOT produce Return, Break, Continue, Halt, or Deopt directly except through internal lowering transition.

---

## 4. OpResult

```text
OpResult =
  | Continue
  | Raise(ErrorHandle)
  | Deopt(DeoptId)
  | VmError(VmError)
```

EIR ops write results to slots.

They do not return source values directly except through slot writes.

---

## 5. TerminatorResult

```text
TerminatorResult =
  | NextBlock(EirBlockId)
  | Return(Value?)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
  | Unwind(PendingControl)
  | Deopt(DeoptId)
  | Halt
  | VmError(VmError)
```

A terminator that exits a region with cleanup MUST produce or update PendingControl and enter Unwind.

---

## 6. PendingControl

```text
PendingControl =
  | PendingReturn(Value?)
  | PendingBreak(ControlRegionId)
  | PendingContinue(ControlRegionId)
  | PendingRaise(ErrorHandle)
```

PendingControl is stored in a hidden runtime slot or frame field.

PendingControl MUST be root-visible if it contains heap references.

---

## 7. VmControl

```text
VmControl =
  | Normal(Value?)
  | Return(Value?)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
```

VmControl is the language-level control result used across helpers and function/frame execution.

VmControl MUST NOT carry Deopt or VmError.

---

## 8. HelperReturn

```text
HelperReturn =
  | Value(Value)
  | Control(VmControl)
  | Unit
  | Deopt(DeoptId)
  | Error(VmError)
```

Language errors from helpers MUST be returned as:

```text
Control(Raise(ErrorHandle))
```

Structural failures MUST be returned as:

```text
Error(VmError)
```

---

## 9. FrameExit

```text
FrameExit =
  | Returned(Value?)
  | Raised(ErrorHandle)
  | PropagateBreak(ControlRegionId)
  | PropagateContinue(ControlRegionId)
  | VmError(VmError)
```

A source-level Break/Continue escaping its valid region is a validation failure or LanguageError depending on phase boundary.

---

## 10. Mapping Rules

### 10.1 Return Mapping

```text
TerminatorResult::Return
  -> if cleanup active: PendingReturn -> Unwind
  -> else FrameExit::Returned
```

### 10.2 Raise Mapping

```text
Raise(ErrorHandle)
  -> PendingRaise if cleanup active
  -> FrameExit::Raised if no handler/cleanup remains
```

### 10.3 Break/Continue Mapping

```text
Break(target)
Continue(target)
  -> PendingBreak/PendingContinue if cleanup active or target outside current region
  -> direct branch only if no cleanup is crossed
```

### 10.4 Helper Mapping

```text
HelperReturn::Value(v) -> OpResult::Continue after dest write
HelperReturn::Unit -> OpResult::Continue
HelperReturn::Control(Normal(v)) -> continue or dest write
HelperReturn::Control(Return/Break/Continue/Raise) -> Terminator/control path
HelperReturn::Deopt(id) -> Deopt
HelperReturn::Error(e) -> VmError
```

---

## 11. Deopt and Control

Deopt MUST preserve:

```text
current EIR location
frame slots
region stack
pending control
source span
live roots
```

Deopt is not catchable by source code.

---

## 12. JIT Requirements

Compiled code MUST use the same control-state model.

Compiled return/raise/break/continue MUST NOT skip active cleanup.

Compiled code MUST expose pending control and region stack state at deopt/unwind safepoints.

---

## 13. Validation

Control-state validation MUST reject:

```text
bare Break without ControlRegionId
bare Continue without ControlRegionId
Return outside function
Break outside loop
Continue outside loop
Raise with non-Error value unless converted to TypeError
Deopt without DeoptId
PendingControl containing heap value without root visibility
compiled control transfer that skips cleanup
```

---

## 14. Audit Tracking

This document completes:

```text
R6
```

It resolves:

```text
B-05
```

It partially supports:

```text
B-06
M-10
M-14
```
