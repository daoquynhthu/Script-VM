# Phase 3 · Runtime Error Registry

Document class: Normative specification  
Normative status: This document defines the canonical runtime error taxonomy for Phase 3 VM specifications.

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
R2: Create central runtime error registry.
```

It resolves blocker:

```text
B-04: Error taxonomy is not centralized.
```

All Phase 3 normative documents that mention runtime errors MUST reference this registry.

---

## 1. Error Layering

Phase 3 distinguishes four error layers.

```text
LanguageError
  Observable language-level error value.

VmStructuralError
  VM invariant failure or malformed internal input. Not ordinary source-level control flow.

DiagnosticError
  Error produced by compiler/validator/diagnostic subsystem before or around execution.

HostBoundaryError
  Host exception/effect failure normalized into LanguageError or VmStructuralError.
```

### 1.1 LanguageError

A LanguageError is represented as a language-level `ErrorObj`.

It can be raised through:

```text
VmControl::Raise(ErrorHandle)
```

### 1.2 VmStructuralError

A VmStructuralError is represented as:

```text
VmError
```

It indicates a VM bug, invalid internal IR, failed validation, corrupted runtime structure, or backend violation.

It MUST NOT be catchable as ordinary source-level error unless explicitly converted.

### 1.3 DiagnosticError

A DiagnosticError belongs to validation/compile/source reporting.

It may prevent execution.

### 1.4 HostBoundaryError

Host errors MUST be normalized at host boundary.

Raw host exceptions MUST NOT leak into language runtime.

---

## 2. RuntimeErrorCode Registry

### 2.1 Required LanguageError Codes

The minimal Phase 3 VM MUST support these language-visible error codes:

| Code | Category | Required? | Raised by |
|---|---|---:|---|
| `NameError` | LanguageError | yes | unresolved source-visible name after validation boundary where applicable |
| `UninitializedBindingError` | LanguageError | yes | initialized binding read before initialization |
| `TypeError` | LanguageError | yes | unsupported operation, non-Bool condition, non-callable call, non-Error raise |
| `TypeContractError` | LanguageError | yes | failed runtime type contract |
| `PatternMatchError` | LanguageError | yes | failed declaration destructuring pattern |
| `ReadOnlyError` | LanguageError | yes | mutation through read-only view or read-only target |
| `AssertionError` | LanguageError | yes | failed assert |
| `ArityError` | LanguageError | yes | wrong function/builtin/constructor arity |
| `IndexError` | LanguageError | yes | invalid list/string slice/index bounds |
| `KeyError` | LanguageError | yes | missing map key where read requires presence |
| `FieldError` | LanguageError | yes | invalid record/module/attribute field access |
| `ImportError` | LanguageError | yes | module resolution/import/export failure |
| `ImportCycleError` | LanguageError | yes | uninitialized export access in circular import |
| `DivisionByZeroError` | LanguageError | yes | division or modulo by zero |
| `NumericOverflowError` | LanguageError | yes | checked fixed-width integer overflow |
| `CapabilityError` | LanguageError | yes | missing required capability |
| `StackOverflowError` | LanguageError | yes | logical VM call stack depth exceeded |
| `ResourceStateError` | LanguageError | yes | invalid resource state transition if policy requires error |
| `InternalVMError` | LanguageError | restricted | source-visible wrapper only when VM elects to expose internal failure safely |

### 2.2 Required VmStructuralError Codes

| Code | Category | Required? | Raised by |
|---|---|---:|---|
| `InvalidEirError` | VmStructuralError | yes | malformed EIR reaching execution/JIT |
| `InvalidRuntimePlanError` | VmStructuralError | yes | malformed RuntimePlan reaching execution |
| `InvalidSlotError` | VmStructuralError | yes | unknown or incompatible SlotId |
| `InvalidObjectHandleError` | VmStructuralError | yes | stale/invalid ObjRef/ObjectId |
| `InvalidHelperError` | VmStructuralError | yes | unknown helper ID or descriptor mismatch |
| `InvalidFrameStateError` | VmStructuralError | yes | corrupted frame/stack/root/deopt state |
| `InvalidRootMapError` | VmStructuralError | yes | missing or invalid root map at required safepoint |
| `InvalidStackMapError` | VmStructuralError | yes | JIT safepoint without valid stack map |
| `InvalidDeoptError` | VmStructuralError | yes | deopt point missing reconstruction data |
| `BackendViolationError` | VmStructuralError | yes | compiled code violates VM metadata contract |

---

## 3. Error Object Requirements

A language-level ErrorObj MUST contain:

```text
error_code: RuntimeErrorCode
message: String
source_span?: SourceSpanId
stack_trace?: StackTrace
details?: Map[String, Value]
cause?: ErrorHandle
suppressed?: List[ErrorHandle]
```

`source_span` is REQUIRED for source-originated may-raise operations when available.

`stack_trace` MAY be disabled in restricted runtime mode, but diagnostics MUST still be source-oriented where possible.

---

## 4. Raise Rules

A source-level `raise` MUST raise only language Error values.

Raising a non-Error value MUST raise:

```text
TypeError
```

Runtime helpers that produce language failure MUST return:

```text
VmControl::Raise(ErrorHandle)
```

Runtime helpers that detect structural VM failure MUST return:

```text
VmError
```

---

## 5. Cleanup and Suppressed Errors

During cleanup:

```text
pending Raise + cleanup Raise
```

MUST preserve the original primary error and attach the cleanup error as suppressed unless the structured unwinding algorithm explicitly defines override.

```text
pending Normal + cleanup Raise
```

MUST make cleanup error the primary error.

```text
pending Return/Break/Continue + cleanup Raise
```

MUST convert pending control to Raise unless the canonical structured unwinding algorithm defines a stronger rule.

---

## 6. Source Mapping

Errors raised during execution MUST map to:

```text
SourceSpanId
SIR NodeId where available
EIR location
frame stack
helper context if relevant
```

VM-internal helper frames MAY be hidden from ordinary source stack traces.

---

## 7. Deferred Error Codes

Future features MAY add error codes for:

```text
FFI boundary
async/cancellation
debugger traps
concurrent execution
native resource ownership
```

Such codes MUST be added to this registry before becoming normative.

---

## 8. Compatibility

Adding a new language-visible error code is a normative language/runtime change.

Changing an existing error's category, visibility, or required status requires reopening Phase 3 or a later compatibility revision.

---

## 9. Audit Tracking

This document completes:

```text
R2
```

It resolves:

```text
B-04
```

It partially supports:

```text
B-06
M-10
M-15
m-06
```
