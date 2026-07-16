//! Recursive-descent parser for Phase 1 bootstrap subset.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §8– (module / declarations / statements / expressions)
//! Implements a minimal surface sufficient for simple scripts (including fib-shaped code).

use script_lex::{lex, Keyword, LexError, Span, Token, TokenKind};

use crate::ast::{
    BinaryOp, Block, Decl, Expr, Item, Module, Stmt, UnaryOp,
};
use crate::error::ParseError;

/// Parse source text into a module AST.
pub fn parse_module(source: &str) -> Result<Module, ParseError> {
    let tokens = lex(source).map_err(lex_to_parse)?;
    Parser::new(tokens).parse_module()
}

fn lex_to_parse(e: LexError) -> ParseError {
    ParseError::new(e.to_string(), e.span())
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .unwrap_or_else(|| self.tokens.last().expect("token stream non-empty"))
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn at_eof(&self) -> bool {
        matches!(self.peek_kind(), TokenKind::Eof)
    }

    fn bump(&mut self) -> &Token {
        let t = &self.tokens[self.pos.min(self.tokens.len() - 1)];
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    fn span_here(&self) -> Span {
        self.peek().span
    }

    fn expect_keyword(&mut self, kw: Keyword) -> Result<Span, ParseError> {
        match self.peek_kind() {
            TokenKind::Keyword(k) if *k == kw => Ok(self.bump().span),
            _ => Err(ParseError::new(
                format!("expected keyword `{}`", kw.as_str()),
                self.span_here(),
            )),
        }
    }

    fn expect_kind(&mut self, pred: impl Fn(&TokenKind) -> bool, msg: &str) -> Result<Span, ParseError> {
        if pred(self.peek_kind()) {
            Ok(self.bump().span)
        } else {
            Err(ParseError::new(msg, self.span_here()))
        }
    }

    fn expect_ident(&mut self) -> Result<(String, Span), ParseError> {
        match self.peek_kind() {
            TokenKind::Ident { nfc, .. } => {
                let name = nfc.clone();
                let span = self.bump().span;
                Ok((name, span))
            }
            _ => Err(ParseError::new("expected identifier", self.span_here())),
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek_kind(), TokenKind::Newline) {
            self.bump();
        }
    }

    fn parse_module(&mut self) -> Result<Module, ParseError> {
        let start = self.span_here().start;
        let mut items = Vec::new();
        self.skip_newlines();
        while !self.at_eof() {
            items.push(self.parse_top_item()?);
            self.skip_newlines();
        }
        let end = self.span_here().end;
        Ok(Module {
            items,
            span: Span::new(start, end),
        })
    }

    fn parse_top_item(&mut self) -> Result<Item, ParseError> {
        match self.peek_kind() {
            TokenKind::Keyword(
                Keyword::Let
                | Keyword::Const
                | Keyword::Def
                | Keyword::Import
                | Keyword::From
                | Keyword::Export,
            ) => Ok(Item::Decl(self.parse_decl()?)),
            _ => Ok(Item::Stmt(self.parse_stmt()?)),
        }
    }

    fn parse_decl(&mut self) -> Result<Decl, ParseError> {
        match self.peek_kind() {
            TokenKind::Keyword(Keyword::Let) => self.parse_let_like(false),
            TokenKind::Keyword(Keyword::Const) => self.parse_let_like(true),
            TokenKind::Keyword(Keyword::Def) => self.parse_function(),
            TokenKind::Keyword(Keyword::Import) => self.parse_import(),
            TokenKind::Keyword(Keyword::From) => self.parse_from_import(),
            TokenKind::Keyword(Keyword::Export) => self.parse_export(),
            _ => Err(ParseError::new("expected declaration", self.span_here())),
        }
    }

    fn parse_module_path(&mut self) -> Result<Vec<String>, ParseError> {
        let mut module_path = Vec::new();
        let (first, _) = self.expect_ident()?;
        module_path.push(first);
        while matches!(self.peek_kind(), TokenKind::Dot) {
            self.bump();
            let (part, _) = self.expect_ident()?;
            module_path.push(part);
        }
        Ok(module_path)
    }

    fn parse_import(&mut self) -> Result<Decl, ParseError> {
        let start = self.expect_keyword(Keyword::Import)?;
        let module_path = self.parse_module_path()?;
        let alias = if matches!(self.peek_kind(), TokenKind::Keyword(Keyword::As)) {
            self.bump();
            let (a, _) = self.expect_ident()?;
            Some(a)
        } else {
            None
        };
        let end = self.span_here();
        self.expect_newline_or_end()?;
        Ok(Decl::Import {
            module_path,
            alias,
            span: Span::new(start.start, end.end),
        })
    }

    fn parse_from_import(&mut self) -> Result<Decl, ParseError> {
        // from module_path import import_list
        let start = self.expect_keyword(Keyword::From)?;
        let module_path = self.parse_module_path()?;
        self.expect_keyword(Keyword::Import)?;
        let mut items = Vec::new();
        loop {
            let (name, _) = self.expect_ident()?;
            let alias = if matches!(self.peek_kind(), TokenKind::Keyword(Keyword::As)) {
                self.bump();
                let (a, _) = self.expect_ident()?;
                Some(a)
            } else {
                None
            };
            items.push(crate::ast::ImportName { name, alias });
            if matches!(self.peek_kind(), TokenKind::Comma) {
                self.bump();
                // trailing comma allowed before end
                if matches!(
                    self.peek_kind(),
                    TokenKind::Newline | TokenKind::Dedent | TokenKind::Eof
                ) {
                    break;
                }
                continue;
            }
            break;
        }
        if items.is_empty() {
            return Err(ParseError::new(
                "expected import name list",
                self.span_here(),
            ));
        }
        let end = self.span_here();
        self.expect_newline_or_end()?;
        Ok(Decl::FromImport {
            module_path,
            items,
            span: Span::new(start.start, end.end),
        })
    }

    fn parse_export(&mut self) -> Result<Decl, ParseError> {
        let start = self.expect_keyword(Keyword::Export)?;
        // export let|const|def ...
        let item = match self.peek_kind() {
            TokenKind::Keyword(Keyword::Let | Keyword::Const | Keyword::Def) => self.parse_decl()?,
            _ => {
                return Err(ParseError::new(
                    "expected declaration after `export`",
                    self.span_here(),
                ))
            }
        };
        let end = match &item {
            Decl::Let { span, .. }
            | Decl::Const { span, .. }
            | Decl::Function { span, .. }
            | Decl::Import { span, .. }
            | Decl::FromImport { span, .. }
            | Decl::Export { span, .. } => span.end,
        };
        Ok(Decl::Export {
            item: Box::new(item),
            span: Span::new(start.start, end),
        })
    }

    fn parse_let_like(&mut self, is_const: bool) -> Result<Decl, ParseError> {
        let start = if is_const {
            self.expect_keyword(Keyword::Const)?
        } else {
            self.expect_keyword(Keyword::Let)?
        };
        let (name, _) = self.expect_ident()?;
        self.expect_kind(|k| matches!(k, TokenKind::Assign), "expected `=`")?;
        let value = self.parse_expr()?;
        let end = value.span_end();
        self.expect_newline_or_end()?;
        let span = Span::new(start.start, end);
        if is_const {
            Ok(Decl::Const { name, value, span })
        } else {
            Ok(Decl::Let { name, value, span })
        }
    }

    fn parse_function(&mut self) -> Result<Decl, ParseError> {
        let start = self.expect_keyword(Keyword::Def)?;
        let (name, _) = self.expect_ident()?;
        self.expect_kind(|k| matches!(k, TokenKind::LParen), "expected `(`")?;
        let mut params = Vec::new();
        if !matches!(self.peek_kind(), TokenKind::RParen) {
            loop {
                let (p, _) = self.expect_ident()?;
                params.push(p);
                if matches!(self.peek_kind(), TokenKind::Comma) {
                    self.bump();
                    continue;
                }
                break;
            }
        }
        self.expect_kind(|k| matches!(k, TokenKind::RParen), "expected `)`")?;
        self.expect_kind(|k| matches!(k, TokenKind::Colon), "expected `:`")?;
        self.expect_kind(|k| matches!(k, TokenKind::Newline), "expected newline after `:`")?;
        let body = self.parse_block()?;
        let span = Span::new(start.start, body.span.end);
        Ok(Decl::Function {
            name,
            params,
            body,
            span,
        })
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let start = self.expect_kind(|k| matches!(k, TokenKind::Indent), "expected indented block")?;
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !matches!(self.peek_kind(), TokenKind::Dedent | TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        let end = self.expect_kind(|k| matches!(k, TokenKind::Dedent), "expected dedent")?;
        // SPEC-P1 §4.4: empty blocks are invalid (no `pass`).
        if stmts.is_empty() {
            return Err(ParseError::new(
                "empty block is invalid",
                Span::new(start.start, end.end),
            ));
        }
        Ok(Block {
            stmts,
            span: Span::new(start.start, end.end),
        })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek_kind() {
            TokenKind::Keyword(Keyword::Return) => {
                let start = self.bump().span;
                let value = if matches!(
                    self.peek_kind(),
                    TokenKind::Newline | TokenKind::Dedent | TokenKind::Eof
                ) {
                    None
                } else {
                    Some(self.parse_expr()?)
                };
                let end = value.as_ref().map(Expr::span_end).unwrap_or(start.end);
                self.expect_newline_or_end()?;
                Ok(Stmt::Return {
                    value,
                    span: Span::new(start.start, end),
                })
            }
            TokenKind::Keyword(Keyword::If) => self.parse_if(),
            TokenKind::Keyword(Keyword::While) => self.parse_while(),
            TokenKind::Keyword(Keyword::For) => self.parse_for(),
            TokenKind::Keyword(Keyword::Break) => {
                let span = self.bump().span;
                self.expect_newline_or_end()?;
                Ok(Stmt::Break { span })
            }
            TokenKind::Keyword(Keyword::Continue) => {
                let span = self.bump().span;
                self.expect_newline_or_end()?;
                Ok(Stmt::Continue { span })
            }
            TokenKind::Keyword(Keyword::Raise) => {
                let start = self.bump().span;
                let value = self.parse_expr()?;
                let end = value.span_end_pub();
                self.expect_newline_or_end()?;
                Ok(Stmt::Raise {
                    value,
                    span: Span::new(start.start, end),
                })
            }
            TokenKind::Keyword(Keyword::Assert) => {
                let start = self.bump().span;
                let cond = self.parse_expr()?;
                let end = cond.span_end_pub();
                self.expect_newline_or_end()?;
                Ok(Stmt::Assert {
                    cond,
                    span: Span::new(start.start, end),
                })
            }
            TokenKind::Keyword(Keyword::Let | Keyword::Const | Keyword::Def | Keyword::Import) => {
                Ok(Stmt::Decl(self.parse_decl()?))
            }
            TokenKind::Ident { .. } => {
                // L-value assign (name / index / attr), aug-assign, or expression.
                if let Some(op) = self.peek_aug_op() {
                    let (name, start) = self.expect_ident()?;
                    self.bump(); // aug op
                    let value = self.parse_expr()?;
                    let end = value.span_end();
                    self.expect_newline_or_end()?;
                    Ok(Stmt::AugAssign {
                        name,
                        op,
                        value,
                        span: Span::new(start.start, end),
                    })
                } else if self.looks_like_assignment_stmt() {
                    let target = self.parse_postfix()?;
                    self.expect_kind(|k| matches!(k, TokenKind::Assign), "expected `=`")?;
                    let value = self.parse_expr()?;
                    let end = value.span_end();
                    self.expect_newline_or_end()?;
                    match target {
                        Expr::Name { name, span } => Ok(Stmt::Assign {
                            name,
                            value,
                            span: Span::new(span.start, end),
                        }),
                        Expr::Index {
                            base,
                            index,
                            span,
                        } => Ok(Stmt::IndexAssign {
                            base: *base,
                            index: *index,
                            value,
                            span: Span::new(span.start, end),
                        }),
                        Expr::Attr { base, name, span } => Ok(Stmt::AttrAssign {
                            base: *base,
                            name,
                            value,
                            span: Span::new(span.start, end),
                        }),
                        other => Err(ParseError::new(
                            format!("invalid assignment target: {other:?}"),
                            other.span_pub(),
                        )),
                    }
                } else {
                    let expr = self.parse_expr()?;
                    let span = expr.span();
                    self.expect_newline_or_end()?;
                    Ok(Stmt::Expr { expr, span })
                }
            }
            _ => {
                let expr = self.parse_expr()?;
                let span = expr.span();
                self.expect_newline_or_end()?;
                Ok(Stmt::Expr { expr, span })
            }
        }
    }

    /// True if this line looks like `postfix_lvalue = expr` (not comparison `==`).
    fn looks_like_assignment_stmt(&self) -> bool {
        if !matches!(self.peek_kind(), TokenKind::Ident { .. }) {
            return false;
        }
        // Fast path: `name =`
        if matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Assign)
        ) {
            return true;
        }
        // Scan postfix chain for trailing `=` (not `==`).
        let mut i = self.pos + 1;
        let mut depth_paren = 0i32;
        let mut depth_brack = 0i32;
        while i < self.tokens.len() {
            match &self.tokens[i].kind {
                TokenKind::LParen => depth_paren += 1,
                TokenKind::RParen => depth_paren -= 1,
                TokenKind::LBracket => depth_brack += 1,
                TokenKind::RBracket => depth_brack -= 1,
                TokenKind::Assign if depth_paren == 0 && depth_brack == 0 => return true,
                TokenKind::Eq | TokenKind::NotEq | TokenKind::PlusAssign
                | TokenKind::MinusAssign | TokenKind::StarAssign | TokenKind::SlashAssign
                | TokenKind::PercentAssign
                    if depth_paren == 0 && depth_brack == 0 =>
                {
                    return false;
                }
                TokenKind::Newline | TokenKind::Eof | TokenKind::Keyword(_)
                    if depth_paren == 0 && depth_brack == 0 && i > self.pos + 1 =>
                {
                    // Allow `.` and `[` chains only; stop at keyword at top level mid-scan
                    if !matches!(
                        self.tokens.get(i).map(|t| &t.kind),
                        Some(TokenKind::Keyword(_))
                    ) {
                        return false;
                    }
                    // keywords inside [] ok via depth
                    if depth_paren == 0 && depth_brack == 0 {
                        return false;
                    }
                }
                TokenKind::Newline | TokenKind::Eof => return false,
                _ => {}
            }
            i += 1;
        }
        false
    }

    fn peek_aug_op(&self) -> Option<crate::ast::AugOp> {
        if !matches!(self.peek_kind(), TokenKind::Ident { .. }) {
            return None;
        }
        match self.tokens.get(self.pos + 1).map(|t| &t.kind) {
            Some(TokenKind::PlusAssign) => Some(crate::ast::AugOp::Add),
            Some(TokenKind::MinusAssign) => Some(crate::ast::AugOp::Sub),
            Some(TokenKind::StarAssign) => Some(crate::ast::AugOp::Mul),
            Some(TokenKind::SlashAssign) => Some(crate::ast::AugOp::Div),
            Some(TokenKind::PercentAssign) => Some(crate::ast::AugOp::Rem),
            _ => None,
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        let start = self.expect_keyword(Keyword::If)?;
        let cond = self.parse_expr()?;
        self.expect_kind(|k| matches!(k, TokenKind::Colon), "expected `:`")?;
        self.expect_kind(|k| matches!(k, TokenKind::Newline), "expected newline")?;
        let then_block = self.parse_block()?;
        let mut elifs = Vec::new();
        while matches!(self.peek_kind(), TokenKind::Keyword(Keyword::Elif)) {
            self.bump();
            let c = self.parse_expr()?;
            self.expect_kind(|k| matches!(k, TokenKind::Colon), "expected `:`")?;
            self.expect_kind(|k| matches!(k, TokenKind::Newline), "expected newline")?;
            let b = self.parse_block()?;
            elifs.push((c, b));
        }
        let else_block = if matches!(self.peek_kind(), TokenKind::Keyword(Keyword::Else)) {
            self.bump();
            self.expect_kind(|k| matches!(k, TokenKind::Colon), "expected `:`")?;
            self.expect_kind(|k| matches!(k, TokenKind::Newline), "expected newline")?;
            Some(self.parse_block()?)
        } else {
            None
        };
        let end = else_block
            .as_ref()
            .map(|b| b.span.end)
            .or_else(|| elifs.last().map(|(_, b)| b.span.end))
            .unwrap_or(then_block.span.end);
        Ok(Stmt::If {
            cond,
            then_block,
            elifs,
            else_block,
            span: Span::new(start.start, end),
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        let start = self.expect_keyword(Keyword::While)?;
        let cond = self.parse_expr()?;
        self.expect_kind(|k| matches!(k, TokenKind::Colon), "expected `:`")?;
        self.expect_kind(|k| matches!(k, TokenKind::Newline), "expected newline")?;
        let body = self.parse_block()?;
        let span = Span::new(start.start, body.span.end);
        Ok(Stmt::While { cond, body, span })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        // for IDENT in expression ":" block
        let start = self.expect_keyword(Keyword::For)?;
        let (name, _) = self.expect_ident()?;
        self.expect_keyword(Keyword::In)?;
        let iter = self.parse_expr()?;
        self.expect_kind(|k| matches!(k, TokenKind::Colon), "expected `:`")?;
        self.expect_kind(|k| matches!(k, TokenKind::Newline), "expected newline")?;
        let body = self.parse_block()?;
        let span = Span::new(start.start, body.span.end);
        Ok(Stmt::For {
            name,
            iter,
            body,
            span,
        })
    }

    fn expect_newline_or_end(&mut self) -> Result<(), ParseError> {
        match self.peek_kind() {
            TokenKind::Newline => {
                self.bump();
                Ok(())
            }
            TokenKind::Dedent | TokenKind::Eof => Ok(()),
            _ => Err(ParseError::new(
                "expected end of statement (newline)",
                self.span_here(),
            )),
        }
    }

    // ---- expressions (Pratt / precedence climbing) ----

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while matches!(self.peek_kind(), TokenKind::Keyword(Keyword::Or)) {
            self.bump();
            let right = self.parse_and()?;
            let span = Span::new(left.span_start(), right.span_end());
            left = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_not()?;
        while matches!(self.peek_kind(), TokenKind::Keyword(Keyword::And)) {
            self.bump();
            let right = self.parse_not()?;
            let span = Span::new(left.span_start(), right.span_end());
            left = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expr, ParseError> {
        if matches!(self.peek_kind(), TokenKind::Keyword(Keyword::Not)) {
            let start = self.bump().span;
            let expr = self.parse_not()?;
            let span = Span::new(start.start, expr.span_end());
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
                span,
            });
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;
        loop {
            let op = match self.peek_kind() {
                TokenKind::Eq => BinaryOp::Eq,
                TokenKind::NotEq => BinaryOp::NotEq,
                TokenKind::Lt => BinaryOp::Lt,
                TokenKind::LtEq => BinaryOp::LtEq,
                TokenKind::Gt => BinaryOp::Gt,
                TokenKind::GtEq => BinaryOp::GtEq,
                TokenKind::Keyword(Keyword::Is) => BinaryOp::Is,
                TokenKind::Keyword(Keyword::In) => BinaryOp::In,
                _ => break,
            };
            self.bump();
            let right = self.parse_term()?;
            let span = Span::new(left.span_start(), right.span_end());
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;
        loop {
            let op = match self.peek_kind() {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.bump();
            let right = self.parse_factor()?;
            let span = Span::new(left.span_start(), right.span_end());
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek_kind() {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Rem,
                _ => break,
            };
            self.bump();
            let right = self.parse_unary()?;
            let span = Span::new(left.span_start(), right.span_end());
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if matches!(self.peek_kind(), TokenKind::Minus) {
            let start = self.bump().span;
            let expr = self.parse_unary()?;
            let span = Span::new(start.start, expr.span_end());
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
                span,
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if matches!(self.peek_kind(), TokenKind::LParen) {
                let args_start = self.bump().span;
                let mut args = Vec::new();
                if !matches!(self.peek_kind(), TokenKind::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if matches!(self.peek_kind(), TokenKind::Comma) {
                            self.bump();
                            continue;
                        }
                        break;
                    }
                }
                let end = self.expect_kind(|k| matches!(k, TokenKind::RParen), "expected `)`")?;
                let span = Span::new(expr.span_start(), end.end);
                let _ = args_start;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    span,
                };
            } else if matches!(self.peek_kind(), TokenKind::LBracket) {
                self.bump();
                let index = self.parse_expr()?;
                let end =
                    self.expect_kind(|k| matches!(k, TokenKind::RBracket), "expected `]`")?;
                let span = Span::new(expr.span_start(), end.end);
                expr = Expr::Index {
                    base: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else if matches!(self.peek_kind(), TokenKind::Dot) {
                self.bump();
                let (name, name_span) = self.expect_ident()?;
                let span = Span::new(expr.span_start(), name_span.end);
                expr = Expr::Attr {
                    base: Box::new(expr),
                    name,
                    span,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let tok = self.peek().clone();
        match &tok.kind {
            TokenKind::Keyword(Keyword::Nil) => {
                self.bump();
                Ok(Expr::Nil { span: tok.span })
            }
            TokenKind::Keyword(Keyword::True) => {
                self.bump();
                Ok(Expr::Bool {
                    value: true,
                    span: tok.span,
                })
            }
            TokenKind::Keyword(Keyword::False) => {
                self.bump();
                Ok(Expr::Bool {
                    value: false,
                    span: tok.span,
                })
            }
            TokenKind::Int(v) => {
                let value = v.clone();
                self.bump();
                Ok(Expr::Int {
                    value,
                    span: tok.span,
                })
            }
            TokenKind::Float(v) => {
                let value = v.clone();
                self.bump();
                Ok(Expr::Float {
                    value,
                    span: tok.span,
                })
            }
            TokenKind::String(v) => {
                let value = v.clone();
                self.bump();
                Ok(Expr::String {
                    value,
                    span: tok.span,
                })
            }
            TokenKind::Ident { nfc, .. } => {
                let name = nfc.clone();
                self.bump();
                Ok(Expr::Name {
                    name,
                    span: tok.span,
                })
            }
            TokenKind::LParen => {
                self.bump();
                let expr = self.parse_expr()?;
                self.expect_kind(|k| matches!(k, TokenKind::RParen), "expected `)`")?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                let start = self.bump().span;
                let mut elements = Vec::new();
                if !matches!(self.peek_kind(), TokenKind::RBracket) {
                    loop {
                        elements.push(self.parse_expr()?);
                        if matches!(self.peek_kind(), TokenKind::Comma) {
                            self.bump();
                            if matches!(self.peek_kind(), TokenKind::RBracket) {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                }
                let end = self.expect_kind(|k| matches!(k, TokenKind::RBracket), "expected `]`")?;
                Ok(Expr::List {
                    elements,
                    span: Span::new(start.start, end.end),
                })
            }
            TokenKind::LBrace => {
                // Map literal `{ k: v, ... }` (SPEC-P1 §6.7)
                let start = self.bump().span;
                let mut entries = Vec::new();
                if !matches!(self.peek_kind(), TokenKind::RBrace) {
                    loop {
                        let key = self.parse_expr()?;
                        self.expect_kind(|k| matches!(k, TokenKind::Colon), "expected `:` in map")?;
                        let value = self.parse_expr()?;
                        entries.push((key, value));
                        if matches!(self.peek_kind(), TokenKind::Comma) {
                            self.bump();
                            if matches!(self.peek_kind(), TokenKind::RBrace) {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                }
                let end = self.expect_kind(|k| matches!(k, TokenKind::RBrace), "expected `}`")?;
                Ok(Expr::Map {
                    entries,
                    span: Span::new(start.start, end.end),
                })
            }
            _ => Err(ParseError::new("expected expression", tok.span)),
        }
    }
}

impl Expr {
    fn span(&self) -> Span {
        match self {
            Self::Nil { span }
            | Self::Bool { span, .. }
            | Self::Int { span, .. }
            | Self::Float { span, .. }
            | Self::String { span, .. }
            | Self::Name { span, .. }
            | Self::Call { span, .. }
            | Self::Unary { span, .. }
            | Self::Binary { span, .. }
            | Self::List { span, .. }
            | Self::Map { span, .. }
            | Self::Index { span, .. }
            | Self::Attr { span, .. } => *span,
        }
    }

    /// Public span accessor for error reporting.
    pub fn span_pub(&self) -> Span {
        self.span()
    }

    fn span_start(&self) -> u32 {
        self.span().start
    }

    fn span_end(&self) -> u32 {
        self.span().end
    }

    /// Public span end for statement builders.
    pub fn span_end_pub(&self) -> u32 {
        self.span_end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_let() {
        let m = parse_module("let x = 1\n").unwrap();
        assert_eq!(m.items.len(), 1);
        assert!(matches!(
            &m.items[0],
            Item::Decl(Decl::Let { name, .. }) if name == "x"
        ));
    }

    #[test]
    fn parse_fib_shape() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        let m = parse_module(src).expect("fib should parse");
        assert!(m.items.len() >= 2);
        assert!(matches!(
            &m.items[0],
            Item::Decl(Decl::Function { name, params, .. })
                if name == "fib" && params == &["n".to_string()]
        ));
    }

    #[test]
    fn parse_call_and_arith() {
        let m = parse_module("print(1 + 2 * 3)\n").unwrap();
        assert!(matches!(&m.items[0], Item::Stmt(Stmt::Expr { .. })));
    }

    #[test]
    fn reject_bad_syntax() {
        assert!(parse_module("let = 1\n").is_err());
        assert!(parse_module("def f(\n").is_err());
    }

    #[test]
    fn parse_while() {
        let src = "while true:\n    let x = 1\n";
        let m = parse_module(src).unwrap();
        assert!(matches!(&m.items[0], Item::Stmt(Stmt::While { .. })));
    }

    #[test]
    fn parse_list_literal() {
        let m = parse_module("let xs = [1, 2, 3]\n").unwrap();
        match &m.items[0] {
            Item::Decl(Decl::Let { value, .. }) => {
                assert!(matches!(value, Expr::List { elements, .. } if elements.len() == 3));
            }
            _ => panic!("expected let"),
        }
    }

    #[test]
    fn parse_for_break_continue() {
        let src = "for x in xs:\n    if x == 0:\n        break\n    continue\n";
        let m = parse_module(src).unwrap();
        assert!(matches!(&m.items[0], Item::Stmt(Stmt::For { name, .. }) if name == "x"));
    }

    #[test]
    fn parse_import_export_raise_assert() {
        let m = parse_module("import foo.bar as b\nexport let x = 1\nraise x\nassert true\n")
            .unwrap();
        assert!(matches!(&m.items[0], Item::Decl(Decl::Import { .. })));
        assert!(matches!(&m.items[1], Item::Decl(Decl::Export { .. })));
        assert!(matches!(&m.items[2], Item::Stmt(Stmt::Raise { .. })));
        assert!(matches!(&m.items[3], Item::Stmt(Stmt::Assert { .. })));
    }

    #[test]
    fn parse_from_import() {
        let m = parse_module("from util.math import sin as s, cos\n").unwrap();
        match &m.items[0] {
            Item::Decl(Decl::FromImport { module_path, items, .. }) => {
                assert_eq!(module_path, &["util".to_string(), "math".to_string()]);
                assert_eq!(items.len(), 2);
                assert_eq!(items[0].name, "sin");
                assert_eq!(items[0].alias.as_deref(), Some("s"));
                assert_eq!(items[1].name, "cos");
            }
            _ => panic!("expected from-import"),
        }
    }

    #[test]
    fn parse_aug_assign() {
        let m = parse_module("let x = 1\nx += 2\n").unwrap();
        assert!(matches!(
            &m.items[1],
            Item::Stmt(Stmt::AugAssign {
                op: crate::ast::AugOp::Add,
                ..
            })
        ));
    }

    #[test]
    fn parse_map_literal() {
        let m = parse_module("let m = {\"a\": 1, \"b\": 2}\n").unwrap();
        match &m.items[0] {
            Item::Decl(Decl::Let { value, .. }) => {
                assert!(matches!(value, Expr::Map { entries, .. } if entries.len() == 2));
            }
            _ => panic!("expected let map"),
        }
    }

    #[test]
    fn reject_missing_or_empty_block() {
        // SPEC-P1 §4.4: compound body required; no empty block / no pass.
        assert!(parse_module("def f():\nlet x = 1\n").is_err());
        assert!(parse_module("while true:\n").is_err());
    }

    #[test]
    fn parse_index_read_and_assign() {
        let m = parse_module("let xs = [1, 2]\nlet a = xs[0]\nxs[1] = 9\n").unwrap();
        assert!(matches!(
            &m.items[1],
            Item::Decl(Decl::Let {
                value: Expr::Index { .. },
                ..
            })
        ));
        assert!(matches!(&m.items[2], Item::Stmt(Stmt::IndexAssign { .. })));
    }

    #[test]
    fn parse_attr_read_and_assign() {
        let m = parse_module("let o = {\"x\": 1}\nlet a = o.x\no.x = 2\n").unwrap();
        assert!(matches!(
            &m.items[1],
            Item::Decl(Decl::Let {
                value: Expr::Attr { name, .. },
                ..
            }) if name == "x"
        ));
        assert!(matches!(
            &m.items[2],
            Item::Stmt(Stmt::AttrAssign { name, .. }) if name == "x"
        ));
    }
}
