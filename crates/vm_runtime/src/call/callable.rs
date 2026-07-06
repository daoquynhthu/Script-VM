//! Callable classification and callability checks.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §4–§5

use std::collections::BTreeMap;

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::{
    CapabilityId, EirFunctionId, FunctionId, ModuleId, ObjectId, RecordId, TypeId,
};
use vm_core::value::Value;

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Canonical callable categories.
#[derive(Debug, Clone, PartialEq)]
pub enum CallableTarget {
    UserFunction(UserFunctionTarget),
    BuiltinFunction(BuiltinFunctionTarget),
    RecordConstructor(RecordConstructorTarget),
    EnumCaseConstructor(EnumCaseConstructorTarget),
    BoundMethod(BoundMethodTarget),
    HostFunction(HostFunctionTarget),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserFunctionTarget {
    pub function_id: FunctionId,
    pub module_id: ModuleId,
    pub entry_eir_function: EirFunctionId,
    pub return_type: Option<TypeId>,
    pub object_id: ObjectId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFunctionTarget {
    pub builtin_id: u32,
    pub arity: u32,
    pub required_capabilities: Vec<CapabilityId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordConstructorTarget {
    pub record_id: RecordId,
    pub arity: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumCaseConstructorTarget {
    pub enum_id: u32,
    pub case_index: u32,
    pub arity: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundMethodTarget {
    pub receiver_id: ObjectId,
    pub function_id: FunctionId,
    pub method_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HostFunctionTarget {
    pub host_function_id: u32,
    pub capability: Option<CapabilityId>,
}

/// Registry mapping heap object ids to callable metadata.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CallableRegistry {
    entries: BTreeMap<u32, CallableTarget>,
}

impl CallableRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, object_id: ObjectId, target: CallableTarget) {
        self.entries.insert(object_id.raw(), target);
    }

    pub fn resolve(&self, callee: &Value) -> RuntimeResult<CallableTarget> {
        match callee {
            Value::ObjectRef(id) => self
                .entries
                .get(&id.raw())
                .cloned()
                .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::TypeError)),
            _ => Err(RuntimeFailure::language(RuntimeErrorCode::TypeError)),
        }
    }
}

/// Step 4 of call protocol: callability check.
pub fn check_callable(callee: &Value, registry: &CallableRegistry) -> RuntimeResult<CallableTarget> {
    registry.resolve(callee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_callable_value_raises_type_error() {
        let registry = CallableRegistry::new();
        let err = check_callable(&Value::Int(1), &registry).expect_err("not callable");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::TypeError)
        ));
    }

    #[test]
    fn registered_function_resolves() {
        let mut registry = CallableRegistry::new();
        let object_id = ObjectId::new(5);
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
        let target = check_callable(&Value::ObjectRef(object_id), &registry).expect("callable");
        assert!(matches!(target, CallableTarget::UserFunction(_)));
    }
}