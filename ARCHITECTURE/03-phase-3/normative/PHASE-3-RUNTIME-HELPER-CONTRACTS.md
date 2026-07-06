# Phase 3 · Runtime Helper Contracts
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.8 helper draft  
Depends on: Phase 3 Structured Control and Unwinding Lowering Round 2 v0.7  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: runtime helper ABI boundary, helper descriptors, helper families, GC/JIT/capability contracts, error/control return discipline, helper validation  
Out of scope: concrete native ABI, concrete Cranelift lowering, concrete LLVM lowering, full standard library, full FFI implementation

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



## 0. Status

Runtime helpers are internal VM functions used by:

```text
EIR fast interpreter
baseline JIT
optimizing JIT
module initialization runtime
capability host boundary
GC-aware allocation paths
structured unwinding runtime
```

Runtime helpers are not:

```text
public native ABI
foreign extension ABI
CPython C API
Python extension boundary
stable plugin ABI
public bytecode helper set
```

Runtime helpers are internal, versioned, VM-controlled, and allowed to change across VM versions.

---

## 1. Purpose

Runtime helpers exist to keep EIR compact while preserving semantics.

They handle operations that are:

```text
complex
rare
effectful
allocation-heavy
capability-gated
error-heavy
unwinding-heavy
JIT-unfriendly
host-boundary-sensitive
```

Examples:

```text
generic call
generic access
map lookup
string slicing
record construction fallback
enum construction fallback
type contract check
pattern fallback
raise construction
structured unwind
defer execution
resource close
module import
capability check
allocation
write barrier
```

A helper is a slow path or semantic boundary, not an escape hatch for unspecified behavior.

---

## 2. Helper Boundary Rule

### 2.1 Internal ABI

Runtime helper ABI is VM-internal.

It may be called by:

```text
interpreter
JIT-generated code
VM builtin implementation
module initialization runtime
GC/runtime services
```

It must not be exposed as:

```text
native extension ABI
public package ABI
external compiler target ABI
user-callable function table
```

### 2.2 Versioning

Helper signatures are versioned by:

```text
VM version
RuntimeHelperTable version
target profile
value layout profile
GC profile
JIT backend profile
```

If helper ABI changes, compiled code and EIR caches that depend on it must be invalidated.

### 2.3 No Semantic Drift

Helper behavior must match Phase 1 and Phase 2 semantics.

Interpreter fast path and helper slow path must produce equivalent observable behavior.

---

## 3. Helper Descriptor

### 3.1 RuntimeHelperDescriptor

```text
RuntimeHelperDescriptor {
  helper_id: RuntimeHelperId
  name: String
  family: RuntimeHelperFamily
  signature: RuntimeHelperSignature
  may_allocate: Bool
  may_raise: Bool
  may_unwind: Bool
  is_safepoint: Bool
  requires_roots_visible: Bool
  required_capability?: CapabilityId
  effect?: EffectId
  gc_behavior: HelperGcBehavior
  jit_call_policy: HelperJitCallPolicy
  source_mapping_policy: HelperSourceMappingPolicy
}
```

### 3.2 RuntimeHelperFamily

```text
RuntimeHelperFamily =
  | Call
  | Access
  | Construction
  | TypeCheck
  | Pattern
  | Error
  | Unwind
  | Resource
  | Module
  | Capability
  | Allocation
  | WriteBarrier
  | Display
  | Numeric
  | Debug
```

### 3.3 RuntimeHelperSignature

```text
RuntimeHelperSignature {
  parameters: List<HelperParam>
  result: HelperResultType
  calling_convention: HelperCallingConvention
}
```

### 3.4 HelperParam

```text
HelperParam {
  name: String
  kind: HelperParamKind
  required: Bool
}
```

```text
HelperParamKind =
  | Value
  | SlotRef
  | FrameRef
  | VmRef
  | RuntimePlanRef
  | CallSiteId
  | AccessSiteId
  | ShapeId
  | TypeId
  | CapabilityId
  | SourceSpanId
  | PendingControlRef
  | RegionRef
  | ModuleId
  | HelperInternal
```

### 3.5 HelperResultType

```text
HelperResultType =
  | Value
  | VmControl
  | Unit
  | Bool
  | ObjectRef
  | ModuleRef
  | ErrorRef
  | HelperInternal
```

### 3.6 HelperCallingConvention

```text
HelperCallingConvention =
  | InterpreterDirect
  | JitRuntimeCall
  | HostBoundaryCall
  | GcRuntimeCall
  | InternalOnly
```

The calling convention is internal.

It is not public ABI.

---

## 4. Helper Return Discipline

### 4.1 HelperReturn

Conceptual result:

```text
HelperReturn =
  | Value(Value)
  | Control(VmControl)
  | Unit
  | Error(VmError)
```

### 4.2 Language Errors

Language-level errors return:

```text
Control(Raise(error))
```

Examples:

```text
TypeError
FieldError
ImportError
CapabilityError
AssertionError
PatternMatchError
```

### 4.3 VM Structural Failures

VM internal structural failures return:

```text
Error(VmError)
```

Examples:

```text
invalid helper ID
corrupt RuntimePlan
invalid SlotId
stale ObjectId
missing root map
broken frame map
```

### 4.4 Rust Panic Boundary

Rust panic is not a helper return mechanism.

A panic indicates implementation bug or unrecoverable host failure.

---

## 5. GC and Root Visibility Contract

### 5.1 Helper GC Declaration

Each helper declares:

```text
may_allocate
is_safepoint
requires_roots_visible
gc_behavior
```

### 5.2 HelperGcBehavior

```text
HelperGcBehavior =
  | NoAllocation
  | MayAllocateNoCollection
  | MayAllocateMayCollect
  | MayMoveObjects
  | GcInternal
```

### 5.3 Root Visibility

If helper may allocate or collect, all live roots must be visible before helper call.

Required visible roots:

```text
live slots
capture cells
module cells
pending control
active errors
region stack
defer callables
resource handles
host roots
JIT frame roots
```

### 5.4 Moving GC Compatibility

A helper that may move objects must not return stale raw object addresses.

All object references crossing helper boundary must use VM-managed handles or relocated references.

### 5.5 Helper Safepoint

A helper marked `is_safepoint` must have a valid `SafepointRecord`.

JIT calls to such helper must provide or reference correct stack/root maps.

---

## 6. JIT Helper Contract

### 6.1 JIT Call Rule

JIT code may call runtime helpers only through the VM-controlled helper table.

JIT code must not directly call arbitrary host/native pointers.

### 6.2 JitRuntimeCall Requirements

A helper callable from JIT must define:

```text
argument representation
return representation
clobbered registers abstractly
safepoint behavior
root visibility requirement
may_raise behavior
may_unwind behavior
deopt interaction
```

This is backend-internal metadata, not public ABI.

### 6.3 Deopt Interaction

If a helper can deopt, unwind, or raise, the call site must have:

```text
source span
frame map
root map
pending control map
region stack map
deopt point if speculative
```

### 6.4 Helper Inlining

A JIT may inline helper logic only if it preserves:

```text
error category
source mapping
capability checks
GC safepoints
write barriers
deopt metadata
unwinding behavior
```

Otherwise helper call must remain explicit.

---

## 7. Capability Contract

### 7.1 Capability Declaration

Any helper that performs effectful host access must declare:

```text
required_capability
effect
```

### 7.2 Capability Check Placement

Capability checks occur before effectful operation.

They may be:

```text
explicit CheckCapability EIR op
or helper-internal check
```

If helper-internal, the helper descriptor must declare required capability.

### 7.3 Capability Failure

Missing capability returns language error:

```text
CapabilityError
```

### 7.4 JIT Rule

JIT code must not bypass capability checks.

A capability-using helper must not be replaced with unchecked native call.

---

## 8. Helper Families

## 8.1 Call Helpers

### 8.1.1 `helper_generic_call`

Purpose:

```text
invoke value whose callable kind is not statically specialized
```

Inputs:

```text
callee: Value
arguments: ValueList
call_site_id: CallSiteId
frame: FrameRef
```

May:

```text
allocate
raise
unwind
be safepoint
update call-site feedback
```

Must preserve:

```text
argument evaluation order already completed by caller
arity checks
default argument call-time evaluation
parameter contract checks
return contract checks
call-site feedback updates
```

### 8.1.2 `helper_bind_method`

Purpose:

```text
construct bound method value
```

Must preserve receiver identity.

May allocate.

### 8.1.3 `helper_call_builtin`

Purpose:

```text
invoke builtin function through VM builtin table
```

Must enforce builtin signature and capability requirements.

---

## 8.2 Access Helpers

### 8.2.1 `helper_get_attribute`

Purpose:

```text
generic attribute read
```

Inputs:

```text
receiver
attribute symbol
access_site_id
```

Required behavior:

```text
record field lookup by shape when possible
module export lookup
method lookup
readonly view delegation
error on unknown field
no dynamic record field creation
```

### 8.2.2 `helper_set_attribute`

Purpose:

```text
generic attribute write
```

Must enforce:

```text
readonly rejection
fixed record shape
field mutability
field type contract
write barrier
```

### 8.2.3 `helper_index_read`

Purpose:

```text
generic index read
```

Supported categories:

```text
List[Int]
Map[Hashable]
```

String indexing remains non-core unless later amended.

### 8.2.4 `helper_index_write`

Purpose:

```text
generic index write
```

Must enforce:

```text
readonly rejection
list bounds
map hashability
write barrier
```

### 8.2.5 `helper_slice_read`

Purpose:

```text
list/string slicing
```

Must enforce:

```text
Int bounds
half-open slicing
negative bound errors
out-of-range errors unless Phase 2 amended
```

---

## 8.3 Construction Helpers

### 8.3.1 `helper_construct_record`

Purpose:

```text
fallback or generic record construction
```

Must enforce:

```text
fixed shape
required fields
default fields at construction time
unknown/duplicate field errors
field contract checks
no partially visible record escape on failure
```

### 8.3.2 `helper_construct_enum`

Purpose:

```text
fallback or generic enum value construction
```

Must enforce:

```text
closed enum cases
payload arity
payload contract checks
case identity
```

### 8.3.3 `helper_construct_map`

Purpose:

```text
construct map with hashability and duplicate-key semantics
```

Must preserve insertion order.

Duplicate key rule:

```text
later value replaces earlier value
first insertion position preserved
```

### 8.3.4 `helper_construct_error`

Purpose:

```text
construct language Error object
```

Must attach:

```text
error code
message
source span
stack trace if requested
suppressed error list
```

---

## 8.4 Type Check Helpers

### 8.4.1 `helper_check_type_contract`

Purpose:

```text
generic type contract check
```

Inputs:

```text
value
type_id
failure_code
source_span
```

Must return original value or raise `TypeContractError`.

### 8.4.2 `helper_check_callable`

Purpose:

```text
generic callable check
```

Must recognize:

```text
Function
BuiltinFunction
RecordConstructor
EnumCaseConstructor
BoundMethod
HostFunction
```

### 8.4.3 `helper_check_hashable`

Purpose:

```text
map key hashability check
```

Must not allow mutable/hash-unstable keys unless a future hash protocol explicitly permits them.

### 8.4.4 `helper_check_shape`

Purpose:

```text
generic shape check
```

Can fall back for non-record/non-enum categories depending on caller context.

---

## 8.5 Pattern Helpers

### 8.5.1 `helper_match_pattern`

Purpose:

```text
generic pattern fallback
```

Must support:

```text
wildcard
literal
binding
record
enum
list
map
or-pattern
```

### 8.5.2 Pattern Failure Mode

Helper must distinguish:

```text
branch failure in match
PatternMatchError in destructuring declaration
```

### 8.5.3 Pattern Binding

Pattern bindings must be written only after successful match unless rollback semantics are explicit.

Recommended rule:

```text
write to temporary binding slots
commit to case binding slots only after full pattern success
```

---

## 8.6 Error Helpers

### 8.6.1 `helper_raise`

Purpose:

```text
convert Error value to pending Raise control
```

Must reject non-Error raise with `TypeError`.

### 8.6.2 `helper_attach_suppressed`

Purpose:

```text
attach cleanup error as suppressed error to primary error
```

If suppressed list unsupported in bootstrap runtime, helper must preserve diagnostic record.

### 8.6.3 `helper_assert_fail`

Purpose:

```text
construct AssertionError and raise
```

Must evaluate assertion message only when assertion fails.

---

## 8.7 Unwind Helpers

### 8.7.1 `helper_perform_unwind`

Purpose:

```text
execute structured unwinding according to RegionStack and PendingControl
```

Must preserve:

```text
defer LIFO order
resource close order
finally execution
finally override rule
primary/suppressed error behavior
loop target resolution
function return resolution
```

### 8.7.2 Inputs

```text
pending_control
frame
region_stack
source_span
```

### 8.7.3 Output

```text
VmControl
```

or updated pending control.

### 8.7.4 JIT Requirement

Compiled frames must expose region stack and root maps to unwind helper.

---

## 8.8 Resource Helpers

### 8.8.1 `helper_register_resource`

Purpose:

```text
register resource acquired by use
```

Must register cleanup only after successful acquisition.

### 8.8.2 `helper_close_resource`

Purpose:

```text
close resource exactly once
```

Must handle:

```text
Open -> Closing -> Closed
close raises
double close policy
suppressed cleanup error
capability-origin tracking
```

### 8.8.3 `helper_register_defer`

Purpose:

```text
register zero-argument deferred callable
```

Must check callable and arity.

### 8.8.4 `helper_execute_defer`

Purpose:

```text
execute one deferred callable
```

Must return Raise if callable raises.

---

## 8.9 Module Helpers

### 8.9.1 `helper_resolve_module`

Purpose:

```text
resolve module name through host-defined deterministic resolver
```

May require module-system capability if host chooses.

### 8.9.2 `helper_initialize_module`

Purpose:

```text
execute module initialization state machine
```

Must preserve:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
```

### 8.9.3 `helper_import_named`

Purpose:

```text
bind named export to local import slot
```

Must detect:

```text
missing export
interface mismatch
uninitialized export in cycle
```

### 8.9.4 `helper_import_module`

Purpose:

```text
bind module object to local import slot
```

### 8.9.5 `helper_seal_exports`

Purpose:

```text
seal module export table after successful initialization
```

---

## 8.10 Capability Helpers

### 8.10.1 `helper_check_capability`

Purpose:

```text
verify capability exists in current capability environment
```

Failure raises `CapabilityError`.

### 8.10.2 `helper_enter_host_call`

Purpose:

```text
prepare roots and capability state before host call
```

May be safepoint.

### 8.10.3 `helper_exit_host_call`

Purpose:

```text
normalize host return or host error into VM result
```

Must not leak host exception representation into language runtime.

---

## 8.11 Allocation Helpers

### 8.11.1 `helper_alloc_object`

Purpose:

```text
allocate generic heap object
```

Must:

```text
make roots visible
honor GC profile
return VM handle
not expose raw pointer
```

### 8.11.2 Specialized Allocation Helpers

Allowed helpers:

```text
alloc_string
alloc_list
alloc_map
alloc_record
alloc_enum_value
alloc_function
alloc_module
alloc_error
alloc_resource
```

### 8.11.3 Allocation Safepoint

Allocation may be safepoint.

If allocation may trigger GC, root maps must be available.

---

## 8.12 Write Barrier Helpers

### 8.12.1 `helper_write_barrier`

Purpose:

```text
record heap reference mutation for future GC
```

Initial implementation may be no-op.

### 8.12.2 Barrier Sites

Barrier hook must exist for:

```text
record field write
list element write
map entry write
capture cell write
module binding write
resource state write where heap refs stored
```

### 8.12.3 Barrier Rule

Even if current GC does not need write barriers, mutation APIs must route through a barrier insertion point.

---

## 8.13 Display Helpers

### 8.13.1 `helper_display`

Purpose:

```text
convert value to display string
```

Used by:

```text
print
format string
debug
diagnostics
```

Must not create implicit coercion for ordinary operators.

### 8.13.2 Display Error

If display conversion can fail, failure must be language error or diagnostic error according to call site.

---

## 8.14 Numeric Helpers

### 8.14.1 `helper_numeric_binary`

Purpose:

```text
fallback numeric operation
```

Must enforce:

```text
no implicit coercion
overflow check
division by zero check
operator support
```

### 8.14.2 `helper_compare`

Purpose:

```text
generic comparison
```

Unsupported comparisons raise `TypeError`.

---

## 9. Helper Source Mapping

### 9.1 SourceSpan Input

Helpers that may raise must receive or reconstruct source span.

### 9.2 Diagnostic Mapping

Helper-generated diagnostics must map to:

```text
EIR op -> SIR node -> source span
```

### 9.3 Helper Internal Frames

Runtime helper frames may be hidden from user stack traces unless debug mode asks for VM internals.

Language stack trace must remain source-oriented.

---

## 10. Helper Validation

RuntimeHelperTable validation must reject:

```text
duplicate helper IDs
missing helper implementation
signature mismatch
helper marked no-allocate but allocation path exists
helper marked no-raise but can raise language error
helper marked non-safepoint but may collect
helper requires capability but descriptor omits it
JIT-callable helper without JIT call policy
GC-moving helper without handle-safe return policy
helper family mismatch
missing source mapping policy for may_raise helper
```

---

## 11. Helper Compatibility

Runtime helpers are VM-internal but cache-sensitive.

Changing helper signatures invalidates:

```text
EIR cache
RuntimePlan cache
JIT compiled code
safepoint maps
deopt metadata
runtime helper table digest
```

### 11.1 Helper Table Digest

Helper table digest includes:

```text
helper IDs
names
families
signatures
may_allocate
may_raise
may_unwind
is_safepoint
required capabilities
GC behavior
JIT call policy
```

### 11.2 Cache Rule

Compiled code must not run if helper table digest differs from the digest used when compiling.

---

## 12. Security and Capability Safety

Runtime helpers are a privilege boundary.

They must not:

```text
perform ambient host access
skip capability checks
expose raw object pointers
expose host exception objects
call arbitrary native code
mutate read-only values
bypass module interface validation
```

Any helper that crosses into host capability territory must be audited as host boundary code.

---

## 13. Bootstrap Policy

The initial VM may implement many helpers in Rust directly.

However:

```text
helper implementation detail must not become public ABI
Rc/RefCell inside helper is bootstrap-only
native pointer identity must not escape helper
helper names are internal descriptors, not user API
```

The bootstrap VM may implement unwinding, allocation, generic access, and pattern fallback through helpers for simplicity.

Later VM may inline or specialize them.

---

## 14. JIT Readiness Matrix

Each helper should be classified:

```text
JitLoweringClass =
  | AlwaysCallHelper
  | InlineFastPathWithHelperFallback
  | InlineAfterGuard
  | InterpreterOnly
  | ForbiddenInJit
```

Examples:

```text
generic call -> InlineFastPathWithHelperFallback
record field access fallback -> InlineAfterGuard
allocation -> AlwaysCallHelper or InlineWithGCProtocol
write barrier -> InlineAfterGuard or AlwaysCallHelper
perform_unwind -> AlwaysCallHelper initially
module import -> AlwaysCallHelper
capability check -> InlineAfterGuard or AlwaysCallHelper
```

This classification guides baseline JIT but does not change semantics.

---

## 15. Non-Goals

This document does not define:

```text
public native ABI
foreign extension ABI
CPython C API
concrete register convention
concrete Cranelift lowering
concrete LLVM lowering
full standard library
host resource implementations
debugger protocol
profiler format
```

---

## 16. Next Work

Next Phase 3 documents should define:

```text
GC root enumeration concrete model
baseline JIT backend interface
fast interpreter concrete data structures
runtime helper implementation plan
Phase 3 consistency audit

```
