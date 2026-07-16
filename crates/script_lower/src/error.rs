//! Lowering errors.

use script_parse::ParseError;

#[derive(Debug)]
pub enum LowerError {
    Parse(ParseError),
    SemaFailed { errors: Vec<String> },
}

impl std::fmt::Display for LowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "{e}"),
            Self::SemaFailed { errors } => {
                write!(f, "semantic analysis failed: {}", errors.join("; "))
            }
        }
    }
}

impl std::error::Error for LowerError {}
