# Post-Freeze Issue / Erratum Trigger Policy

Document class: Non-normative process  
Work Package: WP-19  
Aligns with: AGENT.md §15 Erratum Discipline, PHASE-3-FREEZE.md

## Rule

Agents **must not** edit frozen Phase 1–3 normative specifications during implementation.

## Classification when a gap appears

| Class | Action |
|-------|--------|
| Implementation misunderstanding | Fix code; tests; PROGRESS |
| Missing reference | Cite correct frozen doc; no inventing |
| Editorial issue | ISSUE.md INFO/MINOR; no silent code “fix” of prose |
| Contradiction repair candidate | ISSUE.md MAJOR; stop affected path |
| **Specification erratum candidate** | ISSUE.md MAJOR or BLOCKER; Status OPEN; Required Action = classify as erratum; **stop** affected implementation until Main decides |
| Requires later phase | ISSUE.md DEFERRED; do not invent behavior |

## Trigger to open erratum candidate

```text
1. Two frozen documents conflict on a required behavior, OR
2. Required behavior cannot be located in any frozen document and is needed for correctness, OR
3. Implementing frozen text as written would violate another freeze boundary (e.g. public bytecode)
```

## Recording template

Use ISSUE.md entry with:

```text
Severity: MAJOR or BLOCKER
Status: OPEN
Required Action: classify as erratum candidate
Gate Impact: blocked gate id
```

## Out of scope for erratum process

```text
test coverage gaps (use MINOR/MAJOR OPEN with Required Action: add tests)
bootstrap incompleteness already marked later-phase
performance / engineering convenience
```
