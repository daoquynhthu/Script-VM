//! EIR program package for the interpreter (shared shape with demo codegen).

use vm_core::eir::schema::EirModule;
use vm_core::id::{EirFunctionId, ObjectId};
use vm_runtime::call::callable::{CallableTarget, UserFunctionTarget};

#[derive(Debug, Clone)]
pub struct EirProgram {
    pub module: EirModule,
    pub entry: EirFunctionId,
    pub callables: Vec<(ObjectId, CallableTarget)>,
}

impl EirProgram {
    pub fn install_callables(&self, registry: &mut vm_runtime::call::callable::CallableRegistry) {
        for (id, target) in &self.callables {
            registry.register(*id, target.clone());
        }
    }
}

#[must_use]
pub fn user_fn_target(object_id: ObjectId, function_id: u32, eir_id: u32) -> CallableTarget {
    CallableTarget::UserFunction(UserFunctionTarget {
        function_id: vm_core::id::FunctionId::new(function_id),
        module_id: vm_core::id::ModuleId::new(0),
        entry_eir_function: EirFunctionId::new(eir_id),
        return_type: None,
        object_id,
    })
}
