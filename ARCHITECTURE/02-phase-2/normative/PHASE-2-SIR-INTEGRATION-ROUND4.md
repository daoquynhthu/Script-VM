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


