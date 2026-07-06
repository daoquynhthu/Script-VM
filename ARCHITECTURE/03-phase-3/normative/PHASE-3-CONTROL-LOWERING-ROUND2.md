# Phase 3 · Structured Control and Unwinding Lowering · Round 2
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.7 lowering draft  
Depends on: Phase 3 SIR to RuntimePlan / EIR Lowering Round 1 v0.6  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: lowering for blocks, if, while, for, return, break, continue, raise, try/catch/finally, use, defer, match/patterns, assert/test, module import execution, structured unwinding  
Out of scope: concrete Cranelift lowering, concrete GC implementation, full optimizer, public bytecode, native ABI

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



## 0. Round 2 Scope

This document completes the first VM lowering coverage for structured control and unwinding.

It defines lowering for:

1. block execution
2. if/elif/else
3. while loops
4. for loops
5. loop break/continue targets
6. return
7. raise
8. try/catch/finally
9. use
10. defer
11. match/case
12. pattern decision lowering
13. assert
14. test blocks
15. module import execution
16. module initialization control
17. structured unwinding
18. cleanup ordering
19. suppressed error handling
20. safepoints in control flow
21. deopt metadata for control flow
22. lowering validation

The guiding principle is:

```text
Preserve Phase 2 structured semantics while producing EIR suitable for fast interpretation and later JIT.
```

---

## 1. Control Lowering Overview

### 1.1 Source Structure

Phase 2 SIR preserves structured control:

```text
Block
If
While
For
Match
Try
Catch
Finally
Use
Defer
Return
Break
Continue
Raise
Assert
Test
```

### 1.2 Lowered Structure

Phase 3 lowers these constructs into:

```text
EIR blocks
EIR terminators
RegionPlan
RegionStack operations
RuntimeHelper calls
Safepoints
Deopt metadata
Control target maps
Cleanup plans
```

### 1.3 Required Preservation

Lowering must preserve:

```text
source execution order
Bool-only conditions
no truthiness
loop target identity
return target identity
structured unwinding
defer LIFO order
use close order
finally override rule
primary/suppressed error behavior
match case order
pattern binding scope
guard evaluation order
module import order
diagnostic source spans
```

---

## 2. RegionPlan Lowering

### 2.1 ControlRegionDescriptor to RegionPlan

Each SIR `ControlRegionDescriptor` lowers to a `RegionPlan`.

```text
ControlRegionId -> RegionPlan
```

### 2.2 RegionPlan

```text
RegionPlan {
  region_id: ControlRegionId
  kind: ControlRegionKind
  owner_node: NodeId
  parent?: ControlRegionId
  entry_block?: EirBlockId
  normal_exit_block?: EirBlockId
  unwind_entry_block?: EirBlockId
  cleanup_plan?: CleanupPlan
  source_span?: SourceSpanId
}
```

### 2.3 CleanupPlan

```text
CleanupPlan {
  defers: DeferStackPlan
  resources: ResourceCleanupPlan
  finally_block?: EirBlockId
  cleanup_order: CleanupOrder
}
```

### 2.4 CleanupOrder

The default cleanup order is:

```text
defers
resources
finally
```

For try/finally, the active try/catch block exits first, then finally executes.

Lowering may encode finally as an explicit EIR block or runtime helper target.

### 2.5 Region Stack Operations

Lowering may emit internal operations:

```text
PushRegion
PopRegion
RegisterDefer
RegisterResource
RunCleanup
BeginUnwind
ResumeUnwind
```

These may be represented as EIR ops or RuntimeHelperOp.

Early VM may implement them as helpers.

---

## 3. Pending Control Model

### 3.1 PendingControl

Structured control lowering uses a pending control value.

```text
PendingControl =
  | Normal
  | Return(Value)
  | Break(ControlRegionId)
  | Continue(ControlRegionId)
  | Raise(ErrorHandle)
```

### 3.2 Pending Control Slot

Functions with structured control may reserve a hidden runtime slot:

```text
pending_control_slot: SlotId
```

This slot is not source-visible.

### 3.3 Lowering Rule

When a construct produces non-normal control, lowering writes pending control and jumps to the nearest relevant unwind block.

Example:

```text
return value
  -> write PendingControl::Return(value)
  -> jump function unwind entry
```

### 3.4 JIT/Deopt Rule

Pending control must be represented in deopt metadata.

Deoptimization must reconstruct:

```text
pending control kind
pending value or error
target region
active cleanup state
```

---

## 4. Block Lowering

### 4.1 BlockNode Lowering

A SIR block lowers to one or more EIR blocks.

Basic lowering:

```text
push block region if needed
lower each item in source order
if item completes non-normal -> jump unwind path
pop block region
jump normal exit
```

### 4.2 Block Scope

Block-local bindings lower to slots in the enclosing frame or block scope slot range.

If debug mode requires lexical reconstruction, source maps record block scope boundaries.

### 4.3 Empty Synthetic Blocks

Synthetic empty blocks lower to direct jump.

Source-originating empty blocks should already be rejected by Phase 2 validation.

### 4.4 Block Cleanup

If a block contains defers or resources, it owns cleanup state.

Exiting the block by:

```text
Normal
Return
Break
Continue
Raise
```

must execute block cleanup.

---

## 5. If Lowering

### 5.1 IfNode Lowering

An if node lowers to branch blocks.

Structure:

```text
entry
  evaluate condition0
  CheckBool
  Branch true -> then0, false -> next_condition
then0
  lower branch block
  jump if_exit
next_condition
  evaluate condition1
  CheckBool
  Branch true -> then1, false -> else_or_exit
...
else
  lower else block
if_exit
```

### 5.2 Bool Rule

Each condition must use `CheckBool`.

No truthiness.

### 5.3 Control Propagation

If any branch exits non-normally, it jumps into the relevant unwind path.

### 5.4 Source Mapping

Each condition check maps to the SIR condition source span.

Each branch block maps to the branch source span.

---

## 6. While Lowering

### 6.1 WhileNode Lowering

A while loop lowers to:

```text
loop_entry
  Safepoint? for loop entry
  evaluate condition
  CheckBool
  Branch true -> loop_body, false -> loop_exit

loop_body
  push loop/body regions as needed
  lower body
  on Normal -> loop_backedge
  on Continue(target loop) -> loop_backedge after cleanup
  on Break(target loop) -> loop_exit after cleanup
  on Return/Raise -> outer unwind

loop_backedge
  LoopBackedge terminator with safepoint and hotness counter
  jump loop_entry

loop_exit
```

### 6.2 Loop Safepoints

Loop backedges are safepoint candidates.

Lowering should attach:

```text
SafepointKind::LoopBackedge
hotness counter
live slot map
region state
```

### 6.3 Break/Continue Targeting

Break and continue target the nearest enclosing loop region unless Phase 2 introduces labels.

Current Phase 2 has no labels.

### 6.4 Loop Deopt State

Loop deopt metadata must include:

```text
current iteration state
live locals
active loop region
pending cleanup state
source span
```

---

## 7. For Lowering

### 7.1 ForNode Lowering

A for loop lowers to iterator-style EIR or helper-assisted iteration.

Structure:

```text
evaluate iterable
create iterator helper / iteration state
loop_entry
  poll next
  if done -> loop_exit
  bind iteration value
  lower body
  Normal/Continue -> loop_backedge
  Break -> loop_exit
  Return/Raise -> outer unwind
loop_backedge
  safepoint + hotness counter
  jump loop_entry
loop_exit
```

### 7.2 Iterable Categories

Required initial iterable categories:

```text
List
Map
Range
```

Map iteration yields keys in insertion order.

String iteration is not core.

### 7.3 Iterator State

Iterator state is stored in hidden runtime slots.

```text
iterable_slot
iterator_state_slot
current_value_slot
```

### 7.4 Iteration Binding

Each iteration writes the current value to the loop binding slot.

The binding is immutable per iteration.

Lowering may reuse the same physical slot across iterations if source semantics are preserved.

### 7.5 Pattern Destructuring in For

If future for-target patterns exist, they lower through pattern lowering.

If unsupported, lowering must reject with diagnostic.

---

## 8. Break and Continue Lowering

### 8.1 BreakNode

Lowering:

```text
write PendingControl::Break(loop_region)
jump current region unwind entry
```

### 8.2 ContinueNode

Lowering:

```text
write PendingControl::Continue(loop_region)
jump current region unwind entry
```

### 8.3 Cleanup Rule

Before reaching loop exit or loop backedge, all inner region cleanups must execute.

### 8.4 Target Validation

Lowering must reject:

```text
break without loop target
continue without loop target
target not ancestor loop
```

Phase 2 validation should already catch this; lowering rechecks.

---

## 9. Return Lowering

### 9.1 ReturnNode

Lowering:

```text
evaluate return value or nil
write PendingControl::Return(value)
jump function unwind entry
```

### 9.2 Return Contract

Return contract check may occur:

```text
before writing PendingControl::Return
or at function exit block
```

The chosen location must preserve error order.

Recommended:

```text
evaluate return expression
check return contract
write PendingControl::Return
unwind
```

If cleanup raises after return, cleanup error rules apply.

### 9.3 Function Exit

After all cleanup, function exit terminator returns the pending return value.

### 9.4 Top-Level Return

Top-level return is invalid and must not lower.

---

## 10. Raise Lowering

### 10.1 RaiseNode

Lowering:

```text
evaluate error expression
CheckType Error
write PendingControl::Raise(error)
jump current unwind entry
```

### 10.2 Non-Error Raise

If evaluated value is not Error, VM raises TypeError.

Lowering may represent this as:

```text
CheckType Error failure_code=TypeError
```

### 10.3 Raise Safepoint

Raise boundary is a safepoint candidate.

The safepoint map must include:

```text
error object
live locals
region stack
pending control state
```

---

## 11. Try/Catch/Finally Lowering

### 11.1 TryNode Structure

A try node lowers to:

```text
try_entry
try_body
catch_match
catch_body
finally_entry
try_exit
```

depending on available clauses.

### 11.2 Try Body

Try body executes normally.

If it raises and catch exists, control transfers to catch matching.

If it raises and no catch exists, pending raise continues to finally if present, then outward.

### 11.3 Catch Clause Lowering

Catch lowering:

```text
if pending control is Raise:
  bind error to catch binding
  evaluate catch guard if present
  CheckBool guard
  if guard true -> catch_body
  if guard false -> preserve original Raise
```

### 11.4 Catch Binding

Catch binding lowers to a local slot or cell as required by capture/export rules.

The caught error is initialized before guard evaluation.

### 11.5 Finally Lowering

Finally block executes on all exits:

```text
Normal
Return
Break
Continue
Raise
```

### 11.6 Finally Override

If finally completes normally, prior pending control resumes.

If finally produces Return/Break/Continue/Raise, it replaces prior pending control.

Lowering must encode this explicitly.

### 11.7 Finally Deopt Metadata

Finally deopt state must include:

```text
prior pending control
finally active state
live slots
active regions
source span
```

---

## 12. Use Lowering

### 12.1 UseNode Lowering

A use node lowers to:

```text
evaluate resource expression
bind resource
register resource cleanup in current/use region
lower body
on any exit -> run resource cleanup
```

### 12.2 Resource Registration

Resource registration records:

```text
resource slot
close method symbol
acquisition source span
closed flag
```

### 12.3 Close Rule

If acquisition succeeds, close must be called exactly once.

If acquisition fails, body does not execute and no close is registered.

### 12.4 Close Error Handling

If body exits normally and close raises, use exits with Raise.

If body raises and close raises, body error remains primary and close error is suppressed.

If body returns/breaks/continues and close raises, cleanup error becomes Raise unless Phase 2 amendment specifies otherwise.

### 12.5 Use Safepoint

Resource close is a helper call and safepoint candidate.

---

## 13. Defer Lowering

### 13.1 DeferNode Lowering

A defer node lowers to:

```text
evaluate callable
CheckCallable
CheckArity zero
register defer in current region
```

### 13.2 Defer Registration

Defer registration stores:

```text
callable value
registration source span
owning region
```

### 13.3 Defer Execution

Defers execute when owning region exits.

Order:

```text
last registered
first executed
```

### 13.4 Defer Error Handling

If defer raises while no error is pending, it becomes primary Raise.

If defer raises while Raise is pending, defer error is suppressed.

If defer raises while Return/Break/Continue is pending, lowering follows Phase 2 cleanup error rule.

### 13.5 Defer and JIT

JIT code must not inline away defer registration unless it proves identical cleanup behavior and deopt state.

---

## 14. Match and Pattern Lowering

### 14.1 MatchNode Lowering

A match lowers to an ordered decision sequence.

Structure:

```text
evaluate subject once
for each case in source order:
  attempt pattern
  if fail -> next case
  bind pattern variables
  if guard exists:
    evaluate guard
    CheckBool
    if false -> next case
  execute case body
  jump match_exit or propagate control
no match -> Normal
```

### 14.2 Subject Evaluation

Subject expression is evaluated exactly once.

The value is stored in a temporary slot.

### 14.3 Case Order

Case order is source order.

Lowering may build decision trees only when it preserves source-observable behavior.

### 14.4 Pattern Failure

Pattern failure inside match is branch control, not error.

Pattern failure in declaration destructuring remains `PatternMatchError`.

### 14.5 Pattern Binding Slots

Pattern bindings lower to case-scope slots.

Bindings are initialized only after successful pattern match.

### 14.6 Or-Pattern Lowering

Or-pattern alternatives must bind the same binding set.

Lowering may allocate shared binding slots.

Each alternative writes the same logical binding slots.

### 14.7 Guard Lowering

Guard executes after pattern bindings.

Guard must CheckBool.

Guard false proceeds to next case.

### 14.8 Pattern Operation Lowering

Pattern kinds lower as:

```text
Wildcard -> direct success
Literal -> equality check
Binding -> slot write
Record -> shape check + field subpatterns
Enum -> shape/case check + payload subpatterns
List -> length check + element subpatterns
Map -> key checks + value subpatterns
Or -> alternative branches
```

### 14.9 Decision Tree Rule

Optimized decision trees are allowed only if they preserve:

```text
subject single evaluation
case order where guards or side effects can observe order
binding scope
guard timing
error order
```

Conservative lowering is acceptable.

---

## 15. Assert Lowering

### 15.1 AssertNode

Lowering:

```text
evaluate condition
CheckBool
if true -> continue
if false:
  evaluate message if present
  construct AssertionError
  Raise
```

### 15.2 Message Evaluation

Message is evaluated only on assertion failure.

If message exists, it must be String.

### 15.3 Assertion Mode

Assertions are semantic in checked mode.

They must not be removed unless an explicit unchecked mode exists.

---

## 16. Test Lowering

### 16.1 TestNode

A test node lowers to a test function or test entry point.

Test nodes do not run during ordinary module initialization.

### 16.2 TestPlan

```text
TestPlan {
  test_name: String
  test_region: ControlRegionId
  test_function: EirFunctionId
  source_span?: SourceSpanId
}
```

### 16.3 Test Execution

A test runner invokes test functions explicitly.

Normal completion means success.

Raise or failed assertion means failure.

### 16.4 Test Isolation

Test isolation policy is host/test-runner defined.

The VM must at minimum preserve test source span and module association.

---

## 17. Module Import Execution Lowering

### 17.1 Import Declaration

Import execution lowering uses `ImportPlanEntry`.

Lowered steps:

```text
resolve module
load or retrieve module instance
if needed initialize module
check interface digest
bind imported module or export value
```

### 17.2 Source Order

Imports execute in source order as part of module initialization.

### 17.3 Named Import

Named import lowering:

```text
ensure provider initialized or safely initializing
check export exists
read provider export cell
bind local import slot
```

### 17.4 Whole Module Import

Whole-module import lowering binds the module object.

### 17.5 Circular Import

If imported value is an uninitialized export during a cycle, raise `ImportCycleError`.

### 17.6 Import Safepoint

Module import boundary is a safepoint candidate.

It may allocate, raise, and execute arbitrary module initialization.

---

## 18. Module Initialization Lowering

### 18.1 Initialization Function

Each module has synthetic initialization EIR function.

It handles:

```text
module state transition
top-level item execution
import execution
export table sealing
failure transition
```

### 18.2 Module State Transitions

Lowering emits or helpers implement:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
```

### 18.3 Failure Handling

If top-level execution raises, module state becomes Failed and stores initialization error.

### 18.4 Export Sealing

After successful initialization, export table is sealed.

Further mutation of export table is invalid.

---

## 19. Structured Unwinding Lowering

### 19.1 Unwind Entry

Each region that owns cleanup has an unwind entry block or helper target.

### 19.2 Unwind Algorithm

Lowered unwinding executes:

```text
while pending control not resolved:
  run current region defers in LIFO order
  run current region resource cleanup
  run finally block if attached
  pop region
  if pending control target reached:
    resolve control
  else continue outward
```

### 19.3 Cleanup Error Handling

Cleanup error handling follows Phase 2:

```text
pending Raise + cleanup Raise -> primary preserved, cleanup suppressed
pending Normal + cleanup Raise -> cleanup becomes primary Raise
pending Return/Break/Continue + cleanup Raise -> cleanup Raise supersedes non-error control unless amended
finally non-normal control -> overrides prior pending control
```

### 19.4 Suppressed Error Storage

Suppressed errors attach to primary Error object where supported.

If unsupported in bootstrap VM, the VM must still preserve diagnostics indicating suppressed cleanup error.

### 19.5 Unwind Helper

Early VM may implement unwinding through helper:

```text
RuntimeHelper::perform_unwind
```

Later EIR may inline parts of unwinding.

### 19.6 JIT Rule

JIT code may not skip unwinding.

Compiled frames must expose enough region/defer/resource state for unwinding helper or deopt.

---

## 20. Safepoints in Control Lowering

### 20.1 Required Control Safepoints

Control lowering must seed safepoints at:

```text
loop backedge
function call
runtime helper call
resource close
defer execution
module import
raise boundary
allocation in pattern/list/map/record/enum construction
```

### 20.2 Root Maps

Safepoint root maps must include:

```text
live slots
subject temporaries
pattern binding candidates
pending control value
error values
active region state
resource handles
defer callables
module import state
```

### 20.3 Deopt State

Deopt metadata for control constructs must include enough information to reconstruct:

```text
current source construct
active block
active loop
active match case
active try/catch/finally
pending control
cleanup progress
live values
```

---

## 21. Lowering Validation

Structured control lowering must reject:

```text
block region missing
if condition without Bool check
while condition without Bool check
loop without loop region
break target missing
continue target missing
return outside function
raise without Error check
try without valid region plan
catch binding without slot
finally without override handling
use without resource cleanup plan
defer without owning region
match without subject temp
pattern binding outside case scope
or-pattern with inconsistent binding layout
assert without Bool check
test executing during module initialization
import without ImportPlanEntry
module init without state transition handling
unwind path missing cleanup
cleanup path without suppressed error strategy
```

---

## 22. Compatibility

Control lowering may evolve.

But it must preserve Phase 1 and Phase 2 semantics.

Changing any of the following is a semantic change:

```text
condition Bool-only behavior
loop break/continue target
return cleanup order
raise propagation
finally override behavior
defer LIFO order
use close exactly-once rule
match case order
pattern binding scope
assert message evaluation timing
module import source order
circular import error behavior
suppressed error policy
```

---

## 23. Non-Goals

This document does not define:

```text
concrete machine-code lowering
Cranelift lowering
LLVM lowering
complete GC implementation
complete deopt runtime
complete inline cache machinery
public bytecode
native ABI
debugger protocol
```

---

## 24. Next Work

Next Phase 3 documents should define:

```text
runtime helper contracts
GC root enumeration concrete model
baseline JIT backend interface
EIR structured-control operation round 2 if needed
fast interpreter concrete data structures
Phase 3 audit pass

```
