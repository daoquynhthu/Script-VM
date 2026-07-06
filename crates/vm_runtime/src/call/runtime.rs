//! Call execution orchestration shell.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §3

use vm_core::value::Value;

use crate::call::bind::{bind_arguments, ParameterSpec};
use crate::call::callable::{check_callable, CallableRegistry};
use crate::call::contract::{
    check_parameter_contracts, check_return_contract, TypeContractChecker,
};
use crate::call::default::{fill_defaults, DefaultEvaluator};
use crate::call::input::CallFrameInput;
use crate::module::resolver::CapabilitySet;
use crate::runtime_error::RuntimeResult;

/// Prepared call state ready for frame entry or builtin/host dispatch.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedCall {
    pub binding: crate::call::bind::ArgumentBinding,
    pub target: crate::call::callable::CallableTarget,
}

/// Coordinates canonical call preparation steps 4–8.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CallRuntime {
    pub callable_registry: CallableRegistry,
    pub capabilities: CapabilitySet,
}

impl CallRuntime {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Steps 4–8: callability, bind, defaults, parameter contracts.
    pub fn prepare_call(
        &self,
        input: &CallFrameInput,
        params: &[ParameterSpec],
        evaluator: &mut impl DefaultEvaluator,
        checker: &impl TypeContractChecker,
    ) -> RuntimeResult<PreparedCall> {
        let target = check_callable(&input.callee, &self.callable_registry)?;
        let mut binding = bind_arguments(params, &input.positional_args, &input.named_args)?;
        fill_defaults(params, &mut binding, evaluator)?;
        check_parameter_contracts(params, &binding, checker)?;
        Ok(PreparedCall { binding, target })
    }

    /// Step 11: return contract before exposing to caller.
    pub fn finalize_return(
        &self,
        value: &Value,
        return_type: Option<vm_core::id::TypeId>,
        checker: &impl TypeContractChecker,
    ) -> RuntimeResult<Value> {
        check_return_contract(value, return_type, checker)?;
        Ok(value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::callable::{CallableTarget, UserFunctionTarget};
    use crate::call::contract::StubTypeContractChecker;
    use crate::call::default::DefaultEvaluator;
    use crate::call::input::CallFrameInput;
    use crate::runtime_error::RuntimeFailure;
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::id::{
        CallSiteId, EirFunctionId, FunctionId, ModuleId, ObjectId, SlotId, TypeId,
    };
    use vm_diag::source_span::SourceSpanId;

    struct NoRaiseDefaults;

    impl DefaultEvaluator for NoRaiseDefaults {
        fn evaluate_default(&mut self, default_index: usize) -> RuntimeResult<Value> {
            Ok(Value::Int(default_index as i64))
        }
    }

    #[test]
    fn prepare_call_runs_binding_and_contracts() {
        let mut rt = CallRuntime::new();
        let object_id = ObjectId::new(1);
        rt.callable_registry.register(
            object_id,
            CallableTarget::UserFunction(UserFunctionTarget {
                function_id: FunctionId::new(0),
                module_id: ModuleId::new(0),
                entry_eir_function: EirFunctionId::new(0),
                return_type: Some(TypeId::new(1)),
                object_id,
            }),
        );
        let params = vec![ParameterSpec {
            name: "x".to_string(),
            slot_id: SlotId::new(0),
            required: true,
            default_index: None,
            type_id: Some(TypeId::new(1)),
        }];
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));
        let input = CallFrameInput::new(
            Value::ObjectRef(object_id),
            CallSiteId::new(0),
            SourceSpanId::new(0),
        )
        .with_positional(vec![Value::Int(7)]);
        let prepared = rt
            .prepare_call(&input, &params, &mut NoRaiseDefaults, &checker)
            .expect("prepare");
        assert_eq!(prepared.binding.bound.len(), 1);
    }

    #[test]
    fn prepare_call_stops_when_default_raises() {
        let mut rt = CallRuntime::new();
        let object_id = ObjectId::new(2);
        rt.callable_registry.register(
            object_id,
            CallableTarget::UserFunction(UserFunctionTarget {
                function_id: FunctionId::new(0),
                module_id: ModuleId::new(0),
                entry_eir_function: EirFunctionId::new(0),
                return_type: None,
                object_id,
            }),
        );
        let params = vec![
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
        ];
        struct RaisingDefaults;
        impl DefaultEvaluator for RaisingDefaults {
            fn evaluate_default(&mut self, _: usize) -> RuntimeResult<Value> {
                Err(RuntimeFailure::language(RuntimeErrorCode::TypeError))
            }
        }
        let input = CallFrameInput::new(
            Value::ObjectRef(object_id),
            CallSiteId::new(0),
            SourceSpanId::new(0),
        )
        .with_positional(vec![Value::Int(1)]);
        let err = rt
            .prepare_call(&input, &params, &mut RaisingDefaults, &StubTypeContractChecker::new())
            .expect_err("default raise");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::TypeError)
        ));
    }
}