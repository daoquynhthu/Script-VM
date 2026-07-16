//! Errors for SIR → EIR lowering.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EirLowerError {
    pub message: String,
}

impl EirLowerError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for EirLowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SIR→EIR lower error: {}", self.message)
    }
}

impl std::error::Error for EirLowerError {}
