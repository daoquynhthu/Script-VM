# Phase 1 Language Specification · Change Log

Updated: 2026-06-29 07:39:15

## v1.0 Freeze

Phase 1 high-level language design is frozen.

The freeze patch did not expand features. It closed baseline semantics and governance:

1. marked the language spec as `Version: 1.0 frozen baseline`
2. added explicit CPython C-extension non-compatibility
3. rejected CPython C API, CPython ABI, Python binary wheels, and Python extension modules
4. clarified grammar integration for enum, match, use, defer, assert, test, requires, effect, and contextual case
5. clarified type annotations as runtime contracts
6. defined deterministic unwinding order for defer/use/finally
7. clarified capability, FFI, standard-library, serialization, and foreign boundaries
8. created `PHASE-1-FREEZE.md`

---

# Phase 1 Language Specification · Change Log

Updated: 2026-06-29 07:29:42

## v0.4 Direction

The language surface has been expanded again while preserving the existing design constraints: explicitness, analyzability, safety, and future VM optimizability.

## Major Technical Additions

1. Closed nominal enums:
   - `enum`
   - `case`
   - immutable enum values
   - payload fields
   - match integration

2. Declaration destructuring:
   - list destructuring
   - record destructuring
   - enum-case destructuring
   - duplicate binding rejection
   - runtime `PatternMatchError`

3. Slices and views:
   - half-open slice syntax `a[start..end]`
   - list slicing
   - string slicing over Unicode scalar positions
   - no negative slice bounds
   - `readonly(value)` shallow read-only views

4. String formatting:
   - format strings with `f"...{expr}..."`
   - explicit display conversion
   - no implicit string coercion for `+`
   - `debug(value)`

5. Documentation and testing:
   - `##` documentation comments
   - metadata preservation
   - `test "name":` blocks
   - `assert condition, "message"`

6. Public API and library boundaries:
   - sealed export table
   - explicit re-export pattern
   - core vs standard-library boundary
   - reserved standard module roots
   - capability-gated standard modules

7. Serialization and FFI boundaries:
   - no implicit serialization
   - structurally serializable value set
   - explicit non-serializable values
   - FFI not core
   - opaque host value restrictions

## Design Rationale

This revision expands practical language coverage without accepting unbounded dynamism. The added features are deliberately structured: enums are closed, destructuring is declaration-only, slicing rejects silent clamping and negative indices, formatting is explicit, tests are source-level but runner-independent, and host/serialization boundaries remain explicit.

---

# Phase 1 Language Specification · Change Log

Updated: 2026-06-29 07:26:42

## v0.3 Direction

The Phase 1 language specification has been expanded beyond the v0.2 core into a broader script-language surface while preserving the existing safety/performance/intelligence balance.

## Major Technical Additions

1. Optional runtime type contracts:
   - binding annotations
   - parameter annotations
   - return annotations
   - field annotations
   - union types
   - optional types
   - list/map/function type contracts

2. Pattern matching:
   - `match`
   - `case`
   - wildcard patterns
   - literal patterns
   - binding patterns
   - record patterns
   - list patterns
   - map patterns
   - or-patterns
   - guarded cases

3. Structured resource management:
   - `use`
   - deterministic `close()`
   - `defer`
   - LIFO deferred execution
   - defined interaction with return/break/continue/raise

4. Capability-oriented effect metadata:
   - `requires`
   - `effect[...]`
   - reserved capabilities: `fs`, `net`, `process`, `env`, `clock`, `random`, `ffi`
   - no ambient authority by default

5. Protocolized core operations:
   - Callable
   - Iterable
   - Indexable
   - Resource
   - Displayable
   - Hashable
   - Comparable

6. Concurrency boundary:
   - no core shared-memory threads
   - `async`/`await` remain reserved
   - deterministic core without explicit external capabilities

## Design Rationale

The v0.3 additions increase language coverage without returning to unlimited dynamic behavior. The design continues to prefer explicitness, analyzability, stable object shapes, controlled effects, and runtime-checkable contracts over implicit coercion or metaprogramming magic.

---

# Phase 1 Language Specification · Change Log

Updated: 2026-06-29 07:22:00

## v0.2 Direction

The Phase 1 document has been expanded from a minimal language surface into a broader normative core-language specification.

## Major Technical Changes

1. Added explicit binding declarations:
   - `let`
   - `const`
   - `def`
   - `record`
   - `import`
   - `export`

2. Removed implicit binding creation by assignment.

3. Added lexical block scope.

4. Replaced generalized truthiness with strict Bool-only control-flow conditions.

5. Added no-implicit-coercion rule.

6. Added lists and maps.

7. Added `for` loops over iterable values.

8. Added fixed-shape `record` definitions.

9. Added record fields, mutable fields, constructors, field access, field assignment, and methods.

10. Added explicit module export/import semantics.

11. Added structured runtime errors:
    - `error(code, message)`
    - `raise`
    - `try`
    - `catch`
    - `finally`

12. Added safer default parameter semantics:
    - default expressions evaluate at call time
    - omitted mutable defaults are not shared across calls

13. Added explicit equality vs identity distinction:
    - `==`
    - `is`
    - `is not`

14. Added deterministic container indexing rules:
    - no negative indexing
    - no string iteration by default

15. Expanded diagnostics and error taxonomy.

## Design Problems Addressed

The revision directly addresses several long-term dynamic-language pain points:

- accidental local shadowing
- unbound-local ambiguity
- block-variable leakage
- truthiness bugs
- implicit type coercion
- mutable default arguments
- accidental public module APIs
- wildcard import pollution
- arbitrary object monkey-patching
- unstable object shapes
- exception swallowing
- identity/equality confusion
- silent integer overflow
- negative-indexing surprises
- string-as-iterable bugs
