//! Call-time default argument evaluation hook.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §7

use vm_core::value::Value;

use crate::call::bind::{ArgumentBinding, ParameterSpec};
use crate::runtime_error::RuntimeResult;

/// Evaluates default expressions at call time in parameter declaration order.
pub trait DefaultEvaluator {
    fn evaluate_default(&mut self, default_index: usize) -> RuntimeResult<Value>;
}

/// Fill pending defaults; if evaluation raises, body MUST NOT start.
pub fn fill_defaults(
    params: &[ParameterSpec],
    binding: &mut ArgumentBinding,
    evaluator: &mut impl DefaultEvaluator,
) -> RuntimeResult<()> {
    let pending: Vec<usize> = binding.pending_default_indices.clone();
    for param_index in pending {
        let param = &params[param_index];
        let default_index = param.default_index.expect("pending implies default");
        let value = evaluator.evaluate_default(default_index)?;
        binding.bound.push((param.slot_id, value));
    }
    binding.pending_default_indices.clear();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::bind::bind_arguments;
    use crate::runtime_error::RuntimeFailure;
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::id::SlotId;

    struct StubDefaults {
        raises_on: Option<usize>,
    }

    impl DefaultEvaluator for StubDefaults {
        fn evaluate_default(&mut self, default_index: usize) -> RuntimeResult<Value> {
            if self.raises_on == Some(default_index) {
                return Err(RuntimeFailure::language(RuntimeErrorCode::TypeError));
            }
            Ok(Value::Int(default_index as i64))
        }
    }

    fn two_param_plan() -> Vec<ParameterSpec> {
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
    fn default_evaluated_at_call_time() {
        let mut binding = bind_arguments(&two_param_plan(), &[Value::Int(5)], &[]).expect("bind");
        let mut evaluator = StubDefaults { raises_on: None };
        fill_defaults(&two_param_plan(), &mut binding, &mut evaluator).expect("defaults");
        assert_eq!(binding.bound.len(), 2);
        assert_eq!(binding.bound[1].1, Value::Int(0));
    }

    #[test]
    fn default_raises_prevents_binding_completion() {
        let mut binding = bind_arguments(&two_param_plan(), &[Value::Int(5)], &[]).expect("bind");
        let mut evaluator = StubDefaults {
            raises_on: Some(0),
        };
        let err = fill_defaults(&two_param_plan(), &mut binding, &mut evaluator).expect_err("raise");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::TypeError)
        ));
        assert!(!binding.pending_default_indices.is_empty());
    }
}