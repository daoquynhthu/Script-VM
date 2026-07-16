//! SIR validation and diagnostics (T-P2).
//!
//! Spec: `PHASE-2-IR-SPEC.md` §4 required tables / unit integrity.

pub mod validate;

pub use validate::{validate_ir_unit, ValidationResult};
