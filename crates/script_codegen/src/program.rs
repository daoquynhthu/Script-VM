//! Compiled bootstrap program: EIR module + callable registry seed.

use vm_core::eir::schema::EirModule;
use vm_core::id::{EirFunctionId, ObjectId};
use vm_runtime::call::callable::{CallableTarget, UserFunctionTarget};

/// Output of bootstrap codegen ready for `vm_eval::Interpreter`.
#[derive(Debug, Clone)]
pub struct CompiledProgram {
    pub module: EirModule,
    /// Entry function (`$main` module body).
    pub entry: EirFunctionId,
    /// Callable registrations for user functions (and bootstrap `print`).
    pub callables: Vec<(ObjectId, CallableTarget)>,
}

impl CompiledProgram {
    /// Install callables into an interpreter's registry.
    pub fn install_callables(&self, registry: &mut vm_runtime::call::callable::CallableRegistry) {
        for (id, target) in &self.callables {
            registry.register(*id, target.clone());
        }
    }
}

/// Helper to build a user-function target.
#[must_use]
pub fn user_fn_target(
    object_id: ObjectId,
    function_id: u32,
    eir_id: u32,
) -> CallableTarget {
    CallableTarget::UserFunction(UserFunctionTarget {
        function_id: vm_core::id::FunctionId::new(function_id),
        module_id: vm_core::id::ModuleId::new(0),
        entry_eir_function: EirFunctionId::new(eir_id),
        return_type: None,
        object_id,
    })
}
