# Phase 3 · Cache Compatibility Matrix

Document class: Normative specification  
Normative status: This document defines canonical cache compatibility and invalidation rules for RuntimePlan, EIR, helper table, GC metadata, JIT artifacts, module interfaces, and capability profiles.

Created: 2026-06-29 09:28:58

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
R14: Create canonical cache compatibility matrix.
```

It addresses major finding:

```text
M-11: Internal cache compatibility rules are distributed and need consolidation.
```

---

## 1. Cache Boundary

All Phase 3 caches are internal.

No cache is:

```text
public bytecode
package ABI
native ABI
foreign extension ABI
portable user artifact
```

Caches are discardable.

If compatibility cannot be proven, the VM MUST discard and rebuild the cache.

---

## 2. Cache Kinds

```text
RuntimePlanCache
EirCache
HelperRegistryCache
ModuleInterfaceCache
GcMetadataCache
JitCodeCache
ProfileCache
DiagnosticSourceMapCache
```

---

## 3. Universal Cache Key Components

Every cache key MUST include or be derived from:

```text
vm_version
phase3_schema_version
document/spec revision id or equivalent
target/runtime profile digest where relevant
feature set
source digest where relevant
dependency digest where relevant
```

---

## 4. RuntimePlan Cache Key

RuntimePlan cache key MUST include:

```text
source_sir_digest
phase2_schema_version
phase3_runtimeplan_schema_version
vm_version
RuntimeTargetProfile digest
feature set
dependency module interface digests
stdlib interface digest
helper registry digest
capability profile digest or epoch policy
```

Invalidation required when any component changes.

---

## 5. EIR Cache Key

EIR cache key MUST include:

```text
RuntimePlan digest
EIR schema version
VM version
target profile digest
helper registry digest
GC metadata schema version
source map digest
```

Invalidation required when:

```text
EIR op schema changes
RuntimePlan changes
helper signatures change
target/value layout changes
GC/safepoint profile changes
source mapping changes in a way that affects diagnostics
```

---

## 6. Helper Registry Digest

Helper registry digest MUST include for each helper:

```text
helper id
canonical name
family
signature
result type
may_allocate
may_raise
may_unwind
is_safepoint
requires_roots_visible
required_capability
effect
gc_behavior
jit_call_policy
source_mapping_policy
```

Changing helper implementation without descriptor change MAY avoid cache invalidation only if behavior is semantically identical.

Changing helper descriptor MUST invalidate dependent RuntimePlan/EIR/JIT caches.

---

## 7. Module Interface Cache Key

Module interface cache key MUST include:

```text
module source digest or semantic digest
export table shape
exported names
exported binding identities
exported type/interface descriptors
feature set
dependency interface digests
stdlib interface digest
```

Changing implementation body without interface change MAY preserve dependent import compatibility.

Changing exported shape or required interface MUST invalidate dependents.

---

## 8. GC Metadata Cache Key

GC metadata cache key MUST include:

```text
RootMap schema version
FrameMap schema version
SafepointRecord schema version
StackMap schema version if JIT
GcProfile digest
HeapProfile digest
ValueLayoutProfile digest
write barrier policy
moving/nonmoving policy
```

Invalidation required when:

```text
root representation changes
slot layout changes
frame layout changes
GC profile changes
moving policy changes
barrier policy changes
JIT stack map schema changes
```

---

## 9. JIT Code Cache Key

JIT code cache key MUST include:

```text
EIR digest
RuntimePlan digest
helper registry digest
ValueLayoutProfile digest
HeapProfile digest
GcProfile digest
JitProfile digest
target architecture
pointer width
endianness
backend kind
backend version
compile mode
source map digest
deopt metadata digest
stack map digest
capability profile digest/epoch policy
```

JIT cache MUST be invalidated when any safety-relevant metadata changes.

JIT cache MUST NOT be reused across incompatible target profiles.

---

## 10. Capability Cache Compatibility

Capability environment policy controls cacheability.

```text
ImmutableForVmRun
  cache may assume capability set for one VM run.

ImmutablePerModule
  cache may assume capability set per module, keyed by module capability digest.

MutableWithEpoch
  cache must include capability epoch and invalidate on epoch change.

HostControlledWithInvalidation
  cache valid only if host invalidation protocol is active.
```

JIT may cache capability checks only under an explicit capability profile.

---

## 11. Diagnostic Source Map Cache

Diagnostic source map cache key MUST include:

```text
source file digest
SIR source map digest
RuntimePlan source map digest
EIR source map digest
VM version
```

Source map cache mismatch MUST NOT produce incorrect source diagnostics.

If mismatch is detected, diagnostics cache must be rebuilt or execution rejected in checked mode.

---

## 12. Cache Failure Policy

Cache compatibility failure MUST result in one of:

```text
discard and rebuild
fallback to interpreter/lower tier
VmStructuralError if no safe fallback exists
```

Cache compatibility failure MUST NOT be converted to ordinary language Error.

---

## 13. Public Artifact Rule

The VM MUST NOT promise users that cached RuntimePlan/EIR/JIT artifacts are stable portable files.

A package manager or build system MUST treat such caches as internal, rebuildable artifacts.

---

## 14. Validation

Cache validation MUST reject:

```text
RuntimePlan cache with mismatched SIR digest
EIR cache with mismatched RuntimePlan digest
JIT cache with mismatched ValueLayoutProfile
JIT cache with mismatched helper registry digest
GC metadata cache with mismatched RootMap schema
module interface cache with incompatible export shape
capability-sensitive cache without digest/epoch policy
cache file claiming public bytecode status
cache crossing VM version without compatibility marker
```

---

## 15. Audit Tracking

This document completes:

```text
R14
```

It addresses:

```text
M-11
```

It supports:

```text
R15
```
