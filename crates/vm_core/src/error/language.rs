//! Language-level error objects.
//!
//! Spec: `PHASE-3-RUNTIME-ERROR-REGISTRY.md` §1.1, §3, §5

use std::collections::BTreeMap;

use vm_diag::diagnostic::StackTrace;
use vm_diag::source_span::SourceSpanId;

use crate::error::registry::RuntimeErrorCode;
use crate::id::ErrorHandle;
use crate::value::Value;

/// Language-level error object.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorObj {
    pub error_code: RuntimeErrorCode,
    pub message: String,
    pub source_span: Option<SourceSpanId>,
    pub stack_trace: Option<StackTrace>,
    pub details: Option<BTreeMap<String, Value>>,
    pub cause: Option<ErrorHandle>,
    pub suppressed: Vec<ErrorHandle>,
}

impl ErrorObj {
    #[must_use]
    pub fn new(code: RuntimeErrorCode, message: impl Into<String>) -> Self {
        Self {
            error_code: code,
            message: message.into(),
            source_span: None,
            stack_trace: None,
            details: None,
            cause: None,
            suppressed: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_source_span(mut self, span: SourceSpanId) -> Self {
        self.source_span = Some(span);
        self
    }

    #[must_use]
    pub fn with_stack_trace(mut self, trace: StackTrace) -> Self {
        self.stack_trace = Some(trace);
        self
    }

    #[must_use]
    pub fn with_cause(mut self, cause: ErrorHandle) -> Self {
        self.cause = Some(cause);
        self
    }

    pub fn attach_suppressed(&mut self, handle: ErrorHandle) {
        self.suppressed.push(handle);
    }
}

/// In-memory store for allocated language error objects.
#[derive(Debug, Default)]
pub struct ErrorStore {
    errors: Vec<ErrorObj>,
}

impl ErrorStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self, error: ErrorObj) -> ErrorHandle {
        let handle = ErrorHandle::new(self.errors.len() as u32);
        self.errors.push(error);
        handle
    }

    #[must_use]
    pub fn get(&self, handle: ErrorHandle) -> Option<&ErrorObj> {
        self.errors.get(handle.raw() as usize)
    }

    #[must_use]
    pub fn get_mut(&mut self, handle: ErrorHandle) -> Option<&mut ErrorObj> {
        self.errors.get_mut(handle.raw() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_diag::source_span::SourceSpanId;

    #[test]
    fn cleanup_raise_preserves_primary_and_suppresses_cleanup() {
        let mut store = ErrorStore::new();
        let primary = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "missing"));
        let cleanup = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "during cleanup"));
        store
            .get_mut(primary)
            .expect("primary")
            .attach_suppressed(cleanup);
        let stored = store.get(primary).expect("stored");
        assert_eq!(stored.error_code, RuntimeErrorCode::KeyError);
        assert_eq!(stored.suppressed, vec![cleanup]);
    }

    #[test]
    fn error_obj_supports_required_fields() {
        let span = SourceSpanId::new(7);
        let mut store = ErrorStore::new();
        let primary = store.allocate(
            ErrorObj::new(RuntimeErrorCode::NameError, "name not found").with_source_span(span),
        );
        let cleanup = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "cleanup"));
        let primary_obj = store.get_mut(primary).expect("primary");
        primary_obj.attach_suppressed(cleanup);

        let stored = store.get(primary).expect("stored");
        assert_eq!(stored.error_code, RuntimeErrorCode::NameError);
        assert_eq!(stored.source_span, Some(span));
        assert_eq!(stored.suppressed, vec![cleanup]);
    }
}