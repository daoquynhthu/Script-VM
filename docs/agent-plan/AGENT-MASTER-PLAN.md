# Agent Implementation Master Plan

Document class: Agent implementation plan  
Normative status: Non-normative  
Authority: Subordinate to the frozen Phase 1–3 specifications  
Created: 2026-06-29 10:27:11

---

## 0. Status

This document starts the post-freeze implementation-planning stage.

It does not reopen specification design.

It does not modify Phase 1, Phase 2, or Phase 3 normative semantics.

It defines how a main Agent and up to four subordinate Agents may plan, execute, review, and hand off implementation work against the frozen specification baseline.

---

## 1. Purpose

The purpose of this plan is to provide a disciplined implementation-planning framework for an Agent team working on the Script VM project.

The Agent team model is:

```text
1 main Agent
0–4 subordinate Agents
```

The main Agent owns global context, task decomposition, merge decisions, gate enforcement, and final acceptance.

Subordinate Agents may be used only for bounded, independently verifiable tasks.

This plan is designed to prevent common Agent-based implementation failures:

```text
specification drift
uncontrolled parallelism
token waste
unverifiable subtask output
implementation shortcuts becoming accidental semantics
testing gaps
cross-module integration drift
```

---

## 2. Authority and Boundary

### 2.1 Non-Normative Status

This document is non-normative.

It may define:

```text
task order
Agent roles
work packages
gate checks
handoff rules
traceability rules
risk tracking
implementation sequencing
```

It must not define:

```text
language semantics
IR semantics
VM semantics
runtime error semantics
EIR operation semantics
RuntimePlan schema semantics
GC/JIT compatibility semantics
host boundary semantics
```

### 2.2 Authority Order

The authority order is:

```text
Frozen normative specifications
  > Freeze declarations
  > Agent implementation plans
  > Agent task notes
  > temporary Agent reasoning
```

If this plan conflicts with a frozen specification document, the frozen specification wins.

If a subordinate Agent output conflicts with this plan, this plan wins.

If this plan conflicts with `PHASE-3-FREEZE.md`, `PHASE-3-FREEZE.md` wins.

### 2.3 Forbidden Plan Behavior

This plan and all derived task plans must not:

```text
add new VM semantics
reinterpret frozen normative semantics
copy large blocks of normative text
replace frozen specification references with informal summaries
treat implementation convenience as a normative change
bypass required validation gates
bypass runtime helper registry constraints
bypass structured unwinding semantics
bypass capability checks
bypass cache compatibility rules
expose RuntimePlan or EIR as public bytecode
introduce CPython ABI compatibility
```

---

## 3. Frozen Specification Reference Set

Implementation work must refer to frozen documents by explicit reference.

The canonical frozen baseline is:

```text
PHASE-3-FREEZE.md
```

The primary Phase 3 normative aggregates are:

```text
PHASE-3-VM-SPEC.md
PHASE-3-MINIMAL-VM.md
```

Implementation plans must cite specific subsystem specifications where possible, rather than citing only the aggregate.

---

## 4. Reference Alias System

To keep Agent tasks concise, this plan establishes reference aliases.

These aliases do not replace the actual documents. They are routing labels.

| Alias | Document |
|---|---|
| `SPEC-P3-FREEZE` | `PHASE-3-FREEZE.md` |
| `SPEC-P3-VM` | `PHASE-3-VM-SPEC.md` |
| `SPEC-P3-MIN` | `PHASE-3-MINIMAL-VM.md` |
| `SPEC-P3-KEYWORDS` | `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md` |
| `SPEC-P3-ERRORS` | `PHASE-3-RUNTIME-ERROR-REGISTRY.md` |
| `SPEC-P3-EIR` | `PHASE-3-EIR-SCHEMA-CLOSURE.md` |
| `SPEC-P3-RTP` | `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md` |
| `SPEC-P3-HELPERS` | `PHASE-3-RUNTIME-HELPER-REGISTRY.md` |
| `SPEC-P3-CONTROL` | `PHASE-3-CONTROL-STATE-MODEL.md` |
| `SPEC-P3-UNWIND` | `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` |
| `SPEC-P3-MODULE` | `PHASE-3-MODULE-RUNTIME-CONTRACT.md` |
| `SPEC-P3-LOWERING` | `PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md` |
| `SPEC-P3-GC-META` | `PHASE-3-GC-METADATA-OWNERSHIP.md` |
| `SPEC-P3-PROFILE` | `PHASE-3-TARGET-PROFILE-SCHEMAS.md` |
| `SPEC-P3-VALUES` | `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md` |
| `SPEC-P3-VALID` | `PHASE-3-VALIDATION-MATRIX.md` |
| `SPEC-P3-CACHE` | `PHASE-3-CACHE-COMPATIBILITY-MATRIX.md` |
| `SPEC-P3-CALL` | `PHASE-3-CALL-EXECUTION-PROTOCOL.md` |
| `SPEC-P3-READONLY` | `PHASE-3-READONLY-VIEW-SEMANTICS.md` |
| `SPEC-P3-HOST` | `PHASE-3-HOST-BOUNDARY-CONTRACT.md` |

Phase 1 and Phase 2 references must be added to `TRACEABILITY-MATRIX.md` when work packages require them.

---

## 5. Agent Team Model

### 5.1 Main Agent

The main Agent is the only global coordinator.

Responsibilities:

```text
maintain global project context
read and cite frozen specifications
decompose implementation work
decide whether subordinate Agents are needed
issue bounded tasks
review subordinate output
reject scope drift
merge validated outputs
update plan documents
enforce gates
produce final handoff reports
```

The main Agent must not outsource final architectural interpretation.

The main Agent owns:

```text
specification-reference discipline
task sequencing
gate pass/fail decisions
merge decisions
risk classification
implementation-plan updates
```

### 5.2 Subordinate Agents

Subordinate Agents are bounded execution units.

They may perform:

```text
localized specification trace checks
localized test matrix construction
localized risk scans
localized implementation-option comparison
localized integration-impact analysis
```

They may not perform:

```text
global architecture decisions
normative interpretation changes
specification rewrites
freeze-baseline changes
unbounded implementation planning
direct merges into master plan
```

Subordinate Agent outputs are advisory until accepted by the main Agent.

---

## 6. Parallelism Policy

### 6.1 Maximum Parallelism

The maximum number of subordinate Agents is:

```text
4
```

This is a hard upper bound.

### 6.2 Default Parallelism

The default number of subordinate Agents is:

```text
0 or 1
```

Parallelism is not the default.

### 6.3 When Parallelism Is Allowed

The main Agent may use subordinate Agents only when all conditions hold:

```text
the task can be split cleanly
each subtask has explicit frozen specification references
outputs can be objectively checked
no subordinate Agent writes shared state
the expected gain exceeds token cost
the main Agent can review every output
```

### 6.4 When Parallelism Is Forbidden

Parallelism is forbidden when:

```text
the task boundary is unclear
the task depends on global architecture interpretation
the work requires final normative judgment
the output cannot be objectively validated
the main Agent lacks enough context to review results
parallelism is used merely to appear rigorous
parallelism is used merely to save time
```

### 6.5 Parallelism Justification

Every work package must specify:

```text
Agent Mode:
  main-only
  main+1
  main+2
  main+3
  main+4

Parallelism Justification:
  explicit reason required if not main-only
```

Invalid justification:

```text
faster
more thorough
parallel by default
```

Valid justification example:

```text
The traceability check and negative-test matrix can be independently generated from the same frozen references and merged by the main Agent.
```

---

## 7. Subordinate Agent Task Contract

Every subordinate Agent task must use this contract.

```text
Task ID:
Assigned Agent Role:
Scope:
Frozen Spec References:
Allowed Inputs:
Forbidden Actions:
Expected Output:
Validation Criteria:
Token Budget:
Stop Conditions:
Handoff Format:
```

A task without frozen specification references must not be dispatched.

A task without stop conditions must not be dispatched.

A task whose output cannot be validated must not be dispatched.

---

## 8. Agent Roles

The plan allows four reusable subordinate Agent roles.

These are roles, not always-active workers.

### 8.1 A1 · Spec Trace Agent

Purpose:

```text
verify implementation task references against frozen specs
detect missing references
detect accidental normative rewriting
detect plan/spec mismatch
```

Use when:

```text
a task touches more than one frozen specification
a work package has ambiguous authority
a traceability matrix is being built
```

### 8.2 A2 · Runtime Design Agent

Purpose:

```text
analyze localized runtime implementation structure
identify data/interface dependencies
check whether a work package has enough implementation inputs
```

Use when:

```text
runtime value model
heap/frame/slot layout
RuntimePlan/EIR representation
module runtime
helper dispatch
unwinding machinery
```

A2 must not introduce new runtime semantics.

### 8.3 A3 · Validation/Test Agent

Purpose:

```text
map frozen requirements to tests
construct positive/negative test matrices
identify missing validation gates
```

Use when:

```text
validation pipeline
conformance testing
runtime error behavior
negative test coverage
```

### 8.4 A4 · Integration/Risk Agent

Purpose:

```text
identify cross-package dependencies
detect merge hazards
detect token-cost risks
detect integration sequencing problems
```

Use when:

```text
large work package transition
pre-integration review
pre-release or pre-freeze implementation review
```

---

## 9. Gate System

Implementation work proceeds through gates.

A work package may not move forward unless the relevant gate passes.

### G0 · Scope Gate

Checks:

```text
task is implementation planning or implementation work
task does not reopen frozen specification design
task has explicit frozen spec references
task has clear non-goals
```

Failure result:

```text
task rejected or rewritten
```

### G1 · Spec Reference Gate

Checks:

```text
all implementation requirements cite frozen documents
all aliases resolve to concrete files
no plan-only text is treated as normative
no subordinate output is treated as authoritative without review
```

### G2 · Dependency Gate

Checks:

```text
upstream work packages complete
input artifacts exist
dependency direction is clear
parallel tasks do not write shared state
```

### G3 · Design Gate

Checks:

```text
data structures mapped to specs
interfaces mapped to specs
error behavior mapped to specs
validation behavior mapped to specs
tests identified
forbidden shortcuts listed
```

### G4 · Implementation Gate

Checks:

```text
implementation unit is small enough
rollback path is known
test command is known
expected output is known
owner is known
```

### G5 · Validation Gate

Checks:

```text
schema validation
semantic validation
negative tests
source diagnostics
runtime error checks
capability checks
cache/profile effects
```

### G6 · Integration Gate

Checks:

```text
no frozen-spec regression
no unregistered helper
no unregistered error
no unregistered cache/profile effect
no unwinding bypass
no capability bypass
no host boundary bypass
```

### G7 · Handoff Gate

Every completed unit must report:

```text
completed work
changed files
frozen spec references used
tests added or required
risks discovered
open questions
next recommended work
```

---

## 10. Work Package Model

Every work package must use the following structure:

```text
WP-ID:
Title:
Document Class:
Owner:
Agent Mode:
Parallelism Justification:
Frozen Spec References:
Inputs:
Outputs:
Non-Goals:
Dependencies:
Implementation Tasks:
Validation Gates:
Tests Required:
Risks:
Completion Criteria:
Handoff Notes:
```

A work package is invalid if it omits:

```text
Frozen Spec References
Non-Goals
Validation Gates
Completion Criteria
```

---

## 11. Implementation Roadmap

The implementation roadmap is staged for execution, not for specification design.

### Stage 0 · Agent and Repository Setup

Purpose:

```text
establish process, repository hygiene, and task discipline
```

Expected outputs:

```text
workspace layout
agent task log convention
CI/test command convention
spec reference alias map
```

### Stage 1 · Frozen Spec Ingestion and Traceability

Purpose:

```text
map frozen requirements to implementation work packages
```

Expected outputs:

```text
traceability matrix
work package index
gate checklist
risk register
```

### Stage 2 · Schema and Core Model Skeleton

Purpose:

```text
implement internal data models required before execution
```

Likely areas:

```text
ID types
runtime error codes
RuntimePlan structs
EIR structs
helper descriptors
target/profile descriptors
validation skeleton
```

### Stage 3 · Validation Pipeline

Purpose:

```text
reject malformed internal artifacts before execution
```

Likely areas:

```text
RuntimePlan validation
EIR validation
helper registry validation
module validation
GC metadata validation
cache compatibility validation
```

### Stage 4 · Runtime Core

Purpose:

```text
build executable VM core
```

Likely areas:

```text
Value
Heap
ObjRef
Frame
SlotArray
RegionStack
ControlState
ErrorObj
ReadOnlyView
Host boundary wrapper skeleton
```

### Stage 5 · Interpreter Execution

Purpose:

```text
execute minimal EIR through fast interpreter architecture
```

Likely areas:

```text
dispatch loop
basic expressions
calls
helpers
modules
unwinding
source diagnostics
```

### Stage 6 · Conformance and Regression

Purpose:

```text
prove implementation tracks frozen semantics
```

Expected outputs:

```text
positive tests
negative tests
diagnostic tests
runtime error tests
module cycle tests
unwinding tests
capability tests
```

### Stage 7 · Future-Proofing Hooks

Purpose:

```text
preserve architecture for later GC/JIT/cache/host expansion
```

Expected outputs:

```text
RootMap/FrameMap structures
SafepointRecord structures
JIT backend interface skeleton
cache compatibility checks
target profile checks
host root registry skeleton
```

---

## 12. Work Package Family Index

Detailed work packages will be defined in `WORK-PACKAGE-INDEX.md`.

This master plan reserves the following families.

```text
WP-00 Agent and repository process
WP-01 Frozen spec reference ingestion
WP-02 Traceability matrix construction
WP-03 ID and schema model skeleton
WP-04 Runtime error registry implementation
WP-05 RuntimePlan model and validation
WP-06 EIR model and validation
WP-07 Helper registry and dispatch
WP-08 Value / heap / object reference model
WP-09 Frame / slot / control-state model
WP-10 Structured unwinding implementation
WP-11 Module runtime implementation
WP-12 Call execution protocol implementation
WP-13 ReadOnlyView implementation
WP-14 Host boundary skeleton
WP-15 GC metadata structures
WP-16 Cache compatibility checks
WP-17 Fast interpreter core
WP-18 Conformance test matrix
WP-19 Integration and regression gates
```

No work package may be expanded without frozen spec references.

---

## 13. Traceability Requirements

Every implementation item must trace to at least one frozen specification reference.

Traceability row format:

```text
Implementation Item
Frozen Spec Reference
Required Behavior
Validation Gate
Test Requirement
Status
```

Traceability must not copy full normative text.

It should cite documents and local sections where available.

---

## 14. Risk Management

Risks must be tracked in `RISK-REGISTER.md`.

Minimum risk fields:

```text
Risk ID
Area
Severity
Description
Spec References
Mitigation
Owner
Status
```

Initial high-risk areas:

```text
RuntimePlan/EIR schema drift
helper registry mismatch
structured unwinding edge cases
module circular import behavior
source-span diagnostics
capability/host boundary bypass
cache compatibility invalidation
JIT/GC hook under-specification in implementation
subordinate Agent scope drift
token waste from unnecessary parallelism
```

---

## 15. Change Control

Agent implementation plan documents may change freely before implementation begins.

After implementation begins, plan changes must record:

```text
what changed
why it changed
affected work packages
affected gates
affected risks
affected frozen spec references
```

Plan changes must not modify frozen specifications.

If implementation discovers a possible frozen-spec defect, the main Agent must classify it as:

```text
editorial issue
contradiction repair
specification erratum
requires reopening/later phase
```

using `PHASE-3-FREEZE.md`.

---

## 16. Completion Criteria

This master plan is complete when:

```text
all companion planning files exist
work package index is filled
gate checklist is filled
traceability matrix has initial coverage
risk register has initial risks
handoff template is usable
```

The implementation-planning stage is ready when:

```text
WP-00 through WP-03 pass G0-G3
no work package lacks frozen spec references
no work package requires more than justified parallelism
no subordinate Agent task lacks stop conditions
```

---

## 17. Companion Documents

This directory contains the following companion documents:

```text
AGENT-OPERATING-PROTOCOL.md
WORK-PACKAGE-INDEX.md
GATE-CHECKLIST.md
TRACEABILITY-MATRIX.md
RISK-REGISTER.md
HANDOFF-TEMPLATE.md
README.md
```

At creation time, only this master plan is fully written.

The companion files are placeholders and must be expanded in later planning steps.
---

## 18. Concrete Coding-Plan Requirement

Added: 2026-06-29 11:00:40

The Agent plan is not merely an Agent governance framework.

The Agent plan must be treated as a concrete implementation execution plan.

It must provide:

```text
workspace directories to create
root files to create
documentation placement
Rust workspace layout
crate creation order
module/file creation order
test directory layout
implementation stages
per-stage required actions
per-stage gates
per-stage tests
PROGRESS.md / ISSUE.md update rules
handoff requirements
```

The primary concrete execution document is:

```text
IMPLEMENTATION-CODING-PLAN.md
```

Agents must use it together with:

```text
WORK-PACKAGE-INDEX.md
TRACEABILITY-MATRIX.md
GATE-CHECKLIST.md
RISK-REGISTER.md
HANDOFF-TEMPLATE.md
```

The plan may prescribe coding steps, file creation, crate layout, test layout, and task order.

The plan must not prescribe VM semantics directly.

Whenever a coding step requires semantic content, the Agent must cite the frozen specification document and implement from that reference.

Example:

```text
Correct:
  Create vm_core::eir module.
  Implement EirOp variants according to PHASE-3-EIR-SCHEMA-CLOSURE.md.

Incorrect:
  Invent a new EirOp variant because it is convenient for interpreter dispatch.
```

