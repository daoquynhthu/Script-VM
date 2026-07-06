# Phase 3 · VM Runtime Semantics · Round 1
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.2 runtime draft  
Depends on: Phase 3 VM Framework v0.1  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Depends on: Phase 1 Language Specification v1.0 frozen baseline  
Scope: runtime value model, heap/handle model, environment model, frame model, region stack model, evaluator contracts, runtime error model  
Out of scope: full module resolver, full standard library, FFI implementation, native ABI, NIR/EIR schema, JIT, moving GC

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

This document fills the first concrete VM layer.

Round 1 defines the runtime substrate required before node-by-node SIR execution can be specified:

1. Rust representation strategy
2. runtime value model
3. heap and handle model
4. object identity model
5. aggregate object model
6. record and enum runtime model
7. function and closure model
8. module object model
9. binding and environment model
10. frame and call stack model
11. region stack model
12. control result model
13. runtime error model
14. read-only view model
15. resource handle model
16. evaluator function contracts
17. validation/execution boundary
18. implementation invariants

Round 1 does not yet define complete evaluation semantics for every SIR node. That belongs to later VM execution rounds.

---

## 1. Rust Representation Strategy

### 1.1 Core Principle

The VM should use explicit Rust enums, typed IDs, typed handles, and table-indexed storage.

The VM must not use raw pointers as language-level identity.

The VM must not expose Rust memory layout as ABI.

### 1.2 Internal IDs

Use compact newtype IDs internally.

Conceptual shape:

```rust
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BindingId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScopeId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TypeId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ObjectId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FrameId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RegionId(u32);
```

String-form IDs from SIR serialization should be decoded into typed IDs before execution.

### 1.3 Result Discipline

All VM operations that can fail return explicit results.

Conceptual shape:

```rust
type VmResult<T> = Result<T, VmError>;
```

Rust panics must not be used for ordinary language-level failures.

Language failures include:

```text
TypeError
NameError
IndexError
KeyError
ImportError
AssertionError
PatternMatchError
ReadOnlyError
```

These are VM errors or language `Error` values, not Rust panics.

### 1.4 Unsafe Policy

Round 1 assumes an unsafe-free core VM.

If future implementation introduces `unsafe`, it must not be required for:

```text
ordinary value evaluation
ordinary binding lookup
ordinary module execution
ordinary control flow
ordinary error propagation
```

Unsafe code, if any, must be isolated behind documented invariants.

---

## 2. Runtime Value Model

### 2.1 Value Enum

The VM must represent all frozen Phase 1 runtime value kinds.

Conceptual Rust shape:

```rust
pub enum Value {
    Nil,
    Bool(bool),
    Int(IntValue),
    Float(FloatValue),
    String(ObjRef<StringObj>),
    List(ObjRef<ListObj>),
    Map(ObjRef<MapObj>),
    Range(RangeValue),
    RecordType(ObjRef<RecordTypeObj>),
    RecordInstance(ObjRef<RecordInstanceObj>),
    EnumType(ObjRef<EnumTypeObj>),
    EnumValue(ObjRef<EnumValueObj>),
    ReadOnlyView(ObjRef<ReadOnlyViewObj>),
    Function(ObjRef<FunctionObj>),
    BuiltinFunction(BuiltinFunctionId),
    Module(ObjRef<ModuleObj>),
    Error(ObjRef<ErrorObj>),
    Resource(ObjRef<ResourceObj>),
}
```

`Resource` is a VM-level value kind used to model host resources and `use` semantics. It is not a general user-constructible Phase 1 literal.

### 2.2 Immediate vs Heap Values

Immediate values:

```text
Nil
Bool
Int
Float
Range
BuiltinFunctionId
```

Heap-backed values:

```text
String
List
Map
RecordType
RecordInstance
EnumType
EnumValue
ReadOnlyView
Function
Module
Error
Resource
```

An implementation may choose different physical storage, but language-visible semantics must match this split.

### 2.3 IntValue

Phase 1 requires integers not to silently overflow.

The first VM may choose one of:

```text
checked fixed-width integer
arbitrary precision integer
hybrid small/big integer
```

For semantic simplicity, the recommended initial strategy is arbitrary precision or checked `i64` with explicit `NumericOverflowError`.

If checked `i64` is used, integer literals or operations outside range must raise `NumericOverflowError`.

The choice must be documented in the VM target profile.

### 2.4 FloatValue

Float values are IEEE-754 binary64.

The VM must preserve Phase 1 float semantics.

NaN and infinity serialization remains governed by Phase 1 serialization boundary rules.

### 2.5 String Value

Strings are immutable Unicode scalar sequences.

String indexing is not a core operation.

String slicing uses Unicode scalar positions according to Phase 1.

The VM may internally store UTF-8, but slicing semantics must not split invalid scalar boundaries.

### 2.6 Value Equality

The VM must distinguish:

```text
equality: ==
identity: is
```

Equality uses Phase 1 value equality.

Identity uses VM object identity for heap values and canonical identity for immediate singletons where defined.

`nil`, `true`, and `false` have stable singleton identity.

Numeric value identity must not be used as equality substitute.

### 2.7 Value Display

Display conversion is explicit and used by:

```text
print
format strings
debug(value)
```

The VM must not implement implicit string coercion for `+`.

---

## 3. Heap and Handle Model

### 3.1 Heap Purpose

The heap owns runtime objects that outlive a single expression evaluation.

The heap supports:

```text
object allocation
object lookup
object mutation
object identity
read-only view enforcement
future GC replacement
```

### 3.2 Object Handle

Conceptual handle:

```rust
pub struct ObjRef<T> {
    raw: ObjectId,
    marker: PhantomData<T>,
}
```

The handle is typed in Rust but may erase to a common object reference internally.

### 3.3 ObjectId

`ObjectId` is VM-internal.

It must not be exposed as language ABI.

It may be implemented as:

```text
arena index
generational arena key
Rc allocation identity wrapper
Arc allocation identity wrapper
custom heap handle
```

Recommended first implementation:

```text
generational arena or typed arena with stable indices
```

Reason:

```text
prevents accidental stale references
keeps object identity explicit
does not require moving GC
keeps future GC transition possible
```

### 3.4 Heap Object Header

Every heap object should conceptually contain:

```text
object_id
object_kind
mutable_flag
readonly_origin?
debug_origin?
```

The actual Rust layout may differ.

### 3.5 Mutation Protocol

Mutable objects must be mutated through the heap API.

The evaluator should not directly mutate arbitrary shared structures.

Conceptual API:

```rust
heap.get(obj_ref) -> VmResult<&Obj>
heap.get_mut(obj_ref) -> VmResult<&mut Obj>
heap.set_field(record_ref, field_id, value) -> VmResult<()>
heap.list_set(list_ref, index, value) -> VmResult<()>
heap.map_set(map_ref, key, value) -> VmResult<()>
```

If `RefCell` or `RwLock` is used, borrow failures are VM internal bugs unless caused by a documented reentrancy rule.

### 3.6 Future GC Constraint

The first heap does not need moving GC.

However, the handle model must avoid exposing raw addresses so future moving or compacting GC remains possible.

Forbidden design:

```text
language identity = raw Rust pointer
native extension stores direct pointer to object internals
SIR/EIR cache stores raw address
module interface stores object address
```

---

## 4. Aggregate Objects

### 4.1 ListObj

```text
ListObj {
  elements: Vec<Value>
  readonly: Bool
}
```

Lists preserve order.

List mutation requires:

```text
not readonly
valid index
value accepted by optional element contract where enforced
```

### 4.2 MapObj

```text
MapObj {
  entries: OrderedMap<ValueKey, Value>
  readonly: Bool
}
```

Maps preserve insertion order.

Map keys must be hashable.

### 4.3 ValueKey

Map keys cannot be arbitrary mutable values.

A key must satisfy hashability rules.

Recommended key representation:

```text
Nil
Bool
Int
Float where allowed and canonicalized
String
EnumValue where payload is hashable
RecordInstance only if future Hashable protocol allows it
```

For Phase 3 minimal VM, restrict hashable keys to:

```text
Nil
Bool
Int
Float
String
EnumValue without mutable/hash-unstable payload
```

If this restriction is narrower than future full language intent, it must be documented as a minimal VM limitation.

### 4.4 RangeValue

```text
RangeValue {
  start: IntValue
  end: IntValue
  step: IntValue
}
```

Phase 1 uses `range` as a core builtin boundary.

A range is immutable and iterable.

---

## 5. Record Runtime Model

### 5.1 RecordTypeObj

```text
RecordTypeObj {
  record_id: RecordId
  name: SymbolId
  fields: Vec<RecordFieldRuntimeDesc>
  methods: MethodTable
  interface_id?: InterfaceId
}
```

### 5.2 RecordFieldRuntimeDesc

```text
RecordFieldRuntimeDesc {
  field_id: FieldId
  name: SymbolId
  index: FieldIndex
  mutability: FieldMutability
  type_contract?: TypeId
  has_default: Bool
}
```

### 5.3 RecordInstanceObj

```text
RecordInstanceObj {
  record_type: ObjRef<RecordTypeObj>
  fields: Vec<Value>
  readonly: Bool
}
```

Fields are stored by field index.

Field access should resolve to `FieldId` during validation or pre-execution preparation.

Dynamic field addition is not allowed.

### 5.4 Record Construction

Construction requires:

```text
known record type
known field names
all required fields initialized
defaults evaluated at construction time
field contracts checked
```

Unknown field names raise `FieldError`.

Duplicate initializers are validation errors when statically known and runtime errors otherwise.

### 5.5 Record Identity

A record type is nominal.

Two records with identical field shape are not the same type unless they share the same `RecordId` and module identity.

---

## 6. Enum Runtime Model

### 6.1 EnumTypeObj

```text
EnumTypeObj {
  enum_id: EnumId
  name: SymbolId
  cases: Vec<EnumCaseRuntimeDesc>
  interface_id?: InterfaceId
}
```

### 6.2 EnumCaseRuntimeDesc

```text
EnumCaseRuntimeDesc {
  case_id: CaseId
  name: SymbolId
  payload_fields: Vec<EnumPayloadRuntimeDesc>
}
```

### 6.3 EnumValueObj

```text
EnumValueObj {
  enum_type: ObjRef<EnumTypeObj>
  case_id: CaseId
  payload: Vec<Value>
}
```

### 6.4 Enum Closure

Enums are closed.

The VM must not permit cases to be added dynamically.

Pattern matching may rely on enum closure.

Adding enum cases is a module-interface breaking change under Phase 2.

---

## 7. Function and Closure Runtime Model

### 7.1 FunctionObj

```text
FunctionObj {
  function_id: FunctionId
  name?: SymbolId
  parameters: Vec<ParameterRuntimeDesc>
  return_type?: TypeId
  body: NodeId
  module: ModuleId
  lexical_scope: ScopeId
  captures: CaptureEnv
  effects: Vec<EffectId>
  required_capabilities: Vec<CapabilityId>
}
```

### 7.2 ParameterRuntimeDesc

```text
ParameterRuntimeDesc {
  binding_id: BindingId
  name: SymbolId
  type_contract?: TypeId
  default_value?: NodeId
}
```

### 7.3 CaptureEnv

```text
CaptureEnv {
  entries: Vec<CaptureEntry>
}
```

```text
CaptureEntry {
  binding_id: BindingId
  cell: BindingCellRef
  capture_kind: CaptureKind
}
```

Captured variables are represented as binding cells, not copied values, when write capture is possible.

Immutable captured values may be optimized later but must preserve semantics.

### 7.4 Function Creation

A function declaration creates a function object when execution reaches the declaration.

Functions are not hoisted.

Default argument expressions are stored as SIR nodes and evaluated at call time when omitted.

### 7.5 Call Semantics

A call creates a new frame.

Steps:

1. evaluate callee
2. evaluate arguments left to right
3. resolve positional and named arguments
4. evaluate omitted defaults at call time
5. check parameter contracts
6. create frame and local binding cells
7. execute function body
8. handle return or implicit nil
9. check return contract
10. pop frame

### 7.6 BuiltinFunction

A builtin function is a VM-hosted callable with explicit signature and capability metadata.

Conceptual descriptor:

```text
BuiltinFunctionDesc {
  id: BuiltinFunctionId
  name: SymbolId
  arity: Arity
  effects: Vec<EffectId>
  required_capabilities: Vec<CapabilityId>
  implementation: BuiltinImpl
}
```

Builtins must return `VmControl` or equivalent so they can raise language errors.

---

## 8. Module Runtime Model

### 8.1 ModuleObj

```text
ModuleObj {
  module_id: ModuleId
  name: QualifiedName
  state: ModuleState
  scope: EnvRef
  exports: ExportRuntimeTable
  interface: ModuleInterfaceDescriptor
  initialization_error?: ErrorHandle
}
```

### 8.2 ModuleState

The VM implements Phase 2 module states:

```text
Unloaded
Loading
Initializing
Initialized
Failed
```

### 8.3 ExportRuntimeTable

```text
ExportRuntimeTable {
  entries: OrderedMap<SymbolId, BindingCellRef>
  sealed: Bool
}
```

Exports reference binding cells.

They do not copy values.

### 8.4 Module Initialization

Top-level execution initializes module bindings in source order.

The export table is sealed after successful initialization.

Access to an uninitialized export during circular import raises `ImportCycleError`.

---

## 9. Binding and Environment Model

### 9.1 BindingCell

A binding cell stores runtime binding state.

```text
BindingCell {
  binding_id: BindingId
  state: BindingState
  mutability: BindingMutability
  type_contract?: TypeId
}
```

```text
BindingState =
  | Uninitialized
  | Initialized(Value)
```

### 9.2 Environment

```text
Environment {
  scope_id: ScopeId
  parent?: EnvRef
  cells: Map<BindingId, BindingCellRef>
}
```

The VM uses resolved `BindingId`, not textual lookup, for ordinary variable access.

Textual lookup may exist only for diagnostics or host/tooling.

### 9.3 Binding Initialization

Declaration execution initializes a binding cell.

Reading an uninitialized binding raises `NameError` or `UninitializedBindingError`.

Writing to immutable binding raises `TypeError` or a more specific assignment error.

### 9.4 Scope Chain

The runtime environment chain must match the Phase 2 scope graph.

Function calls create function environments.

Blocks create block environments where required by Phase 1 block scope.

Loop iteration variables are scoped to the loop body.

Pattern bindings are scoped to match case or destructuring context.

### 9.5 Environment Optimization

The VM may optimize local lookup with arrays indexed by precomputed slots.

Such optimization must preserve `BindingId` semantics.

---

## 10. Frame and Call Stack Model

### 10.1 Frame

```text
Frame {
  frame_id: FrameId
  function?: ObjRef<FunctionObj>
  module: ObjRef<ModuleObj>
  env: EnvRef
  region_stack: RegionStack
  call_span?: SourceSpanId
  return_type?: TypeId
}
```

### 10.2 Call Stack

```text
CallStack {
  frames: Vec<Frame>
}
```

The call stack is used for:

```text
function calls
return targeting
runtime diagnostics
stack traces
structured unwinding
debugging later
```

### 10.3 Top-Level Frame

Module initialization executes in a top-level module frame.

Top-level frame is not a function frame.

`return`, `break`, and `continue` are invalid at top level.

### 10.4 Stack Overflow

The VM must detect excessive recursion or stack growth.

If the host stack is used recursively, the VM should still maintain its own logical call stack for diagnostics.

A future implementation may use trampoline or explicit stack evaluation.

---

## 11. Region Stack Model

### 11.1 RegionFrame

```text
RegionFrame {
  region_id: ControlRegionId
  kind: ControlRegionKind
  scope?: EnvRef
  defers: Vec<DeferredCallable>
  resources: Vec<ResourceCleanup>
  finally_block?: NodeId
  loop_target?: LoopTarget
}
```

### 11.2 RegionStack

```text
RegionStack {
  regions: Vec<RegionFrame>
}
```

### 11.3 DeferredCallable

```text
DeferredCallable {
  callable: Value
  registered_at: SourceSpanId?
}
```

The callable must be zero-argument callable at execution time.

### 11.4 ResourceCleanup

```text
ResourceCleanup {
  resource: Value
  close_method: SymbolId
  acquired_at: SourceSpanId?
  closed: Bool
}
```

### 11.5 Region Stack Use

The region stack implements:

```text
return target lookup
break/continue target lookup
defer LIFO execution
use close ordering
finally execution
suppressed error attachment
```

### 11.6 Region Invariant

Every region pushed at runtime must correspond to a valid Phase 2 `ControlRegionDescriptor`.

Synthetic implementation regions may exist internally but must not alter language semantics.

---

## 12. Control Result Model

### 12.1 VmControl

Evaluator functions return control results.

Conceptual shape:

```rust
pub enum VmControl {
    Normal(Value),
    Return(Value),
    Break { target: ControlRegionId },
    Continue { target: ControlRegionId },
    Raise(ErrorHandle),
}
```

### 12.2 Statement Normal Value

Statements that complete normally return `Value::Nil` unless they are expression statements whose value is intentionally preserved by a host mode.

Ordinary language semantics do not expose statement values.

### 12.3 Control Propagation

Control propagation follows Phase 2 structured unwinding.

Evaluator functions must not collapse `Return`, `Break`, `Continue`, or `Raise` into ordinary values.

### 12.4 Rust Panic Boundary

Rust panic indicates VM implementation bug or unrecoverable host failure.

It is not language `raise`.

---

## 13. Runtime Error Model

### 13.1 ErrorObj

```text
ErrorObj {
  code: ErrorCode
  message: String
  details: Map<String, Value>
  source_span?: SourceSpanId
  stack_trace?: StackTrace
  suppressed: Vec<ErrorHandle>
}
```

### 13.2 ErrorCode

Required core error codes:

```text
NameError
UninitializedBindingError
TypeError
TypeContractError
PatternMatchError
ReadOnlyError
AssertionError
ArityError
IndexError
KeyError
FieldError
ImportError
ImportCycleError
DivisionByZeroError
NumericOverflowError
CapabilityError
InternalVMError
```

### 13.3 Primary and Suppressed Errors

The VM must represent primary and suppressed errors.

If a cleanup operation raises while another error is pending, the cleanup error is suppressed unless Phase 2 finally override semantics say otherwise.

### 13.4 Error Raising

Language `raise` requires an `Error` value.

If a program attempts to raise a non-error value, the VM raises `TypeError`.

### 13.5 Diagnostic Conversion

Runtime errors can be converted into diagnostics.

The error object is semantic; diagnostic rendering is presentation.

---

## 14. Read-Only View Model

### 14.1 ReadOnlyViewObj

```text
ReadOnlyViewObj {
  target: Value
}
```

A read-only view is shallow.

It prevents mutation through the view.

It does not recursively freeze the target value.

### 14.2 Mutation Through View

If a mutation target is a read-only view, the VM raises `ReadOnlyError`.

Examples:

```text
readonly(list)[0] = x
readonly(record).field = x
readonly(map)[k] = v
```

### 14.3 Identity

A read-only view is its own object.

`readonly(x) is x` is false unless a future optimization explicitly preserves identity in an observationally equivalent way and Phase 1 permits it.

### 14.4 Read Access

Read access through a read-only view delegates to the target.

---

## 15. Resource Runtime Model

### 15.1 ResourceObj

```text
ResourceObj {
  resource_id: ResourceId
  state: ResourceState
  close_callable: Value
  capability_origin?: CapabilityId
}
```

### 15.2 ResourceState

```text
ResourceState =
  | Open
  | Closing
  | Closed
  | Failed
```

### 15.3 Close Rule

A resource acquired by `use` must be closed exactly once if acquisition succeeds.

Closing an already closed resource is either:

```text
idempotent success
or ResourceStateError
```

The chosen policy must be documented by the resource implementation.

### 15.4 Phase 3 Minimal Policy

The VM core only defines resource protocol.

Concrete resources are host-provided.

No filesystem/network/process resource implementation is required in Round 1.

---

## 16. Evaluator Contracts

### 16.1 Core Evaluator Functions

Conceptual API:

```rust
eval_node(vm: &mut VM, node_id: NodeId, ctx: &mut ExecutionContext) -> VmResult<VmControl>

eval_expr(vm: &mut VM, node_id: NodeId, ctx: &mut ExecutionContext) -> VmResult<Value>

exec_stmt(vm: &mut VM, node_id: NodeId, ctx: &mut ExecutionContext) -> VmResult<VmControl>

call_value(vm: &mut VM, callee: Value, args: CallArgs, ctx: &mut ExecutionContext) -> VmResult<VmControl>
```

### 16.2 Expression Contract

`eval_expr` must return a `Value` or a VM error.

If expression evaluation raises a language error, the evaluator may return either:

```text
VmResult::Ok(VmControl::Raise(error))
```

through a unified control path, or

```text
VmResult::Err(VmError::Language(error))
```

The implementation must choose one consistent convention.

Recommended convention:

```text
language raise is VmControl::Raise
VM structural failure is VmResult::Err
```

### 16.3 Statement Contract

`exec_stmt` returns `VmControl`.

Statement execution may produce:

```text
Normal
Return
Break
Continue
Raise
```

### 16.4 Node Dispatch

Node dispatch should be explicit by SIR node kind.

The VM must reject unknown required node kinds before execution.

### 16.5 Type Contract Checks

Type contract checks are runtime semantics.

Evaluator must check:

```text
let/const declared type
function parameter type
function return type
record field type
enum payload type
list/map contracts where enforced
explicit TypeCheckNode
```

### 16.6 Capability Checks

Evaluator or host boundary must check required capabilities before performing effectful host operation.

Missing capability raises `CapabilityError`.

### 16.7 Source Mapping

Evaluator should preserve current source span for diagnostics.

---

## 17. Validation and Execution Boundary

### 17.1 Required Boundary

The VM must validate SIR before execution.

The minimal VM may initially support a subset, but it must declare that subset.

Executing unvalidated full SIR is invalid.

### 17.2 Prevalidated Runtime Plan

The VM may build a runtime plan after validation.

A runtime plan may include:

```text
node kind cache
binding slot layout
field index layout
enum case index layout
type check cache
region ownership map
module import plan
```

The runtime plan is implementation-private.

It is not public bytecode.

### 17.3 Rejection Rule

If the VM cannot validate or execute required SIR semantics, it must reject the program with diagnostic rather than reinterpret semantics.

---

## 18. Internal Invariants

The VM implementation must preserve:

```text
all BindingId references resolve to binding cells
all ScopeId references map to environment structure or static scope data
all TypeId references map to runtime type descriptors
all ControlRegionId references map to runtime or static region descriptors
all PatternId references map to pattern descriptors
all heap handles are valid or rejected as stale internal errors
all exported bindings are module-scope cells
all read-only view writes are rejected
all capability-gated host operations check capability
```

Violation of these invariants is an implementation bug, not user program behavior.

---

## 19. Minimal Implementation Order

Recommended implementation order:

```text
1. sir crate data model
2. runtime Value and heap handles
3. diagnostics and ErrorObj
4. binding cells and environments
5. frames and call stack
6. region stack
7. literal/binding evaluator
8. declaration evaluator
9. expression evaluator
10. function call evaluator
11. aggregate evaluator
12. control-flow evaluator
13. unwinding evaluator
14. module evaluator
15. capability stubs
```

This order keeps runtime substrate ahead of language node coverage.

---

## 20. Round 1 Completion Criteria

Round 1 is complete when the VM specification defines:

```text
Value representation
heap/handle model
object identity model
aggregate objects
record runtime model
enum runtime model
function/closure model
module object model
binding/environment model
frame/call stack model
region stack model
control result model
runtime error model
read-only view model
resource model
evaluator contracts
validation/execution boundary
internal invariants
```

This document satisfies those criteria at the specification level.

---

## 21. Next Work

Round 2 should define node-by-node SIR execution for:

```text
literals
binding references
let/const
assignment
unary/binary/logical expressions
calls
functions
record construction/access
enum construction
list/map/range operations
format strings
type checks
```

Round 3 should define:

```text
blocks
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
module initialization

```


---

## 22. Performance Architecture Constraints

Round 1 runtime structures are semantic models unless explicitly declared physical.

The following constraints are mandatory:

```text
Value enum is conceptual, not ABI.
ObjectId/ObjRef identity is VM-internal.
Heap handle model must not block moving or tracing GC.
Rc/RefCell may be used only as bootstrap detail.
Hot-path execution must not rely on textual lookup.
BindingId must lower to slots before hot execution.
FieldId must lower to field indices before hot execution.
CaseId must lower to case indices before hot execution.
Region stack must be mappable to safepoint/deopt metadata.
Frame layout must support root enumeration.
Mutation APIs must have future write-barrier insertion points.
```

The runtime substrate must support future:

```text
RuntimePlan
EIR fast interpreter
inline caches
type feedback
shape feedback
baseline JIT
deoptimization
GC safepoints
root maps
frame maps
```

This document's Rust-like types are conceptual unless explicitly designated implementation API.
