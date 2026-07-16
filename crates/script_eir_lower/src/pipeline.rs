//! Normative product pipeline: source → AnalyzedModule → SIR → validate → EIR.
//!
//! Spec path: UNIFIED-IMPLEMENTATION-GUIDANCE T-P1 → T-P2 → T-P3L (bootstrap).

use script_lower::{compile_to_sir, materialize_sir};
use script_sema::analyze_source;
use sir_validate::validate_ir_unit;

use crate::error::EirLowerError;
use crate::lower::lower_sir_to_eir;
use crate::program::EirProgram;

/// Full pipeline to EIR via SIR (not demo AST codegen).
pub fn compile_source_via_sir(source: &str, module_name: &str) -> Result<EirProgram, EirLowerError> {
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
    let unit = materialize_sir(&analyzed, module_name)
        .map_err(|e| EirLowerError::new(e.to_string()))?;
    let v = validate_ir_unit(&unit);
    if !v.is_valid() {
        return Err(EirLowerError::new(format!(
            "SIR invalid: {}",
            v.diagnostics
                .iter()
                .map(|d| d.message.clone())
                .collect::<Vec<_>>()
                .join("; ")
        )));
    }
    // control_regions required after WP-S02
    if unit.control_regions.is_empty() {
        return Err(EirLowerError::new("SIR missing control_regions"));
    }
    lower_sir_to_eir(&unit)
}

/// SIR already materialised + validated by caller.
pub fn eir_from_source_sir(source: &str, module_name: &str) -> Result<EirProgram, EirLowerError> {
    let unit = compile_to_sir(source, module_name).map_err(|e| EirLowerError::new(e.to_string()))?;
    let v = validate_ir_unit(&unit);
    if !v.is_valid() {
        return Err(EirLowerError::new(format!("{:?}", v.diagnostics)));
    }
    lower_sir_to_eir(&unit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::control::ControlState;
    use vm_core::value::Value;
    use vm_eval::Interpreter;

    fn run(src: &str) -> ControlState {
        let prog = compile_source_via_sir(src, "main").expect("pipeline");
        let mut interp = Interpreter::new();
        prog.install_callables(&mut interp.state_mut().callable_registry);
        interp.run_module(&prog.module, prog.entry)
    }

    #[test]
    fn fib_via_sir_eir_path() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

fib(10)
"#;
        let r = run(src);
        assert_eq!(r, ControlState::Return(Some(Value::Int(55))));
    }

    #[test]
    fn print_fib_via_sir() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(55))));
    }

    #[test]
    fn let_arith_via_sir() {
        assert_eq!(
            run("let x = 10\nlet y = 3\nx - y\n"),
            ControlState::Return(Some(Value::Int(7)))
        );
    }
}
