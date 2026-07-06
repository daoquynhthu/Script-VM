# Phase 3 · Technology Stack Selection

Version: 0.1 draft  
Scope: implementation stack for the minimal VM  
Primary language: Rust

---

## 1. Decision

The VM should be implemented in Rust.

This is the default technical direction for Phase 3.

Rust is selected for:

```text
memory safety
explicit ownership
strong algebraic data types
predictable performance
low-level control without C/C++ unsafety by default
good testing and tooling ecosystem
future native/WASM embedding possibilities
```

Rust is also aligned with the project boundary:

```text
no CPython C API
no Python ABI
no public bytecode ABI
capability-gated host boundary
explicit runtime object model
```

---

## 2. Rust Toolchain Policy

### 2.1 Channel

Use stable Rust by default.

Nightly Rust must not be required for core VM builds.

Nightly may be allowed only for optional tooling experiments.

### 2.2 Edition

Use the current stable Rust edition available at project creation time.

The exact edition should be recorded in `Cargo.toml`.

### 2.3 Formatting

Required:

```text
rustfmt
```

### 2.4 Linting

Required:

```text
clippy
```

Recommended policy:

```text
deny unsafe_op_in_unsafe_fn
deny missing_docs later, not initially
deny warnings in CI after bootstrap stabilizes
```

### 2.5 Unsafe Policy

Initial VM code should avoid `unsafe`.

If `unsafe` becomes necessary, it must be isolated behind small modules with documented invariants.

No `unsafe` should be used for ordinary interpreter logic.

---

## 3. Cargo Workspace

Recommended layout:

```text
crates/
  sir/
  sir_validate/
  vm_core/
  vm_runtime/
  vm_eval/
  vm_diag/
  vm_host/
  vm_cli/
  vm_tests/
```

### 3.1 `sir`

Purpose:

```text
Frozen Phase 2 SIR data model
```

Should contain:

```text
ID newtypes
SIR tables
node enums
type descriptors
module interface descriptor
control region descriptors
pattern descriptors
```

### 3.2 `sir_validate`

Purpose:

```text
SIR validator
```

Should contain validation levels:

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

### 3.3 `vm_core`

Purpose:

```text
VM configuration, execution state, errors, feature support
```

### 3.4 `vm_runtime`

Purpose:

```text
Runtime values and heap objects
```

### 3.5 `vm_eval`

Purpose:

```text
SIR evaluator
```

### 3.6 `vm_diag`

Purpose:

```text
Diagnostics and rendering
```

### 3.7 `vm_host`

Purpose:

```text
Host capabilities and module resolver
```

### 3.8 `vm_cli`

Purpose:

```text
Command-line runner
```

This is optional for early library-only development.

### 3.9 `vm_tests`

Purpose:

```text
Integration and conformance tests
```

---

## 4. Dependency Policy

### 4.1 Principle

Use few dependencies at the VM core.

Dependencies are allowed when they clearly reduce risk, but core runtime semantics should not be delegated to large opaque frameworks.

### 4.2 Acceptable Dependency Categories

Acceptable categories:

```text
error handling
small index maps
arena/generational index storage
serialization for internal test fixtures
snapshot testing
property testing
fuzzing
benchmarking
CLI parsing
```

### 4.3 Avoid Initially

Avoid in the core VM:

```text
large async frameworks
general plugin systems
dynamic loading crates
JIT backends
FFI-heavy crates
macro-heavy framework dependencies
global runtime dependencies
```

### 4.4 Version Pinning

Exact crate versions should be pinned by `Cargo.lock`.

The specification should not depend on specific crate versions.

---

## 5. Suggested Crate Categories

This document names categories rather than hard requirements.

### 5.1 Error Handling

Use a structured error strategy.

Potential category:

```text
thiserror-like derive errors
```

Avoid using generic dynamic errors deep inside VM semantics.

Runtime errors should map to language diagnostic categories.

### 5.2 Index Maps

Useful for deterministic insertion order:

```text
index-map-like ordered maps
```

Relevant for:

```text
module exports
map values
diagnostics
interface descriptors
deterministic tests
```

### 5.3 Arena / Generational Storage

Useful for runtime objects and IDs:

```text
generational arena
slot map
typed index arena
custom arena
```

The design should prefer typed handles over raw pointers.

### 5.4 Serialization

Needed for fixtures and future SIR encoding experiments.

Use only for:

```text
test fixtures
debug dumps
canonical encoding experiments
```

Do not let serialization format become public bytecode.

### 5.5 Testing

Recommended categories:

```text
snapshot testing
property-based testing
fuzz testing
benchmarking
```

### 5.6 CLI

CLI is useful but not central.

Use a small CLI parser category only after the library API exists.

---

## 6. Internal API Principles

### 6.1 Newtype IDs

Use Rust newtypes for SIR IDs:

```rust
struct NodeId(u32);
struct BindingId(u32);
struct ScopeId(u32);
struct TypeId(u32);
struct ControlRegionId(u32);
struct PatternId(u32);
```

Do not use raw strings internally for hot-path ID lookup.

String-form IDs may exist at serialization boundaries.

### 6.2 Typed Tables

Use typed tables:

```rust
struct NodeTable { ... }
struct BindingTable { ... }
struct ScopeTable { ... }
```

Avoid loosely typed maps for core structures.

### 6.3 Explicit Results

Use explicit `Result<T, VmError>`.

Do not use panics for language-level errors.

### 6.4 No Host Exceptions

Rust panics are not language exceptions.

Language `raise`, `return`, `break`, and `continue` must be explicit VM control values.

### 6.5 Stable Diagnostics

Diagnostics should use stable codes.

Human-readable messages may evolve, but diagnostic codes should remain stable after freeze.

---

## 7. Unsafe / FFI / Native Boundary

### 7.1 Unsafe

Initial implementation target:

```text
unsafe-free core VM
```

If `unsafe` is introduced:

```text
small scope
documented invariants
tests
Miri where applicable
no unsafe in normal evaluator logic
```

### 7.2 FFI

No FFI implementation in initial Phase 3.

Only boundary stubs and capability checks are in scope.

### 7.3 Dynamic Loading

Dynamic native library loading is deferred.

It must not be part of minimal VM.

---

## 8. Testing Stack

Required:

```text
unit tests
integration tests
negative validation tests
runtime semantic tests
```

Recommended:

```text
snapshot diagnostics
property tests for validators
fuzz tests for SIR decoding/validation
benchmarks for evaluator hot paths
```

Test categories:

```text
valid SIR executes
invalid SIR rejected
Phase 1 semantic examples
module compatibility
capability rejection
unwinding order
pattern matching
type contracts
record/enum behavior
```

---

## 9. Build Profiles

Recommended profiles:

```text
dev
test
release
ci
```

The VM must be correct in debug and release builds.

Debug assertions may catch VM internal bugs, but language behavior must not differ.

---

## 10. Platform Strategy

Initial platform:

```text
Linux/macOS/Windows where Rust stable supports them
```

Implementation must not assume Unix-only behavior in core VM.

Host capability modules may be platform-specific later.

---

## 11. Documentation Strategy

Required documents:

```text
VM architecture
runtime value model
validation pipeline
host capability boundary
diagnostic model
conformance tests
```

Rust docs should explain invariants for:

```text
heap handles
control regions
unwinding
module states
capability gates
```

---

## 12. Final Recommendation

Adopt Rust stable with a small Cargo workspace.

Keep the core VM:

```text
unsafe-free initially
dependency-light
table-driven
ID-based
diagnostic-first
validation-gated
capability-aware
SIR-compatible
```

Do not introduce JIT, native ABI, async runtime, dynamic loading, or public bytecode in Phase 3.


---

## 13. Performance Backend Policy

Rust remains the implementation language.

The VM must define a JIT backend abstraction before binding to a concrete backend.

Recommended staged backend direction:

```text
Tier 0/1/2:
  Rust interpreter and EIR fast interpreter

Tier 3:
  Cranelift-compatible baseline JIT backend

Tier 4:
  optional LLVM ORC or custom optimizing backend
```

Backend rules:

```text
backend choice is not language semantics
backend ABI is VM-internal
runtime helper ABI is VM-internal
compiled code must preserve safepoint/deopt metadata
compiled code must not bypass capability checks
compiled code must not expose object layout as public ABI
```

The technology stack must avoid early dependency choices that force:

```text
CPython-style reference counting
public bytecode
native plugin ABI
single backend lock-in
```

