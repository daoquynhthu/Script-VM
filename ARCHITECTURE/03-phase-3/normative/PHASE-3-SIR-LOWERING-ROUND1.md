# Phase 3 · SIR to RuntimePlan / EIR Lowering · Round 1
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.6 lowering draft  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: SIR-to-RuntimePlan lowering rules, SIR-to-EIR lowering for declarations and expressions, slot allocation, shape/index lowering, call/access site creation, check insertion, source map preservation, lowering validation  
Out of scope: full structured-control lowering, full unwinding lowering, concrete JIT backend lowering, complete GC implementation, public bytecode

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



## 0. Round 1 Scope

This document defines the first lowering rules from frozen SIR into VM execution structures.

It covers:

1. lowering pipeline
2. lowering invariants
3. slot allocation
4. environment and binding lowering
5. type lowering
6. record shape lowering
7. enum shape lowering
8. module/function plan lowering
9. literal lowering
10. binding reference lowering
11. let/const lowering
12. assignment lowering
13. unary/binary/logical expression lowering
14. call lowering
15. access lowering
16. collection construction lowering
17. record/enum construction lowering
18. function construction lowering
19. format string lowering
20. type-check lowering
21. check insertion
22. call/access site creation
23. source map preservation
24. RuntimePlan validation
25. EIR validation
26. non-goals

This round intentionally does not lower:

```text
if
while
for
match
try/catch/finally
raise
return
break
continue
use
defer
assert
test
module import execution
structured unwinding
```

Those belong to later lowering rounds.

---

## 1. Lowering Pipeline

### 1.1 Pipeline

The lowering pipeline is:

```text
Validated SIR
  -> RuntimePlan generation
  -> EIR generation
  -> EIR validation
  -> fast interpreter / JIT-ready execution
```

### 1.2 Required Inputs

Lowering requires:

```text
IRUnit
NodeTable
BindingTable
ScopeTable
TypeTable
PatternTable
ControlRegionTable
ModuleInterfaceDescriptor
CapabilityTable
EffectTable
SourceTable
```

### 1.3 Output Artifacts

Lowering produces:

```text
RuntimePlan
EirModule
LoweringDiagnostics
```

### 1.4 Lowering Mode

Lowering modes:

```text
debug-lowering
checked-lowering
optimized-lowering
```

`debug-lowering` preserves maximal source structure and source spans.

`checked-lowering` is the default.

`optimized-lowering` may generate quickenable EIR and extra guard/deopt metadata.

All modes must preserve Phase 1 and Phase 2 semantics.

---

## 2. Lowering Invariants

Lowering must preserve:

```text
left-to-right evaluation order
binding identity
scope semantics
function default evaluation at call time
function declaration not hoisted
assignment target single evaluation
Bool-only condition semantics
no implicit coercion
checked integer overflow
division by zero errors
record fixed shape
enum closure
map insertion order
duplicate map key replacement semantics
read-only view mutation rejection
type contract checks
capability checks
source span mapping
module interface semantics
```

Lowering must not:

```text
perform textual lookup in hot paths
flatten source diagnostics away
turn SIR into public bytecode
expose EIR as package ABI
introduce CPython ABI assumptions
change error categories
change cleanup semantics
evaluate omitted defaults before call time
```

---

## 3. Slot Allocation

### 3.1 Slot Allocation Purpose

SIR uses `BindingId`.

Execution uses `SlotId`.

Lowering assigns each executable binding to a slot or cell-backed slot.

### 3.2 Slot Assignment Rule

Each runtime binding receives one of:

```text
direct value slot
cell slot
module slot
capture slot
builtin slot
```

### 3.3 Direct Value Slot

Use direct slot when:

```text
binding is local to function/block
binding is not captured mutably
binding is not exported
binding is not shared across module boundary
```

### 3.4 Cell Slot

Use cell slot when:

```text
binding is captured by closure and mutation is possible
binding is exported from module
binding may be observed by imported module during initialization
binding lifetime exceeds current frame
```

### 3.5 Module Slot

Top-level module bindings lower to module slots.

Exported module bindings must be cell-backed or otherwise observable through stable export table references.

### 3.6 Capture Slot

A capture slot references a binding cell or captured immutable value.

Mutable captures require cells.

Immutable captures may be copied only if no semantic observation is changed.

### 3.7 Temporary Slot

Expression lowering may allocate temporaries.

Temporaries:

```text
are not source-visible
must not appear in module interface descriptors
must have source-origin metadata where useful
may be reused when liveness allows
```

### 3.8 Slot Initialization

Declaration lowering must emit EIR that initializes slots at source execution points.

Functions are not hoisted.

A function slot is initialized when execution reaches the function declaration.

### 3.9 Slot Validation

Lowering must produce slot layout where:

```text
every BindingId used at runtime maps to a SlotId
every SlotId belongs to its function/module layout
every cell slot is initialized before cell read where required
every immutable binding has no invalid StoreCell/StoreSlot target
```

---

## 4. Environment and Scope Lowering

### 4.1 Scope to Slot Scope

SIR `ScopeId` maps to one or more execution environments.

For hot execution, scope lookup must be precomputed.

```text
ScopeId + BindingId -> SlotId
```

### 4.2 Block Scope

Block scope may lower to:

```text
slot range in current frame
nested environment object
debug-only scope map
```

Preferred hot path:

```text
slot range in current frame
```

### 4.3 Function Scope

Function scope lowers to a frame slot layout.

Parameters occupy parameter slots.

Locals occupy local slots.

Captures occupy capture slots or capture cells.

### 4.4 Module Scope

Module scope lowers to module slot table.

Module slot table supports:

```text
top-level declarations
imports
exports
module initialization state
circular import checks
```

### 4.5 Debug Scope

Debug tooling may reconstruct lexical scopes using source maps and slot maps.

Runtime execution must not require text-based lookup for ordinary variables.

---

## 5. Type Lowering

### 5.1 TypeDescriptor to RuntimeTypeDesc

Each SIR `TypeId` lowers to a `RuntimeTypeDesc`.

```text
TypeId -> RuntimeTypeDesc
```

### 5.2 Type Check Strategy

Lowering chooses a `TypeCheckStrategy`:

```text
Builtin -> ImmediateKindCheck
Record -> ShapeCheck
Enum -> ShapeCheck
Union -> UnionCheck
Optional -> OptionalCheck
Function -> FunctionSignatureCheck
Any -> AlwaysAccept
Never -> NeverAccept
Extension -> ExtensionCheck
```

### 5.3 Type Contract Check Insertion

Lowering must insert checks for:

```text
let/const declared type
function parameter type
function return type
record field type
enum payload type
explicit SIR TypeCheckNode
```

### 5.4 Contract Check Location

Contract checks must occur at the semantic boundary defined by SIR:

```text
binding initialization
parameter binding
return
field initialization
field assignment
enum construction
explicit check node
```

Lowering must not delay or advance checks in a way that changes error order.

---

## 6. Record Shape Lowering

### 6.1 Record Shape Creation

Each SIR record type lowers to a `RecordShape`.

```text
RecordId -> RecordShape
FieldId -> FieldIndex
```

### 6.2 Field Index Assignment

Field indices are assigned deterministically.

Recommended order:

```text
source declaration order
```

The chosen order is VM-internal but must be stable within a RuntimePlan.

### 6.3 Record Constructor Plan

A record constructor plan records:

```text
shape_id
required fields
default field expressions
field contract checks
field initializer order
```

Defaults remain evaluated at construction time.

### 6.4 Field Access Lowering

A statically resolved record field access lowers to:

```text
LoadField {
  receiver
  expected_shape
  field_index
  access_site_id
}
```

A statically resolved record field assignment lowers to:

```text
StoreField {
  receiver
  expected_shape
  field_index
  value
  access_site_id
}
```

### 6.5 Dynamic Field Rejection

Records have fixed shape.

Unknown dynamic field access must use fallback helper only to raise the correct error or support non-record receiver categories.

It must not add fields.

---

## 7. Enum Shape Lowering

### 7.1 Enum Shape Creation

Each SIR enum type lowers to an `EnumShape`.

```text
EnumId -> EnumShape
CaseId -> CaseIndex
Payload FieldId -> PayloadIndex
```

### 7.2 Enum Constructor Plan

An enum case constructor plan records:

```text
enum shape
case index
payload layout
payload contract checks
```

### 7.3 Enum Construction Lowering

Enum construction lowers to:

```text
ConstructEnumValue {
  dest
  shape_id
  case_index
  payload_values
}
```

### 7.4 Enum Pattern Lowering

Enum pattern checks lower to:

```text
check enum shape
check case index
load payload index
```

Full pattern lowering is completed in a later round.

---

## 8. ModulePlan Lowering

### 8.1 Module Body

The SIR module body lowers to a synthetic module initialization EIR function.

```text
ModuleBodyNode -> initialization_function
```

### 8.2 Top-Level Execution Order

The initialization function preserves top-level source order.

No declaration may be reordered across an import or effectful top-level expression unless proven semantically inert.

### 8.3 Import Lowering

This round creates `ImportPlanEntry` but does not fully lower import execution.

Import execution lowering is deferred to module/control lowering round.

### 8.4 Export Lowering

Each exported binding lowers to `ExportPlanEntry`.

Export entries reference module slots or module cells.

They do not copy values.

### 8.5 Module State

RuntimePlan reserves module state slot.

Module state transitions are implemented in later module execution lowering.

---

## 9. FunctionPlan Lowering

### 9.1 Function Declaration

A SIR function declaration lowers into:

```text
FunctionPlan
ConstructFunction EIR operation at declaration point
function binding slot initialization
```

Function bodies lower to separate EIR functions.

### 9.2 No Hoisting

Function declaration execution creates the function value when the declaration is reached.

Lowering must not initialize function binding at module/function entry unless source semantics prove equivalent.

### 9.3 Parameter Lowering

Each parameter binding lowers to a parameter slot.

Default expressions remain NodeId/EIR blocks evaluated at call time.

### 9.4 Return Contract

FunctionPlan records return type.

Actual return-check lowering is handled in return/control lowering round.

### 9.5 Closure Lowering

Captured bindings lower through `CaptureLayout`.

Mutable captures use cells.

Immutable captures may be optimized only after preserving closure semantics.

---

## 10. Literal Lowering

### 10.1 Nil/Bool/Int/Float

Immediate literals lower to `ConstantOp` or immediate EIR constants.

### 10.2 String

String literals lower to constant pool entries.

The VM may intern strings.

Interning must not change equality semantics.

### 10.3 Literal Source Mapping

Each literal operation should preserve source span for diagnostics, especially numeric overflow or invalid literal representation errors.

---

## 11. Binding Reference Lowering

### 11.1 Read Reference

A SIR `BindingReferenceNode` with `Read` lowers to:

```text
LoadSlot
LoadCell
LoadCapture
LoadModuleSlot
```

depending on slot storage.

### 11.2 WriteTarget Reference

A write target reference lowers to slot/cell target metadata.

The actual write is emitted by assignment lowering.

### 11.3 CallTarget Reference

A call target reference lowers to a value slot plus CallSite lowering.

### 11.4 TypeReference

Type references lower to runtime type descriptor slots or constants.

### 11.5 ModuleReference

Module references lower to module object slots.

### 11.6 Uninitialized Binding

If a binding can be read before initialization, generated EIR must preserve the uninitialized check.

---

## 12. Let/Const Lowering

### 12.1 LetDeclarationNode

Lowering:

1. lower initializer expression to value slot
2. insert type contract check if declared type exists
3. store into binding slot or cell
4. mark binding initialized

### 12.2 ConstDeclarationNode

Same as let, except destination mutability is immutable.

Const is shallow.

### 12.3 Declaration Destructuring

Declaration destructuring lowering is deferred to pattern lowering round.

This round may lower destructuring to helper call or reject unsupported lowering mode.

### 12.4 Initialization Order

Initializer is evaluated before binding becomes initialized.

If initializer raises, binding remains uninitialized.

---

## 13. Assignment Lowering

### 13.1 Binding Assignment

Lowering:

1. lower RHS value
2. resolve target BindingId to writable slot/cell
3. check mutability
4. check type contract if attached
5. store value

Target evaluation must occur once.

### 13.2 Field Assignment

Lowering:

1. lower receiver once to temporary slot
2. lower RHS value
3. emit `CheckReadonly`
4. emit shape check if statically known
5. emit field contract check if required
6. emit `StoreField`

### 13.3 Index Assignment

Lowering:

1. lower receiver once
2. lower index/key once
3. lower RHS value
4. emit readonly check
5. emit list/map-specific store or fallback helper

### 13.4 Augmented Assignment

Augmented assignment lowering must evaluate target once.

Recommended lowering:

```text
evaluate target address/reference once
load current value
evaluate RHS
perform binary op
store result to original target
```

The target address/reference must be represented explicitly in EIR or by a lowering-specific temporary descriptor.

---

## 14. Unary/Binary/Logical Lowering

### 14.1 Unary

SIR unary expressions lower to `UnaryOp`.

`not` emits Bool check or uses UnaryOp semantics that performs Bool check.

### 14.2 Binary

SIR binary expressions lower to `BinaryOp` plus required checks.

Arithmetic ops must preserve overflow/division semantics.

### 14.3 Equality and Identity

Equality lowers to `BinaryOp(Equal/NotEqual)`.

Identity lowers to `BinaryOp(Identity/NotIdentity)`.

### 14.4 Comparisons

Comparisons lower to Bool-producing operations.

Unsupported comparison paths call helper or raise `TypeError`.

### 14.5 Chained Comparisons

Chained comparisons lower to a sequence preserving single evaluation of operands.

Required structure:

```text
eval operand0 -> temp0
eval operand1 -> temp1
compare temp0 op0 temp1
if false -> result false
eval operand2 -> temp2
compare temp1 op1 temp2
...
```

Intermediate operands are evaluated exactly once.

### 14.6 Logical And/Or

Logical expressions lower to control-flow blocks.

They must short-circuit.

They must return Bool.

They must not lower to eager binary operations.

---

## 15. Call Lowering

### 15.1 Call Site Creation

Every SIR call expression creates a `CallSiteRecord`.

```text
CallExpressionNode -> CallSiteId
```

### 15.2 Evaluation Order

Call lowering preserves:

```text
callee first
positional arguments left-to-right
named arguments left-to-right
invoke after all argument evaluation
```

### 15.3 Lowered Shape

A call lowers to:

```text
lower callee -> callee slot
lower args -> argument slots
emit CallOp {
  call_site_id
  callee
  arguments
  fallback_helper
}
```

### 15.4 Defaults

Default argument evaluation occurs inside call handling when omitted.

Lowering must not evaluate defaults at function object construction.

### 15.5 Named Arguments

Named argument layout is recorded in `ArgumentLayout`.

Duplicate named arguments should already be SIR validation errors; lowering may still assert.

### 15.6 Call Feedback

CallOp references feedback and inline cache slots through CallSiteRecord.

---

## 16. Access Lowering

### 16.1 Access Site Creation

Each attribute/index/slice access that can become hot creates `AccessSiteRecord`.

```text
AttributeAccessNode -> AccessSiteId
IndexAccessNode -> AccessSiteId
SliceExpressionNode -> AccessSiteId
```

### 16.2 Attribute Read

If receiver type is statically known record:

```text
LoadField expected_shape field_index
```

Otherwise:

```text
AttributeRead fallback helper
```

### 16.3 Attribute Write

If receiver type is statically known record:

```text
StoreField expected_shape field_index
```

Otherwise fallback helper.

Unknown record fields remain invalid.

### 16.4 Method Access

Method access lowers to method-table lookup or specialized bound-method construction.

It should not use string dictionary lookup for known record methods.

### 16.5 Index Read

List and map index read lower to specialized EIR if receiver type is known.

Otherwise fallback helper.

### 16.6 Slice Read

List/string slicing lowers to `SliceRead` or helper.

Negative bounds and out-of-range behavior must match Phase 2 semantics.

---

## 17. Collection Construction Lowering

### 17.1 List Literal

List literal lowering:

```text
lower each element left-to-right
ConstructList
```

### 17.2 Map Literal

Map literal lowering:

```text
for each entry in source order:
  lower key
  check hashable
  lower value
ConstructMap
```

Duplicate key behavior:

```text
later value replaces earlier value
first insertion position preserved
```

### 17.3 Range

Range construction through core builtin may lower to helper or specialized Range construction.

---

## 18. Record Construction Lowering

### 18.1 RecordConstructionNode

Lowering:

```text
resolve RecordShape
lower field initializers in source order
lower omitted defaults at construction time
check required fields
check duplicate fields
check field contracts
ConstructRecord
```

### 18.2 Default Fields

Record field defaults are evaluated at construction time.

Lowering must preserve source-order and error-order semantics.

### 18.3 Field Contracts

Each field value is checked before record allocation commits or during construction before object publication.

If construction fails, no partially visible record escapes.

---

## 19. Enum Construction Lowering

### 19.1 EnumConstructionNode

Lowering:

```text
resolve EnumShape
resolve CaseIndex
lower payload expressions in source order
check payload contracts
ConstructEnumValue
```

### 19.2 No Dynamic Cases

Enum cases are closed.

Lowering must not generate dynamic case lookup for known enum case constructors except fallback error paths.

---

## 20. Function Construction Lowering

### 20.1 FunctionDeclarationNode

Lowering emits `ConstructFunction` at declaration execution point.

Then stores the function object into the function binding slot.

### 20.2 FunctionValueNode

A function value reference lowers to loading the function binding or constructing closure value if needed.

### 20.3 Capture Slots

Capture slots are collected from `CaptureLayout`.

Mutable captures use cell references.

### 20.4 Defaults

Parameter default NodeIds are retained for call-time lowering/execution.

They are not evaluated during function construction.

---

## 21. Format String Lowering

### 21.1 FormatStringNode

Lowering:

```text
create string builder or helper call
for each part left-to-right:
  text part -> constant string append
  expression part -> evaluate expression
  display convert
  append
produce String
```

### 21.2 Display Conversion

Display conversion is explicit through VM helper or builtin display protocol.

It is not implicit string coercion for arbitrary operations.

### 21.3 Error Order

Expression parts are evaluated left-to-right.

If expression evaluation raises, later parts are not evaluated.

---

## 22. Check Insertion

### 22.1 Required Checks

Lowering inserts EIR checks for:

```text
Bool conditions
type contracts
callability
arity
hashability
readonly mutation
capabilities
shape assumptions
integer overflow
division by zero
```

### 22.2 Check Placement

Checks must be placed before the operation whose semantic precondition they enforce.

A check must not be moved across side-effecting operations if that changes error order.

### 22.3 Guard vs Check

A `CheckOp` enforces language semantics.

A `GuardOp` enforces speculative optimization assumptions.

A failed check raises language error.

A failed guard falls back, deopts, calls helper, or raises only if the guard encodes a semantic precondition.

---

## 23. Source Map Preservation

### 23.1 SourceMap Requirement

Lowering must preserve mapping:

```text
EIR operation -> SIR NodeId -> SourceSpan
```

where source span exists.

### 23.2 Synthetic Operations

Synthetic operations inherit origin from the SIR node that required them.

Examples:

```text
contract checks
overflow checks
hashable checks
readonly checks
shape guards
helper calls
```

### 23.3 Diagnostic Rule

A runtime diagnostic from lowered EIR must report the source location of the original SIR/source construct, not merely the generated operation location.

---

## 24. RuntimePlan Validation

After RuntimePlan generation, validation must check:

```text
all runtime bindings have slots
all exported bindings have module cells or stable export slot references
all captures are represented
all functions have FunctionPlan
all records have RecordShape
all enums have EnumShape
all FieldId values used in EIR have FieldIndex
all CaseId values used in EIR have CaseIndex
all TypeId checks have RuntimeTypeDesc
all call expressions have CallSiteId
all access expressions have AccessSiteId where applicable
all synthetic module init functions exist
all source maps are structurally valid
```

---

## 25. EIR Validation

After EIR generation, validation must check:

```text
all slots referenced exist
all blocks terminate
all block targets exist
all checks reference valid TypeId/ShapeId
all calls reference valid CallSiteId
all accesses reference valid AccessSiteId
all helper references exist
all safepoints referenced exist
all deopt points referenced exist if guards use deopt
all writes have write-barrier path where heap mutation may occur
all source mappings are valid or explicitly synthetic
```

---

## 26. Lowering Failure

Lowering failure produces structured diagnostics.

Lowering must fail rather than generate semantically approximate EIR.

Failure examples:

```text
unsupported SIR node in current lowering mode
missing slot assignment
missing shape
missing helper
unsupported type contract
unrepresentable control region
unsupported pattern lowering
invalid source map
```

---

## 27. Compatibility

RuntimePlan and EIR are internal.

Lowering rules may evolve across VM versions.

However, any lowering from frozen SIR must preserve Phase 1 and Phase 2 semantics.

Lowering cache invalidates on:

```text
SIR digest change
RuntimePlan version change
EIR version change
VM version change
target profile change
helper table change
value layout profile change
GC profile change
capability environment change
dependency interface digest change
```

---

## 28. Non-Goals

This round does not define:

```text
structured-control lowering
unwinding lowering
import execution lowering
pattern lowering completeness
native backend lowering
Cranelift lowering
LLVM lowering
GC root enumeration details
public bytecode
```

---

## 29. Next Work

Next lowering round should define:

```text
block lowering
if lowering
while lowering
for lowering
return/break/continue lowering
raise lowering
try/catch/finally lowering
use/defer lowering
match/pattern lowering
assert/test lowering
module import execution lowering
structured unwinding lowering

```
