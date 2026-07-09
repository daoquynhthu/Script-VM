//! Milestone H1 helper implementations (allocation, error, type, write barrier).
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.2,
//! `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` §8.3–§8.4, §8.11–§8.12

use vm_core::error::language::{ErrorObj, ErrorStore};
use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::TypeId;
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;

use crate::call::callable::{check_callable, CallableRegistry};
use crate::call::contract::TypeContractChecker;
use crate::heap::Heap;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};
use crate::value::value_to_key;
use crate::write_barrier::{value_may_mutate_heap, WriteBarrierHook};

/// Bootstrap argument layout for `helper_alloc_object`:
/// optional `args[0]` Int tag: `0` = empty list (default), `1` = empty map.
pub fn helper_alloc_object(args: &[Value], heap: &mut Heap) -> RuntimeResult<Value> {
    let kind = match args.first() {
        None | Some(Value::None) => 0,
        Some(Value::Int(tag)) => *tag,
        _ => {
            return Err(RuntimeFailure::language(RuntimeErrorCode::TypeError));
        }
    };
    let obj = match kind {
        0 => heap.alloc_list(vec![], false)?,
        1 => heap.alloc_map(false)?,
        _ => return Err(RuntimeFailure::language(RuntimeErrorCode::TypeError)),
    };
    Ok(Value::ObjectRef(obj.id()))
}

/// Bootstrap argument layout for `helper_construct_error`:
/// `args[0]` Int index into `RuntimeErrorCode::ALL`, `args[1]` String message.
pub fn helper_construct_error(
    args: &[Value],
    store: &mut ErrorStore,
    source_span: Option<SourceSpanId>,
) -> RuntimeResult<Value> {
    let (code, message) = parse_construct_error_args(args)?;
    let mut error = ErrorObj::new(code, message);
    if let Some(span) = source_span {
        error = error.with_source_span(span);
    }
    let handle = store.allocate(error);
    Ok(Value::Error(handle))
}

fn parse_construct_error_args(args: &[Value]) -> RuntimeResult<(RuntimeErrorCode, String)> {
    let code_index = match args.first() {
        Some(Value::Int(index)) if *index >= 0 => *index as usize,
        _ => return Err(RuntimeFailure::language(RuntimeErrorCode::TypeError)),
    };
    let code = RuntimeErrorCode::ALL
        .get(code_index)
        .copied()
        .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::TypeError))?;
    let message = match args.get(1) {
        Some(Value::String(text)) => text.clone(),
        _ => return Err(RuntimeFailure::language(RuntimeErrorCode::TypeError)),
    };
    Ok((code, message))
}

/// Bootstrap argument layout: `args[0]` value, `args[1]` Int `TypeId` raw.
pub fn helper_check_type_contract(
    args: &[Value],
    checker: &dyn TypeContractChecker,
) -> RuntimeResult<Value> {
    let value = args
        .first()
        .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::TypeError))?
        .clone();
    let type_id = match args.get(1) {
        Some(Value::Int(raw)) => TypeId::new(*raw as u32),
        _ => return Err(RuntimeFailure::language(RuntimeErrorCode::TypeError)),
    };
    if checker.value_matches_type(&value, type_id) {
        Ok(value)
    } else {
        Err(RuntimeFailure::language(RuntimeErrorCode::TypeContractError))
    }
}

/// Bootstrap argument layout: `args[0]` callee value; returns callee on success.
pub fn helper_check_callable(
    args: &[Value],
    registry: &CallableRegistry,
) -> RuntimeResult<Value> {
    let callee = args
        .first()
        .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::TypeError))?;
    let _target = check_callable(callee, registry)?;
    Ok(callee.clone())
}

/// Bootstrap argument layout: `args[0]` candidate map key value.
pub fn helper_check_hashable(args: &[Value], heap: &Heap) -> RuntimeResult<Value> {
    let value = args
        .first()
        .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::TypeError))?;
    let _key = value_to_key(value, heap)?;
    Ok(value.clone())
}

/// Bootstrap argument layout: `args[0]` previous value (or `None`), `args[1]` new value.
pub fn helper_write_barrier(
    args: &[Value],
    hook: &mut dyn WriteBarrierHook,
) -> RuntimeResult<()> {
    let previous = args.first();
    let new_value = args
        .get(1)
        .or_else(|| args.first())
        .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::TypeError))?;
    if value_may_mutate_heap(previous, new_value) {
        hook.on_heap_mutation(previous, new_value);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::callable::{CallableRegistry, CallableTarget, UserFunctionTarget};
    use crate::call::contract::StubTypeContractChecker;
    use vm_core::id::{EirFunctionId, FunctionId, ModuleId, ObjectId, TypeId};

    #[test]
    fn helper_alloc_object_returns_list_handle_by_default() {
        let mut heap = Heap::new();
        let value = helper_alloc_object(&[], &mut heap).expect("alloc");
        assert!(matches!(value, Value::ObjectRef(_)));
    }

    #[test]
    fn helper_alloc_object_map_tag_allocates_map() {
        let mut heap = Heap::new();
        let value = helper_alloc_object(&[Value::Int(1)], &mut heap).expect("alloc map");
        let Value::ObjectRef(id) = value else {
            panic!("expected object ref");
        };
        assert!(matches!(heap.get(id).expect("get"), crate::heap::object::HeapObject::Map { .. }));
    }

    #[test]
    fn helper_construct_error_returns_error_value() {
        let mut store = ErrorStore::new();
        let value = helper_construct_error(
            &[Value::Int(2), Value::String("msg".into())],
            &mut store,
            None,
        )
        .expect("construct");
        let Value::Error(handle) = value else {
            panic!("expected error handle");
        };
        let error = store.get(handle).expect("stored");
        assert_eq!(error.error_code, RuntimeErrorCode::TypeError);
        assert_eq!(error.message, "msg");
    }

    #[test]
    fn helper_check_type_contract_passes_matching_value() {
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));
        let value = helper_check_type_contract(
            &[Value::Int(7), Value::Int(1)],
            &checker,
        )
        .expect("pass");
        assert_eq!(value, Value::Int(7));
    }

    #[test]
    fn helper_check_type_contract_rejects_mismatch() {
        let checker = StubTypeContractChecker::new();
        let err = helper_check_type_contract(&[Value::Int(7), Value::Int(1)], &checker)
            .expect_err("reject");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeContractError));
    }

    #[test]
    fn helper_check_callable_recognizes_registered_function() {
        let mut registry = CallableRegistry::new();
        let object_id = ObjectId::new(1);
        registry.register(
            object_id,
            CallableTarget::UserFunction(UserFunctionTarget {
                function_id: FunctionId::new(0),
                module_id: ModuleId::new(0),
                entry_eir_function: EirFunctionId::new(0),
                return_type: None,
                object_id,
            }),
        );
        let callee = Value::ObjectRef(object_id);
        let value = helper_check_callable(&[callee.clone()], &registry).expect("callable");
        assert_eq!(value, callee);
    }

    #[test]
    fn helper_check_hashable_accepts_int_key() {
        let heap = Heap::new();
        let value = helper_check_hashable(&[Value::Int(3)], &heap).expect("hashable");
        assert_eq!(value, Value::Int(3));
    }

    #[test]
    fn helper_check_hashable_rejects_list_object() {
        let mut heap = Heap::new();
        let list = heap.alloc_list(vec![], false).expect("list");
        let err = helper_check_hashable(&[Value::ObjectRef(list.id())], &heap).expect_err("reject");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
    }

    #[test]
    fn helper_write_barrier_invokes_hook_for_heap_ref_writes() {
        struct RecordingHook {
            called: bool,
        }
        impl WriteBarrierHook for RecordingHook {
            fn on_heap_mutation(&mut self, _: Option<&Value>, _: &Value) {
                self.called = true;
            }
        }
        let mut hook = RecordingHook { called: false };
        helper_write_barrier(
            &[Value::None, Value::ObjectRef(ObjectId::new(1))],
            &mut hook,
        )
        .expect("barrier");
        assert!(hook.called);
    }

    #[test]
    fn helper_write_barrier_skips_hook_for_immediate_writes() {
        struct RecordingHook {
            called: bool,
        }
        impl WriteBarrierHook for RecordingHook {
            fn on_heap_mutation(&mut self, _: Option<&Value>, _: &Value) {
                self.called = true;
            }
        }
        let mut hook = RecordingHook { called: false };
        helper_write_barrier(&[Value::Int(1), Value::Int(2)], &mut hook).expect("barrier");
        assert!(!hook.called);
    }
}