//! Lower analyzed Phase 1 AST into bootstrap `IrUnit`.
//!
//! Spec: Phase 2 IR unit model; producer is the Phase 1 frontend.
//! Prerequisite: semantic analysis must pass (caller responsibility).

use std::collections::HashMap;

use script_lex::Span;
use script_parse::{
    BinaryOp as AstBin, Block, Decl, Expr, Item, Module, Stmt, UnaryOp as AstUn,
};
use script_sema::{
    analyze_module, analyze_source, AnalyzedModule, BindingKind as SemaKind, SemaResult,
};
use sir::{
    BindingDescriptor, BindingId, BinaryOp, ControlRegionDescriptor, ControlRegionId,
    ControlRegionKind, IrHeader, IrUnit, ModuleDescriptor, ModuleId, NodeEntry, NodeId,
    ScopeDescriptor, ScopeId, SirBindingKind, SirMutability, SirNode, SirVisibility,
    SourceFileRecord, SourceOrigin, SourcePosition, SourceSpan, SourceTable, SymbolDescriptor,
    SymbolId, UnaryOp, Version,
};

use crate::error::LowerError;

/// Materialize SIR from a successful `AnalyzedModule` (T-P2 primary entry).
pub fn materialize_sir(
    analyzed: &AnalyzedModule,
    module_name: &str,
) -> Result<IrUnit, LowerError> {
    if !analyzed.ok() {
        return Err(LowerError::SemaFailed {
            errors: analyzed
                .diagnostics
                .iter()
                .map(|d| d.message.clone())
                .collect(),
        });
    }
    let module = analyzed
        .module
        .as_ref()
        .ok_or_else(|| LowerError::SemaFailed {
            errors: vec!["analyzed module has no AST".into()],
        })?;
    // Re-run lightweight binding snapshot for lowerer prelude (print).
    let sema = analyze_module(module);
    Ok(LowerCtx::new(module_name, &analyzed.source, &sema).lower_module(module))
}

/// Full frontend pipeline: parse is external; lower after successful sema.
pub fn lower_module(module: &Module, module_name: &str, source: &str) -> Result<IrUnit, LowerError> {
    let sema = analyze_module(module);
    if !sema.ok() {
        return Err(LowerError::SemaFailed {
            errors: sema.errors.iter().map(|e| e.message.clone()).collect(),
        });
    }
    Ok(LowerCtx::new(module_name, source, &sema).lower_module(module))
}

/// Parse + sema + lower via `AnalyzedModule` (canonical T-P2 path).
pub fn compile_to_sir(source: &str, module_name: &str) -> Result<IrUnit, LowerError> {
    let analyzed = analyze_source(source);
    materialize_sir(&analyzed, module_name)
}

struct LowerCtx<'a> {
    module_name: String,
    source: &'a str,
    next_symbol: u32,
    next_binding: u32,
    next_scope: u32,
    next_node: u32,
    symbols: Vec<SymbolDescriptor>,
    symbol_index: HashMap<String, SymbolId>,
    scopes: Vec<ScopeDescriptor>,
    bindings: Vec<BindingDescriptor>,
    /// name → binding in current scope chain (stack of maps)
    env: Vec<HashMap<String, BindingId>>,
    nodes: Vec<NodeEntry>,
    control_regions: Vec<ControlRegionDescriptor>,
    region_stack: Vec<ControlRegionId>,
    next_region: u32,
    exports: Vec<String>,
    imports: Vec<String>,
    _sema: &'a SemaResult,
}

impl<'a> LowerCtx<'a> {
    fn new(module_name: &str, source: &'a str, sema: &'a SemaResult) -> Self {
        let mut ctx = Self {
            module_name: module_name.to_string(),
            source,
            next_symbol: 0,
            next_binding: 0,
            next_scope: 0,
            next_node: 0,
            symbols: Vec::new(),
            symbol_index: HashMap::new(),
            scopes: Vec::new(),
            bindings: Vec::new(),
            env: vec![HashMap::new()],
            nodes: Vec::new(),
            control_regions: Vec::new(),
            region_stack: Vec::new(),
            next_region: 0,
            exports: Vec::new(),
            imports: Vec::new(),
            _sema: sema,
        };
        let root = ctx.alloc_scope(None);
        debug_assert_eq!(root.raw(), 0);
        // Module control region (WP-S02).
        let mod_region = ctx.push_region(ControlRegionKind::Module, None);
        debug_assert_eq!(mod_region.raw(), 0);
        // Prelude bindings (print)
        let print_sym = ctx.intern("print");
        let print_b = ctx.alloc_binding(
            print_sym,
            root,
            SirBindingKind::Builtin,
            SirMutability::Immutable,
            SirVisibility::Builtin,
            None,
            None,
        );
        ctx.env[0].insert("print".into(), print_b);
        ctx
    }

    fn push_region(
        &mut self,
        kind: ControlRegionKind,
        owner_node: Option<NodeId>,
    ) -> ControlRegionId {
        let parent = self.region_stack.last().copied();
        let id = ControlRegionId::new(self.next_region);
        self.next_region += 1;
        self.control_regions.push(ControlRegionDescriptor {
            region_id: id,
            kind,
            parent,
            owner_node,
        });
        self.region_stack.push(id);
        id
    }

    fn pop_region(&mut self) {
        self.region_stack.pop();
    }

    fn set_region_owner(&mut self, region: ControlRegionId, owner: NodeId) {
        if let Some(r) = self
            .control_regions
            .iter_mut()
            .find(|r| r.region_id == region)
        {
            r.owner_node = Some(owner);
        }
    }

    fn alloc_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId::new(self.next_scope);
        self.next_scope += 1;
        self.scopes.push(ScopeDescriptor {
            scope_id: id,
            parent,
        });
        id
    }

    fn intern(&mut self, text: &str) -> SymbolId {
        if let Some(id) = self.symbol_index.get(text) {
            return *id;
        }
        let id = SymbolId::new(self.next_symbol);
        self.next_symbol += 1;
        self.symbols.push(SymbolDescriptor {
            symbol_id: id,
            text: text.to_string(),
        });
        self.symbol_index.insert(text.to_string(), id);
        id
    }

    fn alloc_binding(
        &mut self,
        symbol_id: SymbolId,
        scope_id: ScopeId,
        kind: SirBindingKind,
        mutability: SirMutability,
        visibility: SirVisibility,
        initializer_node: Option<NodeId>,
        declaration_node: Option<NodeId>,
    ) -> BindingId {
        let id = BindingId::new(self.next_binding);
        self.next_binding += 1;
        self.bindings.push(BindingDescriptor {
            binding_id: id,
            symbol_id,
            scope_id,
            kind,
            mutability,
            visibility,
            initializer_node,
            declaration_node,
        });
        id
    }

    fn push_scope(&mut self) -> ScopeId {
        let parent = self.scopes.last().map(|s| s.scope_id);
        let id = self.alloc_scope(parent);
        self.env.push(HashMap::new());
        id
    }

    fn pop_scope(&mut self) {
        self.env.pop();
    }

    fn current_scope(&self) -> ScopeId {
        self.scopes.last().expect("scope").scope_id
    }

    fn define_local(
        &mut self,
        name: &str,
        kind: SirBindingKind,
        mutability: SirMutability,
        visibility: SirVisibility,
    ) -> BindingId {
        let sym = self.intern(name);
        let scope = self.current_scope();
        let b = self.alloc_binding(sym, scope, kind, mutability, visibility, None, None);
        self.env
            .last_mut()
            .expect("env")
            .insert(name.to_string(), b);
        b
    }

    fn resolve(&self, name: &str) -> Option<BindingId> {
        for frame in self.env.iter().rev() {
            if let Some(b) = frame.get(name) {
                return Some(*b);
            }
        }
        None
    }

    fn emit(&mut self, kind: SirNode, span: Span) -> NodeId {
        let id = NodeId::new(self.next_node);
        self.next_node += 1;
        let origin = SourceOrigin::from_source(span_to_source(span, 0));
        self.nodes.push(NodeEntry {
            node_id: id,
            kind,
            origin,
        });
        id
    }

    fn lower_module(mut self, module: &Module) -> IrUnit {
        let mut items = Vec::new();
        for item in &module.items {
            items.push(self.lower_item(item));
        }
        let root = self.emit(
            SirNode::ModuleBody { items },
            module.span,
        );

        let source_digest = simple_digest(self.source);
        let semantic_digest = simple_digest(&format!(
            "{}:{}:{}",
            self.module_name,
            self.bindings.len(),
            self.nodes.len()
        ));

        let exports = self.exports.clone();
        IrUnit {
            header: IrHeader {
                ir_schema_version: Version::bootstrap(1, 0, 0),
                language_baseline_version: Version::new(1, 0, 0),
                producer_name: "script_lower".into(),
                producer_version: env!("CARGO_PKG_VERSION").into(),
                source_digest: source_digest.clone(),
                semantic_digest,
            },
            module: ModuleDescriptor {
                module_id: ModuleId::new(0),
                name: self.module_name.clone(),
            },
            sources: SourceTable {
                files: vec![SourceFileRecord {
                    path_or_name: self.module_name.clone(),
                    digest: source_digest,
                    encoding: "utf-8",
                }],
                spans: Vec::new(),
            },
            symbols: self.symbols,
            scopes: self.scopes,
            bindings: self.bindings,
            types: Vec::new(),
            capabilities: Vec::new(),
            effects: Vec::new(),
            nodes: self.nodes,
            patterns: Vec::new(),
            control_regions: self.control_regions,
            interface_exports: exports.clone(),
            root_node: root,
            exports,
            imports: self.imports,
        }
    }

    fn lower_item(&mut self, item: &Item) -> NodeId {
        match item {
            Item::Decl(d) => self.lower_decl(d, false),
            Item::Stmt(s) => self.lower_stmt(s),
        }
    }

    fn lower_decl(&mut self, decl: &Decl, exported: bool) -> NodeId {
        let vis = if exported {
            SirVisibility::Exported
        } else {
            SirVisibility::ModulePrivate
        };
        match decl {
            Decl::Let { name, value, span } => {
                let init = self.lower_expr(value);
                let b = self.define_local(name, SirBindingKind::Let, SirMutability::Mutable, vis);
                if exported {
                    self.exports.push(name.clone());
                }
                let node = self.emit(SirNode::Let { binding: b, init }, *span);
                self.patch_binding_nodes(b, Some(init), Some(node));
                if exported {
                    self.emit(SirNode::ExportMarker { item: node }, *span)
                } else {
                    node
                }
            }
            Decl::Const { name, value, span } => {
                let init = self.lower_expr(value);
                let b = self.define_local(
                    name,
                    SirBindingKind::Const,
                    SirMutability::Immutable,
                    vis,
                );
                if exported {
                    self.exports.push(name.clone());
                }
                let node = self.emit(SirNode::Const { binding: b, init }, *span);
                self.patch_binding_nodes(b, Some(init), Some(node));
                if exported {
                    self.emit(SirNode::ExportMarker { item: node }, *span)
                } else {
                    node
                }
            }
            Decl::Function {
                name,
                params,
                body,
                span,
            } => {
                let b = self.define_local(
                    name,
                    SirBindingKind::Function,
                    SirMutability::Immutable,
                    vis,
                );
                if exported {
                    self.exports.push(name.clone());
                }
                self.push_scope();
                let region = self.push_region(ControlRegionKind::Function, None);
                let mut param_bs = Vec::new();
                for p in params {
                    param_bs.push(self.define_local(
                        p,
                        SirBindingKind::Parameter,
                        SirMutability::Mutable,
                        SirVisibility::Local,
                    ));
                }
                let body_n = self.lower_block(body);
                self.pop_region();
                self.pop_scope();
                let node = self.emit(
                    SirNode::Function {
                        binding: b,
                        params: param_bs,
                        body: body_n,
                    },
                    *span,
                );
                self.set_region_owner(region, node);
                self.patch_binding_nodes(b, None, Some(node));
                if exported {
                    self.emit(SirNode::ExportMarker { item: node }, *span)
                } else {
                    node
                }
            }
            Decl::Import {
                module_path,
                alias,
                span,
            } => {
                let path_s = module_path.join(".");
                self.imports.push(path_s.clone());
                let local_name = alias
                    .clone()
                    .unwrap_or_else(|| module_path.last().cloned().unwrap_or_default());
                let b = self.define_local(
                    &local_name,
                    SirBindingKind::Import,
                    SirMutability::Immutable,
                    SirVisibility::Imported,
                );
                let alias_sym = alias.as_ref().map(|a| self.intern(a));
                self.emit(
                    SirNode::Import {
                        module_path: module_path.clone(),
                        alias: alias_sym,
                        binding: Some(b),
                    },
                    *span,
                )
            }
            Decl::FromImport {
                module_path,
                items,
                span,
            } => {
                self.imports.push(module_path.join("."));
                // Bind each imported name; node reuses Import with first name as alias text.
                let mut last = None;
                for item in items {
                    let local = item.alias.clone().unwrap_or_else(|| item.name.clone());
                    let b = self.define_local(
                        &local,
                        SirBindingKind::Import,
                        SirMutability::Immutable,
                        SirVisibility::Imported,
                    );
                    let alias_sym = Some(self.intern(&local));
                    last = Some(self.emit(
                        SirNode::Import {
                            module_path: module_path.clone(),
                            alias: alias_sym,
                            binding: Some(b),
                        },
                        *span,
                    ));
                }
                last.unwrap_or_else(|| self.emit(SirNode::LiteralNil, *span))
            }
            Decl::Export { item, span } => {
                let inner = self.lower_decl(item, true);
                let _ = span;
                inner
            }
        }
    }

    fn patch_binding_nodes(
        &mut self,
        binding: BindingId,
        init: Option<NodeId>,
        decl: Option<NodeId>,
    ) {
        if let Some(b) = self
            .bindings
            .iter_mut()
            .find(|x| x.binding_id == binding)
        {
            if init.is_some() {
                b.initializer_node = init;
            }
            if decl.is_some() {
                b.declaration_node = decl;
            }
        }
    }

    fn lower_block(&mut self, block: &Block) -> NodeId {
        self.push_scope();
        let region = self.push_region(ControlRegionKind::Block, None);
        let mut stmts = Vec::new();
        for s in &block.stmts {
            stmts.push(self.lower_stmt(s));
        }
        self.pop_region();
        self.pop_scope();
        let node = self.emit(SirNode::Block { stmts }, block.span);
        self.set_region_owner(region, node);
        node
    }

    fn lower_stmt(&mut self, stmt: &Stmt) -> NodeId {
        match stmt {
            Stmt::Expr { expr, span } => {
                let e = self.lower_expr(expr);
                self.emit(SirNode::ExprStmt { expr: e }, *span)
            }
            Stmt::Return { value, span } => {
                let v = value.as_ref().map(|e| self.lower_expr(e));
                self.emit(SirNode::Return { value: v }, *span)
            }
            Stmt::Break { span } => self.emit(SirNode::Break, *span),
            Stmt::Continue { span } => self.emit(SirNode::Continue, *span),
            Stmt::If {
                cond,
                then_block,
                elifs,
                else_block,
                span,
            } => {
                let c = self.lower_expr(cond);
                let t = self.lower_block(then_block);
                let mut es = Vec::new();
                for (ec, eb) in elifs {
                    es.push((self.lower_expr(ec), self.lower_block(eb)));
                }
                let el = else_block.as_ref().map(|b| self.lower_block(b));
                self.emit(
                    SirNode::If {
                        cond: c,
                        then_block: t,
                        elifs: es,
                        else_block: el,
                    },
                    *span,
                )
            }
            Stmt::While { cond, body, span } => {
                let region = self.push_region(ControlRegionKind::Loop, None);
                let c = self.lower_expr(cond);
                let b = self.lower_block(body);
                self.pop_region();
                let node = self.emit(SirNode::While { cond: c, body: b }, *span);
                self.set_region_owner(region, node);
                node
            }
            Stmt::For {
                name,
                iter,
                body,
                span,
            } => {
                let it = self.lower_expr(iter);
                let region = self.push_region(ControlRegionKind::Loop, None);
                self.push_scope();
                let b = self.define_local(
                    name,
                    SirBindingKind::ForIteration,
                    SirMutability::Mutable,
                    SirVisibility::Local,
                );
                let mut stmts = Vec::new();
                for s in &body.stmts {
                    stmts.push(self.lower_stmt(s));
                }
                let body_n = self.emit(SirNode::Block { stmts }, body.span);
                self.pop_scope();
                self.pop_region();
                let node = self.emit(
                    SirNode::For {
                        binding: b,
                        iter: it,
                        body: body_n,
                    },
                    *span,
                );
                self.set_region_owner(region, node);
                node
            }
            Stmt::Assign { name, value, span } => {
                let v = self.lower_expr(value);
                let binding = self.resolve(name).expect("sema guarantees binding");
                self.emit(SirNode::Assign { binding, value: v }, *span)
            }
            Stmt::AugAssign {
                name,
                value,
                span,
                ..
            } => {
                // Bootstrap: treat as plain assign of RHS only (lossy); full desugar in T-P2/T-P3L.
                let v = self.lower_expr(value);
                let binding = self.resolve(name).expect("sema guarantees binding");
                self.emit(SirNode::Assign { binding, value: v }, *span)
            }
            Stmt::Raise { value, span } => {
                let v = self.lower_expr(value);
                self.emit(SirNode::Raise { value: v }, *span)
            }
            Stmt::Assert { cond, span } => {
                let c = self.lower_expr(cond);
                self.emit(SirNode::Assert { cond: c }, *span)
            }
            Stmt::Decl(d) => self.lower_decl(d, false),
        }
    }

    fn lower_expr(&mut self, expr: &Expr) -> NodeId {
        match expr {
            Expr::Nil { span } => self.emit(SirNode::LiteralNil, *span),
            Expr::Bool { value, span } => self.emit(SirNode::LiteralBool { value: *value }, *span),
            Expr::Int { value, span } => {
                self.emit(SirNode::LiteralInt { value: value.clone() }, *span)
            }
            Expr::Float { value, span } => {
                self.emit(SirNode::LiteralFloat { value: value.clone() }, *span)
            }
            Expr::String { value, span } => {
                self.emit(SirNode::LiteralString { value: value.clone() }, *span)
            }
            Expr::Name { name, span } => {
                if let Some(b) = self.resolve(name) {
                    self.emit(SirNode::Name { binding: b }, *span)
                } else {
                    let s = self.intern(name);
                    self.emit(SirNode::SymbolRef { symbol: s }, *span)
                }
            }
            Expr::Call { callee, args, span } => {
                let c = self.lower_expr(callee);
                let a: Vec<_> = args.iter().map(|e| self.lower_expr(e)).collect();
                self.emit(SirNode::Call { callee: c, args: a }, *span)
            }
            Expr::Unary { op, expr, span } => {
                let e = self.lower_expr(expr);
                let op = match op {
                    AstUn::Neg => UnaryOp::Neg,
                    AstUn::Not => UnaryOp::Not,
                };
                self.emit(SirNode::Unary { op, expr: e }, *span)
            }
            Expr::Binary {
                op,
                left,
                right,
                span,
            } => {
                let l = self.lower_expr(left);
                let r = self.lower_expr(right);
                let op = map_bin(*op);
                self.emit(
                    SirNode::Binary {
                        op,
                        left: l,
                        right: r,
                    },
                    *span,
                )
            }
            Expr::List { elements, span } => {
                let els: Vec<_> = elements.iter().map(|e| self.lower_expr(e)).collect();
                self.emit(SirNode::List { elements: els }, *span)
            }
            Expr::Map { entries, span } => {
                let pairs: Vec<_> = entries
                    .iter()
                    .map(|(k, v)| (self.lower_expr(k), self.lower_expr(v)))
                    .collect();
                self.emit(SirNode::Map { entries: pairs }, *span)
            }
        }
    }
}

fn map_bin(op: AstBin) -> BinaryOp {
    match op {
        AstBin::Add => BinaryOp::Add,
        AstBin::Sub => BinaryOp::Sub,
        AstBin::Mul => BinaryOp::Mul,
        AstBin::Div => BinaryOp::Div,
        AstBin::Rem => BinaryOp::Rem,
        AstBin::Eq => BinaryOp::Eq,
        AstBin::NotEq => BinaryOp::NotEq,
        AstBin::Lt => BinaryOp::Lt,
        AstBin::LtEq => BinaryOp::LtEq,
        AstBin::Gt => BinaryOp::Gt,
        AstBin::GtEq => BinaryOp::GtEq,
        AstBin::And => BinaryOp::And,
        AstBin::Or => BinaryOp::Or,
        AstBin::Is => BinaryOp::Is,
        AstBin::In => BinaryOp::In,
    }
}

fn span_to_source(span: Span, module: u32) -> SourceSpan {
    // Byte offsets only at bootstrap; line/col filled as 1,1 placeholders.
    let _ = span;
    SourceSpan::new(
        ModuleId::new(module),
        SourcePosition::new(1, 1),
        SourcePosition::new(1, 1),
    )
}

fn simple_digest(s: &str) -> String {
    // Non-cryptographic bootstrap digest for cache key scaffolding.
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{h:016x}")
}

// silence unused SemaKind if any
#[allow(dead_code)]
fn _map_sema(k: SemaKind) -> SirBindingKind {
    match k {
        SemaKind::Mutable => SirBindingKind::Let,
        SemaKind::Immutable => SirBindingKind::Const,
        SemaKind::Builtin => SirBindingKind::Builtin,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sir::SirNode;

    #[test]
    fn lower_let_and_call() {
        let unit = compile_to_sir("let x = 1\nprint(x)\n", "main").unwrap();
        assert!(unit.node_count() > 0);
        assert!(unit.binding_count() >= 2); // print + x
        assert!(matches!(
            unit.node(unit.root_node).map(|n| &n.kind),
            Some(SirNode::ModuleBody { .. })
        ));
    }

    #[test]
    fn lower_fib() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        let unit = compile_to_sir(src, "fib").unwrap();
        assert!(unit.exports.is_empty());
        assert!(unit
            .bindings
            .iter()
            .any(|b| unit.symbol_text(b.symbol_id) == Some("fib")));
        assert!(matches!(
            unit.node(unit.root_node).map(|n| &n.kind),
            Some(SirNode::ModuleBody { items }) if !items.is_empty()
        ));
    }

    #[test]
    fn lower_rejects_sema_errors() {
        let err = compile_to_sir("x = 1\n", "bad").unwrap_err();
        assert!(matches!(err, LowerError::SemaFailed { .. }));
    }

    #[test]
    fn lower_import_export() {
        let src = "import util.math as m\nexport def id(x):\n    return x\n";
        let unit = compile_to_sir(src, "mod").unwrap();
        assert!(unit.imports.iter().any(|p| p == "util.math"));
        assert!(unit.exports.iter().any(|e| e == "id"));
        assert!(unit.has_required_tables());
        assert!(!unit.sources.files.is_empty());
    }

    #[test]
    fn materialize_from_analyzed() {
        use script_sema::analyze_source;
        use sir::ControlRegionKind;
        let a = analyze_source("let x = 1\nprint(x)\n");
        assert!(a.ok());
        let unit = materialize_sir(&a, "main").unwrap();
        assert!(matches!(
            unit.node(unit.root_node).map(|n| &n.kind),
            Some(SirNode::ModuleBody { .. })
        ));
        assert!(!unit.control_regions.is_empty());
        assert!(unit
            .control_regions
            .iter()
            .any(|r| r.kind == ControlRegionKind::Module));
    }
}
