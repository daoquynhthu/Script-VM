# Phase 3 · JIT Lowering Matrix per EIR Operation
Document class: Normative specification
Normative status: This document defines VM semantics, constraints, interfaces, or required invariants.


Version: 0.14 JIT-lowering draft  
Depends on: Phase 3 GC Implementation Staging Plan v0.13  
Depends on: Phase 3 Baseline JIT Backend Interface v0.10  
Depends on: Phase 3 EIR Operation Semantics Round 1 v0.5  
Depends on: Phase 2 IR Design v1.0 frozen baseline  
Scope: JIT lowering classes for EIR operations, helper fallback requirements, safepoint/root/barrier/deopt requirements, baseline JIT gating, validation matrix  
Out of scope: concrete Cranelift IR, concrete LLVM IR, final machine-code generation, register allocation, optimizing JIT, public bytecode

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

This document classifies EIR operations for baseline JIT lowering.

It does not implement a JIT backend.

It defines which EIR operations can be compiled directly, which need guards, which must call helpers, which require deopt metadata, and which must remain interpreted.

The purpose is to prevent future baseline JIT implementation from silently bypassing:

```text
GC roots
safepoints
write barriers
capability checks
structured unwinding
source diagnostics
deoptimization
```

---

## 1. Lowering Classes

### 1.1 JitLoweringClass

```text
JitLoweringClass =
  | DirectLowering
  | GuardedFastPathWithHelperFallback
  | AlwaysCallHelper
  | DeoptRequired
  | InterpreterOnly
  | Forbidden
```

### 1.2 DirectLowering

The backend may emit direct machine/backend operations.

Requirements:

```text
no helper required
no semantic fallback required
all failure cases are locally checked
root metadata emitted if safepoint possible
```

### 1.3 GuardedFastPathWithHelperFallback

The backend emits:

```text
guard
fast path
fallback helper or generic EIR fallback
```

Requirements:

```text
guard failure path
source span
helper call metadata if helper fallback
deopt metadata if deopt fallback
root map if helper may allocate/collect
```

### 1.4 AlwaysCallHelper

The backend must call a runtime helper.

Use when the operation is:

```text
semantic-heavy
host/capability-sensitive
unwinding-sensitive
allocation/GC-sensitive
generic or megamorphic
```

### 1.5 DeoptRequired

The backend may compile only if a valid deopt point exists.

Use when the operation relies on speculative assumptions that cannot be handled by simple helper fallback.

### 1.6 InterpreterOnly

The operation remains in EIR interpreter.

The function may:

```text
remain interpreted
be split if backend supports partial compilation
call interpreter bridge
```

### 1.7 Forbidden

The operation must never be compiled.

If present in JIT input, compilation fails.

---

## 2. Matrix Columns

Each operation family is classified with:

```text
lowering_class
helper_fallback
safepoint_required
root_map_required
barrier_required
deopt_required
capability_sensitive
may_raise
may_allocate
notes
```

### 2.1 `may_raise`

Means operation can produce language-level Raise.

### 2.2 `may_allocate`

Means operation can allocate or call helper that allocates.

### 2.3 `root_map_required`

Required when operation reaches safepoint, helper call, allocation, or deopt.

### 2.4 `barrier_required`

Required for heap reference writes.

### 2.5 `capability_sensitive`

Operation must preserve capability check.

---

## 3. Constant Operations

### 3.1 ConstantOp

```text
lowering_class: DirectLowering
helper_fallback: none
safepoint_required: no
root_map_required: no
barrier_required: no
deopt_required: no
capability_sensitive: no
may_raise: no, except invalid constant pool is VM error
may_allocate: no, if constants are preallocated/interned
```

If loading a heap-backed constant lazily allocates, it becomes:

```text
AlwaysCallHelper or GuardedFastPathWithHelperFallback
```

with allocation safepoint and root map.

### 3.2 String Constant

Preferred baseline rule:

```text
module initialization preallocates or interns string constants
JIT loads handle
```

Lazy allocation is allowed only through allocation helper.

---

## 4. Load Operations

### 4.1 LoadSlot

```text
lowering_class: DirectLowering
may_raise: only if uninitialized check is required
```

If source slot may be uninitialized, emit check.

Uninitialized source binding raises `UninitializedBindingError`.

Internal uninitialized temporary is VM error.

### 4.2 LoadCell

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: optional helper_load_cell
may_raise: yes
```

Fast path:

```text
load cell
check initialized
load value
```

Failure:

```text
UninitializedBindingError
```

### 4.3 LoadCapture

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
```

Direct if capture layout is stable and initialized facts hold.

Otherwise guard/check.

### 4.4 LoadModuleSlot

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_load_module_slot or helper_import_named
may_raise: yes
```

Must preserve:

```text
module state checks
ImportCycleError
UninitializedBindingError
```

### 4.5 LoadField

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_get_attribute
may_raise: yes
```

Fast path:

```text
load receiver
unwrap readonly view if read
guard record shape
load field index
```

Requires no allocation unless fallback creates diagnostics.

### 4.6 LoadEnumPayload

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: pattern helper or enum payload helper
may_raise: context-dependent
```

Fast path:

```text
guard enum shape
guard case index
load payload index
```

In match-pattern context, failure branches instead of raising.

---

## 5. Store Operations

### 5.1 StoreSlot

```text
lowering_class: DirectLowering
barrier_required: if slot stores into heap-reachable cell/module storage
```

Direct local temporary write generally needs no barrier.

Writing into a source-visible cell uses StoreCell rules.

### 5.2 StoreCell

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_store_cell or helper_write_barrier
may_raise: yes
barrier_required: yes if heap ref write possible
```

Fast path must check:

```text
mutability
type contract if attached
write barrier
```

### 5.3 StoreModuleSlot

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: module slot helper
may_raise: yes
barrier_required: yes
```

Must respect module initialization/export state.

### 5.4 StoreField

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_set_attribute
may_raise: yes
barrier_required: yes
```

Fast path checks:

```text
readonly
record shape
field mutability
field type contract
write barrier
```

### 5.5 StoreListIndex

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_index_write
may_raise: yes
barrier_required: yes
```

Fast path supported for:

```text
List[Int existing index]
```

### 5.6 StoreMapEntry

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_index_write or helper_construct_map path
may_raise: yes
barrier_required: yes
may_allocate: possible
```

Reason:

```text
hashing
key equality
entry insertion/replacement
order preservation
possible table resize
```

A later optimized map layout may introduce guarded fast path.

---

## 6. Unary Operations

### 6.1 Unary Plus

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_numeric_unary or helper_numeric_binary family
may_raise: yes
```

Direct lowering only after type is known numeric.

### 6.2 Unary Minus

```text
lowering_class: GuardedFastPathWithHelperFallback
may_raise: yes
```

Fast path:

```text
int neg with overflow check
float neg
```

Unsupported types fallback/raise `TypeError`.

### 6.3 Not

```text
lowering_class: DirectLowering
may_raise: yes if operand not proven Bool
```

Emit Bool check.

No truthiness.

---

## 7. Binary Operations

### 7.1 Int Arithmetic

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_numeric_binary
may_raise: yes
deopt_required: no if helper fallback exists
```

Fast path:

```text
guard both Int
perform checked op
overflow -> raise NumericOverflowError or helper
division by zero -> raise DivisionByZeroError
```

### 7.2 Float Arithmetic

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_numeric_binary
may_raise: yes for unsupported types/division rules
```

### 7.3 String Add

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_string_concat or helper_numeric_binary equivalent
may_allocate: yes
safepoint_required: yes
root_map_required: yes
```

Could later get specialized allocation fast path.

### 7.4 List Add

```text
lowering_class: AlwaysCallHelper initially
may_allocate: yes
root_map_required: yes
```

Only if list concatenation is included in language semantics.

### 7.5 Equality

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_compare
may_raise: generally no, unless equality protocol can raise later
```

Fast paths:

```text
immediate equality
object identity shortcut
shape-known enum/record structural path if defined
```

### 7.6 Identity

```text
lowering_class: DirectLowering
may_raise: no
```

Uses VM identity, not physical address.

### 7.7 Comparisons

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_compare
may_raise: yes
```

Unsupported comparisons raise `TypeError`.

### 7.8 Membership

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_membership
may_raise: yes
```

Reason:

```text
list scan
map key hash/equality
future protocol possibility
```

---

## 8. Logical Operations

### 8.1 Logical And/Or

Preferred lowering is control flow.

```text
lowering_class: DirectLowering for branch structure
may_raise: yes if Bool check fails
```

Rules:

```text
short-circuit preserved
Bool-only
result is Bool
no operand-return semantics
```

A single eager LogicalOp is forbidden unless operands are already evaluated according to lowered short-circuit structure.

---

## 9. Check Operations

### 9.1 CheckBool

```text
lowering_class: DirectLowering
may_raise: yes
```

Checks tag/kind equals Bool.

### 9.2 CheckType

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_type_contract
may_raise: yes
```

Fast path for:

```text
ImmediateKindCheck
ShapeCheck
OptionalCheck simple
```

Complex union/function/extension checks call helper.

### 9.3 CheckCallable

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_callable
may_raise: yes
```

### 9.4 CheckArity

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_check_arity or generic call helper
may_raise: yes
```

Can later specialize for monomorphic calls.

### 9.5 CheckHashable

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_hashable
may_raise: yes
```

### 9.6 CheckReadonly

```text
lowering_class: DirectLowering
may_raise: yes
```

### 9.7 CheckCapability

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_capability
capability_sensitive: yes
may_raise: yes
```

JIT must never eliminate capability check unless capability state is proven immutable and guard/deopt is present.

### 9.8 CheckShape

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
helper_fallback: helper_check_shape
may_raise: context-dependent
```

Direct shape equality is preferred.

### 9.9 CheckOverflow / CheckDivisionByZero

```text
lowering_class: DirectLowering
may_raise: yes
```

Semantic checks, not speculative guards.

---

## 10. Call Operations

### 10.1 Generic CallOp

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_generic_call
safepoint_required: yes
root_map_required: yes
may_raise: yes
may_allocate: possible
```

### 10.2 Monomorphic User Function Call

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_generic_call
safepoint_required: yes
root_map_required: yes
deopt_required: optional
```

Fast path:

```text
guard callee identity/function id
marshal args
call compiled/interpreter function entry
fallback on mismatch
```

### 10.3 Builtin Call

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_call_builtin
capability_sensitive: maybe
safepoint_required: descriptor-dependent
```

Inlining allowed only with descriptor-preserving checks.

### 10.4 Constructor Call

Record/enum constructor calls may lower as construction ops.

Generic constructor value call uses helper.

### 10.5 Method Call

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_generic_call or helper_bind_method
```

Must preserve receiver binding and source diagnostics.

### 10.6 HostFunction Call

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_enter_host_call + helper_exit_host_call
capability_sensitive: yes
safepoint_required: yes
root_map_required: yes
```

Compiled code must not directly call arbitrary host pointers.

---

## 11. Access Operations

### 11.1 AttributeRead

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_get_attribute
may_raise: yes
```

Fast path for known record shape.

### 11.2 AttributeWrite

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_set_attribute
may_raise: yes
barrier_required: yes
```

### 11.3 MethodRead

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_bind_method
may_allocate: possible
root_map_required: if allocation possible
```

### 11.4 IndexRead

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_index_read
may_raise: yes
```

Fast path for List[Int].

Map read helper initially.

### 11.5 IndexWrite

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_index_write
barrier_required: yes
may_raise: yes
```

Fast path for List[Int].

Map write helper initially.

### 11.6 SliceRead

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_slice_read
may_allocate: yes
safepoint_required: yes
root_map_required: yes
```

---

## 12. Construction Operations

### 12.1 ConstructList

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: allocation helper or construct_list helper
may_allocate: yes
safepoint_required: yes
root_map_required: yes
```

Later direct allocation fast path allowed.

### 12.2 ConstructMap

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_construct_map
may_allocate: yes
safepoint_required: yes
root_map_required: yes
may_raise: yes
```

Needs hashability, duplicate-key semantics, order preservation.

### 12.3 ConstructRecord

```text
lowering_class: GuardedFastPathWithHelperFallback or AlwaysCallHelper initially
helper_fallback: helper_construct_record
may_allocate: yes
safepoint_required: yes
root_map_required: yes
may_raise: yes
```

Direct fast path possible for known shape and prechecked fields.

### 12.4 ConstructEnumValue

```text
lowering_class: GuardedFastPathWithHelperFallback or AlwaysCallHelper initially
helper_fallback: helper_construct_enum
may_allocate: yes
root_map_required: yes
may_raise: yes
```

### 12.5 ConstructFunction

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: alloc_function/helper_construct_function
may_allocate: yes
root_map_required: yes
```

Must preserve declaration-time construction and capture semantics.

### 12.6 ConstructError

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_construct_error
may_allocate: yes
root_map_required: yes
```

---

## 13. Pattern Operations

### 13.1 PatternCheckLiteral

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern or helper_compare
may_raise: context-dependent
```

In match context, failure branches.

In destructuring context, failure raises.

### 13.2 PatternCheckRecordShape

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern
```

### 13.3 PatternCheckEnumCase

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern
```

### 13.4 PatternCheckListLength

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_match_pattern
```

### 13.5 PatternCheckMapKey

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_match_pattern
```

Map key checks require hashing/equality.

### 13.6 PatternBind

```text
lowering_class: DirectLowering
barrier_required: if binding slot/cell write stores heap ref
```

Binding commit must preserve pattern success semantics.

### 13.7 PatternBranch

```text
lowering_class: DirectLowering
```

---

## 14. RuntimeHelperOp

### 14.1 RuntimeHelperOp

```text
lowering_class: AlwaysCallHelper
safepoint_required: descriptor-dependent
root_map_required: if helper may allocate/collect/raise/unwind
capability_sensitive: descriptor-dependent
```

The JIT must use helper descriptor exactly.

### 14.2 Helper That May Collect

Must have:

```text
SafepointRecord
RootMap
StackMap
HelperCallSite metadata
```

### 14.3 Helper That May Raise

Must have:

```text
source span
unwind path
FrameMap
```

### 14.4 Helper That May Unwind

Must have:

```text
region stack metadata
pending control metadata
deopt/unwind bridge
```

---

## 15. SafepointOp

### 15.1 SafepointOp

```text
lowering_class: DirectLowering
safepoint_required: yes
root_map_required: yes
```

Emits poll sequence or safepoint call.

### 15.2 Fast Poll

Loop safepoints may use fast epoch check.

Slow path calls VM safepoint handler.

### 15.3 GC Compatibility

If root map unavailable, JIT compilation must fail.

---

## 16. GuardOp

### 16.1 Semantic Guards

```text
lowering_class: DirectLowering
may_raise: yes
```

Examples:

```text
NonZeroDivisor
NoOverflow
NotReadOnly
```

Semantic guard failure raises language error.

### 16.2 Speculative Guards

```text
lowering_class: DeoptRequired or GuardedFastPathWithHelperFallback
deopt_required: if no helper fallback
```

Examples:

```text
IsCallTarget
HasShape for speculative optimization
IsType for specialized arithmetic
ModuleStateIs for import shortcut
```

### 16.3 Guard Failure

Every guard must have valid failure action:

```text
Fallback
Helper
Deopt
Raise
```

Compilation rejects missing failure path.

---

## 17. Terminators

### 17.1 Jump

```text
lowering_class: DirectLowering
```

Block arguments must be transferred without clobbering.

### 17.2 Branch

```text
lowering_class: DirectLowering
may_raise: yes if condition not proven Bool
```

Must preserve Bool-only condition.

### 17.3 Return

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_perform_unwind
may_raise: yes if cleanup raises
```

If no active cleanup and return contract already checked, direct return allowed.

### 17.4 Raise

```text
lowering_class: AlwaysCallHelper or GuardedFastPathWithHelperFallback
helper_fallback: helper_raise / helper_perform_unwind
may_raise: yes
```

### 17.5 LoopBackedge

```text
lowering_class: DirectLowering
safepoint_required: yes
root_map_required: yes
```

May update hotness counters.

### 17.6 Switch

```text
lowering_class: DirectLowering or GuardedFastPathWithHelperFallback
```

Direct for dense enum case/pattern decisions.

Fallback for generic values.

### 17.7 Unwind

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_perform_unwind
may_raise: yes
may_unwind: yes
root_map_required: yes
```

### 17.8 Unreachable

```text
lowering_class: Forbidden
```

If reached in compiled code, report InternalVMError.

---

## 18. Structured Control Constructs

Although structured control lowers to EIR operations and terminators, the JIT must preserve construct-level invariants.

### 18.1 If

```text
lowering_class: DirectLowering over branches
```

All conditions require Bool check.

### 18.2 While

```text
lowering_class: DirectLowering with safepoint backedge
```

Loop backedge must expose roots.

### 18.3 For

```text
lowering_class: AlwaysCallHelper or GuardedFastPathWithHelperFallback initially
```

Iterator polling may call helper.

Map iteration order must be preserved.

### 18.4 Try/Catch/Finally

```text
lowering_class: AlwaysCallHelper for unwinding portions initially
```

JIT may compile body blocks but must call unwind/finally helper where needed.

### 18.5 Use/Defer

```text
lowering_class: AlwaysCallHelper for registration/execution initially
```

Compiled code must not remove cleanup paths.

### 18.6 Match

```text
lowering_class: DirectLowering for simple shape/case decision
lowering_class: AlwaysCallHelper for generic pattern fallback
```

Case order and guard timing must be preserved.

---

## 19. Module Operations

### 19.1 Import

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_resolve_module / helper_initialize_module / helper_import_named
safepoint_required: yes
root_map_required: yes
may_raise: yes
may_allocate: yes
```

### 19.2 Export Sealing

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_seal_exports
```

### 19.3 Module State Check

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: module helper
may_raise: yes
```

JIT may guard module state if safe.

---

## 20. Capability Operations

### 20.1 Capability Check

```text
lowering_class: GuardedFastPathWithHelperFallback
helper_fallback: helper_check_capability
capability_sensitive: yes
may_raise: yes
```

### 20.2 Host Boundary

```text
lowering_class: AlwaysCallHelper
helper_fallback: helper_enter_host_call / helper_exit_host_call
capability_sensitive: yes
safepoint_required: yes
root_map_required: yes
```

Compiled code cannot directly call host/native effectful functions.

---

## 21. Allocation Operations

### 21.1 Generic Allocation

```text
lowering_class: AlwaysCallHelper initially
helper_fallback: helper_alloc_object
safepoint_required: yes
root_map_required: yes
may_allocate: yes
```

### 21.2 Inline Allocation

Future:

```text
lowering_class: GuardedFastPathWithHelperFallback
```

Requirements:

```text
bump pointer fast path
allocation limit guard
safepoint fallback
root map
object initialization safety
write barrier if storing refs
```

Baseline JIT may defer inline allocation.

---

## 22. Write Barrier Operations

### 22.1 Write Barrier

```text
lowering_class: DirectLowering or AlwaysCallHelper
helper_fallback: helper_write_barrier
barrier_required: yes
```

If GC profile says no barrier needed, lowering may emit no-op.

The barrier site must still exist in lowering metadata.

### 22.2 Generational Profile

Under generational GC profile, JIT must emit actual barrier or call helper.

---

## 23. Source Mapping Requirements

Every compiled EIR op that may raise, call helper, deopt, or hit safepoint must map to:

```text
CodeOffset -> EirLocation -> SIR NodeId -> SourceSpan
```

Missing source map is a JIT validation error for may-raise operations.

---

## 24. Root Map Requirements

Root maps are required at:

```text
helper calls that may allocate/collect/raise/unwind
allocation points
loop backedge safepoints
function call safepoints
host call boundaries
deopt points
raise boundaries
```

If root map is missing, JIT compilation fails.

---

## 25. Deopt Requirements

Deopt metadata required for:

```text
speculative type specialization
speculative shape specialization
speculative call target specialization
inlined calls
elided checks
OSR entry/exit
compiled code side exit without helper fallback
```

Baseline JIT may avoid these features to reduce deopt burden.

---

## 26. Barrier Requirements

Write barrier required for:

```text
StoreField
StoreListIndex
StoreMapEntry
StoreCell when cell stores heap ref
StoreModuleSlot when module slot stores heap ref
ConstructRecord field initialization if object published before all stores
ConstructList/Map internal ref stores under generational/incremental profiles
Error suppressed list mutation
Resource metadata mutation
```

Bootstrap no-op barrier still requires call site.

---

## 27. Capability Requirements

Capability checks must be preserved for:

```text
host calls
module resolver if host marks it capability-gated
fs/net/process/env/random/clock helpers
future FFI
effectful builtins
```

JIT cannot fold capability check away unless capability environment immutability and guard/deopt are specified.

---

## 28. Lowering Matrix Summary

```text
EIR family              Baseline class
ConstantOp              Direct
LoadSlot                Direct
LoadCell                Guarded/Helper
LoadField               Guarded/Helper
StoreSlot               Direct
StoreField              Guarded/Helper + Barrier
StoreMapEntry           Helper
UnaryOp                 Guarded/Helper
BinaryOp                Guarded/Helper
LogicalOp               Direct control-flow
CheckBool               Direct
CheckType               Guarded/Helper
CheckCapability         Guarded/Helper
CallOp generic          Helper
CallOp monomorphic      Guarded/Helper
AccessOp record         Guarded/Helper
AccessOp map/string     Helper initially
ConstructList           Helper initially
ConstructMap            Helper
ConstructRecord         Guarded/Helper or Helper
ConstructEnum           Guarded/Helper or Helper
ConstructFunction       Helper
Pattern simple          Direct/Guarded
Pattern map/generic     Helper
RuntimeHelperOp         Helper
SafepointOp             Direct safepoint
GuardOp semantic        Direct
GuardOp speculative     Deopt/Helper
Jump                    Direct
Branch                  Direct
Return                  Direct only if no cleanup; otherwise Helper/Unwind
Raise                   Helper/Unwind
LoopBackedge            Direct + Safepoint
Switch                  Direct/Guarded
Unwind                  Helper initially
Unreachable             Forbidden
Import                  Helper
Host call               Helper
Allocation              Helper initially
Write barrier           Direct or Helper
```

---

## 29. JIT Validation Rules

JIT validation must reject:

```text
compiled helper call without descriptor
compiled may-collect helper without root map
compiled safepoint without stack map
compiled heap write without barrier path
compiled guard without failure path
compiled speculative op without deopt/helper fallback
compiled may-raise op without source map
compiled return that skips active cleanup
compiled raise that skips unwind
compiled host call that bypasses helper boundary
compiled capability operation without check
compiled operation that assumes Rust Value enum layout
compiled operation that assumes object address identity
```

---

## 30. Implementation Staging

### 30.1 Stage J0 · Classification Only

Add JIT lowering class annotations to EIR op definitions.

### 30.2 Stage J1 · Validation

Implement validation that rejects unsupported JIT lowering cases.

### 30.3 Stage J2 · Direct Lowering Subset

Compile:

```text
ConstantOp
LoadSlot
StoreSlot
CheckBool
simple Branch
Jump
LoopBackedge poll skeleton
Identity comparison
```

### 30.4 Stage J3 · Helper Calls

Compile RuntimeHelperOp and generic helper call sequences with root maps.

### 30.5 Stage J4 · Guarded Record/Int Fast Paths

Compile:

```text
int arithmetic with guards
record field read/write with shape guard
list index read/write with guards
```

### 30.6 Stage J5 · Calls

Compile monomorphic call fast path with helper fallback.

### 30.7 Stage J6 · Structured Control Integration

Compile return/raise/unwind integration with helper_perform_unwind.

### 30.8 Stage J7 · GC Profile Integration

Enable JIT under collecting GC profile only after stack maps/root maps pass validation.

---

## 31. Non-Goals

This document does not define:

```text
Cranelift IR lowering
LLVM IR lowering
machine register allocation
code memory manager
OSR implementation
optimizing JIT
inline function optimization
escape analysis
native ABI
FFI
public bytecode
```

---

## 32. Next Work

Next Phase 3 documents should define:

```text
fast interpreter implementation milestones
Phase 3 consistency audit
Phase 3 freeze-readiness checklist

```
