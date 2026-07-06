# Phase 2 · SIR Concrete Semantics · Round 3

Version: 1.0 frozen baseline  
Depends on: Phase 2 SIR Concrete Semantics Round 2 v0.3  
Depends on: Phase 1 Language Specification v1.0 frozen baseline  
Scope: structured control flow, patterns, error flow, resource flow, test/assert nodes, unwinding semantics  
Out of scope: SIR-to-NIR lowering algorithm, VM dispatch strategy, optimizer IR, executable bytecode, native ABI

---

## 0. Round 3 Scope

Round 3 defines the structured control-flow layer of canonical SIR.

It fills concrete semantics for:

1. block nodes
2. if nodes
3. while nodes
4. for nodes
5. match nodes
6. pattern nodes
7. try/catch/finally nodes
8. raise nodes
9. return nodes
10. break nodes
11. continue nodes
12. use nodes
13. defer nodes
14. test nodes
15. assert nodes
16. control-flow effect representation
17. structured unwinding semantics
18. primary/suppressed error representation
19. Round 3 validation rules
20. compatibility rules for control-flow evolution

Round 3 completes the core SIR representation of the Phase 1 high-level language surface, except for later normalization/lowering details and final freeze review.

---

## 1. Control-Flow Design Principle

SIR preserves structured control flow.

SIR must not prematurely flatten high-level control constructs into unstructured jumps.

The following constructs remain explicit in canonical SIR:

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
Test
Assert
```

Lowering to explicit control states belongs to NIR or EIR.

Canonical SIR chooses semantic clarity and validation strength over low-level execution convenience.

---

## 2. Control Effect Model

### 2.1 ControlEffect

Every statement-like node may complete with a control effect.

```text
ControlEffect =
  | Normal
  | Return
  | Break
  | Continue
  | Raise
```

### 2.2 Control Effect Descriptor

```text
ControlEffectDescriptor {
  effect: ControlEffect
  value_node?: NodeId
  target_region?: ControlRegionId
  error_node?: NodeId
  source_span?: SourceSpanId
}
```

### 2.3 ControlRegionId

Round 3 introduces:

```text
ControlRegionId
```

Encoded as:

```text
region:<local-ordinal>
```

Control region IDs are local to an IR unit.

### 2.4 Control Region Kinds

```text
ControlRegionKind =
  | ModuleRegion
  | FunctionRegion
  | BlockRegion
  | LoopRegion
  | MatchRegion
  | TryRegion
  | CatchRegion
  | FinallyRegion
  | UseRegion
  | DeferRegion
  | TestRegion
```

### 2.5 ControlRegionDescriptor

```text
ControlRegionDescriptor {
  region_id: ControlRegionId
  kind: ControlRegionKind
  owner_node: NodeId
  parent?: ControlRegionId
  scope_id?: ScopeId
}
```

### 2.6 ControlRegionTable

```text
ControlRegionTable {
  regions: List[ControlRegionDescriptor]
}
```

The final consolidated `IRUnit` includes:

```text
control_regions: ControlRegionTable
```

### 2.7 Control Effect Semantics

`Normal` means execution proceeds to the next statement in the current sequence.

`Return` exits the nearest enclosing function region after unwinding.

`Break` exits the nearest enclosing loop region after unwinding.

`Continue` proceeds to the next iteration of the nearest enclosing loop region after unwinding.

`Raise` propagates an error value through catch/finally/use/defer unwinding.

### 2.8 Control Effect Validation

Validation must reject:

1. `Return` without enclosing function region
2. `Break` without enclosing loop region
3. `Continue` without enclosing loop region
4. `Raise` without an error value
5. control region parent cycles
6. control effect targeting a region that is not an ancestor where required
7. malformed region ownership

---

## 3. Block Node

### 3.1 BlockNode

```text
BlockNode {
  header: NodeHeader
  scope_id: ScopeId
  region_id: ControlRegionId
  items: List[NodeId]
}
```

### 3.2 Block Semantics

A block executes `items` in source order.

If an item completes with `Normal`, execution continues to the next item.

If an item completes with `Return`, `Break`, `Continue`, or `Raise`, remaining items are skipped and the control effect propagates after block-local unwinding.

### 3.3 Empty Block

Phase 1 forbids empty blocks.

Valid SIR generated from Phase 1 source must not contain an empty source-originating block.

A synthetic block may be empty only if it is introduced by lowering or normalization and marked `synthetic`.

### 3.4 Block Validation

Validation must reject:

1. block with missing scope
2. block with missing control region
3. source-originating block with zero items
4. item node reference missing
5. block scope not linked to the block region where required
6. block region kind not `BlockRegion` unless the block is owned by a more specific region node

---

## 4. If Node

### 4.1 IfNode

```text
IfNode {
  header: NodeHeader
  branches: List[IfBranch]
  else_block?: NodeId
}
```

```text
IfBranch {
  condition: NodeId
  block: NodeId
  source_span?: SourceSpanId
}
```

### 4.2 If Semantics

Branches are evaluated in source order.

For each branch:

1. evaluate condition
2. require Bool
3. if condition is `true`, execute branch block and skip remaining branches
4. if condition is `false`, continue to next branch

If no branch condition is true and `else_block` exists, execute `else_block`.

If no branch matches and no `else_block` exists, complete with `Normal`.

### 4.3 Condition Rule

SIR must not encode truthiness lowering.

Conditions must evaluate to Bool.

If static type information proves a condition is not Bool, validation may reject the IR.

Otherwise, runtime raises `TypeError`.

### 4.4 If Validation

Validation must reject:

1. zero branches
2. missing branch condition
3. missing branch block
4. else block reference missing
5. statically non-Bool condition where conclusive

---

## 5. While Node

### 5.1 WhileNode

```text
WhileNode {
  header: NodeHeader
  region_id: ControlRegionId
  condition: NodeId
  body: NodeId
}
```

### 5.2 While Semantics

Execution repeats:

1. evaluate condition
2. require Bool
3. if false, complete with `Normal`
4. if true, execute body
5. if body completes `Normal`, repeat
6. if body completes `Continue`, repeat after unwinding the body
7. if body completes `Break`, complete with `Normal` after loop unwinding
8. if body completes `Return` or `Raise`, propagate after loop unwinding

### 5.3 While Validation

Validation must reject:

1. missing loop region
2. region kind not `LoopRegion`
3. missing condition node
4. missing body node
5. statically non-Bool condition where conclusive
6. body not executable statement/block node

---

## 6. For Node

### 6.1 ForNode

```text
ForNode {
  header: NodeHeader
  region_id: ControlRegionId
  iteration_scope: ScopeId
  iterator_binding: BindingId
  iterable: NodeId
  body: NodeId
}
```

### 6.2 For Semantics

A for loop executes as:

1. evaluate `iterable`
2. require iterable value
3. for each produced element:
   - create fresh immutable iteration binding
   - bind element to `iterator_binding`
   - execute body
4. if body completes `Normal`, continue to next element
5. if body completes `Continue`, continue to next element after unwinding body
6. if body completes `Break`, complete loop with `Normal`
7. if body completes `Return` or `Raise`, propagate after unwinding

### 6.3 Iterable Kinds

Phase 1 core iterable values:

```text
List
Map
Range
```

Map iteration yields keys in insertion order.

String is not iterable in the core language.

### 6.4 Iterator Binding

The loop variable is an immutable per-iteration binding.

It is scoped to the loop body.

It is not visible after the loop.

### 6.5 For Validation

Validation must reject:

1. missing loop region
2. region kind not `LoopRegion`
3. missing iteration scope
4. iterator binding not in iteration scope
5. iterator binding not immutable
6. missing iterable node
7. missing body node
8. statically known non-iterable value
9. loop variable visible outside loop body scope

---

## 7. Pattern Node Model

### 7.1 PatternTable

Round 3 introduces a conceptual pattern table.

Canonical SIR stores patterns in `PatternTable` and references them by `PatternId`.

Inline pattern embedding is not canonical SIR.

```text
PatternTable {
  patterns: List<PatternNode>
}
```

The final consolidated `IRUnit` includes:

```text
patterns: PatternTable
```

The pattern table is required, but it may be empty.

### 7.2 PatternId

```text
PatternId
```

Encoded as:

```text
pattern:<local-ordinal>
```

### 7.3 PatternNode

```text
PatternNode =
  | WildcardPatternNode
  | LiteralPatternNode
  | BindingPatternNode
  | RecordPatternNode
  | EnumPatternNode
  | ListPatternNode
  | MapPatternNode
  | OrPatternNode
```

Every pattern node has:

```text
PatternHeader {
  pattern_id: PatternId
  source_span?: SourceSpanId
  bindings: List<BindingId>
}
```

### 7.4 WildcardPatternNode

```text
WildcardPatternNode {
  header: PatternHeader
}
```

Matches any value and binds nothing.

### 7.5 LiteralPatternNode

```text
LiteralPatternNode {
  header: PatternHeader
  literal: NodeId
}
```

Matches by Phase 1 `==` semantics.

The literal node must be a valid literal expression.

### 7.6 BindingPatternNode

```text
BindingPatternNode {
  header: PatternHeader
  binding_id: BindingId
}
```

Matches any value and binds it immutably.

### 7.7 RecordPatternNode

```text
RecordPatternNode {
  header: PatternHeader
  record_type: TypeId
  fields: List[RecordPatternField]
}
```

```text
RecordPatternField {
  field_id: FieldId
  name: SymbolId
  pattern: PatternId
}
```

Matches record instances of the given record type and recursively matches listed fields.

Unmentioned fields are ignored.

### 7.8 EnumPatternNode

```text
EnumPatternNode {
  header: PatternHeader
  enum_type: TypeId
  case_id: CaseId
  payload: List[EnumPatternPayloadField]
}
```

```text
EnumPatternPayloadField {
  field_id: FieldId
  name: SymbolId
  pattern: PatternId
}
```

Matches enum values of the given enum type and case.

### 7.9 ListPatternNode

```text
ListPatternNode {
  header: PatternHeader
  elements: List[PatternId]
}
```

Matches lists with exactly the same length.

Rest patterns are not defined in Phase 1.

### 7.10 MapPatternNode

```text
MapPatternNode {
  header: PatternHeader
  entries: List[MapPatternEntry]
}
```

```text
MapPatternEntry {
  key_literal: NodeId
  value_pattern: PatternId
}
```

Matches if all listed keys exist and their values match recursively.

Only literal keys are allowed.

### 7.11 OrPatternNode

```text
OrPatternNode {
  header: PatternHeader
  alternatives: List[PatternId]
}
```

Matches if any alternative matches.

All alternatives must bind the same set of binding symbols.

### 7.12 Pattern Validation

Validation must reject:

1. missing pattern ID
2. duplicate binding name in one pattern
3. or-pattern alternatives with different binding sets
4. record pattern with unknown field
5. enum pattern with unknown case
6. enum pattern with unknown payload field
7. list pattern with missing element pattern
8. map pattern with non-literal key
9. map pattern with non-hashable literal key
10. pattern binding not scoped to match case or destructuring context

---

## 8. Match Node

### 8.1 MatchNode

```text
MatchNode {
  header: NodeHeader
  region_id: ControlRegionId
  subject: NodeId
  cases: List[MatchCase]
}
```

```text
MatchCase {
  pattern: PatternId
  guard?: NodeId
  scope_id: ScopeId
  block: NodeId
  source_span?: SourceSpanId
}
```

### 8.2 Match Semantics

Execution:

1. evaluate subject exactly once
2. test cases in source order
3. for each case:
   - attempt pattern match
   - if pattern fails, continue
   - bind pattern bindings in case scope
   - if guard exists, evaluate guard and require Bool
   - if guard is false, continue to next case
   - execute case block and complete with its control effect
4. if no case matches, complete with `Normal`

### 8.3 Guard Semantics

A guard is evaluated only after successful pattern match.

Guard evaluation can read pattern bindings.

Guard must evaluate to Bool.

### 8.4 Exhaustiveness

SIR does not require exhaustive match.

However, SIR must preserve enum type and case information so that later diagnostics may perform exhaustiveness analysis.

### 8.5 Match Validation

Validation must reject:

1. missing subject
2. zero cases only if source grammar forbids it; Phase 1 requires at least one case by practical syntax expectation
3. missing case pattern
4. missing case block
5. guard statically known non-Bool
6. case scope not `MatchCaseScope`
7. pattern bindings not in case scope
8. duplicate case-local binding names

---

## 9. Return Node

### 9.1 ReturnNode

```text
ReturnNode {
  header: NodeHeader
  value?: NodeId
  function_region: ControlRegionId
}
```

### 9.2 Return Semantics

A return node produces `ControlEffect.Return`.

If `value` is absent, the return value is `nil`.

Before leaving the function, structured unwinding must execute all relevant defers, use-cleanups, and finally blocks.

If the function declares a return type contract, the return value must satisfy it.

### 9.3 Return Validation

Validation must reject:

1. return without enclosing function region
2. function region kind not `FunctionRegion`
3. return value reference missing
4. statically known return type contract mismatch where conclusive

---

## 10. Break and Continue Nodes

### 10.1 BreakNode

```text
BreakNode {
  header: NodeHeader
  loop_region: ControlRegionId
}
```

### 10.2 ContinueNode

```text
ContinueNode {
  header: NodeHeader
  loop_region: ControlRegionId
}
```

### 10.3 Break Semantics

A break node produces `ControlEffect.Break` targeting the nearest enclosing loop region.

After unwinding the current body, the loop completes with `Normal`.

### 10.4 Continue Semantics

A continue node produces `ControlEffect.Continue` targeting the nearest enclosing loop region.

After unwinding the current body, the loop proceeds to the next iteration step.

### 10.5 Break/Continue Validation

Validation must reject:

1. break without enclosing loop region
2. continue without enclosing loop region
3. loop region kind not `LoopRegion`
4. break/continue targeting a non-ancestor loop region

---

## 11. Raise Node

### 11.1 RaiseNode

```text
RaiseNode {
  header: NodeHeader
  error_value: NodeId
}
```

### 11.2 Raise Semantics

The `error_value` expression is evaluated.

The result must be an `Error` value.

If it is not an `Error`, a runtime `TypeError` is raised.

A valid raise produces `ControlEffect.Raise`.

### 11.3 Raise Validation

Validation must reject:

1. missing error value node
2. statically known non-Error raise value where conclusive

---

## 12. Try/Catch/Finally Nodes

### 12.1 TryNode

```text
TryNode {
  header: NodeHeader
  try_region: ControlRegionId
  try_block: NodeId
  catch_clause?: CatchClause
  finally_clause?: FinallyClause
}
```

### 12.2 CatchClause

```text
CatchClause {
  catch_region: ControlRegionId
  error_binding: BindingId
  guard?: NodeId
  block: NodeId
  source_span?: SourceSpanId
}
```

### 12.3 FinallyClause

```text
FinallyClause {
  finally_region: ControlRegionId
  block: NodeId
  source_span?: SourceSpanId
}
```

### 12.4 Try Semantics

Execution:

1. execute try block
2. if try block completes with `Raise` and catch exists:
   - bind error to catch binding
   - if guard exists, evaluate guard and require Bool
   - if guard is true or absent, execute catch block
   - if guard is false, preserve original raise
3. if finally exists, execute finally before leaving try statement
4. propagate resulting control effect

### 12.5 Catch Guard

A catch guard is evaluated only when an error is raised.

The guard may read the catch error binding.

The guard must evaluate to Bool.

If the guard itself raises, that new error replaces guard evaluation and then finally still executes if present.

### 12.6 Finally Override Rule

If a finally block completes with `Normal`, the prior pending control effect continues.

If a finally block completes with `Return`, `Break`, `Continue`, or `Raise`, the finally control effect replaces the prior pending control effect.

### 12.7 Try Validation

Validation must reject:

1. try node missing try block
2. try region kind not `TryRegion`
3. catch region kind not `CatchRegion`
4. finally region kind not `FinallyRegion`
5. catch binding not in catch scope
6. catch guard statically known non-Bool
7. catch block missing
8. finally block missing where finally clause exists

---

## 13. Error Flow and Suppression

### 13.1 ErrorFlowDescriptor

```text
ErrorFlowDescriptor {
  primary_error: NodeId
  suppressed_errors: List[NodeId]
}
```

### 13.2 Suppressed Error Semantics

A cleanup error may be attached as suppressed information when another error is already primary.

Suppressed errors do not replace the primary error unless a finally block explicitly raises a new error as its own control effect.

### 13.3 Error Flow Requirement

SIR must preserve enough structure for NIR/EIR to implement:

```text
primary error
suppressed cleanup errors
finally override
cleanup ordering
```

Round 3 does not require every error flow to be precomputed as an explicit descriptor.

---

## 14. Use Node

### 14.1 UseNode

```text
UseNode {
  header: NodeHeader
  use_region: ControlRegionId
  resource_binding: BindingId
  resource_expr: NodeId
  body: NodeId
  close_method_symbol: SymbolId
}
```

### 14.2 Use Semantics

Execution:

1. evaluate `resource_expr`
2. require resource value
3. bind resource to `resource_binding`
4. execute body
5. when body exits by any control effect, call `close()`
6. propagate resulting control effect according to cleanup rules

### 14.3 Close Semantics

`close()` is invoked exactly once if resource acquisition succeeds.

If resource acquisition fails, the body is not executed and no close is invoked.

If body completes normally and close raises, the use node completes with `Raise`.

If body raises and close raises, body error remains primary and close error is suppressed if supported.

### 14.4 Use Validation

Validation must reject:

1. use region kind not `UseRegion`
2. missing resource binding
3. resource binding not immutable
4. missing resource expression
5. missing body
6. missing close method symbol
7. statically known non-resource expression where conclusive

---

## 15. Defer Node

### 15.1 DeferNode

```text
DeferNode {
  header: NodeHeader
  defer_region: ControlRegionId
  callable: NodeId
}
```

### 15.2 Defer Semantics

A defer node evaluates its callable expression and registers the resulting zero-argument callable with the current block region.

Registered defers execute when that block region exits.

Defers execute in last-in-first-out order.

The deferred callable must accept zero arguments.

### 15.3 Defer Error Semantics

If a deferred callable raises while no error is pending, the block exit becomes `Raise`.

If a deferred callable raises while another error is pending, the defer error is suppressed unless the implementation lacks suppressed-error support.

If multiple deferred callables raise, the first error in LIFO execution order becomes primary if no prior error exists; later errors are suppressed.

### 15.4 Defer Validation

Validation must reject:

1. defer at module top level
2. defer without enclosing block/function/test region
3. missing callable node
4. statically known non-callable defer target
5. statically known callable with nonzero arity where conclusive

---

## 16. Assert Node

### 16.1 AssertNode

```text
AssertNode {
  header: NodeHeader
  condition: NodeId
  message?: NodeId
}
```

### 16.2 Assert Semantics

Execution:

1. evaluate condition
2. require Bool
3. if true, complete with `Normal`
4. if false, raise `AssertionError`
5. if message exists, evaluate it and require String

Assertions are semantic in the frozen Phase 1 language.

They must not be silently removed unless a separately specified unchecked mode exists.

### 16.3 Assert Validation

Validation must reject:

1. missing condition
2. statically non-Bool condition where conclusive
3. statically non-String message where conclusive

---

## 17. Test Node

### 17.1 TestNode

```text
TestNode {
  header: NodeHeader
  name: String
  test_region: ControlRegionId
  scope_id: ScopeId
  body: NodeId
}
```

### 17.2 Test Semantics

A test node represents a named executable specification block.

Test nodes do not execute during ordinary module initialization.

A host test runner may execute test nodes explicitly.

A test node can access declarations in the same module.

### 17.3 Test Result Semantics

Round 3 does not define an external test runner API.

However, test execution must treat uncaught `Raise` and failed assertions as test failure.

Normal completion is test success.

### 17.4 Test Validation

Validation must reject:

1. missing test name
2. missing test region
3. test region kind not `TestRegion`
4. missing test scope
5. test scope kind not `TestScope`
6. missing body

---

## 18. Structured Unwinding Algorithm

### 18.1 Unwinding Trigger

Unwinding begins when a region exits with:

```text
Return
Break
Continue
Raise
Normal
```

`Normal` also triggers cleanup for regions that own defers or resources.

### 18.2 Region Exit Order

When leaving a region:

1. execute defers registered in that region in LIFO order
2. execute resource cleanups owned by that region in reverse acquisition order
3. if the region is a try/catch active region and has finally, execute finally
4. propagate resulting control effect to the parent region

### 18.3 Try/Finally Specific Order

For a try statement:

1. the active try or catch block exits
2. defers and use-cleanups inside that block execute
3. finally block executes
4. defers and use-cleanups inside finally execute
5. resulting control effect propagates outward

### 18.4 Use Cleanup Order

Nested `use` regions close in reverse acquisition order.

A `use` resource closes after body-local defers have executed if the defers belong to an inner block.

If a defer and a use belong to the same region, defers execute before resource close.

### 18.5 Control Override Rule

The current pending control effect may be replaced by cleanup control effects only according to these rules:

1. finally `Return`, `Break`, `Continue`, or `Raise` replaces prior pending control
2. defer `Raise` replaces prior `Normal`
3. defer `Raise` is suppressed under prior `Raise`
4. use close `Raise` replaces prior `Normal`
5. use close `Raise` is suppressed under prior `Raise`
6. cleanup `Return`, `Break`, or `Continue` is invalid unless produced by a syntactically valid context

### 18.6 Unwinding Representation Requirement

SIR may represent unwinding structurally rather than algorithmically.

However, SIR must preserve:

```text
region nesting
defer registration sites
resource ownership sites
try/catch/finally structure
control effect targets
cleanup order
source-origin information
```

This is sufficient for NIR/EIR to lower unwinding deterministically.

---

## 19. Round 3 Validation Additions

### 19.1 Control Region Validation

Validation must reject:

```text
duplicate control region ID
missing owner node
owner node mismatch
region parent cycle
invalid region kind for node
control target outside valid ancestor chain
```

### 19.2 Block/Control Validation

Validation must reject:

```text
return outside function
break outside loop
continue outside loop
raise with non-error value when statically known
if/while/for/match guards statically non-Bool
try with malformed catch/finally structure
use with statically non-resource value
defer with statically non-callable value
assert with statically non-Bool condition
```

### 19.3 Pattern Validation

Validation must reject:

```text
duplicate pattern binding name
or-pattern binding set mismatch
record pattern field not found
enum pattern case not found
enum payload field not found
map pattern non-literal key
list pattern malformed
pattern binding outside valid pattern scope
```

### 19.4 Unwinding Validation

Validation must reject:

```text
defer registered to missing region
use cleanup without resource ownership
finally region not attached to try node
cleanup control effect targeting inactive region
return from cleanup outside active function
break/continue from cleanup outside active loop
```

---

## 20. Compatibility Rules for Round 3

### 20.1 Safe Minor Additions

The following may be minor-version additions when feature-gated or optional:

```text
new optional control metadata
new optional exhaustiveness diagnostics
new optional source-origin detail
new optional cleanup diagnostic metadata
new optional test metadata
```

### 20.2 Compatibility-Sensitive Additions

The following require explicit feature gates:

```text
new control effect kind
new control region kind
new pattern kind
new cleanup owner kind
new test execution metadata with semantic meaning
```

### 20.3 Breaking Changes

The following require a major schema version change:

```text
changing defer execution order
changing use close ordering
changing finally override semantics
changing match case ordering
changing pattern binding scope
changing loop break/continue target semantics
changing assertion semantics
changing test ordinary-execution behavior
changing Bool-only condition semantics
changing map/list pattern matching rules
```

---

## 21. Freeze Patch Notes

`ControlRegionId` and `PatternId` are canonical ID classes in the final SIR schema.

All canonical patterns are stored in `PatternTable` and referenced by `PatternId`.

Inline pattern embedding is non-canonical and must not be used for digest-producing SIR.

---

## 21. Round 3 Conclusion

Round 3 completes the structured-control layer of canonical SIR.

The SIR can now represent the full frozen Phase 1 control surface:

```text
blocks
if/elif/else
while
for
match/case
patterns
try/catch/finally
raise
return
break
continue
use
defer
assert
test
structured unwinding
error suppression
```

The next round should define final integration and normalization boundaries:

```text
module initialization semantics
import/export execution order
SIR validation completeness
SIR-to-NIR lowering contracts
canonical digest requirements
conformance tests for IR producers
Phase 2 freeze criteria
```
