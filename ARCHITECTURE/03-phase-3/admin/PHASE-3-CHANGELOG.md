# Phase 3 VM Specification · Change Log

Document class: Administrative tracking
Planning status: This document records status or change history. It is not itself a normative specification.

Updated: 2026-06-29 09:33:23

## v0.23 Second-Stage Repairs S2-R4-S2-R5

Created:

- `PHASE-3-NORMATIVE-LANGUAGE-SWEEP.md`
- `PHASE-3-FINAL-FREEZE-READINESS-AUDIT.md`

Updated:

- all normative documents with a normative-interpretation hook where missing
- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-IMPLEMENTATION-PLAN.md`
- `PHASE-3-DOCUMENT-MANIFEST.md`
- `PHASE-3-SECOND-NORMATIVE-AUDIT.md`
- `PHASE-3-AUDIT-REPAIR-LOG.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`
- `WORKSPACE-INDEX.md`
- organized workspace mirrors under `phase3/`

Completed second-stage repairs:

```text
S2-R4 normative planning-language sweep
S2-R5 final freeze-readiness audit
```

Final audit result:

```text
Freeze candidate: yes
Automatically frozen: no
Requires explicit freeze declaration: yes
```

---

## v0.21 Audit Repairs R13-R15

Created:

- `PHASE-3-VALIDATION-MATRIX.md`
- `PHASE-3-CACHE-COMPATIBILITY-MATRIX.md`
- `PHASE-3-SECOND-NORMATIVE-AUDIT.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-IMPLEMENTATION-PLAN.md`
- `PHASE-3-DOCUMENT-MANIFEST.md`
- `PHASE-3-NORMATIVE-CONSISTENCY-AUDIT.md`
- `PHASE-3-AUDIT-REPAIR-LOG.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`
- `WORKSPACE-INDEX.md`
- organized workspace mirrors under `phase3/`

Completed audit repairs:

```text
R13 unified Phase 3 validation matrix
R14 canonical cache compatibility matrix
R15 second audit after repairs
```

Second audit verdict:

```text
Original blocker set addressed.
Freeze approval: not yet.
Remaining major residuals require second-stage normative consolidation.
```

---

## v0.19 Audit Repairs R7-R9

Created:

- `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md`
- `PHASE-3-MODULE-RUNTIME-CONTRACT.md`
- `PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-IMPLEMENTATION-PLAN.md`
- `PHASE-3-DOCUMENT-MANIFEST.md`
- `PHASE-3-NORMATIVE-CONSISTENCY-AUDIT.md`
- `PHASE-3-AUDIT-REPAIR-LOG.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Completed audit repairs:

```text
R7 canonical structured unwinding algorithm
R8 module initialization/runtime contract
R9 SIR lowering coverage matrix
```

Resolved blockers:

```text
B-06
B-07
B-08
```

Original blocker set B-01 through B-08 is now fully addressed by repair documents R2-R9.

Remaining work:

```text
R10-R14 major consistency repairs
R15 second audit
```

---

## v0.17 Audit Repair R1 · Normative Keywords and Glossary

Created:

- `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md`
- `PHASE-3-AUDIT-REPAIR-LOG.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-DOCUMENT-MANIFEST.md`
- `PHASE-3-NORMATIVE-CONSISTENCY-AUDIT.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Completed audit repair:

```text
R1: Add normative keyword policy and terminology glossary.
```

Result:

- normative keywords are now defined.
- document class precedence is now normative.
- "minimal VM" meaning is clarified.
- "internal" and "public bytecode" meanings are clarified.
- CPython compatibility rejection is centralized.
- terminology ownership is established.

Remaining blockers:

```text
B-01 through B-08 remain unresolved.
```

---

## v0.16 Document Classification Repair

Created:

- `PHASE-3-DOCUMENT-MANIFEST.md`
- `PHASE-3-IMPLEMENTATION-PLAN.md`

Repaired:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `STATUS.md`
- individual Phase 3 documents with explicit document-class headers

Correction:

- `PHASE-3-VM-SPEC.md` is now a normative aggregate only.
- `PHASE-3-MINIMAL-VM.md` is now a normative aggregate only.
- implementation plans are excluded from both normative aggregates.
- implementation plans are aggregated separately in `PHASE-3-IMPLEMENTATION-PLAN.md`.
- `PHASE-3-DOCUMENT-MANIFEST.md` now defines document classification and precedence.

Reason:

Earlier drafts incorrectly merged implementation plan documents into normative specification aggregates. This repair restores the distinction between VM specifications and project implementation plans.

---

## v0.15 Fast Interpreter Implementation Milestones

Created:

- `PHASE-3-FAST-INTERPRETER-IMPLEMENTATION-MILESTONES.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added fast interpreter implementation milestones for:

1. I0 workspace and skeleton
2. I1 core runtime values, heap handles, frames, slots
3. I2 RuntimePlan/EIR loader and validation gate
4. I3 constants, load/store, checks
5. I4 unary/binary/logical expression execution
6. I5 helper bridge and P0 helpers
7. I6 aggregates, records, enums, access
8. I7 function call engine
9. I8 structured control and unwinding
10. I9 modules, imports, exports, tests
11. I10 feedback, inline caches, safepoints, root integration
12. I11 conformance gate and freeze-readiness
13. dependency graph
14. minimal executable slice
15. forbidden implementation shortcuts
16. milestone reporting format

This round turns the fast interpreter design into a staged implementation sequence with conformance and architecture gates.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 09:07:21

## v0.14 JIT Lowering Matrix per EIR Operation

Created:

- `PHASE-3-JIT-LOWERING-MATRIX.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added JIT lowering matrix for:

1. lowering classes
2. matrix columns and requirements
3. constant operations
4. load operations
5. store operations
6. unary operations
7. binary operations
8. logical operations
9. check operations
10. call operations
11. access operations
12. construction operations
13. pattern operations
14. RuntimeHelperOp
15. SafepointOp
16. GuardOp
17. terminators
18. structured control constructs
19. module operations
20. capability operations
21. allocation operations
22. write barrier operations
23. source mapping requirements
24. root map requirements
25. deopt requirements
26. barrier requirements
27. capability requirements
28. lowering matrix summary
29. JIT validation rules
30. implementation staging J0-J7

This round fixes backend-independent baseline JIT lowering strategy without binding to concrete Cranelift or LLVM APIs.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 09:03:10

## v0.12 Runtime Helper Implementation Plan

Created:

- `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added implementation plan for:

1. runtime helper implementation principles
2. Rust module organization
3. helper table construction
4. helper priority levels P0-P4
5. helper invocation flow
6. allocation helper plan
7. write barrier helper plan
8. type/check helper plan
9. numeric helper plan
10. access helper plan
11. construction helper plan
12. call helper plan
13. pattern helper plan
14. unwind helper plan
15. resource helper plan
16. module helper plan
17. capability helper plan
18. display helper plan
19. helper testing matrix
20. implementation milestones H0-H7
21. implementation guardrails
22. validation and compatibility rules

This round turns helper contracts into a staged implementation plan without exposing helpers as public native ABI.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:59:10

## v0.10 Baseline JIT Backend Interface

Created:

- `PHASE-3-BASELINE-JIT-BACKEND-INTERFACE.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added baseline JIT backend interface for:

1. JitBackend abstraction
2. JitCompileInput
3. JitContext
4. JitTargetProfile
5. EIR lowering categories
6. CompiledFunction
7. JitCodeHandle
8. runtime helper call ABI
9. safepoint and stack-map emission
10. deopt emission
11. guard lowering
12. value layout interface
13. GC interface for JIT
14. call lowering
15. access lowering
16. arithmetic lowering
17. control/unwind lowering
18. capability and host boundary
19. code cache and invalidation
20. JIT validation
21. installation and dispatch
22. Cranelift-compatible backend boundary
23. security and safety constraints

This round defines the VM-facing baseline JIT interface without committing EIR to public bytecode or binding semantics to a concrete backend.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:50:08

## v0.9 GC Root Enumeration and Safepoint Model

Created:

- `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added GC/safepoint architecture for:

1. GC architecture tiers
2. object reference model
3. root set definition
4. slot roots
5. cell and capture roots
6. module roots
7. constant roots
8. region stack roots
9. pending control roots
10. error and suppressed error roots
11. defer/resource roots
12. iterator/pattern roots
13. helper argument roots
14. host roots
15. JIT frame roots
16. safepoint records
17. root maps
18. frame maps
19. allocation protocol
20. object tracing
21. moving GC update protocol
22. write barrier model
23. safepoint polling
24. interpreter root enumeration
25. JIT root enumeration
26. host boundary roots
27. GC/capability boundary
28. GC validation
29. cache compatibility

This round preserves a path to tracing, generational, and moving GC without requiring a full collector in the first VM.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:47:33

## v0.8 Runtime Helper Contracts

Created:

- `PHASE-3-RUNTIME-HELPER-CONTRACTS.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added runtime helper contracts for:

1. helper boundary rule
2. helper descriptor schema
3. helper return discipline
4. GC/root visibility contract
5. JIT helper contract
6. capability contract
7. call helpers
8. access helpers
9. construction helpers
10. type-check helpers
11. pattern helpers
12. error helpers
13. unwind helpers
14. resource helpers
15. module helpers
16. capability helpers
17. allocation helpers
18. write barrier helpers
19. display helpers
20. numeric helpers
21. source mapping for helpers
22. helper validation
23. helper compatibility and digest rules
24. helper security/capability safety
25. bootstrap policy
26. JIT readiness matrix

This round fixes runtime helpers as VM-internal semantic slow paths, not public native ABI.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:45:22

## v0.7 Structured Control and Unwinding Lowering · Round 2

Created:

- `PHASE-3-CONTROL-LOWERING-ROUND2.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added lowering rules for:

1. RegionPlan lowering
2. PendingControl model
3. block lowering
4. if lowering
5. while lowering
6. for lowering
7. break/continue lowering
8. return lowering
9. raise lowering
10. try/catch/finally lowering
11. use lowering
12. defer lowering
13. match/pattern lowering
14. assert lowering
15. test lowering
16. module import execution lowering
17. module initialization lowering
18. structured unwinding lowering
19. safepoints in control lowering
20. deopt state for control constructs
21. structured-control lowering validation

This round completes the first coherent SIR-to-EIR lowering coverage for structured control flow and cleanup semantics.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:40:54

## v0.6 SIR Lowering · Round 1

Created:

- `PHASE-3-SIR-LOWERING-ROUND1.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added first lowering rules for:

1. lowering pipeline
2. lowering invariants
3. slot allocation
4. environment/scope lowering
5. type lowering
6. record shape lowering
7. enum shape lowering
8. ModulePlan lowering
9. FunctionPlan lowering
10. literal lowering
11. binding reference lowering
12. let/const lowering
13. assignment lowering
14. unary/binary/logical lowering
15. call lowering
16. access lowering
17. collection construction lowering
18. record construction lowering
19. enum construction lowering
20. function construction lowering
21. format string lowering
22. check insertion
23. source map preservation
24. RuntimePlan validation
25. EIR validation
26. lowering failure diagnostics
27. lowering cache compatibility

Structured-control and unwinding lowering are deferred to the next round.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:38:36

## v0.5 EIR Operation Semantics · Round 1

Created:

- `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added first concrete EIR operation semantics for:

1. EIR execution model
2. slot state and slot read/write rules
3. constant operations
4. load operations
5. store operations
6. unary operations
7. binary operations
8. logical operations
9. check operations
10. call operations
11. access operations
12. construction operations
13. pattern operations
14. runtime helper operations
15. safepoint operations
16. guard operations
17. terminator semantics
18. fast interpreter loop
19. operation validation additions
20. semantics preservation requirements
21. JIT readiness classification requirement

This round keeps EIR internal and non-public while making it executable enough for later lowering rules.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:29:07

## v0.4 RuntimePlan and EIR Framework

Created:

- `PHASE-3-RUNTIMEPLAN-EIR-FRAMEWORK.md`

Updated:

- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-CHANGELOG.md`
- `STATUS.md`

Added framework definitions for:

1. RuntimePlan
2. RuntimeTargetProfile
3. ModulePlan
4. ImportPlan
5. ExportPlan
6. FunctionPlan
7. ParameterLayout
8. CaptureLayout
9. SlotLayout
10. TypePlan
11. ShapePlan
12. CallSiteTable
13. AccessSiteTable
14. InlineCache model
15. FeedbackTable
16. EIR module/function/block framework
17. EIR operation families
18. EIR terminators
19. SafepointTable
20. DeoptPointTable
21. RuntimeHelperTable
22. FastInterpreterState
23. RuntimePlan generation validation
24. EIR validation
25. RuntimePlan/EIR cache compatibility rules

This round establishes the bridge from semantic SIR to high-performance internal execution without creating public bytecode.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:26:35

## v0.3 Performance/JIT Architecture Patch

Added high-performance VM architecture framework.

Created:

- `PHASE-3-PERFORMANCE-ARCHITECTURE.md`

Updated:

- `PHASE-3-VM-FRAMEWORK.md`
- `PHASE-3-VM-RUNTIME-ROUND1.md`
- `PHASE-3-VM-SPEC.md`
- `PHASE-3-MINIMAL-VM.md`
- `PHASE-3-TECH-STACK.md`
- `STATUS.md`

Major commitments added:

1. JIT implementation is staged, but JIT architecture is mandatory.
2. SIR interpreter is correctness tier only.
3. RuntimePlan is required before hot execution.
4. EIR fast interpreter is the production interpreter target.
5. EIR remains internal and is not public bytecode.
6. Value physical layout is VM-private; Rust enum layout is not ABI.
7. CPython-style reference counting is rejected as runtime architecture.
8. Rc/RefCell is allowed only as bootstrap detail.
9. Heap handles must preserve a path to tracing/generational/moving GC.
10. Safepoints are architectural.
11. CallSiteId and AccessSiteId are required for hot execution.
12. Inline cache state, type feedback, and shape feedback are reserved.
13. Deopt metadata, frame maps, root maps, write barrier hooks, and runtime helper table are reserved.
14. A Cranelift-compatible backend is recommended for baseline JIT, but backend lock-in is forbidden.
15. VM design is split into Semantic Runtime and Execution Runtime.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:18:10

## v0.2 Runtime Draft · Round 1

Filled the first concrete VM runtime layer.

Added concrete specification for:

1. Rust representation strategy
2. runtime `Value` model
3. heap and handle model
4. object identity model
5. aggregate objects
6. record runtime model
7. enum runtime model
8. function and closure model
9. module runtime model
10. binding and environment model
11. frame and call stack model
12. region stack model
13. `VmControl` control result model
14. runtime error object model
15. read-only view model
16. resource handle model
17. evaluator contracts
18. validation/execution boundary
19. internal VM invariants
20. recommended minimal implementation order

Deferred node-by-node SIR execution to later rounds.

---

# Phase 3 VM Specification · Change Log

Updated: 2026-06-29 08:15:43

## v0.1 Framework Draft

Started Phase 3 Minimal VM specification.

Created:

- `PHASE-3-VM-FRAMEWORK.md`
- `PHASE-3-VM-SPEC.md`
- `PHASE-3-TECH-STACK.md`

Updated:

- `PHASE-3-MINIMAL-VM.md`

Major decisions:

1. Rust is selected as the default implementation language.
2. The VM consumes frozen Phase 2 SIR.
3. The VM is not public bytecode.
4. The VM is not CPython-compatible.
5. The initial execution model is a SIR interpreter.
6. NIR/EIR/OIR remain deferred.
7. The core VM should be unsafe-free initially.
8. The Rust workspace is split into SIR, validation, runtime, evaluator, diagnostics, host, CLI, and tests.
9. The VM must enforce validation before execution.
10. The VM must preserve Phase 1 and Phase 2 structured semantics.
