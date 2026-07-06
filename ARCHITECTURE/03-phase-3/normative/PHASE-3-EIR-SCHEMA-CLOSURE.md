# Phase 3 · EIR Operation Schema Closure

Document class: Normative specification  
Normative status: This document defines the canonical closed minimal EIR operation and terminator schema for Phase 3 VM specifications.

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
R3: Close EIR operation schema.
```

It resolves blocker:

```text
B-01: EIR instruction schema is not closed.
```

All Phase 3 documents that mention EIR operations MUST conform to this schema.

---

## 1. EIR Boundary

EIR is:

```text
VM-internal
RuntimePlan-dependent
discardable
cacheable only under VM cache compatibility rules
not public bytecode
not package ABI
not native ABI
```

EIR is the canonical fast execution input after SIR lowering and RuntimePlan construction.

---

## 2. EirModule

```text
EirModule {
  eir_version: Version
  source_runtime_plan_digest: Digest
  functions: List<EirFunction>
  constants: ConstantPool
  source_map: EirSourceMap
  safepoints: SafepointTable
  deopt_points: DeoptPointTable
}
```

Validation MUST reject an EIR module whose digest does not match the RuntimePlan used for execution.

---

## 3. EirFunction

```text
EirFunction {
  eir_function_id: EirFunctionId
  function_id?: FunctionId
  module_id: ModuleId
  entry_block: EirBlockId
  blocks: List<EirBlock>
  slot_layout: SlotLayoutRef
  frame_map: FrameMapId
  source_span?: SourceSpanId
}
```

Every function MUST have exactly one entry block.

---

## 4. EirBlock

```text
EirBlock {
  block_id: EirBlockId
  parameters: List<BlockParameter>
  ops: List<EirOp>
  terminator: EirTerminator
  source_span?: SourceSpanId
}
```

Blocks MUST NOT fall through.

A block MUST have exactly one terminator.

---

## 5. Common Operation Fields

Every EIR operation has:

```text
op_id?: EirOpId
source_span?: SourceSpanId
debug_origin?: NodeId
```

Any operation that may raise a LanguageError MUST have a source span or a source mapping through `debug_origin`.

---

## 6. EirOp Union

```text
EirOp =
  | ConstantOp
  | LoadOp
  | StoreOp
  | UnaryOp
  | BinaryOp
  | LogicalOp
  | CheckOp
  | CallOp
  | AccessOp
  | ConstructOp
  | PatternOp
  | RuntimeHelperOp
  | SafepointOp
  | GuardOp
  | DebugOp
```

No other EIR op kind is allowed in Phase 3 minimal EIR.

Unknown op kinds MUST be rejected by EIR validation.

---

## 7. ConstantOp

```text
ConstantOp {
  dest: SlotId
  constant: ConstantId
}
```

Validation:

```text
dest resolves to writable temporary/local slot
constant resolves in ConstantPool
```

---

## 8. LoadOp

```text
LoadOp =
  | LoadSlot
  | LoadCell
  | LoadCapture
  | LoadModuleSlot
  | LoadField
  | LoadEnumPayload
```

### 8.1 LoadSlot

```text
LoadSlot {
  dest: SlotId
  source: SlotId
  require_initialized: Bool
}
```

### 8.2 LoadCell

```text
LoadCell {
  dest: SlotId
  cell_slot: SlotId
  binding_id?: BindingId
}
```

### 8.3 LoadCapture

```text
LoadCapture {
  dest: SlotId
  capture_index: UInt
  binding_id?: BindingId
}
```

### 8.4 LoadModuleSlot

```text
LoadModuleSlot {
  dest: SlotId
  module_id: ModuleId
  module_slot: SlotId
  binding_id?: BindingId
}
```

### 8.5 LoadField

```text
LoadField {
  dest: SlotId
  receiver: SlotId
  record_shape: ShapeId
  field_id: FieldId
  field_index: FieldIndex
  access_site_id: AccessSiteId
}
```

### 8.6 LoadEnumPayload

```text
LoadEnumPayload {
  dest: SlotId
  receiver: SlotId
  enum_shape: ShapeId
  case_id: CaseId
  case_index: CaseIndex
  payload_index: UInt
}
```

---

## 9. StoreOp

```text
StoreOp =
  | StoreSlot
  | StoreCell
  | StoreModuleSlot
  | StoreField
  | StoreListIndex
  | StoreMapEntry
```

### 9.1 StoreSlot

```text
StoreSlot {
  dest: SlotId
  value: SlotId
  check_initialized?: Bool
}
```

### 9.2 StoreCell

```text
StoreCell {
  cell_slot: SlotId
  value: SlotId
  binding_id?: BindingId
  check_mutability: Bool
  type_check?: TypeId
}
```

### 9.3 StoreModuleSlot

```text
StoreModuleSlot {
  module_id: ModuleId
  module_slot: SlotId
  value: SlotId
  binding_id?: BindingId
  check_mutability: Bool
  type_check?: TypeId
}
```

### 9.4 StoreField

```text
StoreField {
  receiver: SlotId
  value: SlotId
  record_shape: ShapeId
  field_id: FieldId
  field_index: FieldIndex
  access_site_id: AccessSiteId
  check_readonly: Bool
  check_mutability: Bool
  type_check?: TypeId
}
```

### 9.5 StoreListIndex

```text
StoreListIndex {
  receiver: SlotId
  index: SlotId
  value: SlotId
  access_site_id: AccessSiteId
  check_readonly: Bool
}
```

### 9.6 StoreMapEntry

```text
StoreMapEntry {
  receiver: SlotId
  key: SlotId
  value: SlotId
  access_site_id: AccessSiteId
  check_readonly: Bool
  check_hashable: Bool
}
```

All heap-reference mutation paths MUST route through write barrier policy.

---

## 10. UnaryOp

```text
UnaryOp {
  dest: SlotId
  op: UnaryOperator
  operand: SlotId
}
```

```text
UnaryOperator =
  | Plus
  | Minus
  | Not
```

`Not` MUST require Bool operand.

---

## 11. BinaryOp

```text
BinaryOp {
  dest: SlotId
  op: BinaryOperator
  left: SlotId
  right: SlotId
  overflow_policy?: NumericOverflowPolicy
}
```

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
  | Contains
```

No implicit coercion is allowed.

---

## 12. LogicalOp

Logical operations SHOULD lower to branches.

If retained as EIR op:

```text
LogicalOp {
  dest: SlotId
  op: LogicalOperator
  left_block: EirBlockId
  right_block: EirBlockId
  merge_block: EirBlockId
}
```

```text
LogicalOperator =
  | And
  | Or
```

A simple eager boolean binary op MUST NOT replace short-circuit semantics.

---

## 13. CheckOp

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

Every CheckOp defines:

```text
failure_error: RuntimeErrorCode
```

Semantic check failure raises a LanguageError, not deopt, unless the check is explicitly speculative GuardOp.

---

## 14. CallOp

```text
CallOp {
  dest?: SlotId
  callee: SlotId
  positional_args: List<SlotId>
  named_args: List<NamedArgumentSlot>
  call_site_id: CallSiteId
  result_type_check?: TypeId
  call_kind: CallKind
}
```

```text
CallKind =
  | Generic
  | KnownFunction
  | KnownBuiltin
  | RecordConstructor
  | EnumConstructor
  | BoundMethod
  | HostFunction
```

Argument evaluation MUST already be represented in prior EIR ops in source order.

---

## 15. AccessOp

```text
AccessOp =
  | AttributeRead
  | AttributeWrite
  | MethodRead
  | IndexRead
  | IndexWrite
  | SliceRead
```

AccessOps MUST include `AccessSiteId`.

Generic or megamorphic access MAY lower to RuntimeHelperOp.

---

## 16. ConstructOp

```text
ConstructOp =
  | ConstructList
  | ConstructMap
  | ConstructRecord
  | ConstructEnumValue
  | ConstructFunction
  | ConstructError
```

Construction operations that may allocate MUST be safepoint-compatible and root-map-compatible.

---

## 17. PatternOp

```text
PatternOp =
  | PatternCheckLiteral
  | PatternCheckRecordShape
  | PatternCheckEnumCase
  | PatternCheckListLength
  | PatternCheckMapKey
  | PatternBind
  | PatternBranch
  | PatternCommit
  | PatternRollback
```

Pattern operations MUST distinguish:

```text
MatchCase failure
DestructuringDeclaration failure
```

---

## 18. RuntimeHelperOp

```text
RuntimeHelperOp {
  dest?: SlotId
  helper_id: RuntimeHelperId
  args: List<SlotId>
  call_site?: CallSiteId
  access_site?: AccessSiteId
  safepoint_id?: SafepointId
  deopt_id?: DeoptId
}
```

Validation MUST ensure helper_id resolves in the canonical RuntimeHelperRegistry.

---

## 19. SafepointOp

```text
SafepointOp {
  safepoint_id: SafepointId
  kind: SafepointKind
}
```

The referenced SafepointRecord MUST have a valid RootMap when GC can run.

---

## 20. GuardOp

```text
GuardOp {
  guard_kind: GuardKind
  inputs: List<SlotId>
  on_failure: GuardFailureAction
  deopt_id?: DeoptId
  helper_id?: RuntimeHelperId
  failure_error?: RuntimeErrorCode
}
```

```text
GuardFailureAction =
  | Raise
  | HelperFallback
  | Deopt
  | Branch
```

Every guard MUST have a failure action.

---

## 21. DebugOp

```text
DebugOp {
  debug_kind: DebugKind
  source_span?: SourceSpanId
}
```

DebugOp MUST NOT change source-visible semantics.

---

## 22. EirTerminator Union

```text
EirTerminator =
  | Jump
  | Branch
  | Return
  | Raise
  | LoopBackedge
  | Switch
  | Unwind
  | Unreachable
```

### 22.1 Jump

```text
Jump {
  target: EirBlockId
  args: List<SlotId>
}
```

Block argument transfer MUST be simultaneous.

### 22.2 Branch

```text
Branch {
  condition: SlotId
  then_block: EirBlockId
  else_block: EirBlockId
}
```

Condition MUST be Bool.

### 22.3 Return

```text
Return {
  value?: SlotId
}
```

If active cleanup exists, Return MUST enter PendingControl/unwind path.

### 22.4 Raise

```text
Raise {
  error: SlotId
}
```

Non-Error raise MUST raise TypeError.

### 22.5 LoopBackedge

```text
LoopBackedge {
  target: EirBlockId
  args: List<SlotId>
  safepoint_id: SafepointId
}
```

### 22.6 Switch

```text
Switch {
  scrutinee: SlotId
  cases: List<SwitchCase>
  default: EirBlockId
}
```

### 22.7 Unwind

```text
Unwind {
  pending_control_slot: SlotId
  target_region?: ControlRegionId
}
```

### 22.8 Unreachable

```text
Unreachable {
  reason: String
}
```

Reaching Unreachable is `InternalVMError` or VmStructuralError.

---

## 23. Validation Requirements

EIR validation MUST reject:

```text
unknown op kind
unknown terminator kind
unknown SlotId
unknown TypeId
unknown ShapeId
unknown FieldId
unknown CaseId
unknown RuntimeHelperId
unknown CallSiteId
unknown AccessSiteId
unknown SafepointId
unknown DeoptId
block without terminator
invalid block argument count
may-raise op without source mapping
may-collect helper without root map
heap write without barrier policy
guard without failure action
unreachable public bytecode marker
```

---

## 24. Audit Tracking

This document completes:

```text
R3
```

It resolves:

```text
B-01
```

It partially supports:

```text
M-14
M-10
M-15
```
