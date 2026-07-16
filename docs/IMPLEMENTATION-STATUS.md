# Implementation Status Snapshot

Document class: Non-normative rolling status  
Authority: Subordinate to frozen specs, `AGENT.md`, and plan package  
Rule: This file **may be rewritten** as a snapshot.

Updated: 2026-07-14 (Phase 1 frontend WP-20..23 COMPLETE bootstrap)  
Workspace: Phase 3 VM CLOSED + `script_lex` + `script_parse` + `script_sema`

---

## 0. Where we are

| Track | Status |
|-------|--------|
| Phase 3 bootstrap (WP-00..19) | **CLOSED** |
| WP-20 process | **COMPLETE** |
| WP-21 lexer | **COMPLETE** (`script_lex`) |
| WP-22 parser/AST | **COMPLETE** bootstrap (+ for/break/continue) |
| WP-23 semantic binding | **COMPLETE** (`script_sema`) |
| Next | SIR materialization / lowering toward RuntimePlan (new WP) |

---

## 1. Pipeline capability

```text
source
  -> script_lex::lex
  -> script_parse::parse_module
  -> script_sema::analyze_module / check_source
  -> (not yet) SIR / RuntimePlan / vm_eval
```

**Sema rules (bootstrap)**

- `let` mutable; `const`/`def` immutable; no assign without binding  
- Block scope (if/while/for body); for binds iterator var  
- `break`/`continue` only in loop; top-level `return` invalid  
- Unresolved names; duplicate same-scope bindings  
- Prelude: `print` builtin for samples  

**Tests (approx.)**

- `script_lex` 18 · `script_parse` 7 · `script_sema` 13  

---

## 2. Architecture books for next work

| Goal | Specs |
|------|--------|
| SIR unit | `SPEC-P2-FREEZE`, `SPEC-P2-IR`, `SPEC-P2-FRAMEWORK`, SIR rounds |
| Lowering to VM | `SPEC-P3-LOWERING`, `SPEC-P3-EIR`, `SPEC-P3-RTP` |

---

## 3. Recommended next

1. **WP-24**: materialize Phase 2 SIR (or bootstrap HIR→SIR) from analyzed AST  
2. **WP-25**: SIR → RuntimePlan/EIR lowering into existing `vm_eval`  
3. Expand grammar residual: match/record/import as needed for v0  

---

## 4. Effective open audit

No OPEN Phase 1 blockers recorded.
