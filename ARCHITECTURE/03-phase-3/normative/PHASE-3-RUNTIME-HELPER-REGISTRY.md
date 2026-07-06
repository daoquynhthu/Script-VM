# Phase 3 · Runtime Helper Registry

Document class: Normative specification  
Normative status: This document defines the canonical runtime helper registry for Phase 3 VM specifications.

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
R5: Create canonical helper registry.
```

It resolves blocker:

```text
B-03: Helper names are referenced outside the helper contract without a canonical helper registry.
```

Runtime helpers are VM-internal semantic slow paths. They are not public ABI.

---

## 1. Registry Rule

Every `RuntimeHelperOp` MUST reference a helper declared in this registry.

No Phase 3 normative document may introduce a helper name outside this registry without amending this registry.

Helper IDs are internal and VM-versioned.

Helper names are descriptive and internal.

---

## 2. Helper Descriptor Schema

Every helper has:

```text
RuntimeHelperEntry {
  helper_id: RuntimeHelperId
  name: HelperName
  family: RuntimeHelperFamily
  signature: RuntimeHelperSignature
  result: HelperResultType
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

---

## 3. Canonical Helper Registry

| Canonical name | Family | Result | Allocate | Raise | Unwind | Safepoint | Roots |
|---|---|---|---:|---:|---:|---:|---:|
| `helper_alloc_object` | Allocation | Value | yes | yes | no | yes | yes |
| `helper_write_barrier` | WriteBarrier | Unit | no | no | no | no | no |
| `helper_construct_error` | Error | Value | yes | no | no | yes | yes |
| `helper_raise` | Error | VmControl | no | yes | yes | no | no |
| `helper_attach_suppressed` | Error | Unit | yes | no | no | yes | yes |
| `helper_assert_fail` | Error | VmControl | yes | yes | yes | yes | yes |
| `helper_check_type_contract` | TypeCheck | Value | no | yes | no | no | no |
| `helper_check_callable` | TypeCheck | Value | no | yes | no | no | no |
| `helper_check_hashable` | TypeCheck | Value | no | yes | no | no | no |
| `helper_check_shape` | TypeCheck | Bool | no | no | no | no | no |
| `helper_numeric_unary` | Numeric | Value | no | yes | no | no | no |
| `helper_numeric_binary` | Numeric | Value | no | yes | no | no | no |
| `helper_compare` | Numeric | Value | no | yes | no | no | no |
| `helper_get_attribute` | Access | Value | maybe | yes | no | maybe | maybe |
| `helper_set_attribute` | Access | Unit | maybe | yes | no | maybe | maybe |
| `helper_bind_method` | Access | Value | yes | yes | no | yes | yes |
| `helper_index_read` | Access | Value | maybe | yes | no | maybe | maybe |
| `helper_index_write` | Access | Unit | maybe | yes | no | maybe | maybe |
| `helper_slice_read` | Access | Value | yes | yes | no | yes | yes |
| `helper_membership` | Access | Value | maybe | yes | no | maybe | maybe |
| `helper_construct_list` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_map` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_record` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_enum` | Construction | Value | yes | yes | no | yes | yes |
| `helper_construct_function` | Construction | Value | yes | yes | no | yes | yes |
| `helper_generic_call` | Call | VmControl | maybe | yes | yes | yes | yes |
| `helper_call_builtin` | Call | VmControl | maybe | yes | yes | maybe | maybe |
| `helper_check_arity` | Call | Unit | no | yes | no | no | no |
| `helper_match_pattern` | Pattern | HelperInternal | maybe | yes | no | maybe | maybe |
| `helper_perform_unwind` | Unwind | VmControl | maybe | yes | yes | yes | yes |
| `helper_register_defer` | Resource | Unit | maybe | yes | no | maybe | maybe |
| `helper_execute_defer` | Resource | VmControl | maybe | yes | yes | yes | yes |
| `helper_register_resource` | Resource | Unit | maybe | yes | no | maybe | maybe |
| `helper_close_resource` | Resource | VmControl | maybe | yes | yes | yes | yes |
| `helper_resolve_module` | Module | Value | yes | yes | no | yes | yes |
| `helper_initialize_module` | Module | VmControl | yes | yes | yes | yes | yes |
| `helper_import_named` | Module | Value | maybe | yes | no | maybe | maybe |
| `helper_import_module` | Module | Value | maybe | yes | no | maybe | maybe |
| `helper_seal_exports` | Module | Unit | no | yes | no | no | no |
| `helper_check_capability` | Capability | Unit | no | yes | no | no | no |
| `helper_enter_host_call` | Capability | Unit | maybe | yes | no | yes | yes |
| `helper_exit_host_call` | Capability | Value | maybe | yes | no | yes | yes |
| `helper_display` | Display | Value | yes | yes | no | yes | yes |
| `helper_string_concat` | Display | Value | yes | yes | no | yes | yes |
| `helper_load_cell` | Access | Value | no | yes | no | no | no |
| `helper_store_cell` | Access | Unit | maybe | yes | no | maybe | maybe |
| `helper_load_module_slot` | Module | Value | no | yes | no | no | no |

`maybe` means descriptor MUST resolve to true/false in a concrete VM profile or helper specialization.

---

## 4. Helper Result Types

```text
HelperResultType =
  | Value
  | VmControl
  | Unit
  | Bool
  | ErrorRef
  | HelperInternal
```

`HelperInternal` MUST NOT escape as language-visible value.

---

## 5. Source Mapping Policy

Any helper with `may_raise = yes` MUST receive or reconstruct:

```text
SourceSpanId
EirLocation
SIR NodeId where available
```

---

## 6. JIT Policy

A helper callable from JIT MUST have a JIT call policy.

JIT MUST call helpers through the VM-controlled helper table or VM trampoline.

Compiled code MUST NOT directly call arbitrary host/native pointers.

---

## 7. Capability Policy

A helper that performs host/effectful access MUST declare:

```text
required_capability
effect
```

Missing capability raises:

```text
CapabilityError
```

---

## 8. Validation

Helper registry validation MUST reject:

```text
duplicate helper name
duplicate helper id
RuntimeHelperOp referencing missing helper
helper descriptor without implementation
implementation without descriptor
may_collect helper without roots-visible policy
may_raise helper without source mapping policy
JIT-callable helper without JIT policy
capability helper without capability/effect metadata
```

---

## 9. Audit Tracking

This document completes:

```text
R5
```

It resolves:

```text
B-03
```

It partially supports:

```text
M-14
M-11
```
