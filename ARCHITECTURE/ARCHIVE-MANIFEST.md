# Script VM Full Archive Manifest

Generated: 2026-06-29 09:36:12

## Purpose

This archive classifies and preserves the complete Phase 1 to Phase 3 documentation set.

The archive is organized by phase and document role:

```text
00-project/
01-phase-1/
02-phase-2/
03-phase-3/
99-unclassified/   # present only if unexpected root docs exist
```

## Current Baseline Status

```text
Phase 1: frozen
Phase 2: frozen
Phase 3: Version 1.0 frozen normative baseline
```

Phase 3 freeze declaration:

```text
03-phase-3/freeze/PHASE-3-FREEZE.md
```

## Document Role Policy

```text
normative
  Specification documents that define language, IR, VM, runtime, or semantic rules.

freeze
  Freeze declarations.

aggregates
  Generated normative frontdoors or aggregate documents.

implementation-plans
  Non-normative implementation planning documents.

audits
  Audit and readiness review documents.

admin
  Changelog, manifest, repair log, tech-stack notes, status documents.

project
  Project-level overview and workspace status/index documents.
```

## Project-Level Documents

- `00-project/PROJECT-OVERVIEW.md`
- `00-project/STATUS.md`
- `00-project/WORKSPACE-INDEX.md`

## Phase 1 Documents

- `01-phase-1/admin/PHASE-1-CHANGELOG.md`
- `01-phase-1/freeze/PHASE-1-FREEZE.md`
- `01-phase-1/normative/PHASE-1-LANGUAGE-DESIGN.md`
- `01-phase-1/normative/PHASE-1-LANGUAGE-SPEC.md`

## Phase 2 Documents

- `02-phase-2/admin/PHASE-2-CHANGELOG.md`
- `02-phase-2/freeze/PHASE-2-FREEZE.md`
- `02-phase-2/normative/PHASE-2-IR-DESIGN.md`
- `02-phase-2/normative/PHASE-2-IR-FRAMEWORK.md`
- `02-phase-2/normative/PHASE-2-IR-SPEC.md`
- `02-phase-2/normative/PHASE-2-SIR-INTEGRATION-ROUND4.md`
- `02-phase-2/normative/PHASE-2-SIR-SEMANTICS-ROUND1.md`
- `02-phase-2/normative/PHASE-2-SIR-SEMANTICS-ROUND2.md`
- `02-phase-2/normative/PHASE-2-SIR-SEMANTICS-ROUND3.md`

## Phase 3 Documents

- `03-phase-3/admin/PHASE-3-AUDIT-REPAIR-LOG.md`
- `03-phase-3/admin/PHASE-3-CHANGELOG.md`
- `03-phase-3/admin/PHASE-3-DOCUMENT-MANIFEST.md`
- `03-phase-3/admin/PHASE-3-TECH-STACK.md`
- `03-phase-3/aggregates/PHASE-3-MINIMAL-VM.md`
- `03-phase-3/aggregates/PHASE-3-VM-SPEC.md`
- `03-phase-3/audits/PHASE-3-FINAL-FREEZE-READINESS-AUDIT.md`
- `03-phase-3/audits/PHASE-3-NORMATIVE-CONSISTENCY-AUDIT.md`
- `03-phase-3/audits/PHASE-3-NORMATIVE-LANGUAGE-SWEEP.md`
- `03-phase-3/audits/PHASE-3-SECOND-NORMATIVE-AUDIT.md`
- `03-phase-3/freeze/PHASE-3-FREEZE.md`
- `03-phase-3/implementation-plans/PHASE-3-FAST-INTERPRETER-IMPLEMENTATION-MILESTONES.md`
- `03-phase-3/implementation-plans/PHASE-3-GC-IMPLEMENTATION-STAGING-PLAN.md`
- `03-phase-3/implementation-plans/PHASE-3-IMPLEMENTATION-PLAN.md`
- `03-phase-3/implementation-plans/PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md`
- `03-phase-3/normative/PHASE-3-BASELINE-JIT-BACKEND-INTERFACE.md`
- `03-phase-3/normative/PHASE-3-CACHE-COMPATIBILITY-MATRIX.md`
- `03-phase-3/normative/PHASE-3-CALL-EXECUTION-PROTOCOL.md`
- `03-phase-3/normative/PHASE-3-CONTROL-LOWERING-ROUND2.md`
- `03-phase-3/normative/PHASE-3-CONTROL-STATE-MODEL.md`
- `03-phase-3/normative/PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md`
- `03-phase-3/normative/PHASE-3-EIR-SCHEMA-CLOSURE.md`
- `03-phase-3/normative/PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md`
- `03-phase-3/normative/PHASE-3-GC-METADATA-OWNERSHIP.md`
- `03-phase-3/normative/PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md`
- `03-phase-3/normative/PHASE-3-HOST-BOUNDARY-CONTRACT.md`
- `03-phase-3/normative/PHASE-3-JIT-LOWERING-MATRIX.md`
- `03-phase-3/normative/PHASE-3-MODULE-RUNTIME-CONTRACT.md`
- `03-phase-3/normative/PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md`
- `03-phase-3/normative/PHASE-3-PERFORMANCE-ARCHITECTURE.md`
- `03-phase-3/normative/PHASE-3-READONLY-VIEW-SEMANTICS.md`
- `03-phase-3/normative/PHASE-3-RUNTIME-ERROR-REGISTRY.md`
- `03-phase-3/normative/PHASE-3-RUNTIME-HELPER-CONTRACTS.md`
- `03-phase-3/normative/PHASE-3-RUNTIME-HELPER-REGISTRY.md`
- `03-phase-3/normative/PHASE-3-RUNTIMEPLAN-EIR-FRAMEWORK.md`
- `03-phase-3/normative/PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md`
- `03-phase-3/normative/PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md`
- `03-phase-3/normative/PHASE-3-SIR-LOWERING-ROUND1.md`
- `03-phase-3/normative/PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md`
- `03-phase-3/normative/PHASE-3-TARGET-PROFILE-SCHEMAS.md`
- `03-phase-3/normative/PHASE-3-VALIDATION-MATRIX.md`
- `03-phase-3/normative/PHASE-3-VALUE-KEY-STRING-SEMANTICS.md`
- `03-phase-3/normative/PHASE-3-VM-FRAMEWORK.md`
- `03-phase-3/normative/PHASE-3-VM-RUNTIME-ROUND1.md`

## Unclassified Documents

- none

## Missing Expected Documents

- none

## Archive Integrity Notes

- Root-level working files were copied into classified directories.
- Existing temporary mirror directories under `script_vm_workspace/phase3/` were not recursively copied to avoid duplicate copies.
- Root compatibility files remain in the original workspace; this archive is the clean classified package.
- No document contents were semantically modified during archiving.
