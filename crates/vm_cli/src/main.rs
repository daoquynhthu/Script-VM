//! Script VM CLI — run source through the normative frontend→SIR→EIR path.
//!
//! Usage:
//!   script-vm run <file.script>
//!   script-vm eval "<source>"
//!   script-vm --help

use std::env;
use std::fs;
use std::process::ExitCode;

use script_eir_lower::compile_source_via_sir;
use vm_core::control::ControlState;
use vm_core::value::Value;
use vm_eval::Interpreter;

fn main() -> ExitCode {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return ExitCode::SUCCESS;
    }

    let cmd = args.remove(0);
    let source = match cmd.as_str() {
        "run" => {
            let Some(path) = args.first() else {
                eprintln!("error: missing file path\n");
                print_help();
                return ExitCode::from(2);
            };
            match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: cannot read {path}: {e}");
                    return ExitCode::from(1);
                }
            }
        }
        "eval" | "-e" => {
            let Some(code) = args.first() else {
                eprintln!("error: missing source string\n");
                print_help();
                return ExitCode::from(2);
            };
            code.clone()
        }
        other => {
            eprintln!("error: unknown command `{other}`\n");
            print_help();
            return ExitCode::from(2);
        }
    };

    let module_name = match cmd.as_str() {
        "run" => args
            .first()
            .map(|p| p.replace('\\', "/"))
            .unwrap_or_else(|| "main".into()),
        _ => "eval".into(),
    };

    let prog = match compile_source_via_sir(&source, &module_name) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("compile error: {e}");
            return ExitCode::from(1);
        }
    };

    let mut interp = Interpreter::new();
    prog.install_callables(&mut interp.state_mut().callable_registry);
    let state = interp.run_module(&prog.module, prog.entry);
    match state {
        ControlState::Return(v) => {
            println!("{}", format_value(v.as_ref()));
            ExitCode::SUCCESS
        }
        ControlState::Halt => {
            println!("(halt)");
            ExitCode::SUCCESS
        }
        ControlState::Raise(h) => {
            eprintln!("runtime raise: handle {}", h.raw());
            ExitCode::from(1)
        }
        ControlState::VmError(e) => {
            eprintln!("vm error: {e:?}");
            ExitCode::from(1)
        }
        other => {
            eprintln!("unexpected control state: {other:?}");
            ExitCode::from(1)
        }
    }
}

fn format_value(v: Option<&Value>) -> String {
    match v {
        None => "nil".into(),
        Some(Value::None) => "nil".into(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Int(n)) => n.to_string(),
        Some(Value::Float(f)) => f.to_string(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::ObjectRef(id)) => format!("object#{}", id.raw()),
        Some(Value::Error(h)) => format!("error#{}", h.raw()),
    }
}

fn print_help() {
    eprintln!(
        "Script VM (normative path: source → SIR → EIR → interpret)\n\n\
         Usage:\n\
           script-vm run <file>\n\
           script-vm eval \"<source>\"\n\
           script-vm -e \"<source>\"\n"
    );
}
