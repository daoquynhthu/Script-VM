//! Canonical Phase 3 helper registry entries.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md` §3

use vm_core::id::{CapabilityId, EffectId, RuntimeHelperId};

use super::schema::{
    HelperCallingConvention, HelperGcBehavior, HelperJitCallPolicy, HelperResultType,
    HelperSourceMappingPolicy, RuntimeHelperDescriptor, RuntimeHelperFamily,
    RuntimeHelperSignature,
};

struct Row {
    name: &'static str,
    family: RuntimeHelperFamily,
    result: HelperResultType,
    may_allocate: bool,
    may_raise: bool,
    may_unwind: bool,
    is_safepoint: bool,
    requires_roots_visible: bool,
    gc_behavior: HelperGcBehavior,
    jit_call_policy: HelperJitCallPolicy,
    capability: bool,
}

const ROWS: &[Row] = &[
    row("helper_alloc_object", RuntimeHelperFamily::Allocation, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::GcRuntimeCall, false),
    row("helper_write_barrier", RuntimeHelperFamily::WriteBarrier, HelperResultType::Unit, false, false, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_construct_error", RuntimeHelperFamily::Error, HelperResultType::Value, true, false, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_raise", RuntimeHelperFamily::Error, HelperResultType::VmControl, false, true, true, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_attach_suppressed", RuntimeHelperFamily::Error, HelperResultType::Unit, true, false, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_assert_fail", RuntimeHelperFamily::Error, HelperResultType::VmControl, true, true, true, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_check_type_contract", RuntimeHelperFamily::TypeCheck, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_check_callable", RuntimeHelperFamily::TypeCheck, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_check_hashable", RuntimeHelperFamily::TypeCheck, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_check_shape", RuntimeHelperFamily::TypeCheck, HelperResultType::Bool, false, false, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_numeric_unary", RuntimeHelperFamily::Numeric, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_numeric_binary", RuntimeHelperFamily::Numeric, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_compare", RuntimeHelperFamily::Numeric, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_get_attribute", RuntimeHelperFamily::Access, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_set_attribute", RuntimeHelperFamily::Access, HelperResultType::Unit, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_bind_method", RuntimeHelperFamily::Access, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_index_read", RuntimeHelperFamily::Access, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_index_write", RuntimeHelperFamily::Access, HelperResultType::Unit, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_slice_read", RuntimeHelperFamily::Access, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_membership", RuntimeHelperFamily::Access, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_construct_list", RuntimeHelperFamily::Construction, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::GcRuntimeCall, false),
    row("helper_construct_map", RuntimeHelperFamily::Construction, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::GcRuntimeCall, false),
    row("helper_construct_record", RuntimeHelperFamily::Construction, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::GcRuntimeCall, false),
    row("helper_construct_enum", RuntimeHelperFamily::Construction, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::GcRuntimeCall, false),
    row("helper_construct_function", RuntimeHelperFamily::Construction, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::GcRuntimeCall, false),
    row("helper_generic_call", RuntimeHelperFamily::Call, HelperResultType::VmControl, true, true, true, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_call_builtin", RuntimeHelperFamily::Call, HelperResultType::VmControl, true, true, true, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_check_arity", RuntimeHelperFamily::Call, HelperResultType::Unit, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::JitRuntimeCall, false),
    row("helper_match_pattern", RuntimeHelperFamily::Pattern, HelperResultType::HelperInternal, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InternalOnly, false),
    row("helper_perform_unwind", RuntimeHelperFamily::Unwind, HelperResultType::VmControl, true, true, true, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_register_defer", RuntimeHelperFamily::Resource, HelperResultType::Unit, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_execute_defer", RuntimeHelperFamily::Resource, HelperResultType::VmControl, true, true, true, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_register_resource", RuntimeHelperFamily::Resource, HelperResultType::Unit, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_close_resource", RuntimeHelperFamily::Resource, HelperResultType::VmControl, true, true, true, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_resolve_module", RuntimeHelperFamily::Module, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_initialize_module", RuntimeHelperFamily::Module, HelperResultType::VmControl, true, true, true, true, true, HelperGcBehavior::MayAllocateMayCollect, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_import_named", RuntimeHelperFamily::Module, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_import_module", RuntimeHelperFamily::Module, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_seal_exports", RuntimeHelperFamily::Module, HelperResultType::Unit, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_check_capability", RuntimeHelperFamily::Capability, HelperResultType::Unit, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::HostBoundaryCall, true),
    row("helper_enter_host_call", RuntimeHelperFamily::Capability, HelperResultType::Unit, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::HostBoundaryCall, true),
    row("helper_exit_host_call", RuntimeHelperFamily::Capability, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::HostBoundaryCall, true),
    row("helper_display", RuntimeHelperFamily::Display, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_string_concat", RuntimeHelperFamily::Display, HelperResultType::Value, true, true, false, true, true, HelperGcBehavior::MayAllocateNoCollection, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_load_cell", RuntimeHelperFamily::Access, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_store_cell", RuntimeHelperFamily::Access, HelperResultType::Unit, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_load_module_slot", RuntimeHelperFamily::Module, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
    row("helper_list_len", RuntimeHelperFamily::Access, HelperResultType::Value, false, true, false, false, false, HelperGcBehavior::NoAllocation, HelperJitCallPolicy::InterpreterOnly, false),
];

const fn row(
    name: &'static str,
    family: RuntimeHelperFamily,
    result: HelperResultType,
    may_allocate: bool,
    may_raise: bool,
    may_unwind: bool,
    is_safepoint: bool,
    requires_roots_visible: bool,
    gc_behavior: HelperGcBehavior,
    jit_call_policy: HelperJitCallPolicy,
    capability: bool,
) -> Row {
    Row {
        name,
        family,
        result,
        may_allocate,
        may_raise,
        may_unwind,
        is_safepoint,
        requires_roots_visible,
        gc_behavior,
        jit_call_policy,
        capability,
    }
}

fn source_policy(may_raise: bool) -> HelperSourceMappingPolicy {
    if may_raise {
        HelperSourceMappingPolicy::RequiredOnRaise
    } else {
        HelperSourceMappingPolicy::NotRequired
    }
}

fn calling_convention(policy: HelperJitCallPolicy) -> HelperCallingConvention {
    match policy {
        HelperJitCallPolicy::JitRuntimeCall => HelperCallingConvention::JitRuntimeCall,
        HelperJitCallPolicy::HostBoundaryCall => HelperCallingConvention::HostBoundaryCall,
        HelperJitCallPolicy::GcRuntimeCall => HelperCallingConvention::GcRuntimeCall,
        HelperJitCallPolicy::InternalOnly => HelperCallingConvention::InternalOnly,
        HelperJitCallPolicy::NotJitCallable | HelperJitCallPolicy::InterpreterOnly => {
            HelperCallingConvention::InterpreterDirect
        }
    }
}

/// Build the canonical helper descriptor list (48 entries; +list_len bootstrap).
#[must_use]
pub fn canonical_descriptors() -> Vec<RuntimeHelperDescriptor> {
    ROWS.iter()
        .enumerate()
        .map(|(index, row)| {
            let (required_capability, effect) = if row.capability {
                (
                    Some(CapabilityId::new(0)),
                    Some(EffectId::new(0)),
                )
            } else {
                (None, None)
            };
            RuntimeHelperDescriptor {
                helper_id: RuntimeHelperId::new(index as u32),
                name: row.name.to_string(),
                family: row.family,
                signature: RuntimeHelperSignature {
                    result: row.result,
                    calling_convention: calling_convention(row.jit_call_policy),
                },
                may_allocate: row.may_allocate,
                may_raise: row.may_raise,
                may_unwind: row.may_unwind,
                is_safepoint: row.is_safepoint,
                requires_roots_visible: row.requires_roots_visible,
                required_capability,
                effect,
                gc_behavior: row.gc_behavior,
                jit_call_policy: row.jit_call_policy,
                source_mapping_policy: source_policy(row.may_raise),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_registry_has_48_entries() {
        assert_eq!(canonical_descriptors().len(), 48);
    }

    #[test]
    fn canonical_helper_names_are_unique() {
        let descriptors = canonical_descriptors();
        let mut names = descriptors.iter().map(|d| d.name.as_str()).collect::<Vec<_>>();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), descriptors.len());
    }
}