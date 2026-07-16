//! Bundle EIR program with a bootstrap RuntimePlan shell (WP-R01).
//!
//! Full plan generation from SIR remains deferred — attaches a minimal valid plan
//! with digests derived from source text for cache-key scaffolding.

use vm_core::digest::Digest;
use vm_core::runtime_plan::fixtures::minimal_valid_plan;
use vm_core::runtime_plan::schema::RuntimePlan;
use vm_core::runtime_plan::validate_runtime_plan;

use crate::error::EirLowerError;
use crate::pipeline::compile_source_via_sir;
use crate::program::EirProgram;

/// Executable unit: EIR + RuntimePlan shell for future cache/host loading.
#[derive(Debug, Clone)]
pub struct ExecutableUnit {
    pub eir: EirProgram,
    pub plan: RuntimePlan,
    pub module_name: String,
    pub source_digest: String,
}

/// Compile via SIR path and attach a bootstrap RuntimePlan (validated).
pub fn compile_executable(source: &str, module_name: &str) -> Result<ExecutableUnit, EirLowerError> {
    let eir = compile_source_via_sir(source, module_name)?;
    let mut plan = minimal_valid_plan();
    let dig = simple_digest(source);
    plan.source_sir_digest = Digest(dig);
    validate_runtime_plan(&plan).map_err(|e| {
        EirLowerError::new(format!("RuntimePlan shell failed validation: {e:?}"))
    })?;
    Ok(ExecutableUnit {
        eir,
        plan,
        module_name: module_name.to_string(),
        source_digest: format!("fnv1a64:{dig:016x}"),
    })
}

fn simple_digest(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_fib_validates_plan() {
        let src = "def f():\n    return 1\nf()\n";
        let unit = compile_executable(src, "m").expect("exec");
        assert!(!unit.source_digest.is_empty());
        validate_runtime_plan(&unit.plan).expect("plan still valid");
    }
}
