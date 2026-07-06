# Phase 3 · Structured Unwinding Algorithm

Document class: Normative specification  
Normative status: This document defines the canonical structured unwinding algorithm for Phase 3 VM execution.

Created: 2026-06-29 09:24:10

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
R7: Canonicalize structured unwinding algorithm.
```

It resolves blocker:

```text
B-06: Structured unwinding algorithm is not yet fully executable-spec closed.
```

This document is the canonical source for cleanup ordering, PendingControl updates, finally override, suppressed errors, and deopt-visible cleanup state.

---

## 1. Scope

This algorithm applies to exits caused by:

```text
Normal block exit
Return
Break
Continue
Raise
defer callable raise
resource close raise
finally non-normal completion
host/resource cleanup failure
```

It covers cleanup registered by:

```text
defer
use/resource
finally
try/catch/finally
loop regions crossed by break/continue
function regions crossed by return
```

---

## 2. Canonical Data Structures

### 2.1 PendingControl

```text
PendingControl =
  | PendingReturn(Value?)
  | PendingBreak(ControlRegionId)
  | PendingContinue(ControlRegionId)
  | PendingRaise(ErrorHandle)
```

PendingControl MUST be root-visible if it contains heap references.

### 2.2 RuntimeRegionFrame

```text
RuntimeRegionFrame {
  region_id: ControlRegionId
  region_kind: ControlRegionKind
  cleanup_state: CleanupState
  loop_target?: LoopTarget
  finally_entry?: EirBlockId
  catch_entries?: List<CatchEntry>
  source_span?: SourceSpanId
}
```

### 2.3 CleanupState

```text
CleanupState {
  defer_stack: List<DeferredCallable>
  resource_stack: List<ResourceCleanup>
  finally_state: FinallyState
  cleanup_progress: CleanupProgress
}
```

### 2.4 CleanupProgress

```text
CleanupProgress =
  | NotStarted
  | RunningDefers
  | RunningResources
  | RunningFinally
  | Complete
```

CleanupProgress MUST be preserved across deopt, helper calls, reentrant host calls, and safepoints.

---

## 3. Cleanup Ordering

When leaving a region, cleanup order is:

```text
1. defer callables in LIFO order
2. resources in reverse acquisition order
3. finally block if present
```

This order is canonical for Phase 3.

A future revision may change it only by reopening structured control semantics.

---

## 4. Entry to Unwinding

Unwinding starts when a control transfer crosses a region that owns cleanup.

```text
Return(value)     -> PendingReturn(value)
Break(region)     -> PendingBreak(region)
Continue(region)  -> PendingContinue(region)
Raise(error)      -> PendingRaise(error)
```

Normal block fall-through does not create PendingControl unless the block exits a cleanup-owning region.

---

## 5. Main Algorithm

Canonical pseudocode:

```text
perform_unwind(frame, pending_control):
  while frame.region_stack is not empty:
    region = frame.region_stack.top()

    if pending_control target is inside region and no cleanup remains:
      return resolve_control_inside_region(pending_control)

    if region.cleanup_state.cleanup_progress == NotStarted:
      region.cleanup_state.cleanup_progress = RunningDefers

    if cleanup_progress == RunningDefers:
      while region.defer_stack not empty:
        defer = pop last defer
        result = call defer()
        if result is non-normal:
          pending_control = combine_cleanup_result(pending_control, result)
      cleanup_progress = RunningResources

    if cleanup_progress == RunningResources:
      while region.resource_stack not empty:
        resource = pop last resource
        result = close resource exactly once
        if result is non-normal:
          pending_control = combine_cleanup_result(pending_control, result)
      cleanup_progress = RunningFinally

    if cleanup_progress == RunningFinally:
      if region has finally and finally not yet run:
        result = run finally block
        if result is non-normal:
          pending_control = finally_override(pending_control, result)
      cleanup_progress = Complete

    if cleanup_progress == Complete:
      pop region
      if pending_control target resolved by popped region:
        return resolve_control_after_region(pending_control)

  return propagate_from_frame(pending_control)
```

---

## 6. Defer Semantics

### 6.1 Registration

A defer statement MUST:

```text
evaluate callable immediately
check callable
check zero-argument call compatibility
register callable in current block/function cleanup region
```

Module top-level defer remains rejected unless later normatively amended.

### 6.2 Execution

Defer callables execute in LIFO order.

A defer callable may:

```text
complete normally
raise
return/break/continue only if callable semantics permit, then it exits that callable frame first
```

The defer result visible to the unwinding region is either Normal or Raise unless the VM permits non-local control from defer callables. Phase 3 minimal VM SHOULD normalize defer callable non-return control to LanguageError or VmStructuralError according to call boundary.

### 6.3 Defer Raise

If defer raises during pending Raise:

```text
primary = existing pending raise
suppressed += defer raise
pending remains primary raise
```

If defer raises during pending Return/Break/Continue/Normal:

```text
pending becomes PendingRaise(defer_error)
```

---

## 7. Resource Semantics

### 7.1 Registration

A resource is registered only after acquisition succeeds.

If acquisition fails, no close is registered.

### 7.2 Close Exactly Once

A registered resource MUST be closed exactly once by structured unwinding unless ownership is explicitly transferred by a future normative mechanism.

Resource states:

```text
Open
Closing
Closed
Failed
```

### 7.3 Close Ordering

Resources close in reverse acquisition order.

### 7.4 Close Raise

If resource close raises during pending Raise:

```text
primary = existing pending raise
suppressed += close raise
pending remains primary raise
```

If resource close raises during pending Return/Break/Continue/Normal:

```text
pending becomes PendingRaise(close_error)
```

---

## 8. Finally Semantics

### 8.1 Execution

A finally block MUST execute when control exits its try/finally region, regardless of whether the exit is:

```text
Normal
Return
Break
Continue
Raise
```

### 8.2 Finally Override

If finally completes normally:

```text
pending_control remains unchanged
```

If finally produces non-normal control:

```text
pending_control = finally_result
```

This is the canonical finally override rule.

Examples:

```text
pending Return + finally Raise -> Raise
pending Raise + finally Return -> Return
pending Break + finally Continue -> Continue
```

### 8.3 Finally Error Suppression

A finally Raise overrides a previous pending Raise.

The previous pending Raise SHOULD be attached as suppressed/context if ErrorObj supports it.

If suppressed/context support is unavailable in bootstrap, diagnostic metadata MUST preserve the overwritten error.

---

## 9. Catch Semantics

A catch region handles PendingRaise only.

When PendingRaise reaches a try/catch region:

```text
for catch in source order:
  bind catch error
  if guard absent or guard evaluates Bool true:
    clear PendingRaise
    execute catch body
    result becomes current control
```

Catch guard condition MUST be Bool.

If guard raises, guard raise becomes current PendingRaise.

If no catch matches, PendingRaise continues unwinding.

---

## 10. Break / Continue Target Resolution

Break and Continue carry target `ControlRegionId`.

When PendingBreak or PendingContinue reaches the target loop region:

```text
run all cleanup crossed so far
resolve to loop break/continue target block
clear PendingControl
```

Break/continue MUST NOT skip cleanup.

---

## 11. Return Resolution

PendingReturn resolves when it exits the current function region after all cleanup has run.

Before final frame exit:

```text
return contract check MUST run
```

If return contract check raises, PendingReturn becomes PendingRaise.

---

## 12. Raise Propagation

PendingRaise propagates outward until:

```text
a matching catch handles it
or function/module/test boundary is crossed
```

At outer boundary, PendingRaise becomes frame/module/test failure.

---

## 13. Normal Exit With Cleanup

Normal exit from a cleanup-owning region runs:

```text
defers
resources
finally
```

If cleanup produces Raise, Normal becomes PendingRaise.

---

## 14. Reentrancy and Safepoints

During cleanup:

```text
pending_control
region_stack
defer_stack
resource_stack
current cleanup progress
active errors
```

MUST be visible to GC and deopt.

If a defer/resource/finally calls into VM or host, cleanup state MUST remain explicitly represented.

---

## 15. JIT Requirements

Compiled code MUST NOT inline or elide cleanup unless it preserves this algorithm.

If compiled code cannot prove cleanup correctness, it MUST call:

```text
helper_perform_unwind
```

JIT deopt metadata at cleanup boundaries MUST reconstruct:

```text
PendingControl
RegionStack
CleanupProgress
active resource/defer state
source span
```

---

## 16. Validation

Validation MUST reject:

```text
Return crossing cleanup without unwind path
Break/Continue crossing cleanup without unwind path
Raise crossing cleanup without unwind path
finally block without override handling
defer registration without cleanup region
resource registration without cleanup region
resource close path without exactly-once state
PendingControl with heap references but no root visibility
cleanup state not reconstructable at deopt point
```

---

## 17. Audit Tracking

This document completes:

```text
R7
```

It resolves:

```text
B-06
```

It partially supports:

```text
B-05
M-10
M-15
```
