//! Source diagnostics path for interpreter execution.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §17, `PHASE-3-EIR-SCHEMA-CLOSURE.md`

use vm_core::eir::schema::OpMetadata;
use vm_diag::source_span::SourceSpanId;

use super::state::InterpreterState;

/// Record the most recent source span from op metadata.
pub fn record_op_span(state: &mut InterpreterState, metadata: &OpMetadata) {
    if let Some(span) = metadata.source_span {
        state.last_source_span = Some(span);
    }
}

/// Record span from block-level metadata.
pub fn record_block_span(state: &mut InterpreterState, span: Option<SourceSpanId>) {
    if let Some(s) = span {
        state.last_source_span = Some(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::eir::schema::OpMetadata;

    #[test]
    fn op_metadata_updates_last_source_span() {
        let mut state = InterpreterState::new(Default::default());
        record_op_span(
            &mut state,
            &OpMetadata {
                source_span: Some(SourceSpanId::new(7)),
                ..OpMetadata::default()
            },
        );
        assert_eq!(state.last_source_span, Some(SourceSpanId::new(7)));
    }
}