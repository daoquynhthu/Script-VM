//! Positional and named argument binding.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §6

use std::collections::BTreeSet;

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::{SlotId, TypeId};

use crate::call::input::NamedArgumentValue;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};
use vm_core::value::Value;

/// Parameter metadata for call-time binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterSpec {
    pub name: String,
    pub slot_id: SlotId,
    pub required: bool,
    pub default_index: Option<usize>,
    pub type_id: Option<TypeId>,
}

/// Result of binding caller-supplied arguments (defaults not yet filled).
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentBinding {
    pub bound: Vec<(SlotId, Value)>,
    /// Parameter indices still needing default evaluation (in declaration order).
    pub pending_default_indices: Vec<usize>,
}

fn arity_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::ArityError)
}

/// Bind positional and named arguments per frozen protocol.
pub fn bind_arguments(
    params: &[ParameterSpec],
    positional: &[Value],
    named: &[NamedArgumentValue],
) -> RuntimeResult<ArgumentBinding> {
    if positional.len() > params.len() {
        return Err(arity_error());
    }

    let mut bound_by_index = vec![None::<Value>; params.len()];
    let mut named_used = BTreeSet::new();

    for (index, value) in positional.iter().enumerate() {
        bound_by_index[index] = Some(value.clone());
    }

    for named_arg in named {
        if named_used.contains(&named_arg.name) {
            return Err(arity_error());
        }
        let param_index = params
            .iter()
            .position(|p| p.name == named_arg.name)
            .ok_or_else(arity_error)?;
        if bound_by_index[param_index].is_some() {
            return Err(arity_error());
        }
        bound_by_index[param_index] = Some(named_arg.value.clone());
        named_used.insert(named_arg.name.clone());
    }

    let mut bound = Vec::new();
    let mut pending_default_indices = Vec::new();

    for (index, param) in params.iter().enumerate() {
        match bound_by_index[index].clone() {
            Some(value) => bound.push((param.slot_id, value)),
            None if param.default_index.is_some() => {
                pending_default_indices.push(index);
            }
            None if !param.required => {
                pending_default_indices.push(index);
            }
            None => return Err(arity_error()),
        }
    }

    Ok(ArgumentBinding {
        bound,
        pending_default_indices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> Vec<ParameterSpec> {
        vec![
            ParameterSpec {
                name: "a".to_string(),
                slot_id: SlotId::new(0),
                required: true,
                default_index: None,
                type_id: None,
            },
            ParameterSpec {
                name: "b".to_string(),
                slot_id: SlotId::new(1),
                required: false,
                default_index: Some(0),
                type_id: None,
            },
        ]
    }

    #[test]
    fn wrong_arity_rejected() {
        let err = bind_arguments(&params(), &[Value::Int(1), Value::Int(2), Value::Int(3)], &[])
            .expect_err("too many");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::ArityError)
        ));
    }

    #[test]
    fn duplicate_named_argument_rejected() {
        let named = vec![
            NamedArgumentValue {
                name: "b".to_string(),
                value: Value::Int(2),
            },
            NamedArgumentValue {
                name: "b".to_string(),
                value: Value::Int(3),
            },
        ];
        let err = bind_arguments(&params(), &[Value::Int(1)], &named).expect_err("dup");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::ArityError)
        ));
    }

    #[test]
    fn missing_required_argument_rejected() {
        let err = bind_arguments(&params(), &[], &[]).expect_err("missing");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::ArityError)
        ));
    }

    #[test]
    fn positional_and_named_same_param_rejected() {
        let named = vec![NamedArgumentValue {
            name: "a".to_string(),
            value: Value::Int(9),
        }];
        let err = bind_arguments(&params(), &[Value::Int(1)], &named).expect_err("both");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::ArityError)
        ));
    }

    #[test]
    fn optional_param_defers_default() {
        let binding = bind_arguments(&params(), &[Value::Int(1)], &[]).expect("bind");
        assert_eq!(binding.bound.len(), 1);
        assert_eq!(binding.pending_default_indices, vec![1]);
    }
}