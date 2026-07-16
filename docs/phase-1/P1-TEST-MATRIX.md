# Phase 1 Test Matrix (lexical focus)

Document class: Non-normative  
Spec: `PHASE-1-LANGUAGE-SPEC.md` §3–§6  
Crate: `script_lex`  
Updated: 2026-07-16

| ID | SPEC | Description | Test |
|----|------|-------------|------|
| LX-01 | §5.1 | empty → EOF | `empty_source_is_eof` |
| LX-02 | §5.3/§9 | let binding line | `let_binding_line` |
| LX-03 | §3.5–3.6 | comments / blanks | `comments_and_blank_lines_ignored` |
| LX-04 | §4.3 | indent/dedent | `indent_dedent_if_block` |
| LX-05 | §4.2 | tab indent reject | `tab_in_indent_rejected` |
| LX-06 | §4.3 | bad indent level | `bad_indent_level_rejected` |
| LX-07 | §6.3 | integer rules | `integer_rules` |
| LX-08 | §6.4 | float rules | `float_rules` |
| LX-09 | §6.5 | string escapes | `string_escapes` |
| LX-10 | §6.5 | unicode escape | `unicode_escape` |
| LX-11 | §3.7 | paren continuation | `paren_continuation_no_indent` |
| LX-12 | §3.5 | hash in string | `hash_inside_string_not_comment` |
| LX-13 | §5.4–5.5 | operators/arrows | `operators_and_arrows` |
| LX-14 | §5.3 | contextual idents | `contextual_words_are_identifiers` |
| LX-15 | §5.3 | reserved keyword | `reserved_keyword_not_ident` |
| LX-16 | §3.4 | CRLF | `crlf_line_endings` |
| LX-17 | §6.5 | unterminated string | `unterminated_string` |
| LX-18 | v0 | fib tokenize | `fib_snippet_tokenizes` |
| LX-19 | §3.4 | CR-only newlines | `cr_only_line_endings` |
| LX-20 | §3.3 | NFC identifier form | `identifier_nfc_normalized` |
| LX-21 | §5.2 | Unicode XID ident | `unicode_xid_identifier` |
| LX-22 | §5.3 | keyword table exhaust | `all_defined_and_future_keywords` |
| LX-23 | §5.4 | delimiter set | `delimiters_lexed` |
| LX-24 | §3.7 | bracket/brace join | `bracket_brace_continuation` |
