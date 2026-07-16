//! Phase 1 lexer: source text → token stream.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §3–§6

use unicode_ident::{is_xid_continue, is_xid_start};
use unicode_normalization::UnicodeNormalization;

use crate::error::LexError;
use crate::span::Span;
use crate::token::{Keyword, Token, TokenKind};

/// Lex full source text into tokens (including trailing EOF).
pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    Lexer::new(source).tokenize_all()
}

struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    /// Current byte offset.
    pos: usize,
    /// Indentation stack (spaces); always starts with 0.
    indents: Vec<usize>,
    /// Nesting of `()[]{}` for logical-line continuation.
    paren_depth: i32,
    /// Pending tokens (indent/dedent) flushed before content.
    pending: Vec<Token>,
    /// True after emitting content on the current logical line (before NEWLINE).
    line_has_content: bool,
    /// At start of a physical line that may need indent processing.
    at_line_start: bool,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            indents: vec![0],
            paren_depth: 0,
            pending: Vec::new(),
            line_has_content: false,
            at_line_start: true,
        }
    }

    fn tokenize_all(mut self) -> Result<Vec<Token>, LexError> {
        let mut out = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            out.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(out)
    }

    fn next_token(&mut self) -> Result<Token, LexError> {
        if let Some(t) = self.pending.pop() {
            return Ok(t);
        }

        loop {
            if self.at_line_start && self.paren_depth == 0 {
                self.handle_line_start()?;
                if let Some(t) = self.pending.pop() {
                    return Ok(t);
                }
            }

            self.skip_spaces_and_comments();

            if self.pos >= self.bytes.len() {
                return self.emit_eof();
            }

            let b = self.bytes[self.pos];

            // Physical newline
            if b == b'\n' || b == b'\r' {
                let start = self.pos as u32;
                self.consume_newline();
                if self.paren_depth > 0 {
                    // Logical line continues; treat as whitespace.
                    self.at_line_start = true;
                    continue;
                }
                if self.line_has_content {
                    self.line_has_content = false;
                    self.at_line_start = true;
                    return Ok(Token::new(TokenKind::Newline, Span::new(start, self.pos as u32)));
                }
                // Blank / comment-only line: no NEWLINE token.
                self.at_line_start = true;
                continue;
            }

            self.at_line_start = false;
            self.line_has_content = true;
            return self.lex_content_token();
        }
    }

    fn handle_line_start(&mut self) -> Result<(), LexError> {
        // Count leading spaces; reject tabs in indent.
        let line_byte_start = self.pos;
        let mut spaces = 0usize;
        while self.pos < self.bytes.len() {
            match self.bytes[self.pos] {
                b' ' => {
                    spaces += 1;
                    self.pos += 1;
                }
                b'\t' => {
                    return Err(LexError::TabInIndentation {
                        span: Span::new(self.pos as u32, (self.pos + 1) as u32),
                    });
                }
                _ => break,
            }
        }

        // Comment-only or blank line: leave indent stack unchanged, consume later.
        if self.pos >= self.bytes.len() {
            self.at_line_start = false; // fall through to EOF path
            return Ok(());
        }
        let b = self.bytes[self.pos];
        if b == b'#' || b == b'\n' || b == b'\r' {
            // Do not emit indent tokens; do not change stack.
            // Rewind spaces so blank-line handling sees them as ordinary whitespace.
            self.pos = line_byte_start;
            self.at_line_start = false;
            return Ok(());
        }

        // Non-blank logical line: apply indent stack rules.
        let top = *self.indents.last().unwrap();
        if spaces == top {
            // no token
        } else if spaces > top {
            self.indents.push(spaces);
            self.pending.push(Token::new(
                TokenKind::Indent,
                Span::new(line_byte_start as u32, self.pos as u32),
            ));
        } else {
            while let Some(&level) = self.indents.last() {
                if level == spaces {
                    break;
                }
                if level < spaces {
                    return Err(LexError::IndentationError {
                        span: Span::new(line_byte_start as u32, self.pos as u32),
                    });
                }
                self.indents.pop();
                self.pending.push(Token::new(
                    TokenKind::Dedent,
                    Span::new(line_byte_start as u32, self.pos as u32),
                ));
            }
            if self.indents.last().copied() != Some(spaces) {
                return Err(LexError::IndentationError {
                    span: Span::new(line_byte_start as u32, self.pos as u32),
                });
            }
        }

        // Reverse pending so first pop is first DEDENT/INDENT in order.
        // We pushed DEDENTs outer-first; pop order should be outer-first too if we reverse once.
        // Actually: stack pop order LIFO — we want outermost dedent first for nested blocks.
        // Python emits DEDENT for each level from inside out: for levels [0,2,4] going to 0:
        // emit DEDENT (4→2), DEDENT (2→0). We pushed 4 then 2, so pop gives 2 then 4 — wrong.
        // Reverse so pop order is 4 then 2.
        self.pending.reverse();

        self.at_line_start = false;
        Ok(())
    }

    fn emit_eof(&mut self) -> Result<Token, LexError> {
        let at = self.pos as u32;
        // Finish last logical line if file has no trailing newline.
        if self.line_has_content {
            self.line_has_content = false;
            return Ok(Token::new(TokenKind::Newline, Span::empty(at)));
        }
        // Close any open indents (outermost first).
        if self.indents.len() > 1 {
            while self.indents.len() > 1 {
                self.indents.pop();
                self.pending.push(Token::new(TokenKind::Dedent, Span::empty(at)));
            }
            self.pending.reverse();
            if let Some(t) = self.pending.pop() {
                return Ok(t);
            }
        }
        Ok(Token::new(TokenKind::Eof, Span::empty(at)))
    }

    fn skip_spaces_and_comments(&mut self) {
        loop {
            while self.pos < self.bytes.len() && self.bytes[self.pos] == b' ' {
                self.pos += 1;
            }
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'#' {
                while self.pos < self.bytes.len()
                    && self.bytes[self.pos] != b'\n'
                    && self.bytes[self.pos] != b'\r'
                {
                    self.pos += 1;
                }
                continue;
            }
            break;
        }
    }

    fn consume_newline(&mut self) {
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'\r' {
            self.pos += 1;
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'\n' {
                self.pos += 1;
            }
        } else if self.pos < self.bytes.len() && self.bytes[self.pos] == b'\n' {
            self.pos += 1;
        }
    }

    fn lex_content_token(&mut self) -> Result<Token, LexError> {
        let start = self.pos;
        let ch = self.peek_char().ok_or_else(|| LexError::UnexpectedChar {
            span: Span::empty(start as u32),
            ch: '\0',
        })?;

        // String
        if ch == '"' || ch == '\'' {
            return self.lex_string(ch);
        }

        // Number
        if ch.is_ascii_digit() {
            return self.lex_number();
        }

        // Identifier / keyword
        if ch == '_' || ch.is_ascii_alphabetic() || is_xid_start(ch) {
            return self.lex_ident_or_keyword();
        }

        // Multi-char operators / delimiters
        if let Some(tok) = self.try_lex_operator_or_delim()? {
            return Ok(tok);
        }

        Err(LexError::UnexpectedChar {
            span: Span::new(start as u32, (start + ch.len_utf8()) as u32),
            ch,
        })
    }

    fn try_lex_operator_or_delim(&mut self) -> Result<Option<Token>, LexError> {
        let start = self.pos;
        let b0 = self.bytes[self.pos];
        let b1 = self.bytes.get(self.pos + 1).copied();

        let (kind, len) = match (b0, b1) {
            (b'.', Some(b'.')) => (TokenKind::DotDot, 2),
            (b'-', Some(b'>')) => (TokenKind::Arrow, 2),
            (b'+', Some(b'=')) => (TokenKind::PlusAssign, 2),
            (b'-', Some(b'=')) => (TokenKind::MinusAssign, 2),
            (b'*', Some(b'=')) => (TokenKind::StarAssign, 2),
            (b'/', Some(b'=')) => (TokenKind::SlashAssign, 2),
            (b'%', Some(b'=')) => (TokenKind::PercentAssign, 2),
            (b'=', Some(b'=')) => (TokenKind::Eq, 2),
            (b'!', Some(b'=')) => (TokenKind::NotEq, 2),
            (b'<', Some(b'=')) => (TokenKind::LtEq, 2),
            (b'>', Some(b'=')) => (TokenKind::GtEq, 2),
            (b'+', _) => (TokenKind::Plus, 1),
            (b'-', _) => (TokenKind::Minus, 1),
            (b'*', _) => (TokenKind::Star, 1),
            (b'/', _) => (TokenKind::Slash, 1),
            (b'%', _) => (TokenKind::Percent, 1),
            (b'=', _) => (TokenKind::Assign, 1),
            (b'<', _) => (TokenKind::Lt, 1),
            (b'>', _) => (TokenKind::Gt, 1),
            (b'(', _) => {
                self.paren_depth += 1;
                (TokenKind::LParen, 1)
            }
            (b')', _) => {
                self.paren_depth -= 1;
                (TokenKind::RParen, 1)
            }
            (b'[', _) => {
                self.paren_depth += 1;
                (TokenKind::LBracket, 1)
            }
            (b']', _) => {
                self.paren_depth -= 1;
                (TokenKind::RBracket, 1)
            }
            (b'{', _) => {
                self.paren_depth += 1;
                (TokenKind::LBrace, 1)
            }
            (b'}', _) => {
                self.paren_depth -= 1;
                (TokenKind::RBrace, 1)
            }
            (b',', _) => (TokenKind::Comma, 1),
            (b':', _) => (TokenKind::Colon, 1),
            (b'.', _) => (TokenKind::Dot, 1),
            (b'|', _) => (TokenKind::Pipe, 1),
            (b'?', _) => (TokenKind::Question, 1),
            _ => return Ok(None),
        };

        self.pos += len;
        Ok(Some(Token::new(
            kind,
            Span::new(start as u32, self.pos as u32),
        )))
    }

    fn lex_ident_or_keyword(&mut self) -> Result<Token, LexError> {
        let start = self.pos;
        let ch = self.bump_char().unwrap();
        // first char already validated
        let _ = ch;
        while let Some(c) = self.peek_char() {
            if c == '_' || c.is_ascii_alphanumeric() || is_xid_continue(c) {
                self.bump_char();
            } else {
                break;
            }
        }
        let raw = &self.src[start..self.pos];
        let nfc: String = raw.nfc().collect();
        let kind = if let Some(kw) = Keyword::from_ident(&nfc) {
            TokenKind::Keyword(kw)
        } else {
            TokenKind::Ident {
                nfc,
                raw: raw.to_string(),
            }
        };
        Ok(Token::new(kind, Span::new(start as u32, self.pos as u32)))
    }

    fn lex_number(&mut self) -> Result<Token, LexError> {
        let start = self.pos;

        // Integer or float
        // Read digits with underscore rules
        let int_part = self.read_digits_with_underscores(start)?;
        let after_int = self.pos;

        let is_float = if self.peek_byte() == Some(b'.') {
            // Float "1.0", range "1..", or invalid bare "1." (spec §6.4).
            let next = self.bytes.get(self.pos + 1).copied();
            if next.is_some_and(|b| b.is_ascii_digit()) {
                true
            } else if next == Some(b'.') {
                false // integer then DotDot
            } else {
                return Err(LexError::InvalidFloat {
                    span: Span::new(start as u32, (self.pos + 1) as u32),
                    message: "digits required after decimal point".into(),
                });
            }
        } else if matches!(self.peek_byte(), Some(b'e' | b'E')) {
            true
        } else {
            false
        };

        if !is_float {
            // Validate integer form: no leading zeros except "0"
            if int_part.len() > 1 && int_part.as_bytes()[0] == b'0' {
                return Err(LexError::InvalidInteger {
                    span: Span::new(start as u32, after_int as u32),
                    message: "leading zeros are not allowed".into(),
                });
            }
            return Ok(Token::new(
                TokenKind::Int(int_part),
                Span::new(start as u32, after_int as u32),
            ));
        }

        // Float: digits "." digits [exp] | digits exp
        let mut lexeme = int_part;
        if self.peek_byte() == Some(b'.') {
            self.pos += 1;
            lexeme.push('.');
            if !self.peek_byte().is_some_and(|b| b.is_ascii_digit()) {
                return Err(LexError::InvalidFloat {
                    span: Span::new(start as u32, self.pos as u32),
                    message: "digits required after decimal point".into(),
                });
            }
            let frac = self.read_digits_with_underscores(self.pos)?;
            lexeme.push_str(&frac);
        }

        if matches!(self.peek_byte(), Some(b'e' | b'E')) {
            lexeme.push(self.bump_char().unwrap());
            if matches!(self.peek_byte(), Some(b'+' | b'-')) {
                lexeme.push(self.bump_char().unwrap());
            }
            if !self.peek_byte().is_some_and(|b| b.is_ascii_digit()) {
                return Err(LexError::InvalidFloat {
                    span: Span::new(start as u32, self.pos as u32),
                    message: "digits required in exponent".into(),
                });
            }
            let exp = self.read_digits_with_underscores(self.pos)?;
            lexeme.push_str(&exp);
        }

        // Leading zero rule applies to integer part of float? Spec shows 0.5 valid.
        // Only pure integers ban leading zeros.

        Ok(Token::new(
            TokenKind::Float(lexeme),
            Span::new(start as u32, self.pos as u32),
        ))
    }

    /// Read `digit { "_" digit | digit }` style; returns digits without underscores.
    fn read_digits_with_underscores(&mut self, start: usize) -> Result<String, LexError> {
        if !self.peek_byte().is_some_and(|b| b.is_ascii_digit()) {
            return Err(LexError::InvalidInteger {
                span: Span::new(start as u32, self.pos as u32),
                message: "expected digit".into(),
            });
        }
        let mut out = String::new();
        let mut last_underscore = false;
        while let Some(b) = self.peek_byte() {
            if b.is_ascii_digit() {
                out.push(b as char);
                self.pos += 1;
                last_underscore = false;
            } else if b == b'_' {
                if last_underscore {
                    return Err(LexError::InvalidInteger {
                        span: Span::new(start as u32, (self.pos + 1) as u32),
                        message: "consecutive underscores".into(),
                    });
                }
                last_underscore = true;
                self.pos += 1;
            } else {
                break;
            }
        }
        if last_underscore {
            return Err(LexError::InvalidInteger {
                span: Span::new(start as u32, self.pos as u32),
                message: "trailing underscore".into(),
            });
        }
        Ok(out)
    }

    fn lex_string(&mut self, quote: char) -> Result<Token, LexError> {
        let start = self.pos;
        self.bump_char(); // opening quote
        let mut value = String::new();
        loop {
            let ch = match self.peek_char() {
                Some(c) => c,
                None => {
                    return Err(LexError::InvalidString {
                        span: Span::new(start as u32, self.pos as u32),
                        message: "unterminated string".into(),
                    });
                }
            };
            if ch == '\n' || ch == '\r' {
                return Err(LexError::InvalidString {
                    span: Span::new(start as u32, self.pos as u32),
                    message: "unescaped newline in string".into(),
                });
            }
            if ch == quote {
                self.bump_char();
                break;
            }
            if ch == '\\' {
                self.bump_char();
                let esc = self.peek_char().ok_or_else(|| LexError::InvalidString {
                    span: Span::new(start as u32, self.pos as u32),
                    message: "unterminated escape".into(),
                })?;
                match esc {
                    '\\' => {
                        value.push('\\');
                        self.bump_char();
                    }
                    '\'' => {
                        value.push('\'');
                        self.bump_char();
                    }
                    '"' => {
                        value.push('"');
                        self.bump_char();
                    }
                    'n' => {
                        value.push('\n');
                        self.bump_char();
                    }
                    'r' => {
                        value.push('\r');
                        self.bump_char();
                    }
                    't' => {
                        value.push('\t');
                        self.bump_char();
                    }
                    '0' => {
                        value.push('\0');
                        self.bump_char();
                    }
                    'u' => {
                        self.bump_char();
                        if self.peek_char() != Some('{') {
                            return Err(LexError::InvalidString {
                                span: Span::new(start as u32, self.pos as u32),
                                message: "expected { after \\u".into(),
                            });
                        }
                        self.bump_char();
                        let hex_start = self.pos;
                        while self.peek_char().is_some_and(|c| c.is_ascii_hexdigit()) {
                            self.bump_char();
                        }
                        if self.peek_char() != Some('}') {
                            return Err(LexError::InvalidString {
                                span: Span::new(start as u32, self.pos as u32),
                                message: "expected } in \\u{...}".into(),
                            });
                        }
                        let hex = &self.src[hex_start..self.pos];
                        if hex.is_empty() || hex.len() > 6 {
                            return Err(LexError::InvalidString {
                                span: Span::new(start as u32, self.pos as u32),
                                message: "invalid unicode escape length".into(),
                            });
                        }
                        let code = u32::from_str_radix(hex, 16).map_err(|_| {
                            LexError::InvalidString {
                                span: Span::new(start as u32, self.pos as u32),
                                message: "invalid unicode escape".into(),
                            }
                        })?;
                        let c = char::from_u32(code).ok_or_else(|| LexError::InvalidString {
                            span: Span::new(start as u32, self.pos as u32),
                            message: "invalid unicode scalar".into(),
                        })?;
                        value.push(c);
                        self.bump_char(); // }
                    }
                    other => {
                        return Err(LexError::InvalidString {
                            span: Span::new(start as u32, self.pos as u32),
                            message: format!("unknown escape \\{other}"),
                        });
                    }
                }
            } else {
                value.push(ch);
                self.bump_char();
            }
        }
        Ok(Token::new(
            TokenKind::String(value),
            Span::new(start as u32, self.pos as u32),
        ))
    }

    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    fn kinds(src: &str) -> Vec<TokenKind> {
        lex(src)
            .unwrap()
            .into_iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn empty_source_is_eof() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
    }

    #[test]
    fn let_binding_line() {
        let ks = kinds("let x = 1\n");
        assert_eq!(
            ks,
            vec![
                TokenKind::Keyword(Keyword::Let),
                TokenKind::Ident {
                    nfc: "x".into(),
                    raw: "x".into()
                },
                TokenKind::Assign,
                TokenKind::Int("1".into()),
                TokenKind::Newline,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn comments_and_blank_lines_ignored() {
        let ks = kinds("# header\n\nlet x = 1  # trail\n");
        assert!(matches!(ks[0], TokenKind::Keyword(Keyword::Let)));
        assert!(ks.iter().any(|k| matches!(k, TokenKind::Int(_))));
    }

    #[test]
    fn indent_dedent_if_block() {
        let src = "if true:\n    let x = 1\nlet y = 2\n";
        let ks = kinds(src);
        assert!(ks.iter().any(|k| matches!(k, TokenKind::Indent)));
        assert!(ks.iter().any(|k| matches!(k, TokenKind::Dedent)));
        assert!(ks.iter().any(|k| matches!(k, TokenKind::Keyword(Keyword::If))));
    }

    #[test]
    fn tab_in_indent_rejected() {
        let err = lex("if true:\n\tlet x = 1\n").unwrap_err();
        assert!(matches!(err, LexError::TabInIndentation { .. }));
    }

    #[test]
    fn bad_indent_level_rejected() {
        let err = lex("if true:\n    let x = 1\n  let y = 2\n").unwrap_err();
        assert!(matches!(err, LexError::IndentationError { .. }));
    }

    #[test]
    fn integer_rules() {
        assert!(matches!(
            &kinds("0\n")[0],
            TokenKind::Int(s) if s == "0"
        ));
        assert!(matches!(
            &kinds("1_000\n")[0],
            TokenKind::Int(s) if s == "1000"
        ));
        assert!(lex("01\n").is_err());
        assert!(lex("1_\n").is_err());
        assert!(lex("1__0\n").is_err());
    }

    #[test]
    fn float_rules() {
        assert!(matches!(
            &kinds("1.0\n")[0],
            TokenKind::Float(s) if s == "1.0"
        ));
        assert!(matches!(
            &kinds("10e3\n")[0],
            TokenKind::Float(s) if s == "10e3"
        ));
        assert!(lex("1.\n").is_err());
        // .5 is not a float per spec; may be unexpected or other — must not be Float
        let r = lex(".5\n");
        assert!(r.is_err() || !matches!(r.unwrap()[0].kind, TokenKind::Float(_)));
    }

    #[test]
    fn string_escapes() {
        let ks = kinds(r#""hi\n" + 'a'"#);
        assert!(matches!(&ks[0], TokenKind::String(s) if s == "hi\n"));
        assert!(matches!(&ks[2], TokenKind::String(s) if s == "a"));
    }

    #[test]
    fn unicode_escape() {
        let ks = kinds(r#""\u{41}""#);
        assert!(matches!(&ks[0], TokenKind::String(s) if s == "A"));
    }

    #[test]
    fn paren_continuation_no_indent() {
        let src = "let xs = [\n    1,\n    2,\n]\n";
        let ks = kinds(src);
        // Inside brackets, newlines do not emit NEWLINE/INDENT for the list body
        // We still may get content tokens only until closing.
        assert!(ks.iter().any(|k| matches!(k, TokenKind::LBracket)));
        assert!(ks.iter().any(|k| matches!(k, TokenKind::RBracket)));
        // No Indent for the list continuation lines
        let indent_count = ks.iter().filter(|k| matches!(k, TokenKind::Indent)).count();
        assert_eq!(indent_count, 0);
    }

    #[test]
    fn hash_inside_string_not_comment() {
        let src = "let s = \"# not comment\"";
        let ks = kinds(src);
        assert!(matches!(
            ks.iter().find(|k| matches!(k, TokenKind::String(_))),
            Some(TokenKind::String(s)) if s == "# not comment"
        ));
    }

    #[test]
    fn operators_and_arrows() {
        let ks = kinds("a -> b .. c += 1\n");
        assert!(ks.iter().any(|k| matches!(k, TokenKind::Arrow)));
        assert!(ks.iter().any(|k| matches!(k, TokenKind::DotDot)));
        assert!(ks.iter().any(|k| matches!(k, TokenKind::PlusAssign)));
    }

    #[test]
    fn contextual_words_are_identifiers() {
        let ks = kinds("let case = 1\n");
        assert!(matches!(
            &ks[1],
            TokenKind::Ident { nfc, .. } if nfc == "case"
        ));
    }

    #[test]
    fn reserved_keyword_not_ident() {
        let ks = kinds("let class = 1\n");
        assert!(matches!(&ks[1], TokenKind::Keyword(Keyword::Class)));
    }

    #[test]
    fn crlf_line_endings() {
        let ks = kinds("let x = 1\r\nlet y = 2\r\n");
        assert_eq!(
            ks.iter().filter(|k| matches!(k, TokenKind::Newline)).count(),
            2
        );
    }

    #[test]
    fn unterminated_string() {
        assert!(matches!(
            lex("\"abc"),
            Err(LexError::InvalidString { .. })
        ));
    }

    #[test]
    fn fib_snippet_tokenizes() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        let toks = lex(src).expect("fib should lex");
        assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::Keyword(Keyword::Def))));
        assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::Keyword(Keyword::Return))));
        assert!(matches!(toks.last().map(|t| &t.kind), Some(TokenKind::Eof)));
    }
}
