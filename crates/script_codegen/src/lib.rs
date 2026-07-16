//! Bootstrap Phase 1 → EIR codegen for `vm_eval` (WP-25).
//!
//! Pipeline: source → parse → sema → EIR module + callable registry.
//! Spec: PHASE-1 language surface; PHASE-3 EIR schema (bootstrap subset).

pub mod codegen;
pub mod error;
pub mod program;

pub use codegen::{compile_module, compile_source};
pub use error::CodegenError;
pub use program::CompiledProgram;
