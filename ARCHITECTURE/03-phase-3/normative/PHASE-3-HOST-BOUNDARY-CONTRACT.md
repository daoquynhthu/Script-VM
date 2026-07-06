# Phase 3 · Host Boundary Contract

Document class: Normative specification  
Normative status: This document defines the canonical host boundary, host function wrapper, host object wrapper, host root registry, host error normalization, capability gating, and deferred FFI boundary for Phase 3 VM specifications.

Created: 2026-06-29 09:31:28

---

## 0. Purpose

This document repairs second-stage audit item:

```text
S2-R3: Add canonical host boundary contract.
```

It addresses residual finding:

```text
S2-M03: Host boundary / FFI deferred status should be sharpened.
```

---

## 1. Host Boundary

The host boundary is the VM-controlled interface through which VM execution may interact with host-provided services.

Examples:

```text
module resolver
builtin host functions
fs/net/process/env/random/clock providers
test runner hooks
debug/profiling hooks
future FFI gateway
```

The host boundary is not a public native extension ABI.

---

## 2. Host Boundary Non-Goals

Phase 3 host boundary MUST NOT imply:

```text
CPython C API compatibility
CPython ABI compatibility
Python wheel compatibility
native plugin ABI
stable VM object layout
raw pointer ownership by host
public helper ABI
```

FFI remains DEFERRED unless explicitly enabled by a later normative document.

---

## 3. HostFunctionWrapper

```text
HostFunctionWrapper {
  host_function_id: HostFunctionId
  descriptor: HostFunctionDescriptor
  capability?: CapabilityId
  effect?: EffectId
  source_span?: SourceSpanId
}
```

### 3.1 HostFunctionDescriptor

```text
HostFunctionDescriptor {
  arity: ArityShape
  parameter_policy: HostParameterPolicy
  result_policy: HostResultPolicy
  may_allocate: Bool
  may_raise: Bool
  may_block: Bool
  may_reenter_vm: Bool
  requires_roots_visible: Bool
}
```

---

## 4. HostObjectWrapper

```text
HostObjectWrapper {
  host_object_id: HostObjectId
  descriptor: HostObjectDescriptor
  capability_origin?: CapabilityId
  lifetime: HostObjectLifetime
}
```

HostObjectWrapper may hold native host state.

It MUST NOT expose raw VM object pointers to host.

It MUST NOT let host retain VM values except through HostRootRegistry.

---

## 5. HostRootRegistry

```text
HostRootRegistry {
  roots: Map<HostRootId, HostRootEntry>
}
```

```text
HostRootEntry {
  value: Value
  owner: HostBoundaryId
  lifetime: HostRootLifetime
  capability?: CapabilityId
}
```

```text
HostRootLifetime =
  | CallScoped
  | ResourceScoped
  | ExplicitHandle
```

Host code MUST NOT retain VM values beyond a call unless a HostRootEntry exists.

---

## 6. Host Call Protocol

Host call execution order:

```text
1. resolve HostFunctionWrapper
2. check required capability
3. make VM roots visible if descriptor requires
4. register host call frame
5. marshal VM values to host boundary representation
6. call host function
7. normalize host result or error
8. unregister call-scoped host roots
9. return Value, VmControl::Raise, or VmError
```

Compiled code MUST NOT skip this protocol.

---

## 7. Capability Gating

Every effectful host operation MUST declare:

```text
CapabilityId
EffectId
```

Missing capability raises:

```text
CapabilityError
```

Capability environment mutability and cache invalidation follow Target/Profile schema and cache compatibility matrix.

---

## 8. Host Error Normalization

Host exceptions/errors MUST be normalized.

Possible outcomes:

```text
LanguageError as VmControl::Raise(ErrorHandle)
VmStructuralError as VmError
host cancellation/interruption if later defined
```

Raw host exceptions MUST NOT cross into VM interpreter/JIT as untyped host exceptions.

---

## 9. Host Reentrancy

If host function may reenter VM:

```text
may_reenter_vm = true
```

VM MUST preserve:

```text
call stack
region stack
pending control
roots
helper state
source diagnostics
```

Host reentrancy MUST NOT corrupt structured unwinding.

---

## 10. Host Blocking and Safepoints

If host function may block:

```text
may_block = true
```

The VM SHOULD treat entry/exit as safepoint-capable.

If host call may allocate/trigger GC/reenter VM, roots MUST be visible.

---

## 11. Module Resolver

Module resolver is a host boundary component.

If resolver touches filesystem, network, environment, or other effects, it MUST be capability-gated.

Module resolver failures normalize to:

```text
ImportError
CapabilityError
VmStructuralError
```

depending on failure kind.

---

## 12. Resource Ownership

Host resources exposed to VM MUST be represented by ResourceObj or HostObjectWrapper with explicit lifetime policy.

GC finalization MUST NOT be required for language resource cleanup.

Structured `use`/`defer` cleanup remains canonical.

---

## 13. FFI Deferred Boundary

FFI is DEFERRED in Phase 3 minimal VM.

Deferred FFI is constrained by:

```text
no stable VM object layout exposure
no direct ObjRef raw pointer ownership
no CPython API/ABI compatibility promise
capability-gated effects
host root registration for retained VM values
error normalization
source diagnostics where possible
```

---

## 14. JIT Requirements

JIT compiled code MUST NOT directly call arbitrary host/native pointers.

Host calls from JIT MUST go through:

```text
helper_enter_host_call
host call trampoline
helper_exit_host_call
```

or equivalent VM-controlled boundary preserving this contract.

---

## 15. Validation

Host boundary validation MUST reject:

```text
effectful host call without capability metadata
host function descriptor missing arity policy
host call retaining VM value without HostRoot
host object exposing raw VM pointer
JIT direct host pointer call
host error path without normalization
module resolver effect without capability
FFI feature use without explicit enablement
```

---

## 16. Audit Tracking

This document completes:

```text
S2-R3
```

It addresses:

```text
S2-M03
```

It partially supports:

```text
M-13
M-05
M-11
```
