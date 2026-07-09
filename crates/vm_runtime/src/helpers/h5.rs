//! Milestone H5 module helper implementations.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.6,
//! `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` §8.9,
//! `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §2–§10, §12, §14
//!
//! Bootstrap: resolve/init/import/seal over existing `ModuleRuntime`.
//! Init body execution remains interpreter-owned (state machine advance only).

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::{InterfaceId, ModuleId, SlotId};
use vm_core::value::Value;

use crate::control::VmControl;
use crate::module::instance::ModuleInstance;
use crate::module::resolver::{HostModuleResolver, ModuleResolverRequest};
use crate::module::runtime::ModuleRuntime;
use crate::module::state::ModuleState;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

fn type_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::TypeError)
}

fn import_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::ImportError)
}

fn require_module_runtime(
    runtime: Option<&mut ModuleRuntime>,
) -> RuntimeResult<&mut ModuleRuntime> {
    runtime.ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "module helpers require ModuleRuntime in dispatch env",
        )
    })
}

fn module_id_arg(args: &[Value], index: usize) -> RuntimeResult<ModuleId> {
    match args.get(index) {
        Some(Value::Int(v)) if *v >= 0 => Ok(ModuleId::new(*v as u32)),
        _ => Err(type_error()),
    }
}

fn slot_id_arg(args: &[Value], index: usize) -> RuntimeResult<SlotId> {
    match args.get(index) {
        Some(Value::Int(v)) if *v >= 0 => Ok(SlotId::new(*v as u32)),
        _ => Err(type_error()),
    }
}

fn string_arg(args: &[Value], index: usize) -> RuntimeResult<String> {
    match args.get(index) {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(type_error()),
    }
}

/// Optional interface requirement for import-time compatibility checks.
/// Bootstrap: `args` trailing Int interface_id when present on named/module import.
fn optional_required_interface(args: &[Value], index: usize) -> Option<InterfaceId> {
    match args.get(index) {
        Some(Value::Int(v)) if *v >= 0 => Some(InterfaceId::new(*v as u32)),
        _ => None,
    }
}

/// Provider interface must match importer expectation when both sides declare one.
pub fn check_interface_compatibility(
    required: Option<InterfaceId>,
    provider: &ModuleInstance,
) -> RuntimeResult<()> {
    match (required, provider.interface_descriptor.interface_id) {
        (None, _) => Ok(()),
        (Some(want), Some(got)) if want == got => Ok(()),
        (Some(_), Some(_)) | (Some(_), None) => Err(import_error()),
    }
}

/// Bootstrap: `args[0]` String logical path → `Value::Int(module_id.raw())`.
/// Requires capability-gated resolve via `ModuleRuntime` + host resolver.
pub fn helper_resolve_module(
    args: &[Value],
    runtime: Option<&mut ModuleRuntime>,
    resolver: Option<&dyn HostModuleResolver>,
) -> RuntimeResult<Value> {
    let runtime = require_module_runtime(runtime)?;
    let resolver = resolver.ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "resolve_module requires host module resolver",
        )
    })?;
    let path = string_arg(args, 0)?;
    let module_id = runtime.resolve_module(
        resolver,
        &ModuleResolverRequest {
            logical_path: path,
        },
    )?;
    Ok(Value::Int(module_id.raw() as i64))
}

/// Bootstrap: `args[0]` Int module_id.
/// Advances Unloaded→Loading→Initializing (or Loading→Initializing).
/// Init EIR body is not executed here; returns `VmControl::Normal`.
pub fn helper_initialize_module(
    args: &[Value],
    runtime: Option<&mut ModuleRuntime>,
) -> RuntimeResult<VmControl> {
    let runtime = require_module_runtime(runtime)?;
    let module_id = module_id_arg(args, 0)?;
    let state = runtime
        .registry
        .get(module_id)
        .ok_or_else(|| {
            RuntimeFailure::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("unknown module: {}", module_id.raw()),
            )
        })?
        .state;

    match state {
        ModuleState::Unloaded => {
            runtime.begin_loading(module_id)?;
            runtime.begin_initializing(module_id)?;
        }
        ModuleState::Loading => {
            runtime.begin_initializing(module_id)?;
        }
        ModuleState::Initializing => {
            // Already initializing: idempotent no-op for bootstrap re-entry.
        }
        ModuleState::Initialized | ModuleState::Failed => {
            return Err(RuntimeFailure::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("cannot initialize module in state {state:?}"),
            ));
        }
    }
    Ok(VmControl::Normal(None))
}

/// Bootstrap: importer_id, provider_id, export_name String, local_slot Int,
/// optional required interface_id Int.
pub fn helper_import_named(
    args: &[Value],
    runtime: Option<&mut ModuleRuntime>,
) -> RuntimeResult<Value> {
    let runtime = require_module_runtime(runtime)?;
    let importer_id = module_id_arg(args, 0)?;
    let provider_id = module_id_arg(args, 1)?;
    let export_name = string_arg(args, 2)?;
    let local_slot = slot_id_arg(args, 3)?;
    let required_iface = optional_required_interface(args, 4);

    let provider = runtime.registry.get(provider_id).ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
            format!("unknown provider module: {}", provider_id.raw()),
        )
    })?;
    check_interface_compatibility(required_iface, provider)?;

    runtime.import_named(importer_id, provider_id, &export_name, local_slot)?;
    let importer = runtime.registry.get(importer_id).ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
            format!("unknown importer module: {}", importer_id.raw()),
        )
    })?;
    importer.module_slots.read(local_slot)
}

/// Bootstrap: importer_id, provider_id, local_slot, optional required interface_id.
pub fn helper_import_module(
    args: &[Value],
    runtime: Option<&mut ModuleRuntime>,
) -> RuntimeResult<Value> {
    let runtime = require_module_runtime(runtime)?;
    let importer_id = module_id_arg(args, 0)?;
    let provider_id = module_id_arg(args, 1)?;
    let local_slot = slot_id_arg(args, 2)?;
    let required_iface = optional_required_interface(args, 3);

    let provider = runtime.registry.get(provider_id).ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
            format!("unknown provider module: {}", provider_id.raw()),
        )
    })?;
    check_interface_compatibility(required_iface, provider)?;

    runtime.import_whole_module(importer_id, provider_id, local_slot)?;
    let importer = runtime.registry.get(importer_id).ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
            format!("unknown importer module: {}", importer_id.raw()),
        )
    })?;
    importer.module_slots.read(local_slot)
}

/// Bootstrap: `args[0]` Int module_id → seal export table (Unit).
pub fn helper_seal_exports(
    args: &[Value],
    runtime: Option<&mut ModuleRuntime>,
) -> RuntimeResult<()> {
    let runtime = require_module_runtime(runtime)?;
    let module_id = module_id_arg(args, 0)?;
    runtime.seal_exports(module_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heap::ObjRef;
    use crate::module::resolver::StubModuleResolver;
    use crate::module::state::ModuleState;
    use vm_core::id::{BindingId, CapabilityId, EirFunctionId, ObjectId};
    use vm_core::runtime_plan::schema::{ExportPlan, ExportPlanEntry, ModulePlan};
    use vm_diag::source_span::SourceSpanId;

    fn plan(id: u32, export: &str) -> ModulePlan {
        ModulePlan {
            module_id: ModuleId::new(id),
            module_slot_layout: vm_core::id::SlotLayoutId::new(0),
            initialization_function: EirFunctionId::new(0),
            import_plan: Default::default(),
            export_plan: ExportPlan {
                exports: vec![ExportPlanEntry {
                    exported_name: export.to_string(),
                    binding_id: BindingId::new(0),
                    slot_id: SlotId::new(0),
                    interface_type: None,
                    source_span: SourceSpanId::new(0),
                }],
                seal_after_init: true,
            },
            module_state_slot: SlotId::new(0),
            module_object_slot: SlotId::new(1),
            source_order: vec![],
            source_span: None,
        }
    }

    #[test]
    fn resolve_module_returns_mapped_id() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(1));
        rt.capabilities.grant(CapabilityId::new(1));
        let mut resolver = StubModuleResolver::default();
        resolver.map("pkg.a", ModuleId::new(9));
        let v = helper_resolve_module(
            &[Value::String("pkg.a".into())],
            Some(&mut rt),
            Some(&resolver),
        )
        .expect("resolve");
        assert_eq!(v, Value::Int(9));
    }

    #[test]
    fn resolve_without_capability_fails() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(1));
        let mut resolver = StubModuleResolver::default();
        resolver.map("pkg.a", ModuleId::new(9));
        let err = helper_resolve_module(
            &[Value::String("pkg.a".into())],
            Some(&mut rt),
            Some(&resolver),
        )
        .expect_err("cap");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::CapabilityError)
        );
    }

    #[test]
    fn initialize_advances_state_machine() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(0));
        rt.register_module(&plan(1, "e"), 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("reg");
        helper_initialize_module(&[Value::Int(1)], Some(&mut rt)).expect("init");
        assert_eq!(
            rt.registry.get(ModuleId::new(1)).expect("m").state,
            ModuleState::Initializing
        );
    }

    #[test]
    fn seal_exports_marks_table() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(0));
        rt.register_module(&plan(1, "e"), 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("reg");
        helper_seal_exports(&[Value::Int(1)], Some(&mut rt)).expect("seal");
        assert!(rt.registry.get(ModuleId::new(1)).expect("m").export_table.is_sealed());
    }

    #[test]
    fn import_named_cycle_raises() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(0));
        rt.register_module(&plan(1, "x"), 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("p");
        rt.register_module(&plan(2, "y"), 2, ObjRef::new(ObjectId::new(2), 0))
            .expect("i");
        rt.begin_loading(ModuleId::new(1)).expect("l");
        rt.begin_initializing(ModuleId::new(1)).expect("i1");
        // export x not marked initialized → cycle when reading during Initializing
        let err = helper_import_named(
            &[
                Value::Int(2),
                Value::Int(1),
                Value::String("x".into()),
                Value::Int(0),
            ],
            Some(&mut rt),
        )
        .expect_err("cycle");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::ImportCycleError)
        );
    }

    #[test]
    fn interface_mismatch_rejects_import() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(0));
        rt.register_module(&plan(1, "x"), 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("p");
        rt.register_module(&plan(2, "y"), 2, ObjRef::new(ObjectId::new(2), 0))
            .expect("i");
        rt.begin_loading(ModuleId::new(1)).expect("l");
        rt.begin_initializing(ModuleId::new(1)).expect("i");
        rt.registry
            .get_mut(ModuleId::new(1))
            .expect("p")
            .export_table
            .mark_initialized("x")
            .expect("init exp");
        rt.registry
            .get_mut(ModuleId::new(1))
            .expect("p")
            .module_slots
            .write(SlotId::new(0), Value::Int(7))
            .expect("w");
        rt.complete_initialization(ModuleId::new(1)).expect("done");
        // provider has no interface_id; required interface 1 → mismatch
        let err = helper_import_named(
            &[
                Value::Int(2),
                Value::Int(1),
                Value::String("x".into()),
                Value::Int(0),
                Value::Int(1),
            ],
            Some(&mut rt),
        )
        .expect_err("iface");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ImportError));
    }
}
