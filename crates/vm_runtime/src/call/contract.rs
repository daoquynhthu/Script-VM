//! Parameter and return type contract checks.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §8, §11

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::TypeId;
use vm_core::value::Value;

use crate::call::bind::{ArgumentBinding, ParameterSpec};
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Type contract checking hook (bootstrap: exact tag match when types are known).
pub trait TypeContractChecker {
    fn value_matches_type(&self, value: &Value, type_id: TypeId) -> bool;
}

/// Bootstrap checker using a predeclared type membership table.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StubTypeContractChecker {
    memberships: std::collections::BTreeMap<u32, Vec<&'static str>>,
}

impl StubTypeContractChecker {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn declare_int_type(&mut self, type_id: TypeId) {
        self.memberships
            .entry(type_id.raw())
            .or_default()
            .push("int");
    }
}

impl TypeContractChecker for StubTypeContractChecker {
    fn value_matches_type(&self, value: &Value, type_id: TypeId) -> bool {
        let tags = self.memberships.get(&type_id.raw());
        match (value, tags) {
            (Value::Int(_), Some(tags)) => tags.contains(&"int"),
            _ => false,
        }
    }
}

fn contract_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::TypeContractError)
}

/// Check parameter contracts after binding and before body execution.
pub fn check_parameter_contracts(
    params: &[ParameterSpec],
    binding: &ArgumentBinding,
    checker: &(impl TypeContractChecker + ?Sized),
) -> RuntimeResult<()> {
    for (slot_id, value) in &binding.bound {
        let param = params
            .iter()
            .find(|p| p.slot_id == *slot_id)
            .expect("bound slot must match parameter");
        if let Some(type_id) = param.type_id {
            if !checker.value_matches_type(value, type_id) {
                return Err(contract_error());
            }
        }
    }
    Ok(())
}

/// Check return contract before exposing value to caller.
pub fn check_return_contract(
    value: &Value,
    return_type: Option<TypeId>,
    checker: &(impl TypeContractChecker + ?Sized),
) -> RuntimeResult<()> {
    if let Some(type_id) = return_type {
        if !checker.value_matches_type(value, type_id) {
            return Err(contract_error());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::SlotId;

    #[test]
    fn return_contract_failure_rejected() {
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));
        let err = check_return_contract(&Value::String("x".into()), Some(TypeId::new(1)), &checker)
            .expect_err("mismatch");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::TypeContractError)
        ));
    }

    #[test]
    fn return_contract_success_when_matching() {
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));
        assert!(
            check_return_contract(&Value::Int(3), Some(TypeId::new(1)), &checker).is_ok()
        );
    }

    #[test]
    fn parameter_contract_checked_before_body() {
        let params = vec![ParameterSpec {
            name: "x".to_string(),
            slot_id: SlotId::new(0),
            required: true,
            default_index: None,
            type_id: Some(TypeId::new(1)),
        }];
        let binding = ArgumentBinding {
            bound: vec![(SlotId::new(0), Value::String("bad".into()))],
            pending_default_indices: vec![],
        };
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));
        let err =
            check_parameter_contracts(&params, &binding, &checker).expect_err("contract");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::TypeContractError)
        ));
    }
}