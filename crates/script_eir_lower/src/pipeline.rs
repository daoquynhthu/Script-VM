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
        // print uses helper_display → String return (also prints to stdout).
        assert_eq!(run(src), ControlState::Return(Some(Value::String("55".into()))));
    }

    #[test]
    fn let_arith_via_sir() {
        assert_eq!(
            run("let x = 10\nlet y = 3\nx - y\n"),
            ControlState::Return(Some(Value::Int(7)))
        );
    }

    #[test]
    fn list_and_for_unroll() {
        let src = "let s = 0\nfor x in [1, 2, 3]:\n    s = s + x\ns\n";
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(6))));
    }

    #[test]
    fn for_over_list_value() {
        let src = "let xs = [10, 20, 30]\nlet s = 0\nfor x in xs:\n    s = s + x\ns\n";
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(60))));
    }

    #[test]
    fn for_break_continue() {
        let src = r#"
let s = 0
for x in [1, 2, 3, 4]:
    if x == 2:
        continue
    if x == 4:
        break
    s = s + x
s
"#;
        // 1 + 3 = 4 (skip 2, break before 4)
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(4))));
    }

    #[test]
    fn short_circuit_and() {
        // false and <unevaluated non-bool would fail if evaluated> — right side true
        let src = "false and true\n";
        assert_eq!(run(src), ControlState::Return(Some(Value::Bool(false))));
        let src2 = "true and false\n";
        assert_eq!(run(src2), ControlState::Return(Some(Value::Bool(false))));
        let src3 = "true and true\n";
        assert_eq!(run(src3), ControlState::Return(Some(Value::Bool(true))));
    }

    #[test]
    fn while_break() {
        let src = "let i = 0\nwhile true:\n    i = i + 1\n    if i > 2:\n        break\ni\n";
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(3))));
    }

    #[test]
    fn while_continue() {
        // sum odd steps: 1+3 = 4 (skip even via continue)
        let src = r#"
let i = 0
let s = 0
while i < 4:
    i = i + 1
    if i == 2:
        continue
    s = s + i
s
"#;
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(8))));
        // i=1 s=1; i=2 continue; i=3 s=4; i=4 s=8
    }

    #[test]
    fn map_literal_constructs() {
        let r = run("let m = {\"a\": 1, \"b\": 2}\nm\n");
        assert!(matches!(r, ControlState::Return(Some(Value::ObjectRef(_)))));
        let r2 = run("let m = {1: 10, 2: 20}\nm\n");
        assert!(matches!(r2, ControlState::Return(Some(Value::ObjectRef(_)))));
    }

    #[test]
    fn raise_string() {
        let r = run("raise \"boom\"\n");
        assert!(matches!(r, ControlState::Raise(_)));
    }

    #[test]
    fn assert_false_raises() {
        let r = run("assert false\n");
        assert!(matches!(r, ControlState::Raise(_)));
    }

    #[test]
    fn list_index_read_write() {
        let src = "let xs = [10, 20, 30]\nxs[1] = 99\nxs[1]\n";
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(99))));
    }

    #[test]
    fn map_index_read() {
        let src = "let m = {\"k\": 7}\nm[\"k\"]\n";
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(7))));
    }

    #[test]
    fn attr_as_map_field() {
        let src = "let o = {\"x\": 1}\no.x = 5\no.x\n";
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(5))));
    }

    #[test]
    fn try_finally_runs() {
        let src = r#"
let x = 0
try:
    x = 1
finally:
    x = x + 10
x
"#;
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(11))));
    }

    #[test]
    fn try_catch_handles_raise() {
        let src = r#"
let x = 0
try:
    raise "boom"
catch e:
    x = 42
x
"#;
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(42))));
    }

    #[test]
    fn try_finally_after_return() {
        // finally should run; still return 7
        let src = r#"
def f():
    try:
        return 7
    finally:
        print(1)
f()
"#;
        // print returns display string of 1 as last expr of finally — wait, finally doesn't replace return
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(7))));
    }

    #[test]
    fn record_construct_and_field_read() {
        let src = r#"
record Point:
    field x
    field y
let p = Point(x = 3, y = 4)
p.x + p.y
"#;
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(7))));
    }

    #[test]
    fn record_mutable_field_write() {
        let src = r#"
record Counter:
    mutable field value
let c = Counter(value = 1)
c.value = 10
c.value
"#;
        assert_eq!(run(src), ControlState::Return(Some(Value::Int(10))));
    }
}
