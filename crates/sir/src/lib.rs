//! Script IR (SIR) — internal semantic representation (not public bytecode).
//!
//! Spec: `PHASE-2-IR-SPEC.md`, SIR semantics rounds (bootstrap subset).
//!
//! Boundary: SIR is versioned internal schema only; never a distribution ABI.

pub mod id;
pub mod node;
pub mod source;
pub mod unit;

pub use id::*;
pub use node::{BinaryOp, SirCatch, SirNode, UnaryOp};
pub use source::{SourceOrigin, SourcePosition, SourceSpan};
pub use unit::{
    BindingDescriptor, ControlRegionDescriptor, ControlRegionKind, IrHeader, IrUnit,
    ModuleDescriptor, NodeEntry, ScopeDescriptor, SirBindingKind, SirMutability, SirVisibility,
    SourceFileRecord, SourceTable, SymbolDescriptor, Version, IR_SCHEMA_VERSION,
    LANGUAGE_BASELINE_VERSION,
};

// Note: SourceTable is part of required IR unit tables (SPEC-P2 §4.2).
