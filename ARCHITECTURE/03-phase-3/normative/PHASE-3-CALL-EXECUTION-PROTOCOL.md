# Phase 3 · Call Execution Protocol

Document class: Normative specification  
Normative status: This document defines the canonical call execution protocol for Phase 3 VM specifications.

Created: 2026-06-29 09:31:28

---

## 0. Purpose

This document repairs second-stage audit item:

```text
S2-R1: Add canonical call execution protocol.
```

It addresses residual finding:

```text
S2-M01: Call execution protocol still deserves canonical extraction.
```

All Phase 3 documents that define or use function calls, builtin calls, host calls, constructor calls, default arguments, parameter binding, call-site feedback, or return contracts MUST conform to this protocol.

---

## 1. Call Boundary

The call protocol applies to:

```text
user function calls
builtin function calls
record constructor calls
enum case constructor calls
bound method calls
host function wrapper calls
future FFI calls if enabled
```

The protocol is internal VM semantics.

It is not:

```text
native ABI
public function ABI
CPython C API
Python wheel compatibility
public bytecode call convention
```

---

## 2. Canonical Call Inputs

A call site is represented by:

```text
CallFrameInput {
  callee: Value
  positional_args: List<Value>
  named_args: List<NamedArgumentValue>
  call_site_id: CallSiteId
  source_span: SourceSpanId
  expected_result_type?: TypeId
}
```

Argument values MUST already be evaluated according to source order before entering the actual callee frame.

---

## 3. Evaluation Order

Call expression evaluation order is canonical:

```text
1. evaluate callee
2. evaluate positional arguments left-to-right
3. evaluate named arguments left-to-right
4. perform callability check
5. resolve callable target
6. bind arguments
7. evaluate required defaults at call time
8. check parameter contracts
9. enter frame or helper boundary
10. execute body or builtin/host call
11. check return contract
12. return, raise, or propagate control
```

If any step raises, later steps MUST NOT execute.

---

## 4. Callable Categories

```text
CallableTarget =
  | UserFunction
  | BuiltinFunction
  | RecordConstructor
  | EnumCaseConstructor
  | BoundMethod
  | HostFunctionWrapper
```

Calling a non-callable value raises:

```text
TypeError
```

---

## 5. User Function Call

### 5.1 FunctionObj

```text
FunctionObj {
  function_id: FunctionId
  module_id: ModuleId
  entry_eir_function: EirFunctionId
  parameter_layout: ParameterLayout
  capture_layout: CaptureLayout
  default_argument_plan: DefaultArgumentPlan
  return_type?: TypeId
  effect?: EffectId
  required_capabilities: List<CapabilityId>
  source_span?: SourceSpanId
}
```

### 5.2 Frame Creation

A user function call MUST create a logical VM frame.

The frame MUST contain:

```text
function identity
module identity
slot layout
parameter slots
local slots
capture slots/cells
region stack
source span
frame map
pending control slot
```

The logical VM call stack is independent of host language call stack.

---

## 6. Argument Binding

### 6.1 Positional Arguments

Positional arguments bind to positional parameters in declaration order.

Too many positional arguments raise:

```text
ArityError
```

### 6.2 Named Arguments

Named arguments bind by parameter name.

Duplicate named arguments raise:

```text
ArityError
```

Unknown named arguments raise:

```text
ArityError
```

A parameter receiving both positional and named values raises:

```text
ArityError
```

### 6.3 Missing Arguments

A missing required argument raises:

```text
ArityError
```

A missing optional argument with default MUST evaluate its default at call time.

---

## 7. Default Argument Evaluation

Default expressions are evaluated:

```text
at call time
only when the argument is omitted
in parameter declaration order
within the caller-visible evaluation context defined by lowering
with source spans preserved
```

Default evaluation may raise.

If default evaluation raises, the function body MUST NOT start.

Default expressions MUST NOT be precomputed at function declaration time unless the language later explicitly marks them as constant defaults.

---

## 8. Parameter Contract Checks

After argument/default binding and before body execution:

```text
parameter type contracts MUST be checked
```

Failure raises:

```text
TypeContractError
```

Parameter contract checks MUST preserve source span of the parameter or call site.

---

## 9. Captures and Closures

Closure captures MUST preserve BindingId identity.

Capture storage may be:

```text
Value capture
Cell capture
Module slot capture
Runtime internal capture
```

Mutable captures MUST use cell-like storage.

JIT/interpreter optimizations MUST NOT break observable closure semantics.

---

## 10. Function Body Execution

The function body executes as EIR entry function from the FunctionPlan.

Return, raise, break, and continue are interpreted through the unified control-state model.

A Break/Continue escaping a valid target is invalid and MUST be rejected by validation or converted to a source-level error according to validation boundary.

---

## 11. Return Contract

Before a function returns to its caller:

```text
return type contract MUST be checked if present
```

Failure raises:

```text
TypeContractError
```

If cleanup is active, cleanup runs before final frame exit, but return contract check MUST still happen before exposing the returned value to caller.

---

## 12. Builtin Function Call

BuiltinFunction descriptors MUST define:

```text
builtin_id
arity
parameter contract policy
return contract policy
required_capabilities
effect
may_allocate
may_raise
may_unwind
source mapping policy
```

Builtin calls MAY execute through:

```text
helper_call_builtin
direct VM builtin dispatch
JIT builtin fast path
```

but all paths MUST preserve descriptor semantics.

Effectful builtins MUST be capability-gated.

---

## 13. Constructor Calls

### 13.1 Record Constructor

Record constructor calls MUST enforce:

```text
fixed shape
known fields only
required fields
duplicate fields rejected
field type contracts
field mutability initialization rules
```

Failure raises:

```text
ArityError
FieldError
TypeContractError
```

according to failure kind.

### 13.2 Enum Case Constructor

Enum case constructor calls MUST enforce:

```text
closed enum
case exists
payload arity
payload contracts
```

---

## 14. Bound Method Call

A bound method call MUST preserve:

```text
receiver identity
method function identity
receiver binding position
source span
call-site feedback
```

Method binding may allocate a BoundMethod object, or may be represented internally without allocation if semantics are preserved.

---

## 15. Host Function Wrapper Call

Host calls MUST pass through the host boundary contract.

Compiled code MUST NOT directly call arbitrary host/native pointers.

Host call steps:

```text
check capability
enter host boundary
make roots visible
register host roots if needed
call host wrapper
normalize host result/error
exit host boundary
```

Host exceptions MUST be normalized.

---

## 16. Call-Site Feedback

Every source call expression lowered to EIR MUST have a CallSiteId.

Call-site feedback MAY collect:

```text
observed callable kind
function id
builtin id
receiver shape
arity shape
return type observation
miss count
exception count
```

Feedback MUST NOT change semantics.

Deterministic mode MAY disable adaptive feedback.

---

## 17. Source Mapping

A call that may raise MUST preserve:

```text
call site source span
callee source span where available
argument source spans where relevant
helper/builtin/host context where relevant
```

Stack traces SHOULD hide VM-internal helper frames unless debug mode requests them.

---

## 18. JIT Requirements

JIT call lowering MUST preserve this protocol.

Generic calls initially SHOULD call:

```text
helper_generic_call
```

Monomorphic calls MAY use guarded fast paths if they preserve:

```text
callee guard
arity check
parameter contracts
return contract
cleanup/unwind path
source mapping
deopt state
capability checks
```

---

## 19. Validation

Call validation MUST reject:

```text
CallOp without CallSiteId
call site without source span
unknown callable target kind
arity layout mismatch
default argument plan mismatch
parameter slot mismatch
return contract without TypeId resolution
host call without capability metadata
builtin call without descriptor
compiled call that skips cleanup/unwind
compiled call that bypasses helper/host boundary
```

---

## 20. Audit Tracking

This document completes:

```text
S2-R1
```

It addresses:

```text
S2-M01
```
