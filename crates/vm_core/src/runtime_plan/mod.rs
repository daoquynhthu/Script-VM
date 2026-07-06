//! RuntimePlan schema and validation.
//!
//! Spec: `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md`

pub mod fixtures;
pub mod schema;
pub mod validate;

pub use schema::RuntimePlan;
pub use validate::{validate_runtime_plan, ValidationError};