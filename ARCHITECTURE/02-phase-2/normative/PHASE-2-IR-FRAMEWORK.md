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
