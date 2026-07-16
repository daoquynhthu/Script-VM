//! Bootstrap IR unit container (Phase 2 schema subset).
//!
//! Spec: `PHASE-2-IR-SPEC.md` §4 IR Unit Schema (required tables; node bodies bootstrap)

use crate::id::{
    BindingId, CapabilityId, ControlRegionId, EffectId, ModuleId, NodeId, ScopeId, SymbolId,
    TypeId,
};
use crate::node::SirNode;
use crate::source::{SourceOrigin, SourceSpan};

/// Semantic IR schema version for this bootstrap producer.
pub const IR_SCHEMA_VERSION: &str = "1.0.0-bootstrap";
/// Phase 1 language baseline version.
pub const LANGUAGE_BASELINE_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    /// Optional pre-release label (e.g. bootstrap).
    pub pre: Option<String>,
}

impl Version {
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
        }
    }

    #[must_use]
    pub fn bootstrap(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: Some("bootstrap".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrHeader {
    pub ir_schema_version: Version,
    pub language_baseline_version: Version,
    pub producer_name: String,
    pub producer_version: String,
    pub source_digest: String,
    pub semantic_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDescriptor {
    pub module_id: ModuleId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolDescriptor {
    pub symbol_id: SymbolId,
    /// NFC-normalized text.
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeDescriptor {
    pub scope_id: ScopeId,
    pub parent: Option<ScopeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SirBindingKind {
    Let,
    Const,
    Function,
    Parameter,
    ForIteration,
    Import,
    Builtin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SirMutability {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SirVisibility {
    Local,
    ModulePrivate,
    Exported,
    Imported,
    Builtin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingDescriptor {
    pub binding_id: BindingId,
    pub symbol_id: SymbolId,
    pub scope_id: ScopeId,
    pub kind: SirBindingKind,
    pub mutability: SirMutability,
    pub visibility: SirVisibility,
    pub initializer_node: Option<NodeId>,
    pub declaration_node: Option<NodeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeEntry {
    pub node_id: NodeId,
    pub kind: SirNode,
    pub origin: SourceOrigin,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrUnit {
    pub header: IrHeader,
    pub module: ModuleDescriptor,
    /// Required source table (SPEC-P2 §4.2).
    pub sources: SourceTable,
    pub symbols: Vec<SymbolDescriptor>,
    pub scopes: Vec<ScopeDescriptor>,
    pub bindings: Vec<BindingDescriptor>,
    /// Required empty tables (bootstrap placeholders).
    pub types: Vec<TypeId>,
    pub capabilities: Vec<CapabilityId>,
    pub effects: Vec<EffectId>,
    pub nodes: Vec<NodeEntry>,
    pub patterns: Vec<()>,
    /// Control regions (SPEC-P2 required table; bootstrap kinds).
    pub control_regions: Vec<ControlRegionDescriptor>,
    /// Module interface exports (symbol texts); required table bootstrap.
    pub interface_exports: Vec<String>,
    pub root_node: NodeId,
    /// Exported symbol texts (alias of interface for callers).
    pub exports: Vec<String>,
    /// Imported module paths (bootstrap).
    pub imports: Vec<String>,
}

impl IrUnit {
    /// True if required structural tables are present (may be empty where allowed).
    #[must_use]
    pub fn has_required_tables(&self) -> bool {
        // Sources must list ≥1 file for a materialised unit; root must be valid id.
        !self.sources.files.is_empty() && self.root_node.is_valid()
    }

    #[must_use]
    pub fn node(&self, id: NodeId) -> Option<&NodeEntry> {
        self.nodes.iter().find(|n| n.node_id == id)
    }

    #[must_use]
    pub fn symbol_text(&self, id: SymbolId) -> Option<&str> {
        self.symbols
            .iter()
            .find(|s| s.symbol_id == id)
            .map(|s| s.text.as_str())
    }

    #[must_use]
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }

    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

/// Minimal source file record for the source table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFileRecord {
    pub path_or_name: String,
    pub digest: String,
    pub encoding: &'static str,
}

/// Bootstrap source table (one primary file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceTable {
    pub files: Vec<SourceFileRecord>,
    pub spans: Vec<SourceSpan>,
}

/// Bootstrap control-region kinds (align with Phase 2/3 control model at coarse grain).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlRegionKind {
    Module,
    Function,
    Block,
    Loop,
}

/// One control region row (SPEC-P2 control_regions table bootstrap).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlRegionDescriptor {
    pub region_id: ControlRegionId,
    pub kind: ControlRegionKind,
    pub parent: Option<ControlRegionId>,
    /// Owning SIR node when applicable (function/block/while/for).
    pub owner_node: Option<NodeId>,
}
