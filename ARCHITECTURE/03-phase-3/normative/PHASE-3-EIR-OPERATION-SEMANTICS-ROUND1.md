# Phase 3 · EIR Operation Semantics · Round 1
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.5 operation draft  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 3 Performance and JIT Architecture v0.3  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: EIR operation semantics, terminator semantics, fast interpreter execution loop, operation validation rules  
Out of scope: full SIR-to-EIR lowering rules, concrete JIT backend lowering, concrete GC implementation, final opcode encoding, public bytecode

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

This document defines the first concrete semantics for EIR operations.

It covers:

1. EIR execution model
2. slot read/write rules
3. constant operations
4. load operations
5. store operations
6. unary operations
7. binary operations
8. logical operations
9. check operations
10. call operations
11. access operations
12. construction operations
13. pattern operations
14. runtime helper operations
15. safepoint operations
16. guard operations
17. terminator semantics
18. fast interpreter loop
19. EIR validation additions

EIR remains internal VM execution IR.

EIR is not public bytecode.

---

## 1. EIR Execution Model

### 1.1 EIR Function Execution

An EIR function executes inside a VM frame.

Execution state:

```text
EirExecutionState {
  current_function: EirFunctionId
  current_block: EirBlockId
  instruction_index: UInt
  slots: SlotArray
  region_stack: RegionStack
  pending_control?: VmControl
  feedback_table: FeedbackTable
}
```

### 1.2 Operation Rule

Each `EirOp` performs one of:

```text
read slots
write slots
check condition
call helper
update feedback
register safepoint
perform guarded specialization
```

An operation may complete with:

```text
OpResult =
  | Continue
  | Raise(ErrorHandle)
  | Deopt(DeoptId)
  | InternalError(VmError)
```

Ordinary control flow is handled by terminators.

### 1.3 Terminator Rule

Each `EirBlock` ends with exactly one terminator.

A terminator selects the next control state:

```text
TerminatorResult =
  | NextBlock(EirBlockId)
  | Return(Value)
  | Raise(ErrorHandle)
  | Unwind(VmControl)
  | Deopt(DeoptId)
  | Halt
```

### 1.4 No Fallthrough

EIR blocks do not implicitly fall through.

Every block must terminate explicitly.

---

## 2. Slot Semantics

### 2.1 SlotArray

```text
SlotArray {
  values: List<SlotState>
}
```

### 2.2 SlotState

```text
SlotState =
  | Uninitialized
  | Value(Value)
  | Cell(BindingCellRef)
  | RuntimeInternal(RuntimeValue)
```

### 2.3 Slot Read

Reading an uninitialized slot is invalid unless the operation explicitly permits it.

User-visible uninitialized binding reads raise:

```text
UninitializedBindingError
```

Internal uninitialized temporary reads are VM validation errors or InternalVMError.

### 2.4 Slot Write

Writing a slot must respect:

```text
slot kind
mutability
cell/value storage mode
type contract where attached
write barrier where heap mutation occurs
```

### 2.5 Cell Slot

If a slot stores a cell, ordinary binding read loads the cell's current value.

Writing to an immutable cell raises assignment error.

Writing to a mutable cell updates the cell and triggers write barrier hook if required.

### 2.6 Slot Validation

EIR validation must reject:

```text
read before initialized where statically provable
write to constant slot
write to readonly slot
slot kind mismatch
slot index out of function slot layout
```

---

## 3. Constant Operations

### 3.1 ConstantPool

```text
ConstantPool {
  constants: List<ConstantValue>
}
```

### 3.2 ConstantValue

```text
ConstantValue =
  | Nil
  | Bool
  | Int
  | Float
  | String
  | Symbol
  | TypeRef
  | FunctionRef
  | RecordTypeRef
  | EnumTypeRef
  | ModuleRef
```

### 3.3 ConstantOp

```text
ConstantOp {
  dest: SlotId
  constant_id: ConstantId
}
```

### 3.4 Constant Semantics

Execution loads the constant into `dest`.

Heap-backed constants may be:

```text
interned
allocated at module initialization
loaded from constant pool handle
```

Observable semantics must be identical.

### 3.5 String Constants

String constants are immutable.

The VM may intern strings.

String interning must not change equality semantics.

If identity semantics expose string object identity, interning policy must be documented and stable within a VM run.

---

## 4. Load Operations

### 4.1 LoadOp

```text
LoadOp =
  | LoadSlot
  | LoadCell
  | LoadCapture
  | LoadModuleSlot
  | LoadField
  | LoadEnumPayload
  | LoadConstant
```

### 4.2 LoadSlot

```text
LoadSlot {
  dest: SlotId
  source: SlotId
}
```

Reads value from source and writes to dest.

If source is a cell, this operation copies the cell reference unless explicitly designated as value load.

### 4.3 LoadCell

```text
LoadCell {
  dest: SlotId
  cell_slot: SlotId
}
```

Reads the current value from a cell.

If cell is uninitialized, raises `UninitializedBindingError`.

### 4.4 LoadCapture

```text
LoadCapture {
  dest: SlotId
  capture_index: UInt
}
```

Loads a captured binding value.

Mutable captures read from capture cell.

### 4.5 LoadModuleSlot

```text
LoadModuleSlot {
  dest: SlotId
  module_id: ModuleId
  slot: SlotId
}
```

Loads a module binding slot.

If module is not initialized and the slot is not initialized, raises `ImportCycleError` or `UninitializedBindingError` depending on context.

### 4.6 LoadField

```text
LoadField {
  dest: SlotId
  receiver: SlotId
  expected_shape: ShapeId
  field_index: FieldIndex
  access_site_id?: AccessSiteId
}
```

Semantics:

1. read receiver
2. require record instance or read-only view over record instance
3. check shape
4. load field by index
5. write dest

If shape check fails, use fallback helper or raise `FieldError`.

### 4.7 LoadEnumPayload

```text
LoadEnumPayload {
  dest: SlotId
  enum_value: SlotId
  expected_shape: ShapeId
  expected_case: CaseIndex
  payload_index: PayloadIndex
}
```

Semantics:

1. read enum value
2. check enum shape
3. check case index
4. load payload index
5. write dest

Failure raises `PatternMatchError` or uses pattern fallback depending on context.

---

## 5. Store Operations

### 5.1 StoreOp

```text
StoreOp =
  | StoreSlot
  | StoreCell
  | StoreModuleSlot
  | StoreField
  | StoreListIndex
  | StoreMapEntry
```

### 5.2 StoreSlot

```text
StoreSlot {
  dest: SlotId
  value: SlotId
}
```

Copies value from source slot to destination slot.

Destination must be writable.

### 5.3 StoreCell

```text
StoreCell {
  cell_slot: SlotId
  value: SlotId
}
```

Writes into binding cell.

Checks:

```text
cell initialized rules
mutability
type contract
write barrier
```

### 5.4 StoreModuleSlot

```text
StoreModuleSlot {
  module_id: ModuleId
  slot: SlotId
  value: SlotId
}
```

Used for top-level declarations and module initialization.

Exported module slots must remain cells if external imports can observe them.

### 5.5 StoreField

```text
StoreField {
  receiver: SlotId
  expected_shape: ShapeId
  field_index: FieldIndex
  value: SlotId
  access_site_id?: AccessSiteId
}
```

Semantics:

1. read receiver
2. reject read-only view
3. require record instance
4. check shape
5. check field mutability
6. check field contract if present
7. write field
8. execute write barrier hook

### 5.6 StoreListIndex

```text
StoreListIndex {
  receiver: SlotId
  index: SlotId
  value: SlotId
  access_site_id?: AccessSiteId
}
```

Semantics:

1. require list or read-only view over list
2. reject read-only view
3. require Int index
4. check index bounds
5. write element
6. execute write barrier hook

### 5.7 StoreMapEntry

```text
StoreMapEntry {
  receiver: SlotId
  key: SlotId
  value: SlotId
  access_site_id?: AccessSiteId
}
```

Semantics:

1. require map or read-only view over map
2. reject read-only view
3. require hashable key
4. insert or replace entry
5. preserve map insertion order
6. execute write barrier hook

---

## 6. Unary Operations

### 6.1 UnaryOp

```text
UnaryOp {
  dest: SlotId
  operator: UnaryOperator
  operand: SlotId
}
```

### 6.2 UnaryOperator

```text
UnaryOperator =
  | Plus
  | Minus
  | Not
```

### 6.3 Unary Plus

Allowed for numeric values.

For Int, returns same Int.

For Float, returns same Float.

Other types raise `TypeError`.

### 6.4 Unary Minus

Allowed for numeric values.

Int negation must check overflow if fixed-width integer representation is used.

Float negation follows binary64 behavior.

Other types raise `TypeError`.

### 6.5 Not

`not` requires Bool.

No truthiness.

Other types raise `TypeError`.

---

## 7. Binary Operations

### 7.1 BinaryOp

```text
BinaryOp {
  dest: SlotId
  operator: BinaryOperator
  left: SlotId
  right: SlotId
  feedback_slot?: FeedbackSlotId
}
```

### 7.2 BinaryOperator

```text
BinaryOperator =
  | Add
  | Subtract
  | Multiply
  | Divide
  | Modulo
  | Equal
  | NotEqual
  | Less
  | LessEqual
  | Greater
  | GreaterEqual
  | Identity
  | NotIdentity
  | Membership
```

### 7.3 Numeric Operators

Numeric operators require valid numeric operands.

No implicit string/number coercion.

Int operations must not silently overflow.

Division by zero raises `DivisionByZeroError`.

### 7.4 Add

Allowed categories:

```text
Int + Int
Float + Float
String + String
List + List where language semantics define concatenation
```

If mixed numeric promotion is not defined by Phase 1, mixed `Int + Float` must be rejected unless explicitly lowered by a future amendment.

### 7.5 Comparisons

Comparison operators return Bool.

Operands must be comparable under language rules.

Unsupported comparisons raise `TypeError`.

### 7.6 Equality

`Equal` and `NotEqual` use language equality.

They do not imply identity.

### 7.7 Identity

`Identity` and `NotIdentity` compare language identity.

For heap objects, identity is VM object identity.

For singleton immediates, identity follows singleton semantics.

### 7.8 Membership

Membership uses language membership semantics.

Required initial categories:

```text
value in List
key in Map
```

String membership is not defined unless Phase 1 or standard library later defines it.

---

## 8. Logical Operations

### 8.1 LogicalOp

Logical operators may appear as EIR operations only when short-circuit structure is preserved.

```text
LogicalOp =
  | LogicalAnd
  | LogicalOr
```

### 8.2 Bool Rule

Logical operators require Bool operands.

No truthiness.

### 8.3 Short-Circuit Rule

The preferred lowering of logical expressions is control-flow lowering:

```text
evaluate left
check Bool
branch
evaluate right only if needed
check Bool
write Bool result
```

A single LogicalOp is valid only if it does not evaluate both operands eagerly.

### 8.4 Result Rule

Logical operators return Bool.

They do not return arbitrary operand values.

---

## 9. Check Operations

### 9.1 CheckOp

```text
CheckOp =
  | CheckBool
  | CheckType
  | CheckCallable
  | CheckArity
  | CheckHashable
  | CheckReadonly
  | CheckCapability
  | CheckShape
  | CheckOverflow
  | CheckDivisionByZero
```

### 9.2 CheckBool

Requires value is Bool.

Used for:

```text
if
while
logical operators
match guards
catch guards
assert
```

Failure raises `TypeError`.

### 9.3 CheckType

```text
CheckType {
  value: SlotId
  type_id: TypeId
  failure_code: ErrorCode
}
```

Used for type contracts.

Failure raises `TypeContractError` unless a more specific failure code is supplied.

### 9.4 CheckCallable

Requires callable value.

Callable kinds:

```text
Function
BuiltinFunction
RecordConstructor
EnumCaseConstructor
BoundMethod
HostFunction
```

Failure raises `TypeError`.

### 9.5 CheckArity

Checks positional/named/default argument layout.

Failure raises `ArityError`.

### 9.6 CheckHashable

Checks map key legality.

Failure raises `TypeError`.

### 9.7 CheckReadonly

Fails if mutation target is read-only view.

Failure raises `ReadOnlyError`.

### 9.8 CheckCapability

Requires capability in current capability environment.

Failure raises `CapabilityError`.

### 9.9 CheckShape

Checks record or enum shape.

Failure may:

```text
fall back to generic helper
raise FieldError
raise PatternMatchError
deopt
```

depending on operation context.

### 9.10 Arithmetic Checks

Overflow and division-by-zero checks must precede committing arithmetic results.

---

## 10. Call Operations

### 10.1 CallOp

```text
CallOp {
  dest: SlotId
  call_site_id: CallSiteId
  callee: SlotId
  arguments: List<SlotId>
  named_argument_layout?: ArgumentLayout
  fallback_helper: RuntimeHelperId
}
```

### 10.2 Call Semantics

Execution:

1. read callee
2. read arguments in already-lowered left-to-right order
3. consult call-site cache if enabled
4. verify callable kind
5. check arity
6. evaluate omitted defaults if required
7. bind parameters
8. call target
9. write result or propagate control

### 10.3 Function Calls

User function call:

```text
push frame
bind parameters
execute target EIR function
handle return
check return contract
pop frame
```

### 10.4 Builtin Calls

Builtin calls go through VM builtin table.

Builtins may:

```text
allocate
raise
require capability
be safepoints
```

### 10.5 Constructor Calls

Record constructor calls must:

```text
allocate record instance
evaluate defaults at construction time
check field contracts
initialize fields by FieldIndex
```

Enum case constructor calls must:

```text
allocate enum value
check payload contracts
store payload by payload index
```

### 10.6 Method Calls

Method calls must preserve receiver binding.

A method call may be lowered to:

```text
load method
bind receiver
call function
```

or to a specialized method-call operation.

Semantics must match Phase 1.

### 10.7 Call Cache Rule

A call cache may specialize on:

```text
callee identity
callee kind
function id
arity
argument layout
receiver shape
```

All specializations require guard/fallback.

---

## 11. Access Operations

### 11.1 AccessOp

```text
AccessOp =
  | AttributeRead
  | AttributeWrite
  | MethodRead
  | IndexRead
  | IndexWrite
  | SliceRead
```

### 11.2 AttributeRead

```text
AttributeRead {
  dest: SlotId
  receiver: SlotId
  access_site_id: AccessSiteId
  field_index?: FieldIndex
  fallback_helper: RuntimeHelperId
}
```

If field index is available, use shape-checked field load.

Otherwise call fallback helper.

### 11.3 AttributeWrite

Attribute writes lower to `StoreField` or fallback helper.

Must reject dynamic undeclared fields on records.

### 11.4 MethodRead

Loads or constructs a bound method.

Method lookup should use method table and shape/type descriptor, not string dictionary lookup.

### 11.5 IndexRead

Required categories:

```text
List[Int]
Map[Hashable]
```

String indexing is not core.

Invalid index raises `IndexError` or `KeyError`.

### 11.6 IndexWrite

Required categories:

```text
List[Int] existing index
Map[Hashable] insert or replace
```

Read-only targets raise `ReadOnlyError`.

### 11.7 SliceRead

Required categories:

```text
List[Int?:Int?]
String[Int?:Int?]
```

Slice is half-open.

Negative bounds are errors under current Phase 2 semantics.

Out-of-range behavior follows Phase 2: error, not clamp.

---

## 12. Construction Operations

### 12.1 ConstructOp

```text
ConstructOp =
  | ConstructList
  | ConstructMap
  | ConstructRecord
  | ConstructEnumValue
  | ConstructFunction
  | ConstructError
```

### 12.2 ConstructList

```text
ConstructList {
  dest: SlotId
  elements: List<SlotId>
}
```

Allocates list and stores elements in order.

### 12.3 ConstructMap

```text
ConstructMap {
  dest: SlotId
  entries: List<MapEntrySlots>
}
```

```text
MapEntrySlots {
  key: SlotId
  value: SlotId
}
```

Checks hashable keys.

Duplicate keys replace values while preserving first insertion position.

### 12.4 ConstructRecord

```text
ConstructRecord {
  dest: SlotId
  shape_id: ShapeId
  field_values: List<FieldValueSlot>
}
```

```text
FieldValueSlot {
  field_index: FieldIndex
  value: SlotId
}
```

Checks all field contracts.

Allocates record instance with fixed shape.

### 12.5 ConstructEnumValue

```text
ConstructEnumValue {
  dest: SlotId
  shape_id: ShapeId
  case_index: CaseIndex
  payload_values: List<PayloadValueSlot>
}
```

Checks payload contracts.

Allocates enum value.

### 12.6 ConstructFunction

```text
ConstructFunction {
  dest: SlotId
  function_id: FunctionId
  capture_slots: List<SlotId>
}
```

Creates function object when execution reaches declaration.

Functions are not hoisted.

### 12.7 ConstructError

Creates language Error object.

Used by runtime errors, assertions, and explicit error construction where lowered.

---

## 13. Pattern Operations

### 13.1 PatternOp

```text
PatternOp =
  | PatternCheckLiteral
  | PatternCheckRecordShape
  | PatternCheckEnumCase
  | PatternCheckListLength
  | PatternCheckMapKey
  | PatternBind
  | PatternBranch
```

### 13.2 Pattern Failure

Pattern failure is not always language error.

Inside `match`, pattern failure branches to the next case.

Inside declaration destructuring, pattern failure raises `PatternMatchError`.

### 13.3 PatternBind

Writes matched value into pattern binding slot.

Pattern bindings are immutable unless Phase 1 amendment states otherwise.

### 13.4 Guard Interaction

Pattern guard is ordinary Bool check after pattern bindings are established.

Guard false continues to next case.

Guard non-Bool raises `TypeError`.

---

## 14. Runtime Helper Operations

### 14.1 RuntimeHelperOp

```text
RuntimeHelperOp {
  dest?: SlotId
  helper_id: RuntimeHelperId
  arguments: List<SlotId>
  safepoint_id?: SafepointId
}
```

### 14.2 Helper Semantics

Execution:

1. load helper descriptor
2. check capability if required
3. make roots visible if safepoint
4. call helper
5. handle returned value or control
6. write dest if value returned

### 14.3 Helper Result

Helpers may return:

```text
Value
VmControl
VmError
```

Language errors should become `VmControl::Raise`.

Structural VM failures become `VmError`.

### 14.4 Helper Constraints

Helpers must declare:

```text
may_allocate
may_raise
is_safepoint
required_capability
effect
```

The fast interpreter and JIT must respect those declarations.

---

## 15. Safepoint Operations

### 15.1 SafepointOp

```text
SafepointOp {
  safepoint_id: SafepointId
}
```

### 15.2 Safepoint Semantics

Execution reaches a VM observation point.

The VM may:

```text
poll cancellation
run GC
walk stack
record profile sample
handle interrupt
prepare deopt state
```

Phase 3 does not require all actions.

But the safepoint must expose correct root information.

### 15.3 Required Safepoints

Required candidates:

```text
function call boundary
loop backedge
allocation point
host call boundary
potential raise boundary
module import boundary
helper call if helper declares safepoint
```

---

## 16. Guard Operations

### 16.1 GuardOp

```text
GuardOp {
  guard_id: GuardId
  condition: GuardCondition
  success_next: EirLocation
  failure: GuardFailure
}
```

### 16.2 GuardCondition

```text
GuardCondition =
  | IsType
  | HasShape
  | IsCallTarget
  | HasCapability
  | ModuleStateIs
  | NotReadOnly
  | NoOverflow
  | NonZeroDivisor
```

### 16.3 GuardFailure

```text
GuardFailure =
  | Fallback(EirBlockId)
  | Helper(RuntimeHelperId)
  | Deopt(DeoptId)
  | Raise(ErrorCode)
```

### 16.4 Guard Semantics

If condition holds, continue.

If condition fails, take failure path.

Guard failure must not silently continue along optimized path.

### 16.5 Guard Feedback

Guard failures may update feedback state.

Repeated guard failures may disable quickened path.

---

## 17. Terminator Semantics

### 17.1 Jump

```text
Jump {
  target: EirBlockId
  arguments?: List<SlotId>
}
```

Transfers control to target block.

Block arguments are assigned to block parameters.

### 17.2 Branch

```text
Branch {
  condition: SlotId
  true_target: EirBlockId
  false_target: EirBlockId
}
```

Requires Bool.

No truthiness.

### 17.3 Return

```text
Return {
  value: SlotId
}
```

Returns from current function.

Must execute function-region unwinding and return contract check as required by lowering strategy.

### 17.4 Raise

```text
Raise {
  error: SlotId
}
```

Requires Error value.

If non-error, raises TypeError.

### 17.5 LoopBackedge

```text
LoopBackedge {
  target: EirBlockId
  safepoint_id?: SafepointId
  hotness_counter?: CounterId
}
```

May poll safepoint and update hotness.

Transfers control to loop target.

### 17.6 Switch

```text
Switch {
  discriminant: SlotId
  cases: List<SwitchCase>
  default_target: EirBlockId
}
```

Used for enum cases, pattern lowering, or dense branching.

### 17.7 Unwind

```text
Unwind {
  control: PendingControlRef
}
```

Transfers to structured unwinding machinery.

May be implemented by helper call in early VM.

### 17.8 Unreachable

If reached, signals InternalVMError.

Used after validation-proven impossible paths.

---

## 18. Fast Interpreter Loop

### 18.1 Conceptual Loop

```text
while true:
  block = current_function.blocks[current_block]
  while instruction_index < block.operations.len:
    op = block.operations[instruction_index]
    result = execute_op(op)
    if result == Continue:
      instruction_index += 1
      continue
    else:
      handle_nonlocal_result(result)
  terminator_result = execute_terminator(block.terminator)
  dispatch_terminator_result(terminator_result)
```

### 18.2 Interpreter Requirements

The interpreter must:

```text
use slots for locals
respect initialized state
update feedback where enabled
respect safepoints
call helpers through helper table
preserve source mapping for errors
propagate VmControl explicitly
not use Rust panic for language errors
```

### 18.3 Block Arguments

If EIR uses block parameters, jump arguments must be assigned before executing target block operations.

Assignments must be simultaneous if required to avoid clobbering.

### 18.4 Error Handling

If an operation returns `Raise`, control transfers to structured error propagation.

If an operation returns `InternalError`, execution aborts with VM diagnostic.

### 18.5 Deopt Handling

If an operation returns `Deopt`, the interpreter reconstructs lower-tier state or falls back to generic EIR path.

Early VM may reject optimized EIR containing deopt if deopt runtime is not implemented.

---

## 19. Operation Validation Additions

EIR validation must reject:

```text
operation reads unknown slot
operation writes unknown slot
operation writes non-writable slot
operation references unknown helper
operation references unknown call site
operation references unknown access site
operation references unknown safepoint
operation references unknown deopt point
operation has missing dest where result is required
operation has dest where no result allowed
type check references unknown TypeId
shape check references unknown ShapeId
field access references invalid FieldIndex
enum payload access references invalid PayloadIndex
guard failure has no valid target
terminator target block missing
branch condition not guaranteed or checked Bool
return outside function
raise operand not checked or known Error
```

---

## 20. Semantics Preservation Requirements

EIR operation semantics must preserve:

```text
left-to-right evaluation order
Bool-only conditions
no implicit coercion
checked integer overflow
division by zero errors
function defaults at call time
function declaration not hoisted
assignment target single evaluation
record fixed shape
enum closure
map insertion order
duplicate map-key replacement semantics
read-only view mutation rejection
structured unwinding
primary/suppressed error behavior
capability checks
module initialization order
source diagnostics
```

---

## 21. JIT Readiness

Every EIR operation must be classified by JIT lowering category:

```text
direct machine lowering
guarded fast path plus helper fallback
helper-only
interpreter-only until later tier
```

Round 1 does not assign all categories.

Future EIR operation semantics must include this classification.

---

## 22. Non-Goals

This document does not define:

```text
binary EIR encoding
public bytecode
concrete Cranelift lowering
concrete LLVM lowering
full GC protocol
complete deopt runtime
complete inline cache implementation
complete operation set
complete SIR-to-EIR lowering
```

---

## 23. Next Work

Next documents should define:

```text
SIR-to-RuntimePlan lowering
SIR-to-EIR lowering for expression/declaration nodes
EIR operation semantics round 2 for structured control and unwinding
fast interpreter concrete data structures
baseline JIT backend interface
runtime helper contracts

```
