//! Bundle EIR program with a RuntimePlan derived from SIR metadata (WP-R03).
//!
//! Still uses the minimal plan fixture as structural scaffold, then fills:
//! - source_sir_digest from source
//! - module export_plan from SIR `exports`
//! - module init EIR id → 0 ($main)
//! - function_plans entry_eir for declared user functions when possible

use std::collections::BTreeMap;

use script_lower::{compile_to_sir, materialize_sir};
use script_sema::analyze_source;
use sir::IrUnit;
use sir_validate::validate_ir_unit;
use vm_core::digest::Digest;
use vm_core::id::{BindingId, EirFunctionId, FunctionId, ModuleId, SlotId};
use vm_core::runtime_plan::fixtures::minimal_valid_plan;
use vm_core::runtime_plan::schema::{ExportPlanEntry, FunctionPlan, RuntimePlan};
use vm_core::runtime_plan::validate_runtime_plan;
use vm_diag::source_span::SourceSpanId;

use crate::error::EirLowerError;
use crate::lower::lower_sir_to_eir;
use crate::program::EirProgram;

/// Executable unit: EIR + RuntimePlan with SIR-derived digests/exports.
#[derive(Debug, Clone)]
pub struct ExecutableUnit {
    pub eir: EirProgram,
    pub plan: RuntimePlan,
    pub sir: IrUnit,
    pub module_name: String,
    pub source_digest: String,
}

/// Compile via AnalyzedModule → SIR → validate → EIR, attach RuntimePlan shell.
pub fn compile_executable(source: &str, module_name: &str) -> Result<ExecutableUnit, EirLowerError> {
    let analyzed = analyze_source(source);
    if !analyzed.ok() {
        return Err(EirLowerError::new(format!(
            "frontend failed: {}",
            analyzed
                .diagnostics
                .iter()
                .map(|d| d.message.clone())
                .collect::<Vec<_>>()
                .join("; ")
        )));
    }
    let sir = materialize_sir(&analyzed, module_name)
        .map_err(|e| EirLowerError::new(e.to_string()))?;
    let v = validate_ir_unit(&sir);
    if !v.is_valid() {
        return Err(EirLowerError::new(format!(
            "SIR invalid: {:?}",
            v.diagnostics
        )));
    }
    let eir = lower_sir_to_eir(&sir)?;
    let plan = build_plan_from_sir(source, &sir, &eir)?;
    let dig = simple_digest(source);
    Ok(ExecutableUnit {
        eir,
        plan,
        sir,
        module_name: module_name.to_string(),
        source_digest: format!("fnv1a64:{dig:016x}"),
    })
}

fn build_plan_from_sir(
    source: &str,
    sir: &IrUnit,
    eir: &EirProgram,
) -> Result<RuntimePlan, EirLowerError> {
    let mut plan = minimal_valid_plan();
    let dig = simple_digest(source);
    plan.source_sir_digest = Digest(dig);

    // Module 0: init = main EIR 0; fill exports from SIR.
    if let Some(module) = plan.module_plans.modules.get_mut(&0) {
        module.initialization_function = EirFunctionId::new(0);
        module.export_plan.exports = sir
            .exports
            .iter()
            .enumerate()
            .map(|(i, name)| ExportPlanEntry {
                exported_name: name.clone(),
                binding_id: BindingId::new(i as u32),
                slot_id: SlotId::new(i as u32),
                interface_type: None,
                source_span: SourceSpanId::new(0),
            })
            .collect();
        module.export_plan.seal_after_init = true;
    }

    // Ensure a FunctionPlan exists for each EIR function id used by the program.
    let mut functions = BTreeMap::new();
    let template = plan
        .function_plans
        .functions
        .values()
        .next()
        .cloned()
        .ok_or_else(|| EirLowerError::new("minimal plan missing function template"))?;

    for (idx, f) in eir.module.functions.iter().enumerate() {
        let fid = FunctionId::new(idx as u32);
        let mut fp: FunctionPlan = template.clone();
        fp.function_id = fid;
        fp.module_id = ModuleId::new(0);
        fp.entry_eir_function = f.eir_function_id;
        functions.insert(fid.raw(), fp);
    }
    // Keep at least the template if empty (should not happen).
    if functions.is_empty() {
        functions = plan.function_plans.functions.clone();
    }
    plan.function_plans.functions = functions;

    validate_runtime_plan(&plan).map_err(|e| {
        EirLowerError::new(format!("RuntimePlan failed validation: {e:?}"))
    })?;
    Ok(plan)
}

fn simple_digest(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    // Digest 0 is rejected by validate_runtime_plan.
    if h == 0 {
        h = 1;
    }
    h
}

/// Convenience: SIR-only compile (no EIR) for tooling.
pub fn compile_sir_only(source: &str, module_name: &str) -> Result<IrUnit, EirLowerError> {
    compile_to_sir(source, module_name).map_err(|e| EirLowerError::new(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_export_and_functions() {
        let src = "export def id(x):\n    return x\nid(7)\n";
        let unit = compile_executable(src, "m").expect("exec");
        assert!(unit.sir.exports.iter().any(|e| e == "id"));
        assert!(!unit
            .plan
            .module_plans
            .modules
            .get(&0)
            .unwrap()
            .export_plan
            .exports
            .is_empty());
        assert!(unit.plan.function_plans.functions.len() >= 2);
        validate_runtime_plan(&unit.plan).expect("plan");
    }

    #[test]
    fn package_fib_validates_plan() {
        let src = "def f():\n    return 1\nf()\n";
        let unit = compile_executable(src, "m").expect("exec");
        assert!(!unit.source_digest.is_empty());
        validate_runtime_plan(&unit.plan).expect("plan still valid");
    }
}
