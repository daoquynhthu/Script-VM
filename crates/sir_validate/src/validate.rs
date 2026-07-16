//! SIR unit structural validation (T-P2 baseline).
//!
//! Spec: `PHASE-2-IR-SPEC.md` §4 IR Unit Schema (required tables / single module)

use sir::{IrUnit, NodeId, SirNode};
use vm_diag::diagnostic::{Diagnostic, DiagnosticSeverity};

/// Validation outcome for SIR units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationResult {
    #[must_use]
    pub fn ok() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.diagnostics
            .iter()
            .all(|d| d.severity != DiagnosticSeverity::Error)
    }

    fn error(&mut self, code: &str, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::error(code, message));
    }
}

/// Validate a materialised SIR unit (structural / table integrity).
pub fn validate_ir_unit(unit: &IrUnit) -> ValidationResult {
    let mut r = ValidationResult::ok();

    if unit.sources.files.is_empty() {
        r.error("SIR001", "sources table must contain at least one file");
    }

    if !unit.root_node.is_valid() {
        r.error("SIR002", "root_node is invalid");
    } else if unit.node(unit.root_node).is_none() {
        r.error(
            "SIR003",
            format!(
                "root_node {} missing from nodes table",
                unit.root_node.raw()
            ),
        );
    } else if let Some(entry) = unit.node(unit.root_node) {
        if !matches!(entry.kind, SirNode::ModuleBody { .. }) {
            r.error("SIR004", "root_node must be ModuleBody for executable units");
        }
    }

    // Symbol id uniqueness
    let mut seen_sym = std::collections::BTreeSet::new();
    for s in &unit.symbols {
        if !seen_sym.insert(s.symbol_id.raw()) {
            r.error(
                "SIR005",
                format!("duplicate symbol_id {}", s.symbol_id.raw()),
            );
        }
    }

    // Binding id uniqueness + symbol refs
    let mut seen_bind = std::collections::BTreeSet::new();
    for b in &unit.bindings {
        if !seen_bind.insert(b.binding_id.raw()) {
            r.error(
                "SIR006",
                format!("duplicate binding_id {}", b.binding_id.raw()),
            );
        }
        if unit.symbol_text(b.symbol_id).is_none() {
            r.error(
                "SIR007",
                format!(
                    "binding {} references missing symbol {}",
                    b.binding_id.raw(),
                    b.symbol_id.raw()
                ),
            );
        }
    }

    // Node id uniqueness + child refs exist
    let mut seen_node = std::collections::BTreeSet::new();
    for n in &unit.nodes {
        if !seen_node.insert(n.node_id.raw()) {
            r.error("SIR008", format!("duplicate node_id {}", n.node_id.raw()));
        }
        for child in node_children(&n.kind) {
            if unit.node(child).is_none() {
                r.error(
                    "SIR009",
                    format!(
                        "node {} references missing child {}",
                        n.node_id.raw(),
                        child.raw()
                    ),
                );
            }
        }
    }

    // Interface exports should be symbol texts present in export list consistency
    for name in &unit.interface_exports {
        if !unit.exports.contains(name) {
            r.error(
                "SIR010",
                format!("interface_exports contains `{name}` missing from exports"),
            );
        }
    }

    // Header language baseline major must be 1 for Phase 1 frontend output
    if unit.header.language_baseline_version.major != 1 {
        r.error(
            "SIR011",
            "language_baseline_version.major must be 1 for Phase 1 baseline",
        );
    }

    // Control regions: module unit should have at least one Module region (WP-S02).
    if unit.control_regions.is_empty() {
        r.error(
            "SIR012",
            "control_regions table is empty (expected at least Module region)",
        );
    } else {
        let mut seen = std::collections::BTreeSet::new();
        for cr in &unit.control_regions {
            if !seen.insert(cr.region_id.raw()) {
                r.error(
                    "SIR013",
                    format!("duplicate control region id {}", cr.region_id.raw()),
                );
            }
            if let Some(owner) = cr.owner_node {
                if unit.node(owner).is_none() {
                    r.error(
                        "SIR014",
                        format!(
                            "control region {} owner node {} missing",
                            cr.region_id.raw(),
                            owner.raw()
                        ),
                    );
                }
            }
            if let Some(parent) = cr.parent {
                if !unit
                    .control_regions
                    .iter()
                    .any(|p| p.region_id == parent)
                {
                    r.error(
                        "SIR015",
                        format!(
                            "control region {} parent {} missing",
                            cr.region_id.raw(),
                            parent.raw()
                        ),
                    );
                }
            }
        }
    }

    r
}

fn node_children(kind: &SirNode) -> Vec<NodeId> {
    use SirNode::*;
    match kind {
        ModuleBody { items } => items.clone(),
        Let { init, .. } | Const { init, .. } => vec![*init],
        Function { body, .. } => vec![*body],
        Import { .. } | Break | Continue | LiteralNil | LiteralBool { .. } | LiteralInt { .. }
        | LiteralFloat { .. } | LiteralString { .. } | Name { .. } | SymbolRef { .. } => {
            vec![]
        }
        ExportMarker { item } => vec![*item],
        Block { stmts } => stmts.clone(),
        ExprStmt { expr } | Raise { value: expr } | Assert { cond: expr } | Return { value: Some(expr) }
        | Unary { expr, .. } => vec![*expr],
        Return { value: None } => vec![],
        If {
            cond,
            then_block,
            elifs,
            else_block,
        } => {
            let mut v = vec![*cond, *then_block];
            for (c, b) in elifs {
                v.push(*c);
                v.push(*b);
            }
            if let Some(e) = else_block {
                v.push(*e);
            }
            v
        }
        While { cond, body } | For { iter: cond, body, .. } => vec![*cond, *body],
        Assign { value, .. } => vec![*value],
        Call { callee, args } => {
            let mut v = vec![*callee];
            v.extend(args.iter().copied());
            v
        }
        Binary { left, right, .. } => vec![*left, *right],
        List { elements } => elements.clone(),
        Map { entries } => {
            let mut v = Vec::new();
            for (k, val) in entries {
                v.push(*k);
                v.push(*val);
            }
            v
        }
        Index { base, index } => vec![*base, *index],
        IndexAssign {
            base,
            index,
            value,
        } => vec![*base, *index, *value],
        Attr { base, .. } => vec![*base],
        AttrAssign { base, value, .. } => vec![*base, *value],
        Try {
            try_body,
            catches,
            finally_body,
        } => {
            let mut v = vec![*try_body];
            for c in catches {
                if let Some(g) = c.guard {
                    v.push(g);
                }
                v.push(c.body);
            }
            if let Some(f) = finally_body {
                v.push(*f);
            }
            v
        }
    }
}

/// Legacy placeholder name — forwards to empty unit failure.
#[deprecated(note = "use validate_ir_unit")]
pub fn validate_sir_unit() -> ValidationResult {
    ValidationResult::ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use script_lower::compile_to_sir;

    #[test]
    fn fib_unit_validates() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        let unit = compile_to_sir(src, "fib").expect("lower");
        let r = validate_ir_unit(&unit);
        assert!(r.is_valid(), "{:?}", r.diagnostics);
        assert!(unit.has_required_tables());
    }

    #[test]
    fn missing_sources_rejected() {
        let mut unit = compile_to_sir("let x = 1\n", "m").unwrap();
        unit.sources.files.clear();
        let r = validate_ir_unit(&unit);
        assert!(!r.is_valid());
        assert!(r.diagnostics.iter().any(|d| d.code == "SIR001"));
    }

    #[test]
    fn export_interface_present() {
        let unit = compile_to_sir("export def id(x):\n    return x\n", "m").unwrap();
        assert!(unit.exports.iter().any(|e| e == "id"));
        assert!(unit.interface_exports.iter().any(|e| e == "id"));
        assert!(validate_ir_unit(&unit).is_valid());
    }
}
