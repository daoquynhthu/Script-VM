//! Bootstrap SIR node kinds for the Phase 1 frontend surface.
//!
//! Spec: Phase 2 node taxonomy framework; concrete ops are bootstrap until full
//! SIR node schema rounds are implemented.

use crate::id::{BindingId, NodeId, SymbolId};

/// Executable / structural node kinds used by the bootstrap lowerer.
#[derive(Debug, Clone, PartialEq)]
pub enum SirNode {
    /// Module body: ordered top-level items.
    ModuleBody {
        items: Vec<NodeId>,
    },
    Let {
        binding: BindingId,
        init: NodeId,
    },
    Const {
        binding: BindingId,
        init: NodeId,
    },
    Function {
        binding: BindingId,
        params: Vec<BindingId>,
        body: NodeId,
    },
    Import {
        module_path: Vec<String>,
        alias: Option<SymbolId>,
        binding: Option<BindingId>,
    },
    ExportMarker {
        /// Node of the exported declaration.
        item: NodeId,
    },
    Block {
        stmts: Vec<NodeId>,
    },
    ExprStmt {
        expr: NodeId,
    },
    Return {
        value: Option<NodeId>,
    },
    Break,
    Continue,
    If {
        cond: NodeId,
        then_block: NodeId,
        elifs: Vec<(NodeId, NodeId)>,
        else_block: Option<NodeId>,
    },
    While {
        cond: NodeId,
        body: NodeId,
    },
    For {
        binding: BindingId,
        iter: NodeId,
        body: NodeId,
    },
    Assign {
        binding: BindingId,
        value: NodeId,
    },
    Raise {
        value: NodeId,
    },
    Assert {
        cond: NodeId,
    },
    // Expressions
    LiteralNil,
    LiteralBool {
        value: bool,
    },
    LiteralInt {
        value: String,
    },
    LiteralFloat {
        value: String,
    },
    LiteralString {
        value: String,
    },
    Name {
        binding: BindingId,
    },
    /// Unresolved or free name retained only if lowerer maps via binding table.
    SymbolRef {
        symbol: SymbolId,
    },
    Call {
        callee: NodeId,
        args: Vec<NodeId>,
    },
    Unary {
        op: UnaryOp,
        expr: NodeId,
    },
    Binary {
        op: BinaryOp,
        left: NodeId,
        right: NodeId,
    },
    List {
        elements: Vec<NodeId>,
    },
    /// Map literal entries (key, value) node pairs.
    Map {
        entries: Vec<(NodeId, NodeId)>,
    },
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
