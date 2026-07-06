# Phase 3 · Target and Runtime Profile Schemas

Document class: Normative specification  
Normative status: This document defines ValueLayoutProfile, HeapProfile, GcProfile, InterpreterProfile, JitProfile, TargetProfile, and their cache compatibility roles.

Created: 2026-06-29 09:26:37

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
R11: Define ValueLayoutProfile / HeapProfile / GcProfile / TargetProfile schemas.
```

It addresses major finding:

```text
M-04: ValueLayoutProfile is referenced before being normatively defined.
```

---

## 1. Profile Boundary

Profiles are VM-internal compatibility descriptors.

They are not public ABI.

They participate in:

```text
RuntimePlan cache key
EIR cache key
JIT cache key
helper table compatibility
GC metadata compatibility
module dependency compatibility
```

---

## 2. RuntimeTargetProfile

```text
RuntimeTargetProfile {
  profile_version: Version
  vm_version: Version
  architecture: TargetArchitecture
  operating_system?: TargetOperatingSystem
  pointer_width: UInt
  endianness: Endianness
  value_layout_profile: ValueLayoutProfile
  heap_profile: HeapProfile
  gc_profile: GcProfile
  interpreter_profile: InterpreterProfile
  jit_profile?: JitProfile
  capability_profile: CapabilityProfile
}
```

---

## 3. ValueLayoutProfile

```text
ValueLayoutProfile {
  value_layout_id: String
  representation: ValueRepresentation
  immediate_kinds: List<RuntimeValueKind>
  heap_ref_kinds: List<RuntimeValueKind>
  identity_policy: IdentityPolicy
  numeric_int_policy: NumericIntPolicy
  float_policy: FloatPolicy
}
```

### 3.1 ValueRepresentation

```text
ValueRepresentation =
  | RustEnumBootstrap
  | TaggedPointer
  | NaNBoxing
  | CompressedHandle
  | OpaqueHandle
```

No document may assume Rust enum layout unless profile explicitly says `RustEnumBootstrap`.

Even then, Rust enum layout remains internal and not public ABI.

### 3.2 NumericIntPolicy

```text
NumericIntPolicy =
  | CheckedI64
  | ArbitraryPrecision
```

Silent integer wrap is forbidden.

### 3.3 FloatPolicy

```text
FloatPolicy {
  format: FloatFormat
  finite_only_for_serializable: Bool
  nan_key_policy: FloatNaNKeyPolicy
  negative_zero_policy: NegativeZeroPolicy
}
```

---

## 4. HeapProfile

```text
HeapProfile {
  heap_profile_id: String
  object_reference_model: ObjectReferenceModel
  object_store_model: ObjectStoreModel
  object_identity_model: ObjectIdentityModel
  moving_allowed: Bool
  stale_handle_detection: Bool
}
```

```text
ObjectReferenceModel =
  | ObjRefHandle
  | HandleTable
  | DirectTaggedReference
  | CompressedReference
```

```text
ObjectStoreModel =
  | Arena
  | GenerationalArena
  | SlotMap
  | MovingHeap
  | Custom
```

---

## 5. GcProfile

```text
GcProfile {
  gc_profile_id: String
  collection_model: CollectionModel
  moving: Bool
  generational: Bool
  incremental: Bool
  concurrent: Bool
  requires_write_barrier: Bool
  requires_precise_roots: Bool
  safepoint_policy: SafepointPolicy
}
```

```text
CollectionModel =
  | NoCollectionBootstrap
  | NonMovingTracing
  | GenerationalTracing
  | MovingCompacting
  | Incremental
  | Concurrent
```

Moving GC requires:

```text
requires_precise_roots = true
```

Generational or incremental GC requires:

```text
requires_write_barrier = true
```

---

## 6. InterpreterProfile

```text
InterpreterProfile {
  interpreter_profile_id: String
  dispatch_model: InterpreterDispatchModel
  root_mode: InterpreterRootMode
  quickening_enabled: Bool
  feedback_enabled: Bool
  deterministic_mode: Bool
}
```

```text
InterpreterDispatchModel =
  | MatchDispatch
  | FunctionTableDispatch
  | ThreadedDispatch
  | QuickenedDispatch
```

```text
InterpreterRootMode =
  | AllInitializedSlots
  | LivenessDerivedSlots
  | DebugAllSlots
```

---

## 7. JitProfile

```text
JitProfile {
  jit_profile_id: String
  enabled: Bool
  backend_kind?: JitBackendKind
  backend_version?: Version
  compile_mode?: BaselineCompileMode
  stack_maps_required: Bool
  deopt_required: Bool
  helper_trampoline_abi: HelperTrampolineAbiProfile
}
```

If JIT is disabled, JitProfile may be absent.

If JIT is enabled under moving GC:

```text
stack_maps_required = true
```

---

## 8. CapabilityProfile

```text
CapabilityProfile {
  environment_model: CapabilityEnvironmentModel
  mutability_policy: CapabilityEnvironmentMutability
  digest_or_epoch_policy: CapabilityDigestPolicy
}
```

```text
CapabilityEnvironmentMutability =
  | ImmutableForVmRun
  | ImmutablePerModule
  | MutableWithEpoch
  | HostControlledWithInvalidation
```

JIT may cache capability checks only if mutability policy and invalidation are defined.

---

## 9. TargetArchitecture

```text
TargetArchitecture =
  | X86_64
  | AArch64
  | Wasm
  | InterpreterPseudoTarget
  | Other
```

---

## 10. Digest Participation

RuntimeTargetProfile digest MUST include:

```text
ValueLayoutProfile
HeapProfile
GcProfile
InterpreterProfile
JitProfile if present
CapabilityProfile
architecture
pointer width
endianness
VM version
profile version
```

Changing any profile field invalidates dependent RuntimePlan/EIR/JIT caches unless explicitly marked non-semantic and non-layout-affecting.

---

## 11. Validation

Profile validation MUST reject:

```text
moving GC without precise roots
generational GC without write barrier requirement
JIT enabled under moving GC without stack maps
RustEnumBootstrap exposed as public ABI
CheckedI64 without overflow error policy
capability mutable without epoch/invalidation policy
direct tagged refs with host raw pointer exposure
```

---

## 12. Audit Tracking

This document completes:

```text
R11
```

It addresses:

```text
M-04
M-05 partial
M-11 partial
```
