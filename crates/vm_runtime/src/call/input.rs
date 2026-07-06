//! Canonical call-site input representation.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §2

use vm_core::id::{CallSiteId, TypeId};
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;

/// Named argument with pre-evaluated value.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedArgumentValue {
    pub name: String,
    pub value: Value,
}

/// Inputs for entering a callee frame or helper boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct CallFrameInput {
    pub callee: Value,
    pub positional_args: Vec<Value>,
    pub named_args: Vec<NamedArgumentValue>,
    pub call_site_id: CallSiteId,
    pub source_span: SourceSpanId,
    pub expected_result_type: Option<TypeId>,
}

impl CallFrameInput {
    #[must_use]
    pub fn new(
        callee: Value,
        call_site_id: CallSiteId,
        source_span: SourceSpanId,
    ) -> Self {
        Self {
            callee,
            positional_args: Vec::new(),
            named_args: Vec::new(),
            call_site_id,
            source_span,
            expected_result_type: None,
        }
    }

    pub fn with_positional(mut self, args: Vec<Value>) -> Self {
        self.positional_args = args;
        self
    }

    pub fn with_named(mut self, args: Vec<NamedArgumentValue>) -> Self {
        self.named_args = args;
        self
    }
}