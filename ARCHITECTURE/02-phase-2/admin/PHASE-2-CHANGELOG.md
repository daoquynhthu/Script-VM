# Phase 2 IR Specification · Change Log

Updated: 2026-06-29 08:12:34

## v1.0 Freeze

Phase 2 IR design is frozen as:

```text
Version: 1.0 frozen baseline
```

The freeze incorporates the v0.6 schema integration patch and formally freezes:

1. IR framework
2. SIR foundational schema
3. declaration/expression/function/record/enum nodes
4. structured control-flow and unwinding nodes
5. module initialization and import/export semantics
6. module interface compatibility model
7. digest profiles
8. validation levels
9. SIR-to-NIR lowering contracts
10. producer/consumer conformance
11. no-public-bytecode and no-CPython-ABI commitments

Created:

- `PHASE-2-FREEZE.md`

No freeze-audit file is retained.

---

# Phase 2 IR Specification · Change Log

Updated: 2026-06-29 08:10:53

## v0.6 Freeze Patch

Applied the Phase 2 freeze patch directly to the canonical workspace files.

This patch does not expand IR semantics. It resolves schema integration issues before freeze:

1. Added `ControlRegionId` and `PatternId` to the canonical ID class list.
2. Consolidated final `IRUnit` schema:
   - `NodeTable`
   - `PatternTable`
   - `ControlRegionTable`
3. Made `PatternTable` canonical and required, with empty table allowed.
4. Extended `SIRNode` to include all core Round 3 node variants.
5. Removed standalone `MetadataNode` and `ExtensionNode` from the canonical `SIRNode` union.
6. Normalized embedded node-typed fields to `NodeId` references:
   - assignment targets
   - capture references
   - record methods
7. Added freeze patch notes to Round 2, Round 3, and Round 4.
8. Deleted any freeze-audit file from the workspace.

Current status: ready for focused schema recheck and then Phase 2 freeze.

---

# Phase 2 IR Specification · Change Log

Updated: 2026-06-29 08:07:04

## v0.5 Integration Draft · Round 4

Completed the SIR integration layer.

Added semantics for:

1. ModuleState
2. module instance lifecycle
3. module dependency graph
4. import resolution and execution order
5. export table sealing
6. circular import handling
7. module interface compatibility checks
8. record/enum/function/const compatibility rules
9. canonical digest profiles
10. validation levels V0 through V8
11. SIR-to-NIR lowering contracts
12. allowed lowering transformations
13. forbidden lowering transformations
14. NIR boundary
15. IR producer conformance
16. IR consumer conformance
17. conformance test profiles
18. Phase 2 freeze criteria
19. open issue classification before freeze

Round 4 does not define concrete NIR. It defines what future NIR must preserve.

---

# Phase 2 IR Specification · Change Log

Updated: 2026-06-29 08:03:38

## v0.4 Concrete Draft · Round 3

Filled the structured control-flow layer of canonical SIR.

Added concrete semantics for:

1. ControlEffect
2. ControlRegionId
3. ControlRegionDescriptor
4. ControlRegionTable
5. BlockNode
6. IfNode
7. WhileNode
8. ForNode
9. PatternTable
10. PatternNode variants
11. MatchNode
12. ReturnNode
13. BreakNode
14. ContinueNode
15. RaiseNode
16. TryNode
17. CatchClause
18. FinallyClause
19. ErrorFlowDescriptor
20. UseNode
21. DeferNode
22. AssertNode
23. TestNode
24. Structured unwinding algorithm
25. control-flow validation rules
26. pattern validation rules
27. unwinding validation rules

Round 3 completes the structured-control representation of the frozen Phase 1 high-level language surface.

Deferred to next round:

- module initialization semantics
- import/export execution order
- SIR validation completeness
- SIR-to-NIR lowering contracts
- canonical digest requirements
- conformance tests
- Phase 2 freeze criteria

---

# Phase 2 IR Specification · Change Log

Updated: 2026-06-29 07:58:27

## v0.3 Concrete Draft · Round 2

Filled the first concrete executable semantic node families for SIR.

Added concrete semantics for:

1. NodeTable
2. universal NodeHeader
3. ModuleBodyNode
4. declaration nodes
5. binding reference nodes
6. literal nodes
7. list and map literal nodes
8. unary expression nodes
9. binary expression nodes
10. chained comparison nodes
11. logical short-circuit nodes
12. call expression nodes
13. attribute/index/slice nodes
14. assignment and augmented assignment nodes
15. function declaration/value nodes
16. record declaration/construction/access semantics
17. enum declaration/construction/case semantics
18. format string nodes
19. basic runtime check nodes

Added node-level validation rules and compatibility rules for Round 2 node semantics.

Deferred structured control flow, pattern semantics, try/catch/finally, use/defer, return/break/continue, raise, test/assert, and unwinding to Round 3.

---

# Phase 2 IR Specification · Change Log

Updated: 2026-06-29 07:54:12

## v0.2 Concrete Draft · Round 1

Filled the first concrete SIR semantic layer.

This round defines the foundational substrate, not executable expression/statement semantics.

Added concrete schemas for:

1. schema notation
2. primitive schema types
3. typed ID format and ID invariants
4. IRUnit
5. IRHeader
6. FeatureSet
7. ModuleDescriptor
8. SourceTable
9. SymbolTable
10. ScopeTable
11. BindingTable
12. TypeTable
13. CapabilityTable
14. EffectTable
15. ModuleInterfaceDescriptor
16. ExtensionTable
17. DiagnosticTable

Added foundational validation rules:

- schema validation
- reference validation
- scope validation
- binding validation
- type validation
- interface validation
- capability/effect validation
- extension validation

Added backward compatibility rules for Round 1 structures.

Concrete expression, statement, declaration body, pattern, and control-flow nodes remain deferred.

---

# Phase 2 IR Specification · Change Log

Updated: 2026-06-29 07:43:19

## v0.1 Framework Draft

Started Phase 2 IR language specification.

This first IR document defines the complete framework boundary before filling concrete node semantics.

Major framework decisions:

1. IR exists but is not public bytecode.
2. Source remains the primary compatibility layer.
3. IR compatibility is layered:
   - Tier 0: source language compatibility
   - Tier 1: canonical Semantic IR compatibility
   - Tier 2: Module Interface Descriptor compatibility
   - Tier 3: Executable IR compatibility
   - Tier 4: Optimizer IR compatibility
   - Tier 5: Foreign/native boundary compatibility
4. SIR is the canonical Phase 2 IR layer.
5. NIR/EIR/OIR are distinguished but not yet concretely specified.
6. Module Interface Descriptor is the ABI-like boundary for exported shape, not executable code.
7. IR schema versioning uses major/minor/patch.
8. New executable semantics require feature gates.
9. Unknown required features must be rejected.
10. Unknown optional metadata may be ignored.
11. IR identity uses explicit IDs, not pointer identity.
12. Source mapping, diagnostics, scopes, bindings, type contracts, effects, capabilities, resources, patterns, and module interfaces are all first-class framework concerns.
13. Serialized IR caches are discardable and are not package ABI.
14. Foreign/native ABI is separate from IR and must not rely on CPython/Python ABI compatibility.

Concrete node semantics are intentionally deferred.
