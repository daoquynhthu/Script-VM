# Phase 3 · SIR Lowering Coverage Matrix

Document class: Normative specification  
Normative status: This document defines required Phase 2 SIR node coverage for Phase 3 RuntimePlan/EIR lowering.

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
R9: Add SIR node lowering coverage matrix.
```

It resolves blocker:

```text
B-08: SIR lowering coverage is not demonstrably complete.
```

Every Phase 2 frozen SIR node kind MUST be either:

```text
Lowered
RuntimePlan-only
Validation-rejected
Deferred outside Phase 3 minimal VM
```

---

## 1. Coverage Status Values

```text
Lowered
  Must lower to RuntimePlan and/or EIR.

RuntimePlan-only
  Represented in RuntimePlan metadata, not as executable EIR op.

Validation-rejected
  Invalid in the relevant context and must be rejected before execution.

Deferred
  Not required for Phase 3 minimal VM. Must not be generated unless feature is enabled.
```

---

## 2. Top-Level Nodes

| SIR node | Phase 3 coverage | Required target |
|---|---|---|
| `ModuleBodyNode` | Lowered | ModulePlan + module init EIR |
| `DeclarationNode` | Lowered | binding/slot/type/module plans |
| `BindingNode` | Lowered | SlotDescriptor / StoreOp / PatternOp |
| `ExpressionNode` | Lowered | EirOp / EirTerminator where applicable |
| `AssignmentNode` | Lowered | StoreOp / AccessOp |
| `FunctionNode` | Lowered | FunctionPlan + ConstructFunction + EirFunction |
| `RecordNode` | Lowered | ShapePlan + TypePlan + constructor path |
| `EnumNode` | Lowered | ShapePlan + TypePlan + enum constructor path |
| `BlockNode` | Lowered | EirBlock graph + RegionPlan |
| `IfNode` | Lowered | Branch + blocks |
| `WhileNode` | Lowered | loop blocks + LoopBackedge safepoint |
| `ForNode` | Lowered | iterator plan + loop blocks |
| `MatchNode` | Lowered | PatternOp graph + branch/case blocks |
| `ReturnNode` | Lowered | Return terminator + unwind path if needed |
| `BreakNode` | Lowered | PendingBreak / branch / unwind |
| `ContinueNode` | Lowered | PendingContinue / branch / unwind |
| `RaiseNode` | Lowered | Raise terminator + type check |
| `TryNode` | Lowered | region/catch/finally plan + unwind |
| `UseNode` | Lowered | resource registration + cleanup region |
| `DeferNode` | Lowered | defer registration in cleanup region |
| `AssertNode` | Lowered | branch/check + helper_assert_fail |
| `TestNode` | Lowered | TestPlan / test entry EIR |
| `CheckNode` | Lowered | CheckOp / RuntimePlan metadata |

---

## 3. Declarations

| Declaration | Coverage | Target |
|---|---|---|
| `let` | Lowered | SlotDescriptor + initializer + StoreSlot/StoreCell |
| `const` | Lowered | immutable SlotDescriptor + initializer |
| `def` | Lowered | FunctionPlan + function object construction when reached |
| `record` | Lowered | RecordShape + TypePlan + constructor |
| `enum` | Lowered | EnumShape + TypePlan + case constructors |
| `import` | Lowered | ImportPlan + module helper calls |
| `export` | Lowered | ExportPlan + export table entry |
| `test` | Lowered | TestPlan + test entry |

Function declarations MUST NOT be hoisted unless already specified by Phase 1/2. Function binding is created when declaration executes.

---

## 4. Expressions

| Expression | Coverage | Target |
|---|---|---|
| nil/bool/int/float/string literal | Lowered | ConstantOp |
| list literal | Lowered | ConstructList |
| map literal | Lowered | ConstructMap |
| binding reference | Lowered | LoadSlot/LoadCell/LoadModuleSlot |
| unary expression | Lowered | UnaryOp |
| binary expression | Lowered | BinaryOp / helper fallback |
| logical and/or | Lowered | Branch-based short-circuit graph |
| call expression | Lowered | CallOp + CallSiteId |
| attribute access | Lowered | AccessOp / LoadField |
| index access | Lowered | AccessOp |
| slice access | Lowered | AccessOp / helper_slice_read |
| record construction | Lowered | ConstructRecord |
| enum construction | Lowered | ConstructEnumValue |
| function expression | Lowered | ConstructFunction |
| format string | Lowered | ordered expression eval + helper_display/string concat |
| readonly view | Lowered | helper or ConstructReadOnlyView if represented |
| error construction | Lowered | ConstructError/helper_construct_error |

---

## 5. Assignment Targets

| Assignment target | Coverage | Target |
|---|---|---|
| binding target | Lowered | StoreSlot/StoreCell |
| field target | Lowered | StoreField |
| index target | Lowered | StoreListIndex/StoreMapEntry |
| destructuring target | Lowered | PatternOp + commit/rollback |

Augmented assignment MUST evaluate target once.

---

## 6. Pattern Variants

| Pattern | Coverage | Target |
|---|---|---|
| Wildcard | Lowered | PatternBranch/PatternOp no binding |
| Literal | Lowered | PatternCheckLiteral |
| Binding | Lowered | PatternBind + commit |
| Record | Lowered | PatternCheckRecordShape + field loads |
| Enum | Lowered | PatternCheckEnumCase + payload loads |
| List | Lowered | PatternCheckListLength + element checks |
| Map | Lowered | PatternCheckMapKey + value checks |
| Or | Lowered | alternative subgraphs with same binding set |

Pattern lowering MUST distinguish match failure from declaration destructuring failure.

---

## 7. Structured Control

| Construct | Coverage | Target |
|---|---|---|
| block | Lowered | EirBlock + optional RegionFrame |
| if | Lowered | Branch + merge |
| while | Lowered | loop header/body/exit + LoopBackedge |
| for list | Lowered | iterator loop preserving order |
| for map | Lowered | key iteration in insertion order |
| for range | Lowered | range iterator loop |
| match | Lowered | subject once + ordered case graph |
| try/catch | Lowered | catch region + raise matching |
| finally | Lowered | cleanup region + finally override |
| use | Lowered | resource acquisition + cleanup |
| defer | Lowered | deferred callable registration |
| return | Lowered | Return/PendingReturn/unwind |
| break | Lowered | Break/PendingBreak/unwind |
| continue | Lowered | Continue/PendingContinue/unwind |
| raise | Lowered | Raise/PendingRaise/unwind |

String iteration is not core and MUST NOT be lowered unless later amended.

---

## 8. Module Integration

| SIR source | Coverage | Target |
|---|---|---|
| top-level source order | Lowered | module init EIR |
| import source order | Lowered | ImportPlan order |
| whole module import | Lowered | helper_import_module |
| named import | Lowered | helper_import_named |
| export | Lowered | ExportPlan |
| circular initialized export access | Lowered | module runtime check |
| circular uninitialized export access | Lowered | ImportCycleError |
| module failure | Lowered | ModuleState Failed + initialization_error |

---

## 9. Capability and Effects

| SIR item | Coverage | Target |
|---|---|---|
| `requires` metadata | RuntimePlan-only | CapabilityGatePlan |
| `effect[...]` metadata | RuntimePlan-only | FunctionPlan/CallSite/helper metadata |
| host capability access | Lowered | CheckCapability/helper boundary |
| missing capability | Lowered | CapabilityError |

---

## 10. Validation-Rejected Contexts

The following MUST be rejected before execution:

```text
return outside function
break outside loop
continue outside loop
module top-level defer unless amended
top-level source control transfer
non-Bool condition
or-pattern alternatives with different binding sets
record declaration duplicate fields
enum declaration duplicate cases
unknown import/export binding
assignment to const
assignment to read-only view
```

---

## 11. Deferred Features

The following are not required in Phase 3 minimal VM unless explicitly enabled:

```text
public bytecode
FFI
native extension ABI
async/await
threads
generator/yield
string iteration
user-defined operator protocols
dynamic record field creation
```

If a deferred feature appears in SIR without feature enablement, validation MUST reject it.

---

## 12. Coverage Validation

The lowering validator MUST reject any SIR node that lacks a declared coverage route in this matrix.

Each lowering route MUST preserve:

```text
evaluation order
binding identity
source spans
type checks
capability/effect metadata
module import order
nominal record/enum identity
pattern binding semantics
defer/use/finally order
primary/suppressed error behavior
```

---

## 13. Audit Tracking

This document completes:

```text
R9
```

It resolves:

```text
B-08
```

It partially supports:

```text
M-15
M-10
```
