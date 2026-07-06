# Phase 2 · IR Language Framework Specification

Version: 1.0 frozen baseline  
Depends on: Phase 1 High-Level Language Specification, Version 1.0 frozen baseline  
Scope: IR language framework, compatibility model, structural boundaries  
Out of scope for this document: full node-by-node semantics, VM implementation, optimizer implementation, native ABI implementation, parser algorithm

---

## 0. Freeze Status

This document is part of the frozen Phase 2 IR baseline.

Phase 2 is frozen as:

```text
Version: 1.0 frozen baseline
```

After this freeze, SIR semantics, compatibility boundaries, module interface rules, validation requirements, and lowering contracts may change only by explicit amendment.

Allowed amendments:

1. correcting internal contradictions
2. closing schema incompleteness discovered during Phase 3 VM design
3. correcting safety, capability, or foreign-boundary flaws
4. clarifying wording without changing semantics
5. adding non-semantic optional metadata through the defined extension mechanism

Not allowed without reopening Phase 2:

1. changing the source-first/no-public-bytecode boundary
2. changing SIR node semantics
3. changing binding/scope identity semantics
4. changing module interface compatibility semantics
5. introducing CPython/Python ABI compatibility
6. weakening validation, capability, or foreign-boundary requirements

---

## 0. Status

This document begins Phase 2.

Phase 1 is frozen. The IR design must treat the Phase 1 high-level language as the semantic input baseline.

This document does not yet define all concrete IR nodes. It first defines the IR framework: layers, compatibility strategy, identity model, metadata model, validation model, module-interface model, and forward/backward compatibility rules.

Concrete IR semantics will be filled into this framework later.

---

## 1. Core Position

The IR exists, but it is not public bytecode.

The language remains source-first.

The IR is an internal semantic representation used by the implementation for:

1. preserving language semantics after parsing
2. making semantic checks explicit
3. enabling deterministic execution by the VM
4. supporting diagnostics and source mapping
5. representing module interfaces
6. enabling future optimization without changing the high-level language
7. supporting cache invalidation and compatibility checks

The IR must not become:

1. a public bytecode distribution format
2. a CPython-style extension ABI
3. a JVM/CLR-style portable instruction set
4. a user-facing compilation artifact
5. a stable native ABI
6. a mandatory package format
7. an exposed VM object layout contract

However, the IR must still be designed with controlled compatibility. Otherwise cached IR, tools, module interfaces, future native boundaries, and cross-version execution will become fragile.

The design therefore distinguishes:

```text
not public bytecode
but versioned internal schema

not native ABI
but stable module interface descriptors

not external compiler target
but structured enough for tooling and migration

not user-facing artifact
but compatible enough for cache and ecosystem evolution
```

---

## 2. Compatibility Philosophy

### 2.1 Compatibility Must Be Layered

There is no single universal compatibility boundary.

The system defines different compatibility tiers:

```text
Tier 0: Source Language Compatibility
Tier 1: Canonical Semantic IR Compatibility
Tier 2: Module Interface Descriptor Compatibility
Tier 3: Executable IR Compatibility
Tier 4: Optimizer IR Compatibility
Tier 5: Foreign / Native Boundary Compatibility
```

Each tier has a different stability promise.

### 2.2 Tier 0: Source Language Compatibility

The frozen Phase 1 language is the primary user-facing compatibility layer.

A source program that conforms to Phase 1 should remain meaningful across compatible implementations unless an explicit language amendment says otherwise.

This is the highest semantic authority.

### 2.3 Tier 1: Canonical Semantic IR Compatibility

Canonical Semantic IR is the stable internal semantic layer for a major IR version.

It is allowed to be serialized for implementation-private caching and tooling, but it is not a public package artifact.

Compatibility promise:

```text
major version: may break
minor version: additive only
patch version: clarification only
```

A runtime that supports IR schema `M.N` must accept all canonical Semantic IR documents with the same major version and minor version `<= N`, subject to feature flags.

### 2.4 Tier 2: Module Interface Descriptor Compatibility

Module Interface Descriptors are the closest thing to an ABI in this system.

They describe externally visible module shape, not executable code.

They may include:

```text
exported names
exported function signatures
type contracts
record descriptors
enum descriptors
effect metadata
capability requirements
documentation metadata
semantic digests
```

A module interface descriptor is allowed to be stable across runtime versions.

It must not include executable IR bodies.

This tier exists to avoid ABI chaos while preserving the non-bytecode principle.

### 2.5 Tier 3: Executable IR Compatibility

Executable IR is a lowered form suitable for direct interpretation or VM execution.

It may change between runtime versions.

Compatibility promise:

```text
implementation-private
cacheable
discardable
not a package format
not a public ABI
```

If executable IR cache compatibility fails, the implementation must be able to fall back to source or canonical Semantic IR when available.

### 2.6 Tier 4: Optimizer IR Compatibility

Optimizer IR is fully unstable.

It may be graph-based, SSA-based, specialized, profiled, or target-specific.

No compatibility promise is made.

Optimizer IR must not be exposed as a language artifact.

### 2.7 Tier 5: Foreign / Native Boundary Compatibility

Foreign/native compatibility is separate from IR compatibility.

The IR must not expose VM object layout or native pointer identity as its compatibility model.

The language rejects CPython C API, CPython ABI, Python binary wheel, and Python extension module compatibility.

A future native boundary may define its own ABI, but that ABI must be distinct from the IR and must respect the language capability model.

---

## 3. IR Layer Model

The IR system is divided into layers.

```text
AST
  -> SIR: Semantic IR
  -> NIR: Normalized IR
  -> EIR: Executable IR
  -> OIR: Optimizer IR
```

Only SIR is part of the Phase 2 canonical IR language framework.

NIR, EIR, and OIR may be specified later as implementation or VM-facing layers.

### 3.1 AST

AST is syntax-oriented.

AST preserves surface grammar details.

AST is not IR.

AST may include:

```text
tokens
source spelling
parentheses
syntactic trivia
comments
format string raw text
indentation-derived structure
```

AST is not the execution representation.

### 3.2 SIR · Semantic IR

SIR is the canonical Phase 2 IR layer.

It is semantic, source-linked, structured, versioned, and validation-oriented.

It preserves high-level language constructs when preserving them is semantically important.

SIR may represent:

```text
module structure
declarations
bindings
scopes
functions
records
enums
expressions
statements
patterns
type contracts
effects
capabilities
resources
errors
documentation metadata
source spans
```

SIR is not a bytecode instruction stream.

### 3.3 NIR · Normalized IR

NIR is a lower-level semantic form.

It may desugar or normalize:

```text
for loops
match decision structure
destructuring declarations
format strings
use blocks
defer handling
type contract checks
```

NIR is not required to be stable in Phase 2.

NIR must preserve the semantics of SIR.

### 3.4 EIR · Executable IR

EIR is a VM-executable form.

It may introduce explicit control-flow regions, temporary values, closure environments, and runtime checks.

EIR is implementation-private unless later specified.

### 3.5 OIR · Optimizer IR

OIR is any internal optimization representation.

It may include:

```text
SSA
CFG
specialized nodes
type profiles
inline caches
shape checks
deoptimization metadata
native-code lowering state
```

OIR is explicitly outside the compatibility boundary.

---

## 4. IR Unit Model

### 4.1 IR Unit

An IR unit is the top-level container for canonical Semantic IR.

An IR unit represents exactly one source module after semantic analysis.

An IR unit contains:

```text
header
module identity
source identity
language baseline
IR schema version
feature flags
capability requirements
import table
export table
symbol table
type descriptor table
scope graph
declaration body
diagnostic metadata
extension sections
```

### 4.2 Header

Every IR unit has a header.

Required header fields:

```text
ir_schema_version
language_baseline_version
producer_name
producer_version
source_digest
semantic_digest
feature_flags
required_extensions
optional_extensions
```

The header is used for compatibility, validation, and cache invalidation.

### 4.3 Source Identity

Source identity must include at least:

```text
source_path_or_module_name
source_digest
source_encoding
source_line_map_digest
```

The source digest must be computed over normalized source text according to the source normalization rules.

### 4.4 Semantic Digest

The semantic digest represents the semantic content of the IR unit.

It must not depend on irrelevant formatting or comments except where documentation comments are preserved as semantic metadata.

The digest is used for cache and module-interface invalidation.

### 4.5 Feature Flags

Feature flags declare IR-level features required by an IR unit.

Feature flags are divided into:

```text
required
optional
```

A runtime must reject an IR unit containing an unknown required feature.

A runtime may ignore unknown optional features if doing so does not change program semantics.

### 4.6 Extension Sections

Extension sections are namespaced.

Extension section names must use reverse-domain or implementation-qualified namespaces.

Examples:

```text
org.example.profile
org.example.debug
vm.vendor.optimization-hints
```

An extension section must be marked as either required or optional.

Required extensions participate in compatibility checks.

Optional extensions may be ignored.

---

## 5. Versioning Rules

### 5.1 IR Schema Version

IR schema versions use:

```text
major.minor.patch
```

Example:

```text
1.0.0
```

### 5.2 Major Version

A major version change may break compatibility.

Examples of major changes:

```text
changing core node meaning
removing required fields
changing binding identity semantics
changing module interface semantics
changing control-flow effect semantics
```

### 5.3 Minor Version

A minor version change must be additive.

Allowed minor changes:

```text
adding optional fields
adding optional metadata
adding new node kinds gated behind feature flags
adding new extension points
adding new diagnostics
```

A minor version must not change existing node semantics.

### 5.4 Patch Version

A patch version may clarify wording, fix contradictions, or tighten validation where the prior behavior was already invalid.

Patch versions must not add required features.

### 5.5 Unknown Fields

Unknown optional fields must be ignored.

Unknown required fields must cause validation failure.

This rule requires every field extension to declare whether it is optional or required.

### 5.6 Node Kind Compatibility

New node kinds must be feature-gated.

An older runtime encountering an unknown required node kind must reject the IR unit.

A tool that only reads metadata may skip unknown executable bodies if it is not attempting execution.

### 5.7 No Positional Binary ABI

The canonical IR schema must not depend on raw positional binary layout.

Field names, tags, IDs, and schema versions define structure.

This allows forward-compatible parsing and migration.

---

## 6. Identity Model

### 6.1 Identity Must Not Be Pointer Identity

IR identity must never depend on host memory addresses.

All IR references use explicit IDs.

### 6.2 ID Classes

The framework defines these ID classes:

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
EffectId
CapabilityId
DiagnosticId
ExtensionId
```

IDs are stable within an IR unit.

IDs are not globally stable unless explicitly namespaced.

### 6.3 SymbolId

A `SymbolId` identifies an interned textual symbol.

Symbols include:

```text
names
field names
case names
module path segments
capability names
effect names
diagnostic codes
```

### 6.4 BindingId

A `BindingId` identifies a declared binding.

A binding is distinct from its textual name.

Shadowed names produce different `BindingId` values.

### 6.5 ScopeId

A `ScopeId` identifies a lexical scope.

Scopes form a parent-linked graph.

Block scope, function scope, module scope, catch scope, test scope, and pattern scope must be representable.

### 6.6 NodeId

A `NodeId` identifies an IR node.

Every semantically relevant IR node should have a `NodeId`.

Node IDs support diagnostics, source mapping, profiling, validation, and future incremental compilation.

### 6.7 TypeId

A `TypeId` identifies a type descriptor in the IR unit.

Type descriptors include builtin types, record types, enum types, union types, optional types, list/map/function contracts, and future extensions.

### 6.8 Stable Interface IDs

Module Interface Descriptors must use stable exported IDs derived from names and semantic descriptors, not memory allocation order.

This prevents ABI-like breakage from irrelevant compiler traversal changes.

---

## 7. Source Mapping

### 7.1 Source Span

Every IR node that originates from source text should carry a source span.

A source span contains:

```text
file or module
start line
start column
end line
end column
```

Line and column numbers are one-based.

### 7.2 Synthetic Nodes

Nodes introduced during lowering must be marked as synthetic.

Synthetic nodes should retain origin links to the source node or nodes that caused their creation.

### 7.3 Source Origin Chain

An IR node may carry an origin chain:

```text
source span
AST node origin
SIR node origin
lowering pass origin
```

This supports diagnostics after lowering.

### 7.4 Documentation Metadata

Documentation comments are preserved as declaration metadata.

Documentation metadata participates in module interface descriptors but not necessarily executable semantic digests, unless the implementation chooses to include documentation in interface digests.

---

## 8. Symbol, Binding, and Scope Framework

### 8.1 Symbol Table

The symbol table interns all textual names.

Names are normalized according to Phase 1 identifier normalization rules.

### 8.2 Binding Table

The binding table records all declared bindings.

Each binding descriptor includes:

```text
binding_id
symbol_id
scope_id
binding_kind
mutability
visibility
source_span
type_contract_id
initialization_state
capture_policy
export_status
```

### 8.3 Binding Kinds

Binding kinds include:

```text
let
const
function
record_type
enum_type
import
parameter
field
method
for_iteration
catch_error
pattern_binding
test_local
builtin
```

### 8.4 Mutability

Mutability is represented explicitly:

```text
mutable
immutable
field-mutable
read-only-view
```

Assignment validation must reference binding mutability.

### 8.5 Scope Graph

The scope graph must represent:

```text
module scope
block scope
function scope
record member scope
enum member scope
match case scope
catch scope
test scope
builtin scope
```

Scope lookup is explicit and should not be recomputed from syntax during execution.

### 8.6 Closure Capture

Closure capture is represented through binding references.

A function body that references an outer binding records that captured `BindingId`.

Capture metadata must distinguish:

```text
read capture
write capture
mutable binding capture
immutable binding capture
```

This supports correct closure semantics and future optimization.

---

## 9. Type Contract Framework

### 9.1 Type Contracts Are Runtime Contracts

Phase 1 defines type annotations as runtime-verifiable contracts.

SIR must preserve type contracts.

NIR or EIR may lower contracts into explicit check nodes.

### 9.2 Type Descriptor Table

The IR unit contains a type descriptor table.

Required descriptor categories:

```text
builtin type
record type
enum type
union type
optional type
list contract
map contract
function contract
Any
Never
extension type
```

### 9.3 Contract Sites

Type contracts may appear at:

```text
let declaration
const declaration
parameter
return
record field
enum payload field
list element contract
map key contract
map value contract
function value contract
```

### 9.4 Contract Check Representation

SIR may represent contracts declaratively.

Lowered IR may introduce explicit checks:

```text
CheckType(value, type_contract)
CheckReturn(value, type_contract)
CheckField(value, type_contract)
CheckArgument(value, type_contract)
```

Exact node names are not fixed in this framework.

### 9.5 Compatibility

Adding a new optional type descriptor form requires a minor schema version and a feature flag.

Changing the meaning of an existing type contract requires a major schema version.

---

## 10. Module Interface Descriptor

### 10.1 Purpose

The Module Interface Descriptor is the compatibility boundary for module consumers.

It is not executable code.

It is not bytecode.

It is not native ABI.

It is a stable semantic summary of exported module shape.

### 10.2 Contents

A module interface descriptor contains:

```text
module identity
language baseline
interface schema version
exported binding table
exported function contracts
exported record descriptors
exported enum descriptors
exported const descriptors
exported module aliases
capability requirements
effect metadata
documentation metadata
interface digest
dependency interface digests
```

### 10.3 Exported Function Descriptor

An exported function descriptor includes:

```text
name
parameter names
parameter type contracts
default-argument presence
return type contract
effect metadata
capability metadata
documentation metadata
```

It does not include the function body.

### 10.4 Exported Record Descriptor

An exported record descriptor includes:

```text
record name
field names
field mutability
field type contracts
field default presence
method descriptors
nominal record identity
```

Field order is preserved for diagnostics and constructor display, but external compatibility should depend on field names and field IDs rather than raw position alone.

### 10.5 Exported Enum Descriptor

An exported enum descriptor includes:

```text
enum name
case names
case payload field names
case payload type contracts
nominal enum identity
```

Adding an enum case is a compatibility-sensitive change.

### 10.6 Interface Digest

The interface digest is computed from exported semantic shape.

It must not depend on function bodies unless the exported body is itself part of a future inline or macro interface.

### 10.7 Backward-Compatible Interface Changes

Potentially backward-compatible changes:

```text
adding a new export
adding documentation
adding optional metadata
adding a function overload only if overloads are later supported explicitly
adding a record field only if it has a default and construction compatibility is defined
adding an enum case only if consumers are not assumed exhaustive
```

Since this language currently does not define overloads, adding function overloads is not applicable.

Because pattern matching may assume closed enums, adding an exported enum case should be treated as breaking unless the enum is explicitly declared non-exhaustive by a future amendment.

### 10.8 Breaking Interface Changes

Breaking changes include:

```text
removing an export
renaming an export
changing binding mutability
changing parameter order
removing a default argument
tightening a type contract
changing record field mutability
removing a record field
renaming a record field
removing an enum case
renaming an enum case
changing enum payload shape
changing effect or capability requirements in a stricter direction
```

### 10.9 Module ABI Discipline

The module interface descriptor is the system's ABI-like boundary.

Executable IR bodies are not ABI.

Native object layout is not ABI.

CPython object layout is not ABI.

---

## 11. Effect and Capability Framework

### 11.1 Capability Metadata

Capabilities from Phase 1 must be represented in SIR.

Examples:

```text
fs
net
process
env
clock
random
ffi
```

### 11.2 Requires Clause

A module-level `requires` clause becomes module capability metadata.

### 11.3 Effect Clause

Function-level `effect[...]` declarations become function effect metadata.

### 11.4 Capability Validation

The IR validator must be able to determine:

```text
which module capabilities are required
which function effects are declared
which imports require capabilities
which foreign or host boundary declarations require capability gates
```

### 11.5 Foreign Boundary

Any IR representation of foreign access must carry explicit capability requirements.

The IR must not encode CPython C API assumptions or Python wheel compatibility.

---

## 12. Control-Flow Framework

### 12.1 Structured Control Effects

The IR must distinguish these control effects:

```text
normal
return
break
continue
raise
```

### 12.2 Control Region

Structured control-flow constructs should create regions.

Required conceptual regions:

```text
function region
loop region
try region
catch region
finally region
use region
defer region
match region
test region
block region
```

### 12.3 Unwinding

The IR must explicitly represent or preserve enough structure to derive Phase 1 unwinding order:

```text
defer
use cleanup
finally
pending control flow
suppressed cleanup errors
```

This is mandatory for correctness.

### 12.4 Lowering Policy

SIR may preserve high-level constructs:

```text
Try
Catch
Finally
Use
Defer
Match
For
```

NIR may lower them into explicit control regions.

EIR may lower them further into executable control states.

The first canonical SIR should prefer semantic clarity over low-level control-flow flattening.

---

## 13. Data Model Framework

### 13.1 Literals

IR must represent literals for:

```text
nil
bool
int
float
string
list
map
format string
```

### 13.2 Records

Record definitions must produce record descriptors.

Record construction, field access, field assignment, method declaration, and method call must be representable.

Record fields use stable `FieldId` values.

### 13.3 Enums

Enum definitions must produce enum descriptors.

Enum construction, case identity, payload fields, and match integration must be representable.

Enum cases use stable `CaseId` values.

### 13.4 Lists and Maps

List and map operations must preserve:

```text
element order for lists
insertion order for maps
hashability constraints for keys
mutation operations
read-only view restrictions
```

### 13.5 Read-Only Views

Read-only views must be represented as semantic operations, not as type erasure.

Mutation through a read-only view must remain diagnosable or checkable.

---

## 14. Pattern Framework

### 14.1 Pattern Contexts

Patterns occur in:

```text
match cases
declaration destructuring
```

The IR must distinguish these contexts.

### 14.2 Pattern Kinds

SIR must represent at least:

```text
wildcard
literal
binding
record
enum
list
map
or-pattern
guard
```

### 14.3 Pattern Bindings

Pattern-introduced names become binding descriptors.

A case pattern scope is distinct from the enclosing match scope.

### 14.4 Match Exhaustiveness Metadata

The core language does not require exhaustive checking.

However, IR should preserve enough information for future diagnostics over closed enums.

### 14.5 Or-Pattern Compatibility

Both sides of an or-pattern must bind the same names.

The IR verifier must enforce this.

---

## 15. Resource Framework

### 15.1 Use

A `use` statement represents a resource binding with guaranteed cleanup.

SIR should represent it as a structured resource region.

### 15.2 Defer

A `defer` statement registers a cleanup callable associated with the current block region.

### 15.3 Cleanup Ordering

Cleanup order is semantically relevant and must be preserved.

### 15.4 Suppressed Errors

IR must be capable of representing primary and suppressed errors, even if the first VM version implements a simplified runtime structure.

---

## 16. Diagnostics Framework

### 16.1 Diagnostic Records

Diagnostics should use structured records.

A diagnostic record includes:

```text
diagnostic_id
severity
code
message
source_span
related_spans
node_id
phase
```

### 16.2 Diagnostic Phases

Diagnostic phases include:

```text
lexing
parsing
semantic-analysis
IR-validation
IR-lowering
runtime
```

### 16.3 IR Validation Diagnostics

IR validation may report:

```text
dangling ID reference
invalid scope parent
invalid binding reference
duplicate exported name
invalid type contract
invalid pattern binding
invalid control-flow target
invalid capability reference
invalid module interface descriptor
unknown required feature
schema version mismatch
```

---

## 17. IR Validation Model

### 17.1 Validator Requirement

Every canonical SIR unit must be validatable.

Validation is separate from execution.

### 17.2 Structural Validation

Structural validation checks:

```text
required fields exist
node tags are known or feature-gated
IDs are well-formed
references resolve
tables are internally consistent
extension sections obey required/optional rules
```

### 17.3 Semantic Validation

Semantic validation checks:

```text
scope graph correctness
binding declaration rules
mutability rules
type contract well-formedness
pattern binding rules
control-region rules
module export rules
capability metadata rules
foreign-boundary restrictions
```

### 17.4 Validation Before Execution

A VM must not execute an IR unit that fails required validation.

---

## 18. Cache Compatibility

### 18.1 Cache Is Discardable

IR caches are performance artifacts.

They are not source-of-truth program artifacts.

If a cache is incompatible, stale, or invalid, the implementation must discard it and regenerate IR from source when source is available.

### 18.2 Cache Key

A cache key should include:

```text
source digest
language baseline version
IR schema version
producer version
feature flags
dependency interface digests
capability environment digest
standard library interface digest
```

### 18.3 Cache Invalidation

A cache must be invalidated when:

```text
source changes
language baseline changes incompatibly
IR schema major changes
required feature support changes
dependency interface digest changes
capability environment changes in a relevant way
standard library interface changes incompatibly
```

### 18.4 No Cache ABI Promise

Serialized cached IR is not a package ABI.

It may be implementation-specific.

---

## 19. Canonical Representation Requirements

### 19.1 Keyed Structure

The canonical schema should be defined in terms of keyed fields, not positional binary layout.

This protects forward compatibility.

### 19.2 Stable Ordering

Where ordering affects semantics, order must be explicit.

Examples:

```text
statements in a block
parameters in a function
arguments in a call
fields in source order for diagnostics
enum cases in source order for display
defer registration order
```

Where ordering does not affect semantics, canonical ordering should still be defined for deterministic digest computation.

### 19.3 Deterministic Encoding

A canonical IR representation should be deterministically encodable.

This supports semantic digests, cache keys, reproducible diagnostics, and testing.

### 19.4 Unknown Optional Data

Unknown optional data must not alter semantics.

A reader ignoring optional data must observe the same program behavior.

---

## 20. Node Taxonomy Framework

Concrete node semantics will be defined later.

The framework reserves the following top-level node families:

```text
Module nodes
Declaration nodes
Binding nodes
Type contract nodes
Statement nodes
Expression nodes
Pattern nodes
Control-region nodes
Resource nodes
Error nodes
Module-interface nodes
Metadata nodes
Extension nodes
```

### 20.1 Module Nodes

Represent module identity, imports, exports, capability requirements, and top-level body.

### 20.2 Declaration Nodes

Represent declarations:

```text
let
const
def
record
enum
import
export
test
```

### 20.3 Binding Nodes

Represent binding creation, binding reference, assignment, and capture.

### 20.4 Type Contract Nodes

Represent runtime type contract expressions and contract sites.

### 20.5 Statement Nodes

Represent executable statement semantics.

### 20.6 Expression Nodes

Represent value-producing semantics.

### 20.7 Pattern Nodes

Represent match and destructuring patterns.

### 20.8 Control-Region Nodes

Represent structured control-flow boundaries and unwinding behavior.

### 20.9 Resource Nodes

Represent `use`, `defer`, cleanup ordering, and resource close behavior.

### 20.10 Error Nodes

Represent structured error construction, raising, catching, propagation, and suppressed errors.

### 20.11 Module-Interface Nodes

Represent exported semantic shape.

### 20.12 Metadata Nodes

Represent source spans, docs, effects, capabilities, diagnostics, and debug names.

### 20.13 Extension Nodes

Represent namespaced required or optional extensions.

---

## 21. Backward and Forward Compatibility Rules

### 21.1 Additive Evolution

The IR schema should evolve through additive fields and feature-gated node kinds where possible.

### 21.2 Required Feature Gate

Any semantic addition that changes execution must require a feature gate.

### 21.3 Optional Metadata

Metadata that does not affect execution should be optional.

### 21.4 Deprecation Before Removal

A schema field or node kind should be deprecated before removal.

Removal requires a major version change.

### 21.5 Migration

A newer implementation may provide migration from older SIR to newer SIR.

Migration must preserve semantics.

### 21.6 Rejection Is Better Than Misexecution

If compatibility is uncertain, the runtime must reject the IR unit rather than execute it with guessed semantics.

### 21.7 Interface Stability Over Body Stability

Module interfaces should be more stable than executable bodies.

Changing a function body without changing its exported interface should not invalidate downstream module compatibility, although it may invalidate optimization caches.

---

## 22. Non-Goals

Phase 2 IR framework does not define:

```text
public bytecode
portable bytecode files
CPython C API compatibility
Python wheel compatibility
native object layout ABI
JVM-style verifier
CLR-style metadata system
WASM binary format
optimizer graph format
machine code format
package manager
full standard library ABI
```

---

## 23. Concrete Semantics To Be Filled Later

The following sections are intentionally deferred until after the framework is accepted:

```text
exact SIR node schema
literal node semantics
declaration node semantics
function node semantics
record descriptor schema
enum descriptor schema
module import/export nodes
binding and assignment nodes
call node semantics
attribute/index/slice nodes
match pattern nodes
try/catch/finally nodes
use/defer nodes
type contract check nodes
error propagation nodes
format string nodes
test/assert nodes
module interface descriptor schema details
canonical digest algorithm
```

No implementation should begin from this framework alone without the concrete SIR schema.

---

## 24. Framework Conclusion

The Phase 2 IR must be designed as a semantic compatibility layer, not as public bytecode.

The correct architectural boundary is:

```text
source language: stable user-facing semantics
canonical SIR: versioned internal semantic schema
module interface descriptor: ABI-like exported shape
executable IR: discardable implementation-private execution form
optimizer IR: unstable internal optimization form
foreign ABI: separate capability-gated boundary
```

This preserves three requirements simultaneously:

```text
1. source-first scripting identity
2. future VM and optimizer freedom
3. enough compatibility discipline to avoid ABI chaos
```

---

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

---

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

---

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

---

# Phase 2 · SIR Integration Semantics · Round 4

Version: 1.0 frozen baseline  
Depends on: Phase 2 SIR Concrete Semantics Round 3 v0.4  
Depends on: Phase 1 Language Specification v1.0 frozen baseline  
Scope: module initialization, import/export semantics, validation completeness, SIR-to-NIR lowering contracts, canonical digests, interface compatibility, conformance tests, Phase 2 freeze criteria  
Out of scope: VM instruction set, executable bytecode, optimizer IR, machine code, package manager, native ABI

---

## 0. Round 4 Scope

Round 4 integrates the prior SIR framework and concrete node semantics into a coherent IR-language specification.

It defines:

1. module initialization semantics
2. import/export execution order
3. module dependency graph model
4. circular import handling
5. module interface compatibility checking
6. canonical digest profiles
7. SIR validation completeness
8. SIR-to-NIR lowering contracts
9. allowed lowering transformations
10. forbidden lowering transformations
11. conformance requirements for IR producers
12. conformance requirements for IR consumers
13. compatibility test profiles
14. Phase 2 freeze criteria

Round 4 does not define a concrete NIR schema.

It defines what a valid SIR producer and SIR consumer must preserve when later lowering into NIR, EIR, or VM execution structures.

---

## 1. Integration Principle

SIR is the canonical semantic IR.

SIR must remain:

```text
structured
source-linked
validation-oriented
versioned
digestible
module-interface-aware
safe to reject
not public bytecode
not native ABI
not CPython ABI
```

The integration layer exists to ensure that a SIR unit is not merely a collection of valid nodes, but a coherent module-level semantic object.

A valid SIR unit must satisfy:

```text
schema validity
reference validity
scope validity
binding validity
type validity
node validity
control-flow validity
module-interface validity
dependency validity
capability validity
digest validity
lowering-precondition validity
```

---

## 2. Module State Model

### 2.1 ModuleState

A module at runtime has an initialization state.

```text
ModuleState =
  | Unloaded
  | Loading
  | Initializing
  | Initialized
  | Failed
```

### 2.2 State Meaning

`Unloaded` means the module has not yet been resolved.

`Loading` means the module source or cached representation is being located and validated.

`Initializing` means top-level execution is in progress.

`Initialized` means top-level execution completed successfully and the export table is sealed.

`Failed` means initialization failed with an error.

### 2.3 State Transition

Valid transitions:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
Loading -> Failed
Failed -> Loading
```

The transition `Failed -> Loading` is permitted only if the host explicitly retries module initialization.

A conforming ordinary import does not silently retry a failed module unless the host policy permits it.

### 2.4 Module Instance

A module instance contains:

```text
module_id
module_state
module_scope
export_table
interface_descriptor
initialization_error?
```

### 2.5 Single Initialization Rule

A module initializes at most once per module environment unless the host explicitly creates an isolated environment.

Repeated imports of an initialized module return the same module instance.

---

## 3. Module Dependency Graph

### 3.1 Dependency Graph

The module dependency graph contains nodes for modules and directed edges for imports.

```text
ModuleDependencyGraph {
  modules: List[ModuleId]
  edges: List[ModuleDependencyEdge]
}
```

```text
ModuleDependencyEdge {
  from: ModuleId
  to: ModuleId
  import_descriptor: ImportDescriptor
  required_exports: List[SymbolId]
}
```

### 3.2 Dependency Edge Semantics

An edge `A -> B` means module `A` imports module `B`.

`required_exports` records which exported names from `B` are required by `A`.

For whole-module import, `required_exports` may be empty, because attribute access may occur later.

### 3.3 Dependency Digest

A module's cache key must include dependency interface digests for all statically known imports.

Whole-module imports include the imported module's interface digest.

Named imports include the imported module's interface digest and the imported export names.

### 3.4 Dependency Validation

Validation must reject:

1. import edge to unresolved module unless host resolution is explicitly deferred
2. named import of non-exported name
3. dependency interface digest mismatch
4. dependency requiring stricter capability set than granted
5. dependency cycle that accesses uninitialized export

---

## 4. Import Semantics

### 4.1 Import Resolution

An import resolves a module path to a module identity.

Import resolution is host-defined but must be deterministic within one module environment.

A SIR unit must not assume implicit relative import semantics.

### 4.2 Whole Module Import

For:

```text
import math as m
```

SIR records:

```text
ImportDescriptor {
  imported_items: WholeModule
  alias: m
}
```

Execution binds the module value to the local binding.

### 4.3 Named Import

For:

```text
from math import square as sq
```

SIR records named imports with:

```text
exported_name = square
local_name = sq
binding_id = local import binding
```

Execution binds the exported value to the local immutable import binding.

### 4.4 Import Execution Order

Top-level import declarations execute in source order as part of module initialization.

Before executing a top-level import:

1. resolve module path
2. load or retrieve module instance
3. initialize imported module if needed
4. validate required exports
5. bind imported value or module

### 4.5 Import Binding Mutability

All import bindings are immutable.

Assignment to an import binding is invalid.

### 4.6 Import Failure

Import failure raises an `ImportError` or `ImportCycleError`.

An import failure during module initialization transitions the importing module to `Failed`.

### 4.7 Import Validation

Validation must reject:

1. wildcard import
2. unresolved named export when dependency interface is available
3. duplicate local import binding in same scope
4. import binding marked mutable
5. import descriptor missing local binding
6. import descriptor inconsistent with binding table
7. import requiring unavailable capability when known

---

## 5. Export Semantics

### 5.1 Export Table

A module export table maps exported names to bindings.

```text
ExportTable {
  exports: List[ExportDescriptor]
  sealed: Bool
}
```

### 5.2 Export Execution

Export declarations do not copy values.

An export descriptor exposes a binding from module scope.

When the module finishes initialization, the export table becomes sealed.

### 5.3 Export Timing

An exported binding may be declared before or after its export marker depending on source syntax, but valid SIR must reference a resolved module-scope binding.

Access to an exported binding before it has been initialized during a circular import raises `ImportCycleError`.

### 5.4 Exported Consts

An exported const exposes its binding's value after initialization.

The const binding remains immutable.

### 5.5 Exported Functions

An exported function exposes the function value created during module initialization.

Functions are not hoisted.

If an imported module accesses an exported function before the declaration executed, this is access to an uninitialized export and raises `ImportCycleError`.

### 5.6 Exported Records and Enums

Exported records and enums expose nominal type bindings.

Their descriptors participate in the Module Interface Descriptor.

### 5.7 Export Validation

Validation must reject:

1. export of non-module-scope binding
2. duplicate exported name
3. exported binding missing from binding table
4. exported type missing from type table
5. export table mismatch with Module Interface Descriptor
6. export descriptor with unstable interface ID
7. export table mutation after sealing

---

## 6. Circular Import Semantics

### 6.1 Cycle Policy

Circular imports are permitted only when accessing already-initialized exported bindings.

Accessing an uninitialized export in a cycle raises `ImportCycleError`.

### 6.2 Cycle Example

If `A` imports `B`, and `B` imports `A`, then:

1. `A` enters `Initializing`
2. `A` imports `B`
3. `B` enters `Initializing`
4. `B` imports `A`
5. access to `A` is allowed only for exports already initialized
6. access to not-yet-initialized export raises `ImportCycleError`

### 6.3 SIR Requirement

SIR must preserve source-order top-level execution so that initialization state is meaningful.

An implementation must not reorder top-level declarations across imports unless it proves no observable initialization or export-read behavior changes.

### 6.4 Circular Import Validation

Static validation may detect definite illegal cycles.

Dynamic validation must catch cycles not statically decidable.

---

## 7. Module Interface Compatibility

### 7.1 Interface Compatibility Check

A consumer module can be checked against a provider interface.

```text
InterfaceCompatibilityCheck {
  consumer_module: ModuleId
  provider_module: ModuleId
  required_exports: List[SymbolId]
  expected_interface_digest?: Digest
}
```

### 7.2 Required Export Check

For each required export, the provider interface must contain an export with that name.

### 7.3 Type Contract Check

The provider export must satisfy the consumer's expected type contract.

If exact contract comparison is unavailable, the implementation must reject rather than assume compatibility.

### 7.4 Capability Check

If a provider interface adds a required capability compared to the consumer's recorded dependency, the change is breaking unless the host grants the new capability explicitly and the consumer accepts the updated interface.

### 7.5 Effect Check

If a provider function's declared effects become stricter, the change is breaking.

If effects become weaker, the change is potentially compatible.

### 7.6 Record Compatibility

Breaking record changes:

```text
remove field
rename field
change field mutability
tighten field type contract
remove field default
change nominal record identity
```

Potentially compatible record changes:

```text
add documentation
add optional metadata
add field with default only if constructor compatibility is explicitly accepted
```

By default, adding an exported record field is compatibility-sensitive and should be treated as breaking unless the interface declares constructor compatibility.

### 7.7 Enum Compatibility

Because enums are closed and pattern matching may rely on closure, adding, removing, or renaming an enum case is breaking.

Changing payload shape is breaking.

Changing payload type contract in a stricter direction is breaking.

### 7.8 Function Compatibility

Breaking function changes:

```text
remove parameter
rename parameter if named calls are allowed
change parameter order
remove default
tighten parameter contract
widen return contract in a way consumer rejects
add required capability
add stricter effect
```

Potentially compatible changes:

```text
weaken parameter contract
tighten return contract
add documentation
weaken effects
remove required capability
```

### 7.9 Const Compatibility

Changing an exported const's type contract is compatibility-sensitive.

Changing the value of an exported const is not necessarily interface-breaking unless the value digest is part of the interface profile used by the consumer.

### 7.10 Compatibility Rule

When compatibility cannot be determined, reject.

Misexecution is worse than recompilation or revalidation.

---

## 8. Canonical Digest Profiles

### 8.1 Digest Profiles

Round 4 defines these digest profiles:

```text
source-normalized
sir-semantic
sir-interface
sir-with-docs
sir-cache
```

### 8.2 source-normalized

Digest of normalized source text.

Includes:

```text
UTF-8 normalized text
line terminator normalization
identifier normalization where source normalization applies
```

Excludes:

```text
filesystem path
timestamp
producer metadata
```

### 8.3 sir-semantic

Digest of canonical SIR semantic body.

Includes:

```text
module identity
symbol table normalized names
scope graph
binding table
type table
capability table
effect table
node table semantic fields
control regions
patterns
module body
```

Excludes:

```text
producer metadata
created_at
non-semantic debug names
optional optimizer hints
cache-only metadata
```

### 8.4 sir-interface

Digest of the Module Interface Descriptor.

Includes:

```text
exported names
exported type contracts
record descriptors
enum descriptors
function descriptors
capability requirements
effect summaries
dependency interface digests if profile requires closed-world interface
```

Excludes:

```text
function bodies
local non-exported bindings
local statement bodies
runtime profile metadata
optimizer hints
```

### 8.5 sir-with-docs

Digest of SIR semantic content plus documentation metadata.

Used by documentation tooling.

It must not be used as executable compatibility digest unless a host explicitly chooses documentation-sensitive compatibility.

### 8.6 sir-cache

Digest for implementation cache.

Includes:

```text
sir-semantic digest
producer version
runtime target profile
enabled feature flags
dependency interface digests
capability environment digest
standard library interface digest
lowering profile
```

### 8.7 Digest Determinism

Canonical digest computation must use:

1. deterministic table ordering
2. deterministic map key ordering
3. stable ID normalization
4. canonical union type normalization
5. canonical field ordering
6. explicit digest profile name
7. explicit schema version

### 8.8 Digest Validation

Validation must reject:

1. missing required digest
2. unsupported digest algorithm in strict mode
3. digest profile mismatch
4. interface digest mismatch for dependency
5. cache digest mismatch when executing from cache

---

## 9. SIR Validation Completeness

### 9.1 Validation Levels

SIR validation has levels:

```text
V0 Schema
V1 References
V2 Tables
V3 Node Semantics
V4 Control Flow
V5 Module Interface
V6 Dependency Compatibility
V7 Capability Safety
V8 Lowering Preconditions
```

### 9.2 V0 Schema

Checks raw schema structure.

Rejects malformed records, missing required fields, unknown required variants, and invalid scalar encodings.

### 9.3 V1 References

Checks all IDs and cross-references.

Rejects dangling references and ID-class mismatches.

### 9.4 V2 Tables

Checks source, symbol, scope, binding, type, capability, effect, diagnostic, extension, and control-region tables.

### 9.5 V3 Node Semantics

Checks node-specific invariants.

Examples:

```text
call argument order
duplicate named args
assignment mutability
record constructor fields
enum payload fields
logical operands where statically conclusive
format string parts
```

### 9.6 V4 Control Flow

Checks structured control semantics.

Examples:

```text
return inside function
break/continue inside loop
try/catch/finally structure
defer validity
use cleanup validity
match case scope
pattern binding scope
```

### 9.7 V5 Module Interface

Checks exported shape.

Examples:

```text
exported names unique
export descriptors match bindings
interface descriptors contain no function bodies
interface digest exists
record/enum/function interface descriptors match type and binding tables
```

### 9.8 V6 Dependency Compatibility

Checks imported module interfaces and dependency digests.

Examples:

```text
named import exists
interface digest matches expected
provider capability requirements accepted
provider effect contract compatible
```

### 9.9 V7 Capability Safety

Checks capability-sensitive operations.

Examples:

```text
ffi access requires ffi
fs module requires fs
net module requires net
foreign boundary not ambient
effect metadata references valid capabilities
```

### 9.10 V8 Lowering Preconditions

Checks whether SIR is ready for NIR lowering.

Examples:

```text
all source names resolved
all bindings linked
all scopes linked
all type descriptors canonicalized
control regions well-formed
patterns validated
module dependencies resolved or explicitly deferred
```

### 9.11 Execution Gate

A VM or lowering pipeline must not execute or lower SIR that fails required validation.

A tool may inspect invalid SIR only in diagnostic mode.

---

## 10. SIR-to-NIR Lowering Contract

### 10.1 Purpose

NIR is the normalized semantic IR.

SIR-to-NIR lowering may simplify execution but must preserve Phase 1 semantics.

Round 4 defines lowering contracts, not a concrete NIR schema.

### 10.2 Required Preservation

Lowering must preserve:

```text
evaluation order
binding identity
scope semantics
closure capture
type contract check points
capability/effect metadata
module initialization order
record nominal identity
enum closure
pattern binding semantics
defer/use/finally unwinding order
primary/suppressed error behavior
source-origin chain
diagnostic mapping
```

### 10.3 Allowed Lowerings

Allowed SIR-to-NIR transformations include:

```text
desugar for loops into iterator protocol regions
lower match into decision tree
lower destructuring into pattern checks plus bindings
lower format strings into display conversions and concatenation-like operations
lower use into resource region with explicit close
lower defer into region cleanup stack
lower type annotations into explicit Check nodes
lower chained comparisons into temporaries preserving single evaluation
lower method calls into receiver-bound call form
lower record field access into field ID access
lower enum construction into case constructor form
```

### 10.4 Forbidden Lowerings

Lowering must not:

```text
change left-to-right evaluation order
duplicate side-effecting expressions
evaluate default arguments at function creation time
turn Bool-only conditions into truthiness tests
turn logical operators into operand-returning operators
remove assertion checks in ordinary checked mode
reorder top-level imports across observable initialization
eliminate capability checks
erase type contract checks required by Phase 1
flatten try/finally in a way that changes finally override behavior
drop suppressed cleanup errors when the runtime supports them
turn read-only view mutation into ordinary mutation
treat enum cases as open
treat records as dynamic field maps
expose executable IR as public bytecode
```

### 10.5 Temporary Values

Lowering may introduce temporaries.

Temporaries must:

1. have explicit binding or value IDs
2. not be visible to source-level name lookup
3. preserve source-origin information where relevant
4. not appear in Module Interface Descriptors

### 10.6 Lowering Diagnostics

If lowering fails due to unsupported feature, invalid SIR, capability mismatch, or target limitation, the implementation must produce a structured diagnostic.

It must not silently reinterpret SIR.

---

## 11. NIR Boundary

### 11.1 NIR Is Not Yet Frozen

Round 4 does not freeze NIR.

It only defines obligations that any future NIR must satisfy.

### 11.2 NIR May Introduce

Future NIR may introduce:

```text
explicit temporaries
normalized blocks
control regions
decision trees
explicit cleanup stacks
explicit type-check nodes
explicit capability gates
explicit module initialization states
```

### 11.3 NIR Must Not Introduce

Future NIR must not introduce:

```text
public bytecode ABI
native object layout ABI
CPython ABI dependence
source-invisible semantic changes
unguarded feature assumptions
```

---

## 12. IR Producer Conformance

### 12.1 Producer Definition

An IR producer converts Phase 1 source, AST, or equivalent frontend representation into canonical SIR.

### 12.2 Producer Requirements

A conforming SIR producer must:

1. accept valid Phase 1 source
2. reject invalid Phase 1 source or produce fatal diagnostics
3. resolve names before emitting binding references
4. construct symbol, scope, binding, type, capability, effect, node, pattern, and control-region tables
5. preserve source spans where available
6. emit stable Module Interface Descriptors
7. emit schema versions and feature sets
8. emit required validation metadata
9. avoid CPython/Python ABI assumptions
10. avoid public bytecode emission as the required artifact

### 12.3 Producer Must Not

A producer must not:

```text
emit unresolved textual variable references for normal names
erase block scope
erase type contracts
erase capability requirements
erase record/enum nominal identity
convert conditions to truthiness
hoist functions contrary to Phase 1
evaluate defaults at definition time
expose executable cache as package ABI
```

---

## 13. IR Consumer Conformance

### 13.1 Consumer Definition

An IR consumer reads SIR for validation, tooling, lowering, interpretation, or compilation.

### 13.2 Consumer Profiles

Consumer profiles:

```text
metadata-reader
validator
lowerer
interpreter
compiler
documentation-tool
interface-checker
```

### 13.3 Metadata Reader

A metadata reader may ignore executable bodies if it only reads source spans, documentation, exports, and interface descriptors.

It must reject unknown required extensions affecting the metadata it consumes.

### 13.4 Validator

A validator must implement required validation levels for its declared profile.

A full validator implements V0 through V8.

### 13.5 Lowerer

A lowerer must satisfy SIR-to-NIR lowering contracts.

It must reject unsupported required features.

### 13.6 Interpreter/Compiler

An interpreter or compiler must validate before execution.

It must preserve Phase 1 semantics.

It must not execute invalid SIR.

### 13.7 Interface Checker

An interface checker must compare Module Interface Descriptors and report compatibility or incompatibility.

When uncertain, it must reject compatibility.

---

## 14. Conformance Test Profiles

### 14.1 Test Profile Categories

IR conformance tests should be organized into:

```text
schema tests
reference tests
scope/binding tests
type contract tests
node semantic tests
control-flow tests
module/interface tests
capability tests
digest tests
lowering preservation tests
negative tests
```

### 14.2 Positive Tests

Positive tests contain valid Phase 1 source and expected valid SIR properties.

They should check:

```text
expected binding count
expected scope graph
expected export table
expected type descriptors
expected node kinds
expected control regions
expected module interface descriptor
expected digest stability
```

### 14.3 Negative Tests

Negative tests contain invalid source or malformed SIR.

They should check rejection for:

```text
dangling IDs
duplicate bindings
invalid scope parent
assignment to immutable binding
invalid record constructor
invalid enum case
invalid pattern binding
invalid control target
unresolved import
unknown required feature
invalid digest
capability violation
CPython ABI assumption
```

### 14.4 Round-Trip Tests

Round-trip tests should verify:

```text
source -> SIR -> canonical encoding -> SIR
```

The semantic digest must remain stable.

### 14.5 Lowering Preservation Tests

Lowering tests should verify that SIR-to-NIR preserves:

```text
evaluation order
default argument evaluation time
short-circuit behavior
chained comparison single evaluation
match ordering
defer/use/finally order
module initialization order
```

### 14.6 Interface Compatibility Tests

Interface tests should verify:

```text
adding export compatibility
removing export breakage
record field mutation compatibility
enum case addition breakage
function parameter contract changes
capability requirement changes
effect changes
```

---

## 15. Phase 2 Freeze Criteria

Phase 2 may be frozen when the following criteria are met.

### 15.1 Framework Completeness

The IR framework must define:

```text
IR layer model
compatibility tiers
versioning
feature flags
extension model
ID model
source mapping
module interface boundary
foreign boundary
```

### 15.2 SIR Structural Completeness

SIR must define one final consolidated schema for:

```text
Final consolidated IRUnit schema
IRUnit
Header
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
DiagnosticTable
ExtensionTable
```

### 15.3 SIR Semantic Completeness

SIR must represent all frozen Phase 1 semantics:

```text
declarations
bindings
expressions
assignments
functions
records
enums
containers
format strings
checks
blocks
if/while/for
match/patterns
try/catch/finally
raise
return/break/continue
use/defer
assert/test
modules
imports/exports
capabilities/effects
foreign boundary
```

### 15.4 Validation Completeness

Validation must cover:

```text
schema
references
tables
nodes
control flow
patterns
modules
interfaces
capabilities
effects
digests
lowering preconditions
```

### 15.5 Compatibility Completeness

Compatibility rules must cover:

```text
schema version changes
feature flags
unknown required/optional fields
module interface changes
dependency digest changes
capability changes
effect changes
record changes
enum changes
function changes
cache invalidation
```

### 15.6 Lowering Contract Completeness

The SIR-to-NIR contract must define:

```text
required semantic preservation
allowed lowerings
forbidden lowerings
diagnostic requirements
source-origin preservation
```

### 15.7 No Public Bytecode Violation

Phase 2 must not introduce a public bytecode artifact.

Executable IR and optimizer IR must remain outside the public compatibility boundary.

### 15.8 No CPython ABI Violation

Phase 2 must not depend on:

```text
CPython C API
CPython ABI
Python wheels
Python extension modules
PyObject layout
reference counting semantics
GIL semantics
```

### 15.9 Open Issues Must Be Classified

Before freeze, open issues must be classified as:

```text
blocker
amendment-required
deferred-to-NIR
deferred-to-VM
deferred-to-standard-library
non-goal
```

No blocker may remain unresolved.

---

## 16. Current Open Issues Before Freeze

Round 4 identifies the following non-blocking issues for review.

### 16.1 Exact Canonical Encoding

The schema is defined abstractly.

A concrete canonical encoding is not yet fixed.

Classification:

```text
deferred-to-encoding-profile
```

This is not a blocker for semantic IR design, but it must be fixed before cross-implementation cache sharing.

### 16.2 Digest Algorithm

Digest profiles are defined, but exact hash algorithms are not fixed.

Classification:

```text
deferred-to-encoding-profile
```

### 16.3 NIR Schema

SIR-to-NIR contracts are defined, but NIR node schema is not.

Classification:

```text
deferred-to-NIR
```

### 16.4 Module Resolver

Host module resolution is not specified.

Classification:

```text
deferred-to-module-system
```

### 16.5 Standard Library Interfaces

Reserved module roots exist, but full standard library APIs are not defined.

Classification:

```text
deferred-to-standard-library
```

### 16.6 Native ABI

Foreign boundary constraints are defined, but native ABI is not.

Classification:

```text
deferred-to-foreign-interface
```

---

## 17. Round 4 Conclusion

Round 4 integrates SIR into a coherent canonical semantic IR specification.

The IR system now has:

```text
framework
schema substrate
node semantics
control-flow semantics
module/interface semantics
validation levels
digest profiles
lowering contracts
conformance profiles
freeze criteria
```

After review, Phase 2 is close to freeze.

The recommended next action is:

```text
Phase 2 review pass
  -> contradiction check
  -> schema consistency check
  -> missing Phase 1 coverage check
  -> compatibility check
  -> freeze patch
```

---

## 18. Freeze Patch Resolution

The Phase 2 freeze patch resolves the schema integration issues identified before freeze.

Resolved items:

```text
1. ControlRegionId and PatternId are now canonical ID classes.
2. IRUnit has one final consolidated schema including NodeTable, PatternTable, and ControlRegionTable.
3. SIRNode includes all core node variants through Round 3.
4. Direct embedded node-typed fields were normalized to NodeId references.
5. PatternTable is canonical and required, with empty table allowed.
6. Metadata and extension data are table/header-based, not standalone SIRNode variants.
```

These fixes do not expand IR semantics.

They only consolidate and normalize schemas that had already been introduced across prior rounds.
