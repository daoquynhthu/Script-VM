# Phase 2 · SIR Concrete Semantics · Round 1

Version: 1.0 frozen baseline  
Depends on: Phase 2 IR Framework v0.1  
Depends on: Phase 1 Language Specification v1.0 frozen baseline  
Scope: foundational Semantic IR schema and validation semantics  
Out of scope: concrete expression nodes, concrete statement nodes, full lowering rules, VM execution rules, optimizer IR

---

## 0. Round 1 Scope

This document fills the first concrete layer of the canonical Semantic IR, abbreviated as SIR.

Round 1 defines the structural substrate required before expression and statement nodes can be specified:

1. schema notation
2. canonical field rules
3. ID format
4. IR unit schema
5. header schema
6. version and feature schema
7. source identity schema
8. symbol table schema
9. source map table schema
10. scope graph schema
11. binding table schema
12. type descriptor table schema
13. capability and effect descriptor schema
14. module interface descriptor schema
15. extension section schema
16. validation rules for all above structures

Round 1 does not yet define the full semantics of:

1. expression nodes
2. statement nodes
3. declaration body nodes
4. pattern nodes
5. control-region nodes
6. resource nodes
7. error propagation nodes
8. SIR-to-NIR lowering
9. VM execution

The goal is to freeze enough substrate so that later node semantics can reference stable IDs, tables, scope, binding, type, capability, and interface structures.

---

## 1. Schema Notation

### 1.1 Record Form

SIR schema records are written as:

```text
RecordName {
  field_name: FieldType
  optional_field?: FieldType
  repeated_field: List[FieldType]
}
```

### 1.2 Field Requirement

A field without `?` is required.

A field with `?` is optional.

An optional field may be absent.

An optional field present with `null` is equivalent to absence only where the field explicitly permits `Null`.

### 1.3 Lists

`List[T]` is ordered.

List ordering is semantically relevant unless the field explicitly says canonical sorting is required.

### 1.4 Maps

`Map[K, V]` is keyed.

Canonical encoding of a map must sort keys in deterministic order for digest computation.

### 1.5 Tagged Union

Tagged unions are written:

```text
UnionName =
  | VariantA { ... }
  | VariantB { ... }
```

Every encoded variant must contain a tag field:

```text
kind: "VariantA"
```

### 1.6 Compatibility Field Classes

Every field belongs to one of:

```text
required_semantic
optional_semantic
optional_metadata
extension
```

Rules:

1. Unknown `required_semantic` data invalidates the IR unit.
2. Unknown `optional_semantic` data may be ignored only if explicitly declared ignorable.
3. Unknown `optional_metadata` must not affect execution semantics.
4. Unknown `extension` data follows extension-section rules.

### 1.7 Canonical Encoding Requirement

The canonical SIR schema is abstract.

A concrete encoding may be JSON-like, binary, or implementation-specific.

However, any canonical encoding used for digest computation must preserve:

1. explicit field names
2. deterministic field ordering
3. deterministic map key ordering
4. explicit variant tags
5. exact integer values
6. exact string scalar sequences
7. normalized identifiers
8. stable ID references

No canonical SIR representation may depend on raw host pointer layout or allocation order.

---

## 2. Primitive Schema Types

### 2.1 Scalar Types

The schema uses:

```text
Bool
Int
UInt
String
Bytes
Version
Digest
```

### 2.2 Version

```text
Version {
  major: UInt
  minor: UInt
  patch: UInt
}
```

Version comparison is lexicographic by `major`, then `minor`, then `patch`.

### 2.3 Digest

```text
Digest {
  algorithm: String
  value: Bytes
}
```

Allowed digest algorithms are implementation-defined in this draft, but canonical SIR must record the algorithm name.

A digest algorithm name must be stable and case-sensitive.

### 2.4 NormalizedName

```text
NormalizedName {
  text: String
}
```

`text` must be NFC-normalized.

A `NormalizedName` must not be empty.

### 2.5 QualifiedName

```text
QualifiedName {
  segments: List[SymbolId]
}
```

A qualified name must contain at least one segment.

---

## 3. ID Format

### 3.1 ID Principle

All cross-references in SIR use explicit IDs.

SIR must not use host memory identity.

### 3.2 ID Encoding

All IDs are encoded as typed strings:

```text
<id-class>:<local-ordinal>
```

Examples:

```text
node:1
scope:2
binding:5
type:3
symbol:9
region:4
pattern:7
```

The prefix identifies the ID class.

The ordinal is a positive integer.

Ordinal `0` is reserved and invalid unless a specific ID class defines a sentinel.

### 3.3 ID Classes

SIR defines:

```text
ModuleId
SymbolId
BindingId
ScopeId
NodeId
TypeId
RecordId
EnumId
FieldId
CaseId
FunctionId
BlockId
ControlRegionId
PatternId
EffectId
CapabilityId
DiagnosticId
ExtensionId
SourceFileId
SourceSpanId
InterfaceId
```

### 3.4 ID Uniqueness

IDs are unique within their own class and IR unit.

Two IDs of different classes may share the same ordinal but are not equal.

Example:

```text
binding:1 != node:1
```

### 3.5 Stable vs Local IDs

SIR distinguishes:

```text
local_id
stable_interface_id
```

Local IDs are stable only inside an IR unit.

Stable interface IDs are used in Module Interface Descriptors and must not depend on traversal order.

### 3.6 Stable Interface ID Derivation

A stable interface ID is derived from:

```text
module identity
exported name
descriptor kind
semantic signature
language baseline
interface schema version
```

The exact digest algorithm is not fixed in Round 1, but the derivation inputs are normative.

---

## 4. IR Unit Schema

### 4.1 IRUnit

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

### 4.2 Required Tables

The following tables are required even if empty:

```text
sources
symbols
scopes
bindings
types
capabilities
effects
nodes
patterns
control_regions
interface
```


`patterns` is required as a table, but it may be empty.

`control_regions` is required as a table, but it may be empty only for incomplete diagnostic-mode SIR. Executable or lowerable SIR must contain the required module/body regions.

### 4.3 Body Reference

```text
ModuleBodyRef {
  root_node: NodeId
}
```

Round 1 does not define the node schema for `root_node`.

The root node must be defined by a later SIR node schema.

### 4.4 IR Unit Invariant

An IR unit represents exactly one source module after semantic analysis.

It must not contain multiple independent modules.

Multi-module programs are represented as multiple IR units plus dependency/interface metadata.

---

## 5. Header Schema

### 5.1 IRHeader

```text
IRHeader {
  ir_schema_version: Version
  language_baseline_version: Version
  producer: ProducerDescriptor
  source_digest: Digest
  semantic_digest: Digest
  feature_set: FeatureSet
  required_extensions: List[ExtensionRef]
  optional_extensions: List[ExtensionRef]
  created_at?: String
}
```

### 5.2 ProducerDescriptor

```text
ProducerDescriptor {
  name: String
  version: Version
  build?: String
}
```

The producer descriptor identifies the frontend or compiler component that produced SIR.

It must not be used as a semantic compatibility substitute for schema versioning.

### 5.3 Language Baseline

For this phase, the language baseline must identify Phase 1:

```text
major = 1
minor = 0
patch = 0
```

### 5.4 Semantic Digest

The semantic digest is computed over canonical semantic content.

It must include:

```text
module identity
exported interface shape
declarations
binding graph
type descriptors
capability metadata
effect metadata
body semantics
```

It must not include:

```text
producer name
producer version
created_at timestamp
optional optimizer hints
non-semantic cache metadata
```

Documentation metadata inclusion is determined by the digest profile.

### 5.5 Digest Profile

Future versions may define multiple digest profiles:

```text
semantic-body
semantic-interface
semantic-with-docs
```

Round 1 requires only that the profile be recorded when multiple profiles exist.

---

## 6. Feature Set Schema

### 6.1 FeatureSet

```text
FeatureSet {
  required: List[FeatureDescriptor]
  optional: List[FeatureDescriptor]
}
```

### 6.2 FeatureDescriptor

```text
FeatureDescriptor {
  name: String
  version?: Version
  namespace?: String
}
```

Feature names are case-sensitive.

A feature in `required` must be understood by a runtime attempting to validate or execute the IR unit.

A feature in `optional` may be ignored only if it is metadata-only or explicitly declared ignorable.

### 6.3 Builtin Phase 1 Feature Names

The following feature names are reserved for Phase 1 constructs:

```text
records
enums
modules
type-contracts
pattern-matching
structured-errors
resource-management
capability-effects
test-blocks
readonly-views
format-strings
```

Because Phase 1 is frozen with all these features, an implementation that claims full Phase 1 support should understand all of them.

Feature flags remain useful for partial tooling, staged runtimes, and future evolution.

---

## 7. Module Descriptor Schema

### 7.1 ModuleDescriptor

```text
ModuleDescriptor {
  module_id: ModuleId
  module_name: QualifiedName
  source_root?: String
  visibility: ModuleVisibility
  capability_requirements: List[CapabilityId]
  imports: List[ImportDescriptor]
  exports: List[ExportDescriptor]
}
```

### 7.2 ModuleVisibility

```text
ModuleVisibility =
  | RootModule
  | LibraryModule
  | TestModule
  | InternalModule
```

### 7.3 ImportDescriptor

```text
ImportDescriptor {
  import_id: NodeId
  module_name: QualifiedName
  imported_items: ImportItems
  alias?: SymbolId
  source_span?: SourceSpanId
}
```

```text
ImportItems =
  | WholeModule
  | NamedItems { items: List[NamedImport] }
```

```text
NamedImport {
  exported_name: SymbolId
  local_name: SymbolId
  binding_id: BindingId
}
```

Wildcard imports are not representable in SIR.

### 7.4 ExportDescriptor

```text
ExportDescriptor {
  export_id: NodeId
  exported_name: SymbolId
  binding_id: BindingId
  stable_interface_id: InterfaceId
  source_span?: SourceSpanId
}
```

An export descriptor references a binding.

The binding must exist in module scope.

The export table is sealed after module initialization.

---

## 8. Source Table Schema

### 8.1 SourceTable

```text
SourceTable {
  files: List[SourceFileDescriptor]
  spans: List[SourceSpanDescriptor]
}
```

### 8.2 SourceFileDescriptor

```text
SourceFileDescriptor {
  source_file_id: SourceFileId
  logical_name: String
  physical_path?: String
  encoding: String
  digest: Digest
  line_map_digest?: Digest
}
```

Encoding must be `UTF-8` for Phase 1 source.

### 8.3 SourceSpanDescriptor

```text
SourceSpanDescriptor {
  source_span_id: SourceSpanId
  source_file_id: SourceFileId
  start_line: UInt
  start_column: UInt
  end_line: UInt
  end_column: UInt
}
```

Line and column numbers are one-based.

`end_line:end_column` is exclusive.

### 8.4 Synthetic Source Span

Synthetic nodes may omit direct source spans but should carry origin metadata later.

Round 1 permits a node to have no source span if it is synthetic.

---

## 9. Symbol Table Schema

### 9.1 SymbolTable

```text
SymbolTable {
  symbols: List[SymbolDescriptor]
}
```

### 9.2 SymbolDescriptor

```text
SymbolDescriptor {
  symbol_id: SymbolId
  normalized: NormalizedName
  original_spelling?: String
}
```

### 9.3 Symbol Invariants

1. `normalized.text` must be NFC-normalized.
2. Two symbol descriptors in one table must not have the same normalized text unless they are intentionally interned to the same `SymbolId`.
3. If `original_spelling` exists, it may differ from `normalized.text` only by normalization, not by semantic identity.

---

## 10. Scope Table Schema

### 10.1 ScopeTable

```text
ScopeTable {
  scopes: List[ScopeDescriptor]
}
```

### 10.2 ScopeDescriptor

```text
ScopeDescriptor {
  scope_id: ScopeId
  kind: ScopeKind
  parent?: ScopeId
  owner_node?: NodeId
  bindings: List[BindingId]
  source_span?: SourceSpanId
}
```

### 10.3 ScopeKind

```text
ScopeKind =
  | BuiltinScope
  | ModuleScope
  | BlockScope
  | FunctionScope
  | RecordMemberScope
  | EnumMemberScope
  | MatchCaseScope
  | CatchScope
  | ForIterationScope
  | TestScope
  | PatternScope
```

### 10.4 Scope Invariants

1. There must be exactly one `ModuleScope`.
2. There may be exactly one `BuiltinScope` reference if builtins are represented in the IR unit.
3. `ModuleScope` must not have a parent except optionally `BuiltinScope`.
4. No scope parent chain may contain a cycle.
5. Every binding listed in `bindings` must reference the same `scope_id`.
6. A scope may have zero bindings.

---

## 11. Binding Table Schema

### 11.1 BindingTable

```text
BindingTable {
  bindings: List[BindingDescriptor]
}
```

### 11.2 BindingDescriptor

```text
BindingDescriptor {
  binding_id: BindingId
  symbol_id: SymbolId
  scope_id: ScopeId
  kind: BindingKind
  mutability: BindingMutability
  visibility: BindingVisibility
  type_contract?: TypeId
  initializer_node?: NodeId
  declaration_node?: NodeId
  source_span?: SourceSpanId
  capture?: CaptureDescriptor
  export?: ExportBindingDescriptor
}
```

### 11.3 BindingKind

```text
BindingKind =
  | Let
  | Const
  | Function
  | RecordType
  | EnumType
  | Import
  | Parameter
  | Field
  | Method
  | ForIteration
  | CatchError
  | PatternBinding
  | TestLocal
  | Builtin
```

### 11.4 BindingMutability

```text
BindingMutability =
  | Mutable
  | Immutable
  | FieldMutable
  | ReadOnlyView
```

### 11.5 BindingVisibility

```text
BindingVisibility =
  | Local
  | ModulePrivate
  | Exported
  | Imported
  | Builtin
```

### 11.6 CaptureDescriptor

```text
CaptureDescriptor {
  captured_from: BindingId
  capture_kind: CaptureKind
}
```

```text
CaptureKind =
  | Read
  | Write
  | ReadWrite
```

A captured binding descriptor represents the use of an outer binding inside a nested function or closure-relevant region.

### 11.7 ExportBindingDescriptor

```text
ExportBindingDescriptor {
  exported_name: SymbolId
  interface_id: InterfaceId
}
```

### 11.8 Binding Invariants

1. Binding IDs are unique.
2. A binding's `symbol_id` must exist in the symbol table.
3. A binding's `scope_id` must exist in the scope table.
4. No two bindings in the same scope may use the same symbol unless one is an explicit permitted overload in a future language amendment.
5. Function definitions are immutable bindings.
6. Record and enum definitions are immutable bindings.
7. Imported bindings are immutable.
8. Assignment to an immutable binding is invalid.
9. Field assignment is valid only for field bindings marked `FieldMutable`.
10. Pattern bindings are immutable unless a later language amendment says otherwise.

---

## 12. Type Table Schema

### 12.1 TypeTable

```text
TypeTable {
  types: List[TypeDescriptor]
}
```

### 12.2 TypeDescriptor

```text
TypeDescriptor =
  | BuiltinTypeDescriptor
  | RecordTypeDescriptor
  | EnumTypeDescriptor
  | UnionTypeDescriptor
  | OptionalTypeDescriptor
  | ListTypeDescriptor
  | MapTypeDescriptor
  | FunctionTypeDescriptor
  | AnyTypeDescriptor
  | NeverTypeDescriptor
  | ExtensionTypeDescriptor
```

### 12.3 BuiltinTypeDescriptor

```text
BuiltinTypeDescriptor {
  kind: "Builtin"
  type_id: TypeId
  name: BuiltinTypeName
}
```

```text
BuiltinTypeName =
  | Nil
  | Bool
  | Int
  | Float
  | String
  | List
  | Map
  | Range
  | Function
  | Module
  | Error
  | ReadOnlyView
```

### 12.4 AnyTypeDescriptor

```text
AnyTypeDescriptor {
  kind: "Any"
  type_id: TypeId
}
```

### 12.5 NeverTypeDescriptor

```text
NeverTypeDescriptor {
  kind: "Never"
  type_id: TypeId
}
```

### 12.6 OptionalTypeDescriptor

```text
OptionalTypeDescriptor {
  kind: "Optional"
  type_id: TypeId
  inner: TypeId
}
```

`Optional[T]` is semantically equivalent to `Union[T, Nil]`, but it is preserved distinctly to retain source intent.

### 12.7 UnionTypeDescriptor

```text
UnionTypeDescriptor {
  kind: "Union"
  type_id: TypeId
  members: List[TypeId]
}
```

Union members must be canonicalized.

Canonicalization rules:

1. flatten nested unions
2. remove duplicate members
3. sort members deterministically by canonical type identity
4. if the result has one member, the union is invalid as a union descriptor and should reference the single member directly

### 12.8 ListTypeDescriptor

```text
ListTypeDescriptor {
  kind: "List"
  type_id: TypeId
  element: TypeId
}
```

### 12.9 MapTypeDescriptor

```text
MapTypeDescriptor {
  kind: "Map"
  type_id: TypeId
  key: TypeId
  value: TypeId
}
```

The key type must be compatible with hashability rules.

### 12.10 FunctionTypeDescriptor

```text
FunctionTypeDescriptor {
  kind: "Function"
  type_id: TypeId
  parameters: List[TypeId]
  return_type: TypeId
}
```

### 12.11 RecordTypeDescriptor

```text
RecordTypeDescriptor {
  kind: "Record"
  type_id: TypeId
  record_id: RecordId
  name: SymbolId
  fields: List[RecordFieldDescriptor]
  methods: List[FunctionId]
  source_span?: SourceSpanId
  stable_interface_id?: InterfaceId
}
```

### 12.12 RecordFieldDescriptor

```text
RecordFieldDescriptor {
  field_id: FieldId
  name: SymbolId
  type_contract?: TypeId
  mutability: FieldMutability
  has_default: Bool
  default_node?: NodeId
  source_span?: SourceSpanId
}
```

```text
FieldMutability =
  | Immutable
  | Mutable
```

### 12.13 EnumTypeDescriptor

```text
EnumTypeDescriptor {
  kind: "Enum"
  type_id: TypeId
  enum_id: EnumId
  name: SymbolId
  cases: List[EnumCaseDescriptor]
  source_span?: SourceSpanId
  stable_interface_id?: InterfaceId
}
```

### 12.14 EnumCaseDescriptor

```text
EnumCaseDescriptor {
  case_id: CaseId
  name: SymbolId
  payload: List[EnumPayloadFieldDescriptor]
  source_span?: SourceSpanId
}
```

### 12.15 EnumPayloadFieldDescriptor

```text
EnumPayloadFieldDescriptor {
  field_id: FieldId
  name: SymbolId
  type_contract?: TypeId
  source_span?: SourceSpanId
}
```

### 12.16 ExtensionTypeDescriptor

```text
ExtensionTypeDescriptor {
  kind: "Extension"
  type_id: TypeId
  namespace: String
  name: String
  required_feature?: FeatureDescriptor
  payload?: ExtensionPayloadRef
}
```

### 12.17 Type Invariants

1. Type IDs are unique.
2. Referenced type IDs must exist.
3. Record field names must be unique within one record.
4. Enum case names must be unique within one enum.
5. Enum payload field names must be unique within one case.
6. Map key type must be hashable or must be rejected by validation.
7. `Never` must not be accepted as a runtime value type.
8. `Any` accepts all runtime values.
9. Optional and union descriptors must be canonicalized for stable digest computation.

---

## 13. Capability Table Schema

### 13.1 CapabilityTable

```text
CapabilityTable {
  capabilities: List[CapabilityDescriptor]
}
```

### 13.2 CapabilityDescriptor

```text
CapabilityDescriptor {
  capability_id: CapabilityId
  name: SymbolId
  source_span?: SourceSpanId
}
```

### 13.3 Reserved Capability Names

Reserved names:

```text
fs
net
process
env
clock
random
ffi
```

Unknown capability names are permitted only if represented as implementation or library-defined symbols.

### 13.4 Capability Invariants

1. Capability IDs are unique.
2. Capability names must exist in the symbol table.
3. Duplicate capability names in one module requirement set must be canonicalized or rejected.

---

## 14. Effect Table Schema

### 14.1 EffectTable

```text
EffectTable {
  effects: List[EffectDescriptor]
}
```

### 14.2 EffectDescriptor

```text
EffectDescriptor {
  effect_id: EffectId
  name: SymbolId
  required_capability?: CapabilityId
  source_span?: SourceSpanId
}
```

### 14.3 Effect Invariants

1. Effect IDs are unique.
2. Effect names must exist in the symbol table.
3. If `required_capability` exists, it must reference the capability table.
4. A function effect list must reference valid `EffectId` values.

---

## 15. Module Interface Descriptor Schema

### 15.1 ModuleInterfaceDescriptor

```text
ModuleInterfaceDescriptor {
  interface_id: InterfaceId
  module_id: ModuleId
  interface_schema_version: Version
  language_baseline_version: Version
  exported_bindings: List[InterfaceExportDescriptor]
  exported_records: List[InterfaceRecordDescriptor]
  exported_enums: List[InterfaceEnumDescriptor]
  exported_functions: List[InterfaceFunctionDescriptor]
  exported_consts: List[InterfaceConstDescriptor]
  capability_requirements: List[CapabilityId]
  effect_summary: List[EffectId]
  documentation?: DocumentationBlock
  dependency_interfaces: List[DependencyInterfaceRef]
  interface_digest: Digest
}
```

### 15.2 InterfaceExportDescriptor

```text
InterfaceExportDescriptor {
  exported_name: SymbolId
  binding_kind: BindingKind
  binding_id: BindingId
  stable_interface_id: InterfaceId
  type_contract?: TypeId
  documentation?: DocumentationBlock
}
```

### 15.3 InterfaceFunctionDescriptor

```text
InterfaceFunctionDescriptor {
  function_id: FunctionId
  exported_name: SymbolId
  stable_interface_id: InterfaceId
  parameters: List[InterfaceParameterDescriptor]
  return_type?: TypeId
  effects: List[EffectId]
  required_capabilities: List[CapabilityId]
  has_body: Bool
  documentation?: DocumentationBlock
}
```

### 15.4 InterfaceParameterDescriptor

```text
InterfaceParameterDescriptor {
  name: SymbolId
  type_contract?: TypeId
  has_default: Bool
}
```

Default expression bodies are not part of the interface descriptor.

Only default presence is recorded.

### 15.5 InterfaceRecordDescriptor

```text
InterfaceRecordDescriptor {
  record_id: RecordId
  exported_name: SymbolId
  stable_interface_id: InterfaceId
  fields: List[InterfaceRecordFieldDescriptor]
  methods: List[InterfaceFunctionDescriptor]
  documentation?: DocumentationBlock
}
```

### 15.6 InterfaceRecordFieldDescriptor

```text
InterfaceRecordFieldDescriptor {
  field_id: FieldId
  name: SymbolId
  mutability: FieldMutability
  type_contract?: TypeId
  has_default: Bool
}
```

### 15.7 InterfaceEnumDescriptor

```text
InterfaceEnumDescriptor {
  enum_id: EnumId
  exported_name: SymbolId
  stable_interface_id: InterfaceId
  cases: List[InterfaceEnumCaseDescriptor]
  documentation?: DocumentationBlock
}
```

### 15.8 InterfaceEnumCaseDescriptor

```text
InterfaceEnumCaseDescriptor {
  case_id: CaseId
  name: SymbolId
  payload: List[InterfaceEnumPayloadFieldDescriptor]
}
```

### 15.9 InterfaceEnumPayloadFieldDescriptor

```text
InterfaceEnumPayloadFieldDescriptor {
  field_id: FieldId
  name: SymbolId
  type_contract?: TypeId
}
```

### 15.10 InterfaceConstDescriptor

```text
InterfaceConstDescriptor {
  binding_id: BindingId
  exported_name: SymbolId
  stable_interface_id: InterfaceId
  type_contract?: TypeId
  value_digest?: Digest
  documentation?: DocumentationBlock
}
```

Constant values need not be embedded in the interface descriptor unless the implementation supports cross-module constant folding.

### 15.11 DependencyInterfaceRef

```text
DependencyInterfaceRef {
  module_name: QualifiedName
  interface_digest: Digest
  required_exports: List[SymbolId]
}
```

### 15.12 DocumentationBlock

```text
DocumentationBlock {
  text: String
  source_span?: SourceSpanId
}
```

### 15.13 Interface Invariants

1. Exported names must be unique.
2. Every exported binding must exist in the binding table.
3. Every exported record must have an exported binding or be reachable through one.
4. Every exported enum must have an exported binding or be reachable through one.
5. Exported function descriptors must not contain executable bodies.
6. Interface digest must be computed from exported semantic shape, not local allocation order.
7. Adding an enum case is breaking unless a future non-exhaustive enum feature is introduced.
8. Tightening a type contract is breaking.
9. Removing a default argument is breaking.
10. Adding a new required capability is breaking.

---

## 16. Extension Table Schema

### 16.1 ExtensionTable

```text
ExtensionTable {
  extensions: List[ExtensionSection]
}
```

### 16.2 ExtensionSection

```text
ExtensionSection {
  extension_id: ExtensionId
  namespace: String
  name: String
  version?: Version
  required: Bool
  payload: Bytes
  payload_encoding: String
}
```

### 16.3 Extension Rules

1. Unknown required extensions invalidate validation.
2. Unknown optional extensions may be ignored.
3. Optional extensions must not affect required execution semantics.
4. Required extensions must be named in the IR header.
5. Optional extensions must be named in the IR header if their absence may affect diagnostics or tooling.

---

## 17. Diagnostic Table Schema

### 17.1 DiagnosticTable

```text
DiagnosticTable {
  diagnostics: List[DiagnosticDescriptor]
}
```

### 17.2 DiagnosticDescriptor

```text
DiagnosticDescriptor {
  diagnostic_id: DiagnosticId
  severity: DiagnosticSeverity
  code: String
  message: String
  source_span?: SourceSpanId
  related_spans: List[SourceSpanId]
  node_id?: NodeId
  phase: DiagnosticPhase
}
```

### 17.3 DiagnosticSeverity

```text
DiagnosticSeverity =
  | Info
  | Warning
  | Error
  | Fatal
```

### 17.4 DiagnosticPhase

```text
DiagnosticPhase =
  | Lexing
  | Parsing
  | SemanticAnalysis
  | IRValidation
  | IRLowering
  | Runtime
```

### 17.5 Diagnostic Rule

A canonical SIR unit must not contain unresolved fatal diagnostics and still be considered executable.

A SIR unit may preserve non-fatal diagnostics for tooling.

---

## 18. Foundational Validation Rules

### 18.1 Validation Order

A validator should validate in this order:

1. schema version and features
2. extension requirements
3. ID format
4. source table
5. symbol table
6. scope table
7. binding table
8. type table
9. capability and effect tables
10. module descriptor
11. module interface descriptor
12. body root reference
13. cross-table invariants

### 18.2 Schema Validation

Validation must reject:

```text
missing required fields
unknown required fields
unknown required variant tags
invalid field type
invalid ID class
malformed version
malformed digest
```

### 18.3 Reference Validation

Validation must reject:

```text
dangling SymbolId
dangling BindingId
dangling ScopeId
dangling TypeId
dangling NodeId where required
dangling CapabilityId
dangling EffectId
dangling InterfaceId
dangling SourceSpanId
```

### 18.4 Scope Validation

Validation must reject:

```text
scope parent cycles
multiple module scopes
binding listed in wrong scope
duplicate symbol binding in same scope
invalid parent for builtin scope
invalid parent for module scope
```

### 18.5 Binding Validation

Validation must reject:

```text
assignment target referring to immutable binding
field mutation of immutable field
duplicate binding in same scope
binding with missing symbol
binding with missing type contract reference
invalid capture reference
exported binding not in module scope
```

Concrete assignment nodes are defined later, but the binding table must already carry enough mutability data to support this validation.

### 18.6 Type Validation

Validation must reject:

```text
unknown type descriptor kind
union with fewer than two members
non-canonical union descriptor
map with non-hashable key contract
duplicate record fields
duplicate enum cases
duplicate enum payload field names
dangling type references
Never used as accepted runtime value contract outside valid positions
```

### 18.7 Interface Validation

Validation must reject:

```text
duplicate exported names
export descriptor without binding
exported function body embedded in interface descriptor
record interface with duplicate fields
enum interface with duplicate cases
interface digest missing
dependency interface digest missing
stricter dependency interface than declared support
```

### 18.8 Capability/Effect Validation

Validation must reject:

```text
effect referencing missing capability
foreign access without required ffi capability
module requiring unknown capability in strict mode
function effect metadata referencing missing effect
```

### 18.9 Extension Validation

Validation must reject:

```text
unknown required extension
required extension not listed in header
extension ID collision
extension payload encoding missing
```

---

## 19. Backward Compatibility Rules for Round 1 Structures

### 19.1 Safe Additions

The following are safe minor-version additions if feature-gated or optional:

```text
new optional metadata field
new diagnostic severity if readers may treat it as Warning
new optional extension section
new optional documentation metadata
new optional digest profile
new optional interface metadata
```

### 19.2 Compatibility-Sensitive Additions

The following require explicit compatibility review:

```text
new binding kind
new scope kind
new type descriptor kind
new capability semantics
new effect semantics
new interface descriptor category
new required validation invariant
```

These may be minor-version additions only if older runtimes can reject via feature gate rather than misexecute.

### 19.3 Breaking Changes

The following require a major version change:

```text
changing ID semantics
changing binding lookup semantics
changing scope graph semantics
changing type contract meaning
changing interface digest meaning
changing exported descriptor meaning
changing mutability semantics
changing capability enforcement meaning
removing required fields
renaming required fields
```

### 19.4 Interface Evolution Rule

Module interface descriptors must prioritize explicit rejection over guessed compatibility.

If a consumer cannot determine whether an interface change is compatible, it must treat the change as incompatible.

---

## 20. Round 1 Conclusion

Round 1 establishes the concrete substrate for SIR.

The following are now stable enough for later node semantics to reference:

```text
IRUnit
IRHeader
Version
Digest
ID classes
ModuleDescriptor
SourceTable
SymbolTable
ScopeTable
BindingTable
TypeTable
CapabilityTable
EffectTable
NodeTable
PatternTable
ControlRegionTable
ModuleInterfaceDescriptor
ExtensionTable
DiagnosticTable
foundational validation rules
compatibility rules
```

The next concrete IR round should define:

```text
module body nodes
declaration nodes
binding reference nodes
literal expression nodes
basic expression nodes
assignment nodes
function declaration and call nodes
record and enum construction nodes
```
