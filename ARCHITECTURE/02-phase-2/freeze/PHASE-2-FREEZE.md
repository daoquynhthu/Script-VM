# Phase 2 · IR Design Freeze

Frozen At: 2026-06-29 08:12:34

## Status

Phase 2 IR design is frozen as:

```text
Version: 1.0 frozen baseline
```

## Dependency

Phase 2 depends on:

```text
Phase 1 High-Level Language Specification
Version: 1.0 frozen baseline
```

## Canonical Frozen Files

- `PHASE-2-IR-FRAMEWORK.md`
- `PHASE-2-SIR-SEMANTICS-ROUND1.md`
- `PHASE-2-SIR-SEMANTICS-ROUND2.md`
- `PHASE-2-SIR-SEMANTICS-ROUND3.md`
- `PHASE-2-SIR-INTEGRATION-ROUND4.md`
- `PHASE-2-IR-SPEC.md`
- `PHASE-2-IR-DESIGN.md`

## Frozen Scope

The freeze applies to:

1. IR layer model
2. compatibility tiers
3. versioning rules
4. feature flags
5. extension model
6. ID model
7. source mapping
8. IRUnit schema
9. symbol table
10. scope table
11. binding table
12. type table
13. capability table
14. effect table
15. node table
16. pattern table
17. control region table
18. diagnostic table
19. module interface descriptor
20. module initialization semantics
21. import/export semantics
22. circular import handling
23. SIR node semantics
24. structured control-flow semantics
25. structured unwinding semantics
26. validation levels V0-V8
27. canonical digest profiles
28. SIR-to-NIR lowering contracts
29. IR producer/consumer conformance
30. ABI and foreign-boundary constraints

## Frozen Architectural Commitments

Phase 2 freezes the following commitments:

```text
SIR is canonical Semantic IR.
SIR is not public bytecode.
SIR is not package ABI.
SIR is not native ABI.
Module Interface Descriptor is the ABI-like exported semantic-shape boundary.
Executable IR is implementation-private and discardable.
Optimizer IR is unstable and outside compatibility boundary.
Foreign/native ABI is separate from IR.
CPython C API compatibility is rejected.
CPython ABI compatibility is rejected.
Python wheel compatibility is rejected.
Python extension module compatibility is rejected.
```

## Frozen Compatibility Model

Compatibility remains layered:

```text
Tier 0: Source Language Compatibility
Tier 1: Canonical Semantic IR Compatibility
Tier 2: Module Interface Descriptor Compatibility
Tier 3: Executable IR Compatibility
Tier 4: Optimizer IR Compatibility
Tier 5: Foreign / Native Boundary Compatibility
```

## Final Freeze Patch Included

The freeze includes the v0.6 freeze patch:

1. `ControlRegionId` and `PatternId` are canonical ID classes.
2. Final `IRUnit` schema is consolidated.
3. `PatternTable` is canonical and required, with empty table allowed.
4. `SIRNode` includes all core node variants through Round 3.
5. Direct concrete-node fields were normalized to `NodeId` references.
6. Standalone `MetadataNode` and `ExtensionNode` were removed from canonical `SIRNode`.
7. Canonical SIR uses table-based pattern representation.
8. No audit file is retained in the workspace.

## Post-Freeze Change Rule

After this freeze, Phase 2 may change only through an explicit amendment.

Allowed amendments:

1. contradiction fix
2. schema closure fix
3. semantic incompleteness discovered during Phase 3 VM design
4. safety/capability/foreign-boundary flaw fix
5. non-semantic wording clarification
6. optional metadata extension that does not affect execution semantics

Not allowed without reopening Phase 2:

1. adding new core SIR semantic node families
2. changing evaluation order
3. changing binding identity semantics
4. changing scope graph semantics
5. changing type contract semantics
6. changing module interface compatibility semantics
7. changing structured unwinding order
8. changing digest compatibility meaning
9. weakening validation requirements
10. introducing public bytecode
11. introducing CPython/Python ABI compatibility

## Ready for Phase 3

Phase 2 is sufficient as input for Phase 3 VM design.

Phase 3 must treat Phase 2 as the IR semantic baseline.

If Phase 3 discovers a required change to SIR, it must be recorded as a Phase 2 amendment, not silently absorbed into VM implementation.
