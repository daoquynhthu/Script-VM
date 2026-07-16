# Phase 1 Gap Matrix (SPEC-P1 vs implementation)

Document class: Non-normative tracking  
Authority: `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`  
Spec: `ARCHITECTURE/01-phase-1/normative/PHASE-1-LANGUAGE-SPEC.md`  
Updated: 2026-07-16  
Track: **T-P1**

Status values:

```text
YES      — implemented with tests
PARTIAL  — partially implemented; gaps noted
NO       — not implemented
DEFER    — explicitly deferred past T-P1 v0 (must not silent-skip)
```

---

## 1. Source text & lexical (§3–§6) — WP-L01 focus

| SPEC | Summary | Status | Evidence | Stage |
|------|---------|--------|----------|-------|
| §3.1 | Source file = module | YES | parse module API | L2 |
| §3.2 | UTF-8 source; ill-formed is lexical error | PARTIAL | Rust `&str` implies UTF-8; no separate invalid-UTF-8 path | L1 |
| §3.3 | Identifier NFC for comparison | PARTIAL | `script_lex` NFC on idents; same-scope NFC clash is **sema** (L3) | L1/L3 |
| §3.3 | Confusable mixed-script diagnose | DEFER | Optional for conformance | — |
| §3.4 | LF / CRLF / CR terminators | YES | lexer tests crlf; CR covered in L1 | L1 |
| §3.5 | `#` comments; not inside strings | YES | lexer tests | L1 |
| §3.6 | Blank lines ignore indent | YES | lexer | L1 |
| §3.7 | Logical lines; join inside `()[]{}` | YES | paren_continuation test | L1 |
| §4.1–4.3 | Indent stack INDENT/DEDENT | YES | indent tests | L1 |
| §4.2 | Tabs in leading indent invalid | YES | tab_in_indent_rejected | L1 |
| §4.4 | Empty blocks invalid | NO | parser/sema (no `pass`) | L2/L3 |
| §5.1 | Token categories | YES | TokenKind | L1 |
| §5.2 | Ident XID + `_` | YES | unicode-ident + tests L1 | L1 |
| §5.3 | Defined + future reserved keywords | YES | Keyword enum | L1 |
| §5.3 | Contextual keywords as idents | YES | case etc. as Ident | L1 |
| §5.4 | Delimiters incl. `..` `->` `?` `\|` | YES | lexer | L1 |
| §5.5 | Operators incl. augmented | YES | lexer | L1 |
| §6.1–6.2 | nil/true/false keywords | YES | Keyword | L1 |
| §6.3 | Integer rules / underscores | YES | integer_rules | L1 |
| §6.4 | Float rules; reject `1.` `.5` | YES | float_rules | L1 |
| §6.5 | Strings + escapes + `\u{…}` | YES | string tests | L1 |
| §6.5 | No triple/raw strings | YES | not implemented = correct | L1 |
| §6.6–6.8 | List/map/record **literals** (syntax) | PARTIAL | list parse exists; map/record L2 | L2 |

---

## 2. Grammar / AST (§7–§10+) — WP-L02

| SPEC | Summary | Status | Stage |
|------|---------|--------|-------|
| §8 Module / top_level | PARTIAL | module of decl/stmt | L2 |
| §9 let/const/def | PARTIAL | bootstrap | L2 |
| §9.6 import | PARTIAL | `import a.b as c` only; no `from` import | L2 |
| §9.7 export | PARTIAL | export let/const/def | L2 |
| §10 if/while/for/return/break/continue | PARTIAL | for/break/continue parse; raise/assert parse | L2 |
| §10 assignment / augmented | PARTIAL | `=` only; no `+=` stmt form | L2 |
| match/try/record/enum/defer/use | DEFER or NO | not v0 required surface | L2 GAP |
| §5 / expr full | PARTIAL | call, arith, compare, list; no map/index/attr full | L2 |

---

## 3. Semantics (§2, binding, control) — WP-L03

| SPEC | Summary | Status | Stage |
|------|---------|--------|-------|
| §2.1 No implicit binding by assign | YES | script_sema | L3 |
| §2.2 Block scope | YES | if/while/for scopes | L3 |
| §2.3 Conditions must be Bool | NO | not enforced in sema | L3 |
| const/def immutable assign | YES | sema | L3 |
| break/continue only in loop | YES | sema | L3 |
| top-level return invalid | YES | sema | L3 |
| duplicate binding same scope | YES | sema | L3 |
| NFC same-scope clash | NO | L3 | L3 |
| export visibility marking | PARTIAL | parse only | L3 |
| §2.4+ type/coercion/etc. | DEFER | beyond T-P1 binding skeleton | later |

---

## 4. Diagnostics & API — WP-L04

| Item | Status | Stage |
|------|--------|-------|
| Lex errors with byte Span | YES | L1 |
| Line/col from span | PARTIAL | L1 adds helper; full diag crate L4 |
| Unified frontend diagnostic type | NO | L4 |
| `AnalyzedModule` stable API | NO | L4 |
| Parse/sema errors with span | PARTIAL | ParseError/SemaError have span | L4 |

---

## 5. Out of T-P1 (recorded)

| Item | Track |
|------|-------|
| Full SIR materialization | T-P2 |
| SIR → RuntimePlan/EIR normative lowering | T-P3L |
| `script_codegen` demo path | T-DEMO (quarantined) |
| Production print I/O | T-P3L / host |

---

## 6. WP-L00 completion

```text
[x] Unified guide published
[x] This GAP matrix v0
[x] WP-L* registered in WORK-PACKAGE-INDEX
[x] WP-20..25 superseded as plan IDs
```
