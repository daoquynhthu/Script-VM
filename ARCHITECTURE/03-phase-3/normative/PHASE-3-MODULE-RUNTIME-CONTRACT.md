# Phase 3 · Module Runtime Contract

Document class: Normative specification  
Normative status: This document defines the canonical module initialization, import, export, and circular import runtime contract for Phase 3 VM specifications.

Created: 2026-06-29 09:24:10

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
R8: Close module initialization/runtime contract.
```

It resolves blocker:

```text
B-07: Module initialization semantics are not closed at EIR/runtime level.
```

---

## 1. Module Runtime Objects

### 1.1 ModuleInstance

```text
ModuleInstance {
  module_id: ModuleId
  state: ModuleState
  module_object: ObjRef
  module_slots: SlotArray
  export_table: ExportTable
  interface_descriptor: ModuleInterfaceDescriptor
  initialization_error?: ErrorHandle
  initialization_function: EirFunctionId
  source_span?: SourceSpanId
}
```

### 1.2 ModuleState

```text
ModuleState =
  | Unloaded
  | Loading
  | Initializing
  | Initialized
  | Failed
```

No other module states are allowed in Phase 3.

---

## 2. State Transitions

Allowed transitions:

```text
Unloaded -> Loading
Loading -> Initializing
Initializing -> Initialized
Initializing -> Failed
Loading -> Failed
Failed -> Loading only by explicit host retry
```

Forbidden transitions:

```text
Initialized -> Loading
Initialized -> Initializing
Failed -> Initializing directly
Failed -> Initialized directly
Unloaded -> Initializing
```

---

## 3. Explicit Retry Policy

A Failed module MUST NOT automatically retry initialization.

Retry is allowed only if the host explicitly requests retry through a VM-controlled module reload/retry operation.

On retry:

```text
previous initialization_error remains diagnosable
new attempt uses fresh initialization state
module identity policy must be host-defined and deterministic
```

If Phase 3 minimal VM does not expose retry, `Failed -> Loading` remains a reserved transition.

---

## 4. Module Initialization Function

Each module has one synthetic initialization function:

```text
module_init(module_id) -> VmControl
```

It executes top-level declarations and imports in source order.

Top-level `return`, `break`, and `continue` are invalid and MUST be rejected before execution.

A top-level raise fails module initialization.

---

## 5. Import Execution

Imports execute in source order.

Import resolution is deterministic and host-defined.

No implicit relative import is allowed unless the module resolver explicitly defines it.

Import kinds:

```text
WholeModuleImport
NamedImport
AliasedNamedImport
```

---

## 6. Whole Module Import

Whole module import binds a module value to the local import binding.

If provider module is not initialized, VM MUST initialize or continue initializing it according to the module graph rules.

If provider initialization fails, importer fails with ImportError or propagated initialization error.

---

## 7. Named Import

Named import binds an exported binding value from provider module.

Named import MUST check:

```text
provider module resolved
export exists
export initialized
interface compatible
local binding slot valid
```

If export exists but is uninitialized due to circular import, VM raises:

```text
ImportCycleError
```

---

## 8. Export Table

### 8.1 ExportTable

```text
ExportTable {
  entries: Map<String, ExportEntry>
  sealed: Bool
}
```

### 8.2 ExportEntry

```text
ExportEntry {
  name: String
  binding_id: BindingId
  slot_id: SlotId
  initialized: Bool
  type_id?: TypeId
  source_span: SourceSpanId
}
```

### 8.3 Sealing

Export table MUST be sealed after successful module initialization.

After sealing, export table shape MUST NOT change.

Exported binding values may remain live through their slots/cells.

---

## 9. Circular Imports

Circular imports are permitted only under strict access rules.

If module A imports module B while B is Initializing:

```text
A may access already initialized exports of B.
A MUST NOT access uninitialized exports of B.
```

Accessing uninitialized circular export raises:

```text
ImportCycleError
```

This applies to both named imports and module object export access.

---

## 10. Initialization Failure

If module initialization raises:

```text
state = Failed
initialization_error = raised error
export table remains unsealed or marked failed
partially initialized exports are not considered successfully initialized for future imports unless explicitly allowed by retry policy
```

Future ordinary imports of Failed module MUST fail.

They MUST NOT silently reinitialize.

---

## 11. Module Rooting

During Loading/Initializing/Initialized/Failed, module instance and its live values MUST be GC roots if reachable from module environment or import graph.

Rooted values include:

```text
module object
module slots
export table entries
initialization_error
imported module references
module constants
```

---

## 12. Module Resolver and Capabilities

Module resolver is host-defined.

If resolver performs effectful host access, it MUST be capability-gated.

Missing capability raises:

```text
CapabilityError
```

The capability environment policy must be included in RuntimePlan/module cache compatibility when relevant.

---

## 13. Interface Compatibility

A module import MUST validate provider interface against required `ModuleInterfaceDescriptor`.

If interface contains unknown required fields, conservative rejection is required.

Compatibility failure raises:

```text
ImportError
```

or a more specific module-interface error if added to the runtime error registry.

---

## 14. EIR Runtime Contract

Module initialization EIR MUST:

```text
use ModulePlan initialization_function
use module slot layout
write exports through export slots/cells
execute source order
call module helpers for import/resolve/seal
preserve source spans
use RuntimePlan ImportPlan and ExportPlan
```

Module import operations MUST go through canonical helpers:

```text
helper_resolve_module
helper_initialize_module
helper_import_named
helper_import_module
helper_seal_exports
```

---

## 15. Test Blocks

Test blocks are not executed during ordinary module initialization.

Test runner MAY execute test blocks through separate test entry functions.

Test failures MUST report source spans.

---

## 16. Validation

Module runtime validation MUST reject:

```text
module without initialization function
module plan without module slot layout
import entry without source span
export entry without binding slot
duplicate export names
export table mutation after sealing
named import of missing export
access to uninitialized circular export
automatic retry after Failed
top-level return/break/continue
module helper missing from helper registry
effectful resolver without capability declaration
```

---

## 17. Audit Tracking

This document completes:

```text
R8
```

It resolves:

```text
B-07
```

It partially supports:

```text
M-10
M-11
M-13
M-15
```
