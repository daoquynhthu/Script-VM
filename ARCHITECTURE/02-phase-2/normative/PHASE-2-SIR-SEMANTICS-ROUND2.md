# Phase 2 · SIR Concrete Semantics · Round 2

Version: 1.0 frozen baseline  
Depends on: Phase 2 SIR Concrete Semantics Round 1 v0.2  
Depends on: Phase 1 Language Specification v1.0 frozen baseline  
Scope: module body, declaration nodes, binding/reference nodes, basic expression nodes, assignment nodes, functions, records, enums  
Out of scope: full structured control-flow nodes, pattern semantics, try/catch/finally, use/defer lowering, module initialization algorithm, VM execution strategy

---

## 0. Round 2 Scope

Round 2 defines the first executable semantic node families for SIR.

It fills concrete semantics for:

1. universal node envelope
2. module body node
3. declaration nodes
4. binding reference nodes
5. literal expression nodes
6. collection literal nodes
7. unary and binary expression nodes
8. logical expression nodes
9. call expression nodes
10. attribute/index/slice expression nodes
11. assignment and mutation nodes
12. function declaration and function body nodes
13. parameter/default representation
14. closure capture representation at node level
15. record definition/constructor/access/method nodes
16. enum definition/constructor/access nodes
17. format string nodes
18. basic node validation rules

Round 2 deliberately does not yet define:

1. `if`
2. `while`
3. `for`
4. `match`
5. `try`
6. `catch`
7. `finally`
8. `raise`
9. `use`
10. `defer`
11. `test`
12. `assert`
13. detailed error propagation
14. structured unwinding

Those are reserved for Round 3 because they require a unified control-region model.

---

## 1. Universal Node Model

### 1.1 NodeTable

A SIR unit contains a conceptual node table.

Round 1 referenced `NodeId` but did not define the node table. Round 2 defines the canonical node table.

```text
NodeTable {
  nodes: List<SIRNode>
}
```

`IRUnit` is amended to include:

```text
nodes: NodeTable
```

The amended `IRUnit` schema is:

```text
IRUnit {
  header: IRHeader
  module: ModuleDescriptor
  sources: SourceTable
  symbols: SymbolTable
  scopes: ScopeTable
  bindings: BindingTable
  types: TypeTable
  capabilities: CapabilityTable
  effects: EffectTable
  nodes: NodeTable
  patterns: PatternTable
  control_regions: ControlRegionTable
  interface: ModuleInterfaceDescriptor
  body: ModuleBodyRef
  diagnostics?: DiagnosticTable
  extensions?: ExtensionTable
}
```

This is a Round 2 schema amendment and is backward-compatible with Round 1 because Round 1 intentionally deferred node storage.

### 1.2 SIRNode

```text
SIRNode =
  | ModuleBodyNode
  | DeclarationNode
  | BindingNode
  | ExpressionNode
  | AssignmentNode
  | FunctionNode
  | RecordNode
  | EnumNode
  | BlockNode
  | IfNode
  | WhileNode
  | ForNode
  | MatchNode
  | ReturnNode
  | BreakNode
  | ContinueNode
  | RaiseNode
  | TryNode
  | UseNode
  | DeferNode
  | AssertNode
  | TestNode
  | CheckNode
```

Each concrete node variant must include a `NodeHeader`.

Metadata and extension data are table-based or header-attached in canonical SIR. They are not independent `SIRNode` variants unless a future feature-gated schema version defines such variants.

### 1.3 NodeHeader

```text
NodeHeader {
  node_id: NodeId
  node_kind: String
  source_span?: SourceSpanId
  synthetic: Bool
  origin?: OriginDescriptor
  metadata?: NodeMetadata
}
```

### 1.4 OriginDescriptor

```text
OriginDescriptor {
  origin_node?: NodeId
  origin_span?: SourceSpanId
  reason?: String
}
```

Synthetic nodes should carry origin information when they correspond to lowered or normalized source constructs.

### 1.5 NodeMetadata

```text
NodeMetadata {
  documentation?: DocumentationBlock
  effects?: List[EffectId]
  required_capabilities?: List[CapabilityId]
  type_hint?: TypeId
  debug_name?: SymbolId
  extensions?: List[ExtensionRef]
}
```

Metadata must not change semantics unless the metadata field is explicitly defined as semantic metadata.

`effects`, `required_capabilities`, and `type_hint` are semantic metadata.

`documentation` and `debug_name` are non-execution metadata.

### 1.6 Node Invariants

1. Every node has a unique `NodeId`.
2. Every `node_kind` must be known or feature-gated.
3. A non-synthetic source-originating node should have `source_span`.
4. Synthetic nodes may omit `source_span` but should provide `origin`.
5. Any referenced ID must exist in the relevant table.
6. Required semantic metadata must not be ignored by a validator or executor.

---

## 2. Node Category Model

### 2.1 DeclarationNode

```text
DeclarationNode =
  | LetDeclarationNode
  | ConstDeclarationNode
  | FunctionDeclarationNode
  | RecordDeclarationNode
  | EnumDeclarationNode
  | ImportDeclarationNode
  | ExportDeclarationNode
```

### 2.2 BindingNode

```text
BindingNode =
  | BindingReferenceNode
  | BindingCreateNode
  | CaptureReferenceNode
```

### 2.3 ExpressionNode

```text
ExpressionNode =
  | LiteralNode
  | CollectionLiteralNode
  | BindingReferenceNode
  | UnaryExpressionNode
  | BinaryExpressionNode
  | LogicalExpressionNode
  | CallExpressionNode
  | AttributeAccessNode
  | IndexAccessNode
  | SliceExpressionNode
  | FormatStringNode
  | RecordConstructionNode
  | EnumConstructionNode
  | FunctionValueNode
```

### 2.4 AssignmentNode

```text
AssignmentNode =
  | BindingAssignmentNode
  | FieldAssignmentNode
  | IndexAssignmentNode
  | AugmentedAssignmentNode
```

### 2.5 FunctionNode

```text
FunctionNode =
  | FunctionDeclarationNode
  | FunctionValueNode
  | ReturnPlaceholderNode
```

`ReturnPlaceholderNode` is reserved for Round 3. Round 2 defines function structure but not return-control semantics.

### 2.6 RecordNode

```text
RecordNode =
  | RecordDeclarationNode
  | RecordConstructionNode
  | FieldAccessNode
  | FieldAssignmentNode
  | MethodAccessNode
```

### 2.7 EnumNode

```text
EnumNode =
  | EnumDeclarationNode
  | EnumConstructionNode
  | EnumCaseReferenceNode
```

---

## 3. Module Body Node

### 3.1 ModuleBodyNode

```text
ModuleBodyNode {
  header: NodeHeader
  module_scope: ScopeId
  items: List<NodeId>
}
```

`items` are top-level declaration or statement nodes in source order.

Round 2 permits only declarations and expression-like top-level items already defined by Round 2.

Full top-level control statements are defined in later rounds.

### 3.2 Module Body Semantics

A module body represents source-order top-level execution.

It is not a function.

It has a module scope.

Top-level `return`, `break`, and `continue` are invalid at the language level and must not appear in valid SIR.

### 3.3 Module Body Invariants

1. `module_scope` must refer to the unique `ModuleScope`.
2. Every `items` node must exist.
3. `items` ordering is semantically relevant.
4. Top-level declarations that produce bindings must reference bindings in module scope unless nested in a block-like node defined later.

---

## 4. Declaration Nodes

### 4.1 LetDeclarationNode

```text
LetDeclarationNode {
  header: NodeHeader
  binding_id: BindingId
  binding_pattern?: BindingPatternRef
  declared_type?: TypeId
  initializer: NodeId
}
```

### 4.2 Let Semantics

A `let` declaration creates a mutable binding.

The initializer expression is evaluated before the binding becomes initialized.

For destructuring declarations, the initializer value is matched against `binding_pattern`.

If the pattern fails, `PatternMatchError` is raised.

If `declared_type` exists, the initialized value must satisfy the runtime type contract.

### 4.3 ConstDeclarationNode

```text
ConstDeclarationNode {
  header: NodeHeader
  binding_id: BindingId
  binding_pattern?: BindingPatternRef
  declared_type?: TypeId
  initializer: NodeId
}
```

### 4.4 Const Semantics

A `const` declaration creates an immutable binding.

The binding cannot be reassigned after initialization.

The const binding is shallow: the referenced value may still be mutable if its value kind is mutable.

### 4.5 Declaration Pattern Rule

If `binding_pattern` is absent, `binding_id` names the direct binding introduced by the declaration.

If `binding_pattern` is present:

1. `binding_id` may represent a synthetic declaration container binding, or
2. `binding_id` may be omitted by a future schema amendment

Round 2 keeps `binding_id` required for uniform declaration identity, but concrete pattern binding details are deferred to the pattern round.

### 4.6 ImportDeclarationNode

```text
ImportDeclarationNode {
  header: NodeHeader
  import_descriptor: ImportDescriptor
}
```

Import semantics are represented primarily by `ImportDescriptor`.

Round 2 does not define module initialization. It only defines the node linking source order to the import descriptor.

### 4.7 ExportDeclarationNode

```text
ExportDeclarationNode {
  header: NodeHeader
  export_descriptor: ExportDescriptor
}
```

Export nodes reference export descriptors.

An export node does not duplicate exported declaration body semantics.

### 4.8 Declaration Validation

Validation must reject:

1. declaration node referencing missing `BindingId`
2. declaration node initializer referencing missing `NodeId`
3. declared type referencing missing `TypeId`
4. `let` binding whose binding table mutability is not `Mutable`
5. `const` binding whose binding table mutability is not `Immutable`
6. import declaration without corresponding import descriptor
7. export declaration without corresponding export descriptor
8. export descriptor referencing non-module binding

---

## 5. Binding Reference Nodes

### 5.1 BindingReferenceNode

```text
BindingReferenceNode {
  header: NodeHeader
  binding_id: BindingId
  access_kind: BindingAccessKind
}
```

```text
BindingAccessKind =
  | Read
  | WriteTarget
  | CallTarget
  | TypeReference
  | ModuleReference
```

### 5.2 Binding Reference Semantics

A binding reference names a resolved binding.

SIR must not represent unresolved textual lookup for ordinary variable references.

Name resolution has already occurred before SIR.

### 5.3 Read Reference

`Read` evaluates to the runtime value stored in the binding.

Reading an uninitialized binding raises an unbound-binding runtime error.

### 5.4 WriteTarget Reference

`WriteTarget` marks the binding as a target of assignment.

The binding must be mutable.

### 5.5 CallTarget Reference

`CallTarget` marks the binding as a callee source.

The referenced value must be callable at runtime.

### 5.6 TypeReference

`TypeReference` refers to a type-level binding such as record or enum type.

### 5.7 ModuleReference

`ModuleReference` refers to an imported module binding.

### 5.8 CaptureReferenceNode

```text
CaptureReferenceNode {
  header: NodeHeader
  local_binding_id: BindingId
  captured_binding_id: BindingId
  capture_kind: CaptureKind
}
```

A capture reference makes closure capture explicit at node level.

Round 1 already defined capture metadata in binding descriptors. Round 2 adds node-level capture references to support function body analysis.

### 5.9 Binding Reference Validation

Validation must reject:

1. missing binding reference
2. `WriteTarget` to immutable binding
3. `TypeReference` to non-type binding
4. `ModuleReference` to non-module import binding
5. capture reference where `captured_binding_id` is not in an enclosing scope
6. capture kind inconsistent with binding mutability

---

## 6. Literal Nodes

### 6.1 LiteralNode

```text
LiteralNode =
  | NilLiteralNode
  | BoolLiteralNode
  | IntLiteralNode
  | FloatLiteralNode
  | StringLiteralNode
```

### 6.2 NilLiteralNode

```text
NilLiteralNode {
  header: NodeHeader
}
```

Evaluates to the singleton `nil`.

### 6.3 BoolLiteralNode

```text
BoolLiteralNode {
  header: NodeHeader
  value: Bool
}
```

Evaluates to `true` or `false`.

### 6.4 IntLiteralNode

```text
IntLiteralNode {
  header: NodeHeader
  decimal_digits: String
  value_digest?: Digest
}
```

The literal is exact at the language level.

`decimal_digits` stores normalized literal digits without separators.

The implementation may store parsed numeric value in an optional extension, but canonical SIR preserves exact literal text normalization.

### 6.5 FloatLiteralNode

```text
FloatLiteralNode {
  header: NodeHeader
  source_digits: String
  binary64_bits?: Bytes
}
```

A float literal denotes a binary64 value.

`source_digits` stores normalized source spelling.

`binary64_bits` may be present to make parsing deterministic across implementations.

If absent, the implementation must parse according to Phase 1 float rules.

### 6.6 StringLiteralNode

```text
StringLiteralNode {
  header: NodeHeader
  value: String
}
```

`value` stores the decoded Unicode scalar sequence.

Escape spelling is not semantically preserved in SIR except through source mapping.

### 6.7 Literal Validation

Validation must reject:

1. invalid integer digit representation
2. invalid float source representation
3. invalid binary64 encoding when present
4. string values not valid Unicode scalar sequences
5. literal nodes missing required values

---

## 7. Collection Literal Nodes

### 7.1 CollectionLiteralNode

```text
CollectionLiteralNode =
  | ListLiteralNode
  | MapLiteralNode
```

### 7.2 ListLiteralNode

```text
ListLiteralNode {
  header: NodeHeader
  elements: List[NodeId]
  element_type_hint?: TypeId
}
```

Elements are evaluated left to right.

The resulting value is a mutable list.

If `element_type_hint` is present, stored elements must satisfy it when type-contract enforcement applies.

### 7.3 MapLiteralNode

```text
MapLiteralNode {
  header: NodeHeader
  entries: List[MapLiteralEntry]
  key_type_hint?: TypeId
  value_type_hint?: TypeId
}
```

```text
MapLiteralEntry {
  key: NodeId
  value: NodeId
  source_span?: SourceSpanId
}
```

Entries are evaluated left to right.

The resulting map preserves insertion order.

Duplicate keys are resolved by later entries replacing earlier values while preserving the insertion position of the first occurrence unless Phase 1 is amended otherwise.

### 7.4 Map Key Rule

Map literal keys must evaluate to hashable values.

Hashability is runtime-checked unless statically knowable from type contracts.

### 7.5 Collection Validation

Validation must reject:

1. missing element node
2. missing map key node
3. missing map value node
4. key/value type hint referencing missing type
5. statically non-hashable map key type where known

---

## 8. Unary Expression Nodes

### 8.1 UnaryExpressionNode

```text
UnaryExpressionNode {
  header: NodeHeader
  operator: UnaryOperator
  operand: NodeId
}
```

```text
UnaryOperator =
  | Plus
  | Minus
  | Not
```

### 8.2 Unary Semantics

`Plus` and `Minus` require numeric operands.

`Not` requires a Bool operand and returns Bool.

There is no truthiness lowering in SIR.

### 8.3 Unary Validation

Validation must reject missing operand references.

Type invalidity may be left to runtime unless statically knowable from type contracts.

---

## 9. Binary Expression Nodes

### 9.1 BinaryExpressionNode

```text
BinaryExpressionNode {
  header: NodeHeader
  operator: BinaryOperator
  left: NodeId
  right: NodeId
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
  | Membership
```

### 9.2 Binary Semantics

Binary operations follow Phase 1 semantics.

Evaluation order is left-to-right.

`Identity` represents `is`.

`NotIdentity` represents `is not`.

`Membership` represents `in`.

### 9.3 Chained Comparison Representation

Chained comparisons are represented by a distinct node:

```text
ChainedComparisonNode {
  header: NodeHeader
  operands: List[NodeId]
  operators: List[ComparisonOperator]
}
```

```text
ComparisonOperator =
  | Equal
  | NotEqual
  | Less
  | LessEqual
  | Greater
  | GreaterEqual
  | Membership
```

For `n` operands there must be `n - 1` operators.

Chained comparison semantics evaluate each operand exactly once from left to right.

### 9.4 Binary Validation

Validation must reject:

1. missing left or right node
2. unknown operator
3. chained comparison with fewer than two operands
4. chained comparison operator count not equal to operand count minus one

---

## 10. Logical Expression Nodes

### 10.1 LogicalExpressionNode

```text
LogicalExpressionNode {
  header: NodeHeader
  operator: LogicalOperator
  left: NodeId
  right: NodeId
}
```

```text
LogicalOperator =
  | And
  | Or
```

### 10.2 Logical Semantics

Logical operands must be Bool.

Logical nodes short-circuit.

`And` evaluates left; if false, returns false without evaluating right.

`Or` evaluates left; if true, returns true without evaluating right.

Unlike Python, logical operators do not return arbitrary operand values.

### 10.3 Logical Validation

Validation must reject missing operand references.

Static type-contract information may allow early rejection of non-Bool operands.

---

## 11. Call Expression Nodes

### 11.1 CallExpressionNode

```text
CallExpressionNode {
  header: NodeHeader
  callee: NodeId
  arguments: List[CallArgument]
}
```

### 11.2 CallArgument

```text
CallArgument =
  | PositionalArgument
  | NamedArgument
```

```text
PositionalArgument {
  kind: "Positional"
  value: NodeId
  source_span?: SourceSpanId
}
```

```text
NamedArgument {
  kind: "Named"
  name: SymbolId
  value: NodeId
  source_span?: SourceSpanId
}
```

### 11.3 Call Semantics

Evaluation order:

1. evaluate callee
2. evaluate positional arguments left to right
3. evaluate named arguments left to right
4. invoke callable

Positional arguments must precede named arguments in source and in SIR.

Duplicate named arguments are invalid.

### 11.4 Callable Kinds

The callee may evaluate to:

```text
Function
BuiltinFunction
RecordConstructor
Method
EnumCaseConstructor
```

### 11.5 Call Validation

Validation must reject:

1. missing callee node
2. missing argument value node
3. named argument referencing missing symbol
4. positional argument after named argument
5. duplicate named argument
6. statically known non-callable callee where type information is conclusive

---

## 12. Attribute, Index, and Slice Nodes

### 12.1 AttributeAccessNode

```text
AttributeAccessNode {
  header: NodeHeader
  receiver: NodeId
  attribute: SymbolId
  access_kind: AttributeAccessKind
}
```

```text
AttributeAccessKind =
  | Read
  | CallReceiver
  | WriteTarget
```

### 12.2 Attribute Semantics

Attribute access is defined for:

```text
record instances
modules
methods
read-only views where underlying read is valid
```

Dynamic undeclared field access is not represented as a distinct successful operation.

Undefined attributes are runtime errors.

### 12.3 IndexAccessNode

```text
IndexAccessNode {
  header: NodeHeader
  receiver: NodeId
  index: NodeId
  access_kind: IndexAccessKind
}
```

```text
IndexAccessKind =
  | Read
  | WriteTarget
```

Index access is defined for lists and maps.

String indexing is not defined.

### 12.4 SliceExpressionNode

```text
SliceExpressionNode {
  header: NodeHeader
  receiver: NodeId
  start?: NodeId
  end?: NodeId
}
```

Slicing is defined for lists and strings.

Bounds must evaluate to Int.

Negative bounds are runtime errors unless statically rejected earlier.

Out-of-range bounds are errors and are not silently clamped.

### 12.5 Access Validation

Validation must reject:

1. missing receiver node
2. missing attribute symbol
3. missing index node
4. slice with neither start nor end is valid and represents full copy
5. statically known string index access
6. write target through known read-only view where statically known

---

## 13. Assignment and Mutation Nodes

### 13.1 BindingAssignmentNode

```text
BindingAssignmentNode {
  header: NodeHeader
  target: NodeId
  value: NodeId
}
```

The target node must be a `BindingReferenceNode` with access kind `WriteTarget`.

The referenced binding must be mutable.

### 13.2 FieldAssignmentNode

```text
FieldAssignmentNode {
  header: NodeHeader
  target: NodeId
  value: NodeId
}
```

The target node must be an `AttributeAccessNode` with access kind `WriteTarget`.

The resolved field must be mutable.

If the receiver is a read-only view, assignment raises `ReadOnlyError`.

### 13.3 IndexAssignmentNode

```text
IndexAssignmentNode {
  header: NodeHeader
  target: NodeId
  value: NodeId
}
```

The target node must be an `IndexAccessNode` with access kind `WriteTarget`.

For lists, index assignment mutates an existing element.

For maps, index assignment inserts or replaces a key/value entry.

If the receiver is a read-only view, assignment raises `ReadOnlyError`.

### 13.4 AugmentedAssignmentNode

```text
AugmentedAssignmentNode {
  header: NodeHeader
  target: AssignmentTargetRef
  operator: AugmentedOperator
  value: NodeId
}
```

```text
AssignmentTargetRef =
  | BindingTarget { target_node: NodeId }
  | FieldTarget { target_node: NodeId }
  | IndexTarget { target_node: NodeId }
```

```text
AugmentedOperator =
  | AddAssign
  | SubtractAssign
  | MultiplyAssign
  | DivideAssign
  | ModuloAssign
```

### 13.5 Augmented Assignment Semantics

`a += b` evaluates the target only once, evaluates `b`, applies the corresponding binary operator, and writes the result back to the target.

### 13.6 Assignment Validation

Validation must reject:

1. binding assignment target not marked `WriteTarget`
2. binding assignment to immutable binding
3. field assignment target not marked `WriteTarget`
4. field assignment to immutable field when statically known
5. index assignment target not marked `WriteTarget`
6. augmented assignment with invalid target ref
7. augmented assignment with unknown operator

---

## 14. Function Nodes

### 14.1 FunctionDeclarationNode

```text
FunctionDeclarationNode {
  header: NodeHeader
  function_id: FunctionId
  binding_id: BindingId
  name: SymbolId
  parameters: List[ParameterDescriptor]
  return_type?: TypeId
  body: FunctionBody
  function_scope: ScopeId
  captures: List[NodeId]
  effects: List[EffectId]
  required_capabilities: List[CapabilityId]
}
```

### 14.2 ParameterDescriptor

```text
ParameterDescriptor {
  binding_id: BindingId
  name: SymbolId
  type_contract?: TypeId
  default_value?: NodeId
  source_span?: SourceSpanId
}
```

### 14.3 FunctionBody

```text
FunctionBody {
  body_node: NodeId
}
```

Round 2 does not define the internal statement form of the body node. It only requires the body to be a valid node defined by the statement/control-flow rounds.

### 14.4 FunctionValueNode

```text
FunctionValueNode {
  header: NodeHeader
  function_id: FunctionId
}
```

A function declaration creates a function value.

The function value captures its lexical environment according to the capture list.

### 14.5 Function Declaration Semantics

A function declaration creates an immutable binding.

The function binding is initialized when execution reaches the declaration.

Function declarations are not hoisted.

### 14.6 Default Argument Semantics

Default argument nodes are evaluated at call time when the corresponding argument is omitted.

Default nodes belong to the function declaration but are not evaluated at function creation.

Each call evaluates omitted defaults anew.

### 14.7 Function Contract Semantics

Parameter contracts are checked when arguments are bound.

Return contracts are checked when a value is returned.

Round 2 records return contracts but Round 3 defines return node semantics.

### 14.8 Function Validation

Validation must reject:

1. missing function ID
2. missing binding ID
3. function binding not immutable
4. duplicate parameter names
5. parameter binding not in function scope
6. default parameter followed by non-default parameter
7. missing body node
8. function scope not marked `FunctionScope`
9. capture node not referencing a valid `CaptureReferenceNode` for an enclosing binding
10. return type referencing missing `TypeId`
11. effect reference missing from effect table
12. capability reference missing from capability table

---

## 15. Record Nodes

### 15.1 RecordDeclarationNode

```text
RecordDeclarationNode {
  header: NodeHeader
  record_id: RecordId
  type_id: TypeId
  binding_id: BindingId
  name: SymbolId
  fields: List[RecordFieldDescriptor]
  methods: List[NodeId]
  member_scope: ScopeId
}
```

### 15.2 Record Declaration Semantics

A record declaration creates an immutable binding for a nominal fixed-shape record type.

Record fields are fixed after declaration.

No dynamic field addition exists.

### 15.3 RecordConstructionNode

```text
RecordConstructionNode {
  header: NodeHeader
  record_type: TypeId
  arguments: List[RecordFieldInitializer]
}
```

```text
RecordFieldInitializer {
  field_id: FieldId
  name: SymbolId
  value: NodeId
  source_span?: SourceSpanId
}
```

### 15.4 Record Construction Semantics

Record construction requires named field initializers.

All fields without defaults must be initialized.

Fields with defaults may be omitted.

Unknown fields are invalid.

Duplicate fields are invalid.

Field default expressions are evaluated at construction time.

### 15.5 FieldAccessNode

`FieldAccessNode` is semantically represented by `AttributeAccessNode` when the receiver type is known to be a record instance.

A later lowering layer may replace it with a field-specific node.

### 15.6 MethodAccessNode

Method access is represented by `AttributeAccessNode` with `CallReceiver` where the attribute resolves to a record method.

A method call passes the receiver as the first argument.

### 15.7 Record Validation

Validation must reject:

1. missing record type descriptor
2. duplicate field IDs
3. duplicate field names
4. method node not referencing a `FunctionDeclarationNode` in record member scope
5. method first parameter missing where required by method-call semantics
6. construction with unknown field
7. construction missing required field when statically knowable
8. duplicate field initializer
9. initializer referencing missing node
10. field type contract referencing missing type

---

## 16. Enum Nodes

### 16.1 EnumDeclarationNode

```text
EnumDeclarationNode {
  header: NodeHeader
  enum_id: EnumId
  type_id: TypeId
  binding_id: BindingId
  name: SymbolId
  cases: List[EnumCaseDescriptor]
  member_scope: ScopeId
}
```

### 16.2 Enum Declaration Semantics

An enum declaration creates an immutable binding for a nominal closed sum type.

Enum cases are closed.

New cases cannot be added outside the enum declaration.

### 16.3 EnumCaseReferenceNode

```text
EnumCaseReferenceNode {
  header: NodeHeader
  enum_type: TypeId
  case_id: CaseId
}
```

This node references a case constructor.

### 16.4 EnumConstructionNode

```text
EnumConstructionNode {
  header: NodeHeader
  enum_type: TypeId
  case_id: CaseId
  payload: List[EnumPayloadInitializer]
}
```

```text
EnumPayloadInitializer {
  field_id: FieldId
  name: SymbolId
  value: NodeId
  source_span?: SourceSpanId
}
```

### 16.5 Enum Construction Semantics

Enum construction requires named payload field initializers for cases with payload fields.

Cases without payload fields require no payload.

All payload fields must be provided unless a future amendment defines defaults.

Unknown payload fields are invalid.

Duplicate payload fields are invalid.

### 16.6 Enum Validation

Validation must reject:

1. missing enum type descriptor
2. duplicate case IDs
3. duplicate case names
4. duplicate payload field names
5. enum construction with unknown case
6. construction missing required payload field
7. construction with duplicate payload initializer
8. payload initializer referencing missing node
9. payload type contract referencing missing type

---

## 17. Format String Nodes

### 17.1 FormatStringNode

```text
FormatStringNode {
  header: NodeHeader
  parts: List[FormatStringPart]
}
```

### 17.2 FormatStringPart

```text
FormatStringPart =
  | FormatTextPart
  | FormatExpressionPart
```

```text
FormatTextPart {
  kind: "Text"
  text: String
}
```

```text
FormatExpressionPart {
  kind: "Expression"
  expression: NodeId
  source_span?: SourceSpanId
}
```

### 17.3 Format Semantics

Parts are evaluated left to right.

Text parts contribute their literal text.

Expression parts are evaluated and converted through display representation.

The result is a `String`.

### 17.4 Format Validation

Validation must reject:

1. missing expression node
2. invalid Unicode text part
3. empty format string only if source grammar forbids it; Phase 1 does not forbid it

---

## 18. Basic Runtime Check Nodes

### 18.1 CheckNode

Round 2 defines a generic check node family for runtime contracts.

```text
CheckNode =
  | TypeCheckNode
  | CallableCheckNode
  | HashableCheckNode
  | AssignableCheckNode
```

### 18.2 TypeCheckNode

```text
TypeCheckNode {
  header: NodeHeader
  value: NodeId
  type_contract: TypeId
  failure_code: String
}
```

A type check evaluates `value`, verifies it against `type_contract`, and either returns the value unchanged or raises `TypeContractError`.

### 18.3 CallableCheckNode

```text
CallableCheckNode {
  header: NodeHeader
  value: NodeId
}
```

Checks that value is callable.

### 18.4 HashableCheckNode

```text
HashableCheckNode {
  header: NodeHeader
  value: NodeId
}
```

Checks that value may be used as a map key.

### 18.5 AssignableCheckNode

```text
AssignableCheckNode {
  header: NodeHeader
  target: NodeId
}
```

Checks that target is assignable.

Assignable checks are mostly validation-time checks but may remain explicit for dynamic field/index cases.

### 18.6 Check Node Semantics

Check nodes are semantic, not optimizer hints.

A lowering layer may insert them explicitly from declared type contracts.

### 18.7 Check Validation

Validation must reject:

1. missing checked value
2. missing type contract
3. check node referencing invalid target
4. empty failure code

---

## 19. Round 2 Validation Additions

Round 2 extends foundational validation with node-level rules.

### 19.1 Node Table Validation

Validation must reject:

```text
duplicate NodeId
node with unknown kind
node missing header
node header ID mismatch
node referencing missing source span
node referencing missing metadata ID
```

### 19.2 Module Body Validation

Validation must reject:

```text
missing module body root
module body root not a ModuleBodyNode
module body scope not ModuleScope
module body item missing from node table
```

### 19.3 Expression Validation

Validation must reject:

```text
missing child expression
invalid operator
invalid call argument ordering
duplicate named argument
invalid attribute symbol
invalid slice bounds node reference
invalid format expression reference
```

### 19.4 Declaration Validation

Validation must reject:

```text
declaration binding mismatch
declaration scope mismatch
initializer node missing
declared type missing
function parameter binding mismatch
record descriptor mismatch
enum descriptor mismatch
```

### 19.5 Mutation Validation

Validation must reject:

```text
assignment to immutable binding
assignment to immutable field when statically known
assignment through read-only view when statically known
augmented assignment with invalid target
```

### 19.6 Contract Validation

Validation must reject:

```text
type check with missing type contract
return contract on non-function context
field contract mismatch where statically knowable
argument contract mismatch where statically knowable
```

---

## 20. Compatibility Rules for Round 2 Nodes

### 20.1 Safe Minor Additions

The following may be minor-version additions if feature-gated:

```text
new expression node kind
new literal metadata field
new optional operator metadata
new optional call metadata
new optional source-origin detail
new optional validation diagnostic
```

### 20.2 Compatibility-Sensitive Additions

The following require careful feature gating:

```text
new operator
new assignment target kind
new callable kind
new attribute access category
new collection literal kind
new check node kind
```

### 20.3 Breaking Changes

The following require major version changes:

```text
changing evaluation order
changing call argument order
changing default argument evaluation time
changing assignment target evaluation count
changing record construction semantics
changing enum case closure semantics
changing logical operator return type
changing map duplicate key semantics
changing string format expression evaluation order
```

---

## 21. Round 2 Conclusion

Round 2 defines enough concrete SIR nodes to represent:

```text
module bodies
declarations
bindings and references
literals
lists and maps
unary and binary operations
logical short-circuit operations
calls
attribute/index/slice access
assignments
functions
records
enums
format strings
runtime checks
```

The next round should define structured control semantics:

```text
if
while
for
match
patterns
try/catch/finally
raise
return
break
continue
use
defer
test/assert
structured unwinding
error propagation

```


## 22. Freeze Patch Notes

The following schema normalizations are part of the Phase 2 freeze patch:

1. `SIRNode` includes all core node variants defined through Round 3.
2. Metadata and extension data are not independent `SIRNode` variants in canonical SIR.
3. Node relationships use `NodeId` references rather than embedded concrete node values.
4. Assignment targets, capture references, and record methods are referenced by `NodeId` with validation constraining the referenced node kind.

