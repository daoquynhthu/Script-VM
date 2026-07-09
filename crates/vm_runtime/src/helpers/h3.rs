//! Milestone H3 call-engine helper implementations.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.4 / §12,
//! `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` §8.1,
//! `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §3–§8, §12
//!
//! Bootstrap scope: prepare/bind/contract/capability validation and method bind
//! over existing `call/` substrate. Full interpreter body execution and frame
//! push/pop remain delegated (see ISSUE for deferred §12 body path).

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::{CallSiteId, FunctionId, ObjectId, SlotId, TypeId};
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;

use crate::call::bind::{bind_arguments, ParameterSpec};
use crate::call::builtin::{validate_builtin_call, BuiltinCallDescriptor};
use crate::call::callable::{
    check_callable, BoundMethodTarget, CallableRegistry, CallableTarget,
};
use crate::call::contract::{check_parameter_contracts, TypeContractChecker};
use crate::call::default::{fill_defaults, DefaultEvaluator};
use crate::call::input::CallFrameInput;
use crate::control::VmControl;
use crate::heap::Heap;
use crate::module::resolver::CapabilitySet;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Bootstrap call-site feedback updated by generic/builtin call helpers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CallSiteFeedback {
    pub last_callee_kind: Option<&'static str>,
    pub last_function_id: Option<u32>,
    pub last_builtin_id: Option<u32>,
    pub last_arity: Option<u32>,
    pub miss_count: u32,
}

impl CallSiteFeedback {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn record_success(
        &mut self,
        kind: &'static str,
        function_id: Option<u32>,
        builtin_id: Option<u32>,
        arity: u32,
    ) {
        self.last_callee_kind = Some(kind);
        self.last_function_id = function_id;
        self.last_builtin_id = builtin_id;
        self.last_arity = Some(arity);
    }

    fn record_miss(&mut self) {
        self.miss_count = self.miss_count.saturating_add(1);
    }
}

/// Defaults that never raise; used when no pending defaults exist.
#[derive(Debug, Default)]
pub struct NoopDefaultEvaluator;

impl DefaultEvaluator for NoopDefaultEvaluator {
    fn evaluate_default(&mut self, default_index: usize) -> RuntimeResult<Value> {
        Ok(Value::Int(default_index as i64))
    }
}

fn type_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::TypeError)
}

fn arity_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::ArityError)
}

fn require_arg<'a>(args: &'a [Value], index: usize) -> RuntimeResult<&'a Value> {
    args.get(index).ok_or_else(type_error)
}

fn int_arg(args: &[Value], index: usize) -> RuntimeResult<i64> {
    match args.get(index) {
        Some(Value::Int(v)) => Ok(*v),
        _ => Err(type_error()),
    }
}

fn object_ref(args: &[Value], index: usize) -> RuntimeResult<ObjectId> {
    match args.get(index) {
        Some(Value::ObjectRef(id)) => Ok(*id),
        _ => Err(type_error()),
    }
}

fn callee_kind_name(target: &CallableTarget) -> &'static str {
    match target {
        CallableTarget::UserFunction(_) => "UserFunction",
        CallableTarget::BuiltinFunction(_) => "BuiltinFunction",
        CallableTarget::RecordConstructor(_) => "RecordConstructor",
        CallableTarget::EnumCaseConstructor(_) => "EnumCaseConstructor",
        CallableTarget::BoundMethod(_) => "BoundMethod",
        CallableTarget::HostFunction(_) => "HostFunction",
    }
}

/// Bootstrap: `args[0]` expected arity Int, `args[1..]` actual argument values.
/// Returns Unit on match; raises ArityError on mismatch.
pub fn helper_check_arity(args: &[Value]) -> RuntimeResult<()> {
    let expected = int_arg(args, 0)?;
    if expected < 0 {
        return Err(type_error());
    }
    let actual = args.len().saturating_sub(1) as i64;
    if actual != expected {
        return Err(arity_error());
    }
    Ok(())
}

/// Bootstrap: `args[0]` receiver ObjectRef, `args[1]` Int function_id,
/// `args[2]` String method_name.
/// Allocates a callable shell, registers `BoundMethod`, returns ObjectRef.
/// Preserves receiver identity on the bound target.
pub fn helper_bind_method(
    args: &[Value],
    heap: &mut Heap,
    registry: &mut CallableRegistry,
) -> RuntimeResult<Value> {
    let receiver_id = object_ref(args, 0)?;
    let function_raw = int_arg(args, 1)?;
    if function_raw < 0 {
        return Err(type_error());
    }
    let method_name = match require_arg(args, 2)? {
        Value::String(s) => s.clone(),
        _ => return Err(type_error()),
    };
    let shell = heap.alloc_function()?;
    let object_id = shell.id();
    registry.register(
        object_id,
        CallableTarget::BoundMethod(BoundMethodTarget {
            receiver_id,
            function_id: FunctionId::new(function_raw as u32),
            method_name,
        }),
    );
    Ok(Value::ObjectRef(object_id))
}

/// Build positional-only required parameter specs for bootstrap prepare.
fn positional_params(count: usize, type_ids: &[Option<TypeId>]) -> Vec<ParameterSpec> {
    (0..count)
        .map(|i| ParameterSpec {
            name: format!("p{i}"),
            slot_id: SlotId::new(i as u32),
            required: true,
            default_index: None,
            type_id: type_ids.get(i).copied().flatten(),
        })
        .collect()
}

fn prepare_positional(
    callee: &Value,
    positional: &[Value],
    registry: &CallableRegistry,
    checker: &dyn TypeContractChecker,
    evaluator: &mut impl DefaultEvaluator,
    type_ids: &[Option<TypeId>],
) -> RuntimeResult<(CallableTarget, crate::call::bind::ArgumentBinding)> {
    let target = check_callable(callee, registry)?;
    let params = positional_params(positional.len(), type_ids);
    // If target declares fixed arity, enforce before bind.
    if let Some(expected) = declared_arity(&target) {
        if positional.len() as u32 != expected {
            return Err(arity_error());
        }
    }
    let mut binding = bind_arguments(&params, positional, &[])?;
    fill_defaults(&params, &mut binding, evaluator)?;
    check_parameter_contracts(&params, &binding, checker)?;
    Ok((target, binding))
}

fn declared_arity(target: &CallableTarget) -> Option<u32> {
    match target {
        CallableTarget::BuiltinFunction(b) => Some(b.arity),
        CallableTarget::RecordConstructor(r) => Some(r.arity),
        CallableTarget::EnumCaseConstructor(e) => Some(e.arity),
        CallableTarget::UserFunction(_)
        | CallableTarget::BoundMethod(_)
        | CallableTarget::HostFunction(_) => None,
    }
}

/// Bootstrap generic call prepare path (protocol steps 4–8).
///
/// Args: `args[0]` callee, `args[1..]` positional arguments.
/// Returns `VmControl::Normal(Some(callee))` after successful prepare.
/// Does **not** execute callee body (interpreter-owned); updates feedback when provided.
pub fn helper_generic_call(
    args: &[Value],
    registry: &CallableRegistry,
    checker: &dyn TypeContractChecker,
    capabilities: &CapabilitySet,
    feedback: Option<&mut CallSiteFeedback>,
    call_depth: u32,
    max_call_depth: u32,
) -> RuntimeResult<VmControl> {
    if call_depth >= max_call_depth {
        if let Some(fb) = feedback {
            fb.record_miss();
        }
        return Err(RuntimeFailure::language(RuntimeErrorCode::StackOverflowError));
    }
    let callee = require_arg(args, 0)?.clone();
    let positional = if args.len() > 1 { &args[1..] } else { &[] };

    let mut evaluator = NoopDefaultEvaluator;
    let result = prepare_positional(
        &callee,
        positional,
        registry,
        checker,
        &mut evaluator,
        &[],
    );

    let (target, binding) = match result {
        Ok(v) => v,
        Err(err) => {
            if let Some(fb) = feedback {
                fb.record_miss();
            }
            return Err(err);
        }
    };

    // Builtin/host capability checks when prepare targets effectful categories.
    if let CallableTarget::BuiltinFunction(b) = &target {
        let descriptor = BuiltinCallDescriptor {
            builtin_id: b.builtin_id,
            arity: b.arity,
            required_capabilities: b.required_capabilities.clone(),
            may_raise: true,
            may_allocate: true,
        };
        if let Err(err) = validate_builtin_call(&descriptor, &binding, capabilities) {
            if let Some(fb) = feedback {
                fb.record_miss();
            }
            return Err(err);
        }
    }
    if let CallableTarget::HostFunction(h) = &target {
        if let Some(cap) = h.capability {
            if !capabilities.has(cap) {
                if let Some(fb) = feedback {
                    fb.record_miss();
                }
                return Err(RuntimeFailure::language(RuntimeErrorCode::CapabilityError));
            }
        }
    }

    if let Some(fb) = feedback {
        let (function_id, builtin_id) = match &target {
            CallableTarget::UserFunction(u) => (Some(u.function_id.raw()), None),
            CallableTarget::BoundMethod(m) => (Some(m.function_id.raw()), None),
            CallableTarget::BuiltinFunction(b) => (None, Some(b.builtin_id)),
            _ => (None, None),
        };
        fb.record_success(
            callee_kind_name(&target),
            function_id,
            builtin_id,
            binding.bound.len() as u32,
        );
    }

    // Body execution / frame push-pop: delegated to interpreter call engine.
    Ok(VmControl::Normal(Some(callee)))
}

/// Bootstrap builtin call: validate descriptor arity + capabilities.
///
/// Args: `args[0]` builtin callee ObjectRef, `args[1..]` positional args.
/// Returns `VmControl::Normal(Some(callee))` on success.
pub fn helper_call_builtin(
    args: &[Value],
    registry: &CallableRegistry,
    capabilities: &CapabilitySet,
    feedback: Option<&mut CallSiteFeedback>,
) -> RuntimeResult<VmControl> {
    let callee = require_arg(args, 0)?.clone();
    let positional = if args.len() > 1 { &args[1..] } else { &[] };
    let target = match check_callable(&callee, registry) {
        Ok(t) => t,
        Err(err) => {
            if let Some(fb) = feedback {
                fb.record_miss();
            }
            return Err(err);
        }
    };
    let CallableTarget::BuiltinFunction(builtin) = target else {
        if let Some(fb) = feedback {
            fb.record_miss();
        }
        return Err(type_error());
    };

    let params = positional_params(positional.len(), &[]);
    let binding = match bind_arguments(&params, positional, &[]) {
        Ok(b) => b,
        Err(err) => {
            if let Some(fb) = feedback {
                fb.record_miss();
            }
            return Err(err);
        }
    };
    let descriptor = BuiltinCallDescriptor {
        builtin_id: builtin.builtin_id,
        arity: builtin.arity,
        required_capabilities: builtin.required_capabilities.clone(),
        may_raise: true,
        may_allocate: true,
    };
    if let Err(err) = validate_builtin_call(&descriptor, &binding, capabilities) {
        if let Some(fb) = feedback {
            fb.record_miss();
        }
        return Err(err);
    }

    if let Some(fb) = feedback {
        fb.record_success(
            "BuiltinFunction",
            None,
            Some(builtin.builtin_id),
            binding.bound.len() as u32,
        );
    }
    Ok(VmControl::Normal(Some(callee)))
}

/// Build a CallFrameInput for tests / future interpreter wiring.
#[must_use]
pub fn bootstrap_call_input(callee: Value, positional: Vec<Value>) -> CallFrameInput {
    CallFrameInput::new(callee, CallSiteId::new(0), SourceSpanId::new(0))
        .with_positional(positional)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::callable::{BuiltinFunctionTarget, UserFunctionTarget};
    use crate::call::contract::StubTypeContractChecker;
    use vm_core::id::{CapabilityId, EirFunctionId, ModuleId};

    #[test]
    fn check_arity_accepts_matching_count() {
        helper_check_arity(&[Value::Int(2), Value::Int(1), Value::Int(2)]).expect("ok");
    }

    #[test]
    fn check_arity_rejects_mismatch() {
        let err = helper_check_arity(&[Value::Int(1), Value::Int(1), Value::Int(2)])
            .expect_err("arity");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ArityError));
    }

    #[test]
    fn bind_method_preserves_receiver_identity() {
        let mut heap = Heap::new();
        let mut registry = CallableRegistry::new();
        let receiver = heap.alloc_list(vec![], false).expect("recv");
        let bound = helper_bind_method(
            &[
                Value::ObjectRef(receiver.id()),
                Value::Int(9),
                Value::String("m".into()),
            ],
            &mut heap,
            &mut registry,
        )
        .expect("bind");
        let Value::ObjectRef(id) = bound else {
            panic!("object");
        };
        match registry.resolve(&Value::ObjectRef(id)).expect("resolve") {
            CallableTarget::BoundMethod(m) => {
                assert_eq!(m.receiver_id, receiver.id());
                assert_eq!(m.function_id, FunctionId::new(9));
                assert_eq!(m.method_name, "m");
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn generic_call_prepares_user_function() {
        let mut registry = CallableRegistry::new();
        let object_id = ObjectId::new(3);
        registry.register(
            object_id,
            CallableTarget::UserFunction(UserFunctionTarget {
                function_id: FunctionId::new(1),
                module_id: ModuleId::new(0),
                entry_eir_function: EirFunctionId::new(0),
                return_type: None,
                object_id,
            }),
        );
        let checker = StubTypeContractChecker::new();
        let caps = CapabilitySet::new();
        let mut feedback = CallSiteFeedback::new();
        let control = helper_generic_call(
            &[Value::ObjectRef(object_id), Value::Int(1)],
            &registry,
            &checker,
            &caps,
            Some(&mut feedback),
            0,
            64,
        )
        .expect("call");
        assert_eq!(control, VmControl::Normal(Some(Value::ObjectRef(object_id))));
        assert_eq!(feedback.last_callee_kind, Some("UserFunction"));
        assert_eq!(feedback.last_function_id, Some(1));
    }

    #[test]
    fn call_builtin_requires_capability() {
        let mut registry = CallableRegistry::new();
        let object_id = ObjectId::new(4);
        registry.register(
            object_id,
            CallableTarget::BuiltinFunction(BuiltinFunctionTarget {
                builtin_id: 7,
                arity: 1,
                required_capabilities: vec![CapabilityId::new(2)],
            }),
        );
        let caps = CapabilitySet::new();
        let err = helper_call_builtin(
            &[Value::ObjectRef(object_id), Value::Int(1)],
            &registry,
            &caps,
            None,
        )
        .expect_err("cap");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::CapabilityError)
        );
    }
}
