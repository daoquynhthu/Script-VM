# Source-First Script VM Project · Overview

## Project Positioning

This project explores a Python-like scripting language whose execution model is not based on a public, stable bytecode layer.

The language is source-first:

```text
write source
run source
VM takes over execution
```

The implementation may use internal IR, but the IR is not bytecode in the JVM/CLR sense. It is not a public artifact, not a stable ABI, not a distribution format, and not a contract for external compilers.

The intended execution chain is:

```text
source
  -> lexer
  -> parser
  -> AST
  -> semantic analysis
  -> internal IR
  -> minimal semantic VM
```

## Core Principles

1. Source is the only stable first-class program form.
2. IR is an internal VM representation.
3. There is no public bytecode layer.
4. The first VM is experimental and minimal.
5. The first goal is semantic closure, not industrial optimization.
6. The language borrows Python-like readability, but does not pursue Python compatibility.
7. The architecture should not block future specialization, optimization, JIT, object shapes, or native compilation.

## Three Main Phases

```text
Phase 1: High-Level Language Design
Phase 2: Internal IR Design
Phase 3: Minimal Virtual Machine Design and Implementation
```

Each phase has its own planning file:

- `PHASE-1-LANGUAGE-DESIGN.md`
- `PHASE-2-IR-DESIGN.md`
- `PHASE-3-MINIMAL-VM.md`

## First-Version Success Standard

The first experimental version succeeds if it can run a source file through the full pipeline:

```text
source -> AST -> IR -> VM execution
```

with support for:

- variables
- literals
- arithmetic expressions
- function definitions
- function calls
- conditionals
- while loops
- return
- print
- basic runtime errors
- source-position diagnostics

A minimal target program:

```python
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
```

Expected output:

```text
55
```

## Explicit Non-Goals for v0

The first version does not implement:

- Python compatibility
- public bytecode
- bytecode file format
- module packaging
- industrial JIT
- industrial GC
- object shapes
- inline caches
- deoptimization
- native FFI
- async runtime
- generators
- decorators
- metaclasses
- complex standard library
