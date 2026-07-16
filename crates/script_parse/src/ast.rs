//! Phase 1 AST nodes (bootstrap subset).
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` grammar sections (descriptive AST only).
//! AST shape is implementation-defined (spec §1: AST not prescribed).

use script_lex::Span;

/// Entire source module.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    Let {
        name: String,
        value: Expr,
        span: Span,
    },
    Const {
        name: String,
        value: Expr,
        span: Span,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Block,
        span: Span,
    },
    /// `import path.to.mod [as name]`
    Import {
        module_path: Vec<String>,
        alias: Option<String>,
        span: Span,
    },
    /// `from path import a [as b], c`
    FromImport {
        module_path: Vec<String>,
        items: Vec<ImportName>,
        span: Span,
    },
    /// `export` wrapping a declaration.
    Export {
        item: Box<Decl>,
        span: Span,
    },
}

/// One name in a `from … import` list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportName {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr {
        expr: Expr,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
    If {
        cond: Expr,
        then_block: Block,
        elifs: Vec<(Expr, Block)>,
        else_block: Option<Block>,
        span: Span,
    },
    While {
        cond: Expr,
        body: Block,
        span: Span,
    },
    /// `for name in iter: block`
    For {
        name: String,
        iter: Expr,
        body: Block,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Assign {
        name: String,
        value: Expr,
        span: Span,
    },
    /// Index assignment: `base[index] = value` (SPEC-P1 assignment_target)
    IndexAssign {
        base: Expr,
        index: Expr,
        value: Expr,
        span: Span,
    },
    /// Augmented assignment: `x += 1` etc. (SPEC-P1 §10.2)
    AugAssign {
        name: String,
        op: AugOp,
        value: Expr,
        span: Span,
    },
    Raise {
        value: Expr,
        span: Span,
    },
    Assert {
        cond: Expr,
        span: Span,
    },
    /// Nested declaration allowed in block scope (let/const/def).
    Decl(Decl),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Nil {
        span: Span,
    },
    Bool {
        value: bool,
        span: Span,
    },
    Int {
        /// Digits without underscores.
        value: String,
        span: Span,
    },
    Float {
        value: String,
        span: Span,
    },
    String {
        value: String,
        span: Span,
    },
    Name {
        name: String,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
    List {
        elements: Vec<Expr>,
        span: Span,
    },
    /// Map literal `{ k: v, ... }` (SPEC-P1 §6.7)
    Map {
        entries: Vec<(Expr, Expr)>,
        span: Span,
    },
    /// Index / subscript: `base[index]`
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AugOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Is,
    In,
}
