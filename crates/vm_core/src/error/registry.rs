//! Canonical runtime error code registry.
//!
//! Spec: `PHASE-3-RUNTIME-ERROR-REGISTRY.md` §2

/// Error layering per frozen registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorLayer {
    LanguageError,
    VmStructuralError,
    DiagnosticError,
    HostBoundaryError,
}

/// Language-visible runtime error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RuntimeErrorCode {
    NameError,
    UninitializedBindingError,
    TypeError,
    TypeContractError,
    PatternMatchError,
    ReadOnlyError,
    AssertionError,
    ArityError,
    IndexError,
    KeyError,
    FieldError,
    ImportError,
    ImportCycleError,
    DivisionByZeroError,
    NumericOverflowError,
    CapabilityError,
    StackOverflowError,
    ResourceStateError,
    InternalVMError,
}

/// VM structural failure codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VmStructuralErrorCode {
    InvalidEirError,
    InvalidRuntimePlanError,
    InvalidSlotError,
    InvalidObjectHandleError,
    InvalidHelperError,
    InvalidFrameStateError,
    InvalidRootMapError,
    InvalidStackMapError,
    InvalidDeoptError,
    BackendViolationError,
}

impl RuntimeErrorCode {
    /// All required language-visible codes from the frozen registry.
    pub const ALL: &'static [Self] = &[
        Self::NameError,
        Self::UninitializedBindingError,
        Self::TypeError,
        Self::TypeContractError,
        Self::PatternMatchError,
        Self::ReadOnlyError,
        Self::AssertionError,
        Self::ArityError,
        Self::IndexError,
        Self::KeyError,
        Self::FieldError,
        Self::ImportError,
        Self::ImportCycleError,
        Self::DivisionByZeroError,
        Self::NumericOverflowError,
        Self::CapabilityError,
        Self::StackOverflowError,
        Self::ResourceStateError,
        Self::InternalVMError,
    ];

    #[must_use]
    pub const fn layer(self) -> ErrorLayer {
        ErrorLayer::LanguageError
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::NameError => "NameError",
            Self::UninitializedBindingError => "UninitializedBindingError",
            Self::TypeError => "TypeError",
            Self::TypeContractError => "TypeContractError",
            Self::PatternMatchError => "PatternMatchError",
            Self::ReadOnlyError => "ReadOnlyError",
            Self::AssertionError => "AssertionError",
            Self::ArityError => "ArityError",
            Self::IndexError => "IndexError",
            Self::KeyError => "KeyError",
            Self::FieldError => "FieldError",
            Self::ImportError => "ImportError",
            Self::ImportCycleError => "ImportCycleError",
            Self::DivisionByZeroError => "DivisionByZeroError",
            Self::NumericOverflowError => "NumericOverflowError",
            Self::CapabilityError => "CapabilityError",
            Self::StackOverflowError => "StackOverflowError",
            Self::ResourceStateError => "ResourceStateError",
            Self::InternalVMError => "InternalVMError",
        }
    }

    #[must_use]
    pub const fn is_restricted(self) -> bool {
        matches!(self, Self::InternalVMError)
    }
}

impl VmStructuralErrorCode {
    /// All required structural codes from the frozen registry.
    pub const ALL: &'static [Self] = &[
        Self::InvalidEirError,
        Self::InvalidRuntimePlanError,
        Self::InvalidSlotError,
        Self::InvalidObjectHandleError,
        Self::InvalidHelperError,
        Self::InvalidFrameStateError,
        Self::InvalidRootMapError,
        Self::InvalidStackMapError,
        Self::InvalidDeoptError,
        Self::BackendViolationError,
    ];

    #[must_use]
    pub const fn layer(self) -> ErrorLayer {
        ErrorLayer::VmStructuralError
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::InvalidEirError => "InvalidEirError",
            Self::InvalidRuntimePlanError => "InvalidRuntimePlanError",
            Self::InvalidSlotError => "InvalidSlotError",
            Self::InvalidObjectHandleError => "InvalidObjectHandleError",
            Self::InvalidHelperError => "InvalidHelperError",
            Self::InvalidFrameStateError => "InvalidFrameStateError",
            Self::InvalidRootMapError => "InvalidRootMapError",
            Self::InvalidStackMapError => "InvalidStackMapError",
            Self::InvalidDeoptError => "InvalidDeoptError",
            Self::BackendViolationError => "BackendViolationError",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_all_required_language_codes() {
        assert_eq!(RuntimeErrorCode::ALL.len(), 19);
        for code in RuntimeErrorCode::ALL {
            assert_eq!(code.layer(), ErrorLayer::LanguageError);
            assert!(!code.name().is_empty());
        }
        assert!(RuntimeErrorCode::InternalVMError.is_restricted());
        assert!(!RuntimeErrorCode::TypeError.is_restricted());
    }

    #[test]
    fn registry_contains_all_required_structural_codes() {
        assert_eq!(VmStructuralErrorCode::ALL.len(), 10);
        for code in VmStructuralErrorCode::ALL {
            assert_eq!(code.layer(), ErrorLayer::VmStructuralError);
            assert!(!code.name().is_empty());
        }
    }
}