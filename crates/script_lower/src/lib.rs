//! Phase 1 AST → bootstrap SIR lowering.
//!
//! Spec: `PHASE-2-IR-SPEC.md` (IR unit), Phase 1 language as input.
//! SIR remains internal; not public bytecode.

pub mod error;
pub mod lower;

pub use error::LowerError;
pub use lower::{compile_to_sir, lower_module, materialize_sir};
