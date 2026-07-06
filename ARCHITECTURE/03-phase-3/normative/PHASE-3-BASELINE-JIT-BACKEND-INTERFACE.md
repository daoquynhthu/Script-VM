# Phase 3 · Baseline JIT Backend Interface
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.10 JIT interface draft  
Depends on: Phase 3 GC Root Enumeration and Safepoint Model v0.9  
Depends on: Phase 3 Runtime Helper Contracts v0.8  
Depends on: Phase 3 RuntimePlan and EIR Framework v0.4  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: baseline JIT backend abstraction, JIT context, compiled function model, code handle, helper-call ABI boundary, safepoint/stack-map/deopt emission, backend-independent lowering categories, cache invalidation, validation  
Out of scope: concrete Cranelift implementation, concrete LLVM implementation, optimizing JIT, register allocator design, native ABI, public bytecode

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

This document defines the first JIT backend interface for Phase 3.

It does not implement JIT.

It fixes the architecture so that a baseline JIT can later be implemented without rewriting the VM.

The core rule remains:

```text
JIT implementation: staged
JIT architecture: mandatory
```

The recommended first implementation target is a Cranelift-compatible baseline backend.

However:

```text
backend choice is not language semantics
backend ABI is VM-internal
backend lock-in is forbidden
EIR remains internal and non-public
```

---

## 1. Baseline JIT Purpose

The baseline JIT exists to reduce interpreter dispatch overhead and compile hot EIR functions quickly.

It is not the optimizing tier.

It should prioritize:

```text
fast compilation
semantic equivalence
stack-map correctness
helper-call correctness
safepoint correctness
deopt metadata preservation
capability-check preservation
GC compatibility
```

It may avoid complex optimizations.

The baseline JIT may still perform local fast-path lowering with guards and helper fallbacks.

---

## 2. JIT Tier Position

The execution tiers are:

```text
Tier 0: SIR correctness interpreter
Tier 1: RuntimePlan-driven interpreter
Tier 2: EIR fast interpreter
Tier 3: baseline JIT
Tier 4: optimizing JIT
```

This document defines Tier 3.

Tier 3 consumes:

```text
RuntimePlan
EIR
RuntimeHelperTable
SafepointTable
DeoptPointTable
FeedbackTable
GC target profile
Value layout profile
```

Tier 3 produces:

```text
CompiledFunction
StackMapTable
JitSafepointTable
JitDeoptTable
CodeMetadata
CacheMetadata
```

---

## 3. Backend Abstraction

### 3.1 JitBackend

Conceptual Rust-like interface:

```rust
pub trait JitBackend {
    fn backend_id(&self) -> JitBackendId;

    fn target_profile(&self) -> JitTargetProfile;

    fn supports(&self, input: &JitCompileInput) -> JitSupportReport;

    fn compile_function(
        &mut self,
        input: JitCompileInput,
        ctx: &mut JitContext,
    ) -> Result<CompiledFunction, JitCompileError>;

    fn invalidate(&mut self, reason: JitInvalidationReason);
}
```

This is conceptual.

The final Rust API may differ.

### 3.2 Backend Rule

A backend must compile from VM-internal EIR, not from source text or public bytecode.

A backend must not require EIR to become a public stable ABI.

### 3.3 Backend Kinds

```text
JitBackendKind =
  | CraneliftCompatibleBaseline
  | CustomBaseline
  | LlvmOrcOptimizing
  | WasmBackend
  | AotBackend
  | InterpreterAdapter
```

Only the baseline backend interface is in scope.

---

## 4. JitCompileInput

### 4.1 JitCompileInput

```text
JitCompileInput {
  eir_module: EirModuleRef
  eir_function: EirFunctionId
  runtime_plan: RuntimePlanRef
  function_plan: FunctionPlanRef
  helper_table: RuntimeHelperTableRef
  type_feedback: FeedbackTableRef
  shape_feedback: FeedbackTableRef
  inline_cache_state: InlineCacheTableRef
  safepoints: SafepointTableRef
  deopt_points: DeoptPointTableRef
  target_profile: JitTargetProfile
  compile_mode: BaselineCompileMode
}
```

### 4.2 BaselineCompileMode

```text
BaselineCompileMode =
  | Debug
  | Checked
  | Fast
```

`Debug` preserves maximal diagnostics and disables aggressive fast paths.

`Checked` is default.

`Fast` may emit more guarded fast paths but must preserve deopt/fallback correctness.

### 4.3 Input Validation

Before compilation, the backend must reject:

```text
invalid EIR
missing RuntimePlan
unsupported value layout
unsupported helper ABI
unsupported GC profile
missing safepoint data
missing stack-map requirements
unsupported operation family
missing deopt metadata for speculative guard
```

---

## 5. JitContext

### 5.1 JitContext

```text
JitContext {
  vm_version: Version
  runtime_plan_digest: Digest
  eir_digest: Digest
  helper_table_digest: Digest
  value_layout_profile: ValueLayoutProfile
  heap_profile: HeapProfile
  gc_profile: GcProfile
  target_profile: JitTargetProfile
  code_allocator: CodeAllocatorRef
  relocation_registry: RelocationRegistryRef
  safepoint_registry: SafepointRegistryRef
  stack_map_registry: StackMapRegistryRef
  deopt_registry: DeoptRegistryRef
  helper_call_registry: HelperCallRegistryRef
  diagnostics: DiagnosticSink
}
```

### 5.2 Context Rule

JitContext is VM-internal.

It may contain backend-specific state, but no backend-specific state may become language ABI.

### 5.3 Diagnostics

The JIT must emit structured diagnostics for:

```text
unsupported EIR op
invalid metadata
helper ABI mismatch
safepoint emission failure
stack map failure
deopt metadata failure
backend codegen failure
```

Compilation failure should fall back to EIR interpretation where safe.

---

## 6. JitTargetProfile

### 6.1 JitTargetProfile

```text
JitTargetProfile {
  architecture: TargetArchitecture
  operating_system?: TargetOperatingSystem
  pointer_width: UInt
  endianness: Endianness
  value_layout_profile: ValueLayoutProfile
  calling_convention_profile: CallingConventionProfile
  gc_profile: GcProfile
  code_model: CodeModel
  relocation_model: RelocationModel
  backend_feature_set: BackendFeatureSet
}
```

### 6.2 TargetArchitecture

```text
TargetArchitecture =
  | X86_64
  | AArch64
  | Wasm
  | InterpreterPseudoTarget
  | Other
```

### 6.3 Target Rule

Unsupported targets must fall back to interpreter.

The VM must not reinterpret semantics to satisfy a backend limitation.

---

## 7. EIR Lowering Categories

### 7.1 JitLoweringClass

Every EIR operation must eventually be classified:

```text
JitLoweringClass =
  | DirectLowering
  | GuardedFastPathWithHelperFallback
  | AlwaysCallHelper
  | DeoptRequired
  | InterpreterOnly
  | Forbidden
```

### 7.2 DirectLowering

Can be directly emitted as backend operations.

Examples:

```text
slot move
immediate constant load
simple branch
integer add with explicit overflow guard
shape comparison
case-index comparison
```

### 7.3 GuardedFastPathWithHelperFallback

Emits fast path plus fallback helper.

Examples:

```text
record field load
method read
monomorphic call
list index read
integer binary op
type descriptor check
```

### 7.4 AlwaysCallHelper

Always calls VM helper initially.

Examples:

```text
generic call
module import
perform_unwind
resource close
pattern fallback
map generic lookup
string slicing
allocation if GC protocol not inlined
```

### 7.5 DeoptRequired

May compile only if deopt metadata exists.

Examples:

```text
speculative shape-specialized call
speculative type-specialized arithmetic
inlined function body
elided check
```

### 7.6 InterpreterOnly

Backend does not compile this op.

Function remains interpreted or is split if supported.

### 7.7 Forbidden

The op must never appear in compiled code.

Examples:

```text
invalid VM-internal marker
debug-only assertion without runtime meaning
malformed synthetic op
```

---

## 8. CompiledFunction

### 8.1 CompiledFunction

```text
CompiledFunction {
  compiled_function_id: CompiledFunctionId
  source_eir_function: EirFunctionId
  function_id?: FunctionId
  code_handle: JitCodeHandle
  entry_points: CompiledEntryPoints
  stack_maps: StackMapTable
  safepoints: JitSafepointTable
  deopt_points: JitDeoptTable
  helper_calls: HelperCallSiteTable
  relocations: RelocationTable
  assumptions: AssumptionSet
  cache_key: JitCacheKey
  diagnostics?: DiagnosticTable
}
```

### 8.2 CompiledEntryPoints

```text
CompiledEntryPoints {
  normal_entry: CodeOffset
  checked_entry?: CodeOffset
  deopt_entry?: CodeOffset
  osr_entry_points?: List<OsrEntryPoint>
}
```

OSR is optional and deferred.

### 8.3 JitCodeHandle

```text
JitCodeHandle {
  code_id: CodeId
  memory_region: CodeMemoryRegion
  executable: Bool
  writable: Bool
  lifetime: CodeLifetime
}
```

### 8.4 Code Lifetime

```text
CodeLifetime =
  | UntilInvalidated
  | UntilModuleUnload
  | UntilVmShutdown
```

### 8.5 W^X Rule

If the platform supports it, code memory should not be writable and executable at the same time.

The spec does not require final code memory policy in Phase 3, but the backend interface must allow safe code memory management.

---

## 9. Runtime Helper Call ABI

### 9.1 Helper Call Site

```text
HelperCallSite {
  helper_id: RuntimeHelperId
  call_offset: CodeOffset
  safepoint_id?: SafepointId
  deopt_id?: DeoptId
  argument_map: HelperArgumentMap
  result_map: HelperResultMap
}
```

### 9.2 HelperArgumentMap

```text
HelperArgumentMap {
  arguments: List<HelperArgumentLocation>
}
```

```text
HelperArgumentLocation =
  | Register
  | StackSlot
  | Constant
  | RuntimeHandle
  | FrameRef
  | VmRef
```

### 9.3 Helper Result Handling

The JIT must handle helper returns:

```text
Value -> continue
VmControl::Raise -> enter unwind/deopt path
VmControl::Return/Break/Continue -> enter structured control path if helper can produce it
VmError -> abort compiled execution and report VM diagnostic
```

### 9.4 Helper Safepoint

If helper is safepoint, helper call site must have stack/root maps.

### 9.5 Helper ABI Rule

Helper ABI is internal.

Changing helper signature invalidates compiled code.

---

## 10. Safepoint and Stack Map Emission

### 10.1 JIT Safepoint Emission

For every compiled safepoint, backend emits:

```text
code offset
safepoint kind
live value locations
frame state
source span
deopt link if needed
```

### 10.2 StackMapTable

```text
StackMapTable {
  stack_maps: List<StackMap>
}
```

### 10.3 StackMap

```text
StackMap {
  stack_map_id: StackMapId
  code_offset: CodeOffset
  live_value_locations: List<ValueLocation>
  frame_state: FrameStateRef
  source_span?: SourceSpanId
}
```

### 10.4 ValueLocation

```text
ValueLocation =
  | Register
  | StackSlot
  | SpillSlot
  | RuntimeHandle
  | Immediate
  | Constant
```

Derived pointers are not recommended for baseline JIT.

If used, base object must be recorded.

### 10.5 Stack Map Correctness

If GC profile permits moving collection, every live heap reference must be updateable.

---

## 11. Deopt Emission

### 11.1 JitDeoptTable

```text
JitDeoptTable {
  deopt_points: List<JitDeoptRecord>
}
```

### 11.2 JitDeoptRecord

```text
JitDeoptRecord {
  deopt_id: DeoptId
  code_offset: CodeOffset
  source_eir_location: EirLocation
  frame_map: FrameMap
  local_slot_map: LocalSlotMap
  value_locations: List<ValueLocation>
  region_stack_state: RegionStackState
  pending_control_state?: PendingControlState
  resume_target: DeoptResumeTarget
}
```

### 11.3 DeoptResumeTarget

```text
DeoptResumeTarget =
  | EirInterpreterLocation
  | GenericEirFallback
  | FunctionEntry
  | UnwindHelper
```

### 11.4 Baseline Deopt Policy

Baseline JIT may minimize speculative deopt.

If a compiled operation uses no speculative assumption, it may not need deopt.

If it emits guard-based specialization, it must emit deopt or helper fallback.

### 11.5 Deopt Validation

JIT validation must reject compiled guards without valid failure paths.

---

## 12. Guard Lowering

### 12.1 Guard Lowering

EIR guards lower to backend checks.

Possible actions on failure:

```text
jump to helper fallback
jump to generic EIR fallback
deopt
raise language error
invalidate inline cache
```

### 12.2 Guard Types

Required guard support:

```text
type guard
shape guard
call target guard
capability guard
module state guard
readonly guard
overflow guard
division-by-zero guard
```

### 12.3 Semantic Guard vs Speculative Guard

Semantic guard failure raises language error.

Speculative guard failure falls back or deopts.

The backend must distinguish them.

---

## 13. Value Layout Interface

### 13.1 ValueLayoutProfile

JIT backend sees value representation only through `ValueLayoutProfile`.

It must not assume Rust enum layout.

### 13.2 Required Abstract Operations

Backend may request lowering hooks for:

```text
is_immediate
is_heap_ref
tag_of
unbox_int
box_int
unbox_float
box_float
load_obj_ref
compare_identity
```

### 13.3 Layout Non-Commitment

The VM may later change from:

```text
Rust enum
to tagged pointer
to NaN boxing
to compressed handles
```

without changing language semantics.

Such change invalidates compiled code.

---

## 14. GC Interface for JIT

### 14.1 JitGcInterface

```text
JitGcInterface {
  emit_safepoint
  emit_stack_map
  emit_write_barrier
  emit_allocation_call
  register_compiled_frame_layout
}
```

### 14.2 Allocation Lowering

Allocation may lower to:

```text
helper_alloc_object
specialized allocation helper
inline bump allocation with safepoint fallback
```

Baseline JIT may initially always call allocation helpers.

### 14.3 Write Barrier Lowering

Heap mutation must emit:

```text
inline barrier
or helper_write_barrier
```

If GC profile has no barrier need, barrier may lower to no-op.

The barrier call site must still exist in lowering logic.

### 14.4 Root Map Rule

Any compiled code that can reach allocation/helper safepoint must have valid root map.

---

## 15. Call Lowering

### 15.1 Generic Call

Generic call lowers to:

```text
helper_generic_call
```

initially.

### 15.2 Monomorphic Call Fast Path

If CallSite feedback is monomorphic:

```text
guard callee identity/function id
marshal arguments
call compiled function or interpreter entry
fallback helper on mismatch
```

### 15.3 Builtin Call

Builtin call may lower to helper call.

Inlining builtin is allowed only if it preserves:

```text
capability checks
error category
source mapping
safepoint behavior
```

### 15.4 Function Call Boundary

Function calls are safepoint candidates.

Compiled-to-compiled calls must preserve frame maps.

Compiled-to-interpreted calls must bridge argument and result representation.

---

## 16. Access Lowering

### 16.1 Record Field Read

Fast path:

```text
load receiver
guard shape
load field by FieldIndex
```

Fallback:

```text
helper_get_attribute
```

### 16.2 Record Field Write

Fast path:

```text
load receiver
guard shape
check readonly
check field mutability
check type contract if needed
write field
emit write barrier
```

Fallback:

```text
helper_set_attribute
```

### 16.3 Index Read/Write

List index access may use guarded fast path.

Map access usually uses helper until map layout is optimized.

String indexing is not core.

### 16.4 Method Read

Method read may use shape/method-table guard and construct bound method.

If construction allocates, root map is required.

---

## 17. Arithmetic Lowering

### 17.1 Int Arithmetic

Int arithmetic must check overflow if using fixed-width representation.

Overflow raises `NumericOverflowError`.

### 17.2 Division

Division must check zero divisor.

Zero divisor raises `DivisionByZeroError`.

### 17.3 Float Arithmetic

Float arithmetic follows VM float semantics.

### 17.4 Mixed Numeric Types

If mixed numeric promotion is not defined, backend must not invent one.

Unsupported combinations call helper or raise `TypeError`.

---

## 18. Control and Unwind Lowering

### 18.1 Return

Compiled return must:

```text
respect return contract
enter cleanup if active regions exist
return to caller only after cleanup
```

### 18.2 Raise

Compiled raise must enter unwind path or call unwind helper.

### 18.3 Break/Continue

Compiled break/continue must preserve target region and cleanup.

### 18.4 Try/Finally

Compiled code may not skip finally.

If lowering cannot compile correct finally semantics, function remains interpreted or calls helper.

### 18.5 Use/Defer

Compiled code may not skip resource close or defer execution.

Region/defer/resource state must be visible to unwinding helper.

---

## 19. Capability and Host Boundary

### 19.1 Capability Lowering

Capability checks may lower to:

```text
inline capability guard
or helper_check_capability
```

Missing capability raises `CapabilityError`.

### 19.2 Host Call

Host calls always go through helper/host boundary.

Compiled code must not directly call arbitrary host/native function pointers.

### 19.3 FFI

FFI remains out of scope.

Future FFI must use VM-controlled capability-gated boundary.

---

## 20. Code Cache

### 20.1 JitCacheKey

```text
JitCacheKey {
  eir_digest: Digest
  runtime_plan_digest: Digest
  helper_table_digest: Digest
  value_layout_profile_digest: Digest
  gc_profile_digest: Digest
  target_profile_digest: Digest
  backend_id: JitBackendId
  backend_version: Version
  compile_mode: BaselineCompileMode
}
```

### 20.2 Invalidation Reasons

Compiled code invalidates on:

```text
EIR change
RuntimePlan change
helper table change
value layout change
GC profile change
target profile change
backend version change
module interface incompatible change
capability environment incompatible change
shape invalidation if dynamic shapes later exist
deopt metadata schema change
stack map schema change
```

### 20.3 Cache Rule

Compiled code is discardable.

It is not package ABI.

It must not be loaded if cache key does not match current VM.

---

## 21. JIT Validation

### 21.1 JitValidationReport

```text
JitValidationReport {
  compiled_function_id: CompiledFunctionId
  valid: Bool
  diagnostics: List<Diagnostic>
}
```

### 21.2 Validation Checks

JIT validation must check:

```text
all safepoints have stack maps
all helper safepoints expose roots
all guards have valid failure path
all heap writes have barrier path
all may-raise calls have unwind path
all capability operations preserve checks
all compiled returns preserve cleanup
all deopt points have frame maps
all source spans map to SIR nodes
all helper IDs match helper table digest
all value layout assumptions match profile
```

### 21.3 Validation Failure

If validation fails, compiled function must not be installed.

VM falls back to EIR interpretation.

---

## 22. Installation and Dispatch

### 22.1 Installation

A compiled function may be installed only after:

```text
successful compilation
successful validation
cache key match
helper table match
GC profile match
target profile match
```

### 22.2 Dispatch

Function dispatch may choose:

```text
EIR interpreter
baseline compiled function
optimized compiled function
```

based on hotness, validity, and current runtime state.

### 22.3 Deoptimization Dispatch

On deopt:

```text
reconstruct frame
restore slots
restore region stack
restore pending control
resume EIR interpreter or unwind helper
```

---

## 23. Cranelift-Compatible Boundary

### 23.1 Recommended First Backend

A Cranelift-compatible backend is recommended for baseline JIT.

The VM-side interface should provide:

```text
EIR operation stream
value layout hooks
runtime helper signatures
safepoint emission requests
stack map emission requests
deopt metadata
relocation records
```

### 23.2 Boundary Rule

The spec must not depend on Cranelift-specific IR names or APIs.

Cranelift is an implementation target, not a semantic dependency.

### 23.3 Future LLVM Backend

LLVM ORC or another backend may be introduced later as optimizing or AOT backend.

It must consume the same VM semantic metadata.

---

## 24. Security and Safety

Compiled code must not:

```text
bypass capability checks
mutate read-only values
skip write barriers
call arbitrary native pointers
expose VM object layout
assume CPython ABI
assume public bytecode ABI
skip structured unwinding
hide language errors as VM internal errors
```

Any violation is a VM correctness bug.

---

## 25. Non-Goals

This document does not define:

```text
actual machine code generation
register allocation
instruction selection
concrete Cranelift API
concrete LLVM API
optimizing JIT
OSR implementation
native ABI
FFI
public bytecode
debugger protocol
profiler format
```

---

## 26. Next Work

Next Phase 3 documents should define:

```text
fast interpreter concrete data structures
runtime helper implementation plan
GC implementation staging plan
JIT lowering matrix per EIR operation
Phase 3 consistency audit

```
