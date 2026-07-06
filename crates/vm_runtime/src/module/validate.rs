//! Module runtime validation helpers.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §4, §16; `PHASE-3-VALIDATION-MATRIX.md` P3-V7

use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::runtime_plan::schema::ExportPlan;

use crate::control::VmControl;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Reject top-level `return`/`break`/`continue` from module initialization.
pub fn reject_top_level_control(control: &VmControl) -> RuntimeResult<()> {
    match control {
        VmControl::Return(_) | VmControl::Break(_) | VmControl::Continue(_) => {
            Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidEirError,
                "top-level return/break/continue is invalid in module initialization",
            ))
        }
        VmControl::Normal(_) | VmControl::Raise(_) => Ok(()),
    }
}

/// Reject duplicate export names in a plan before instance construction.
pub fn validate_export_plan(plan: &ExportPlan) -> RuntimeResult<()> {
    let mut seen = std::collections::BTreeSet::new();
    for entry in &plan.exports {
        if !seen.insert(entry.exported_name.clone()) {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("duplicate export name: {}", entry.exported_name),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::{BindingId, SlotId};
    use vm_core::runtime_plan::schema::ExportPlanEntry;
    use vm_core::value::Value;
    use vm_diag::source_span::SourceSpanId;

    #[test]
    fn top_level_return_rejected() {
        let err = reject_top_level_control(&VmControl::Return(Some(Value::Int(1))))
            .expect_err("return");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn top_level_break_rejected() {
        let err = reject_top_level_control(&VmControl::Break(vm_core::id::ControlRegionId::new(0)))
            .expect_err("break");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn normal_control_allowed_at_module_top() {
        assert!(reject_top_level_control(&VmControl::Normal(None)).is_ok());
    }

    #[test]
    fn duplicate_export_in_plan_rejected() {
        let plan = ExportPlan {
            exports: vec![
                ExportPlanEntry {
                    exported_name: "dup".to_string(),
                    binding_id: BindingId::new(0),
                    slot_id: SlotId::new(0),
                    interface_type: None,
                    source_span: SourceSpanId::new(0),
                },
                ExportPlanEntry {
                    exported_name: "dup".to_string(),
                    binding_id: BindingId::new(1),
                    slot_id: SlotId::new(1),
                    interface_type: None,
                    source_span: SourceSpanId::new(1),
                },
            ],
            seal_after_init: true,
        };
        let err = validate_export_plan(&plan).expect_err("dup");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }
}