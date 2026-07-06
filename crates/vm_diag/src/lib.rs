//! VM diagnostics and error reporting
//!
//! This crate provides diagnostic types and source-span support.

pub mod diagnostic;
pub mod source_span;

#[cfg(test)]
mod tests {
    use super::diagnostic::{Diagnostic, DiagnosticError, StackTrace};
    use super::source_span::{SourceFileId, SourceLocation, SourceSpanId};

    #[test]
    fn diagnostic_error_prevents_execution() {
        let err = DiagnosticError::new(Diagnostic::error("E0001", "invalid module"));
        assert!(err.prevents_execution());
    }

    #[test]
    fn stack_trace_attaches_source_oriented_frames() {
        let span = SourceSpanId::new(3);
        let trace = StackTrace::default().with_frame("main", Some(span));
        assert_eq!(trace.frames.len(), 1);
        assert_eq!(trace.frames[0].span, Some(span));
    }

    #[test]
    fn source_span_id_round_trip() {
        let file = SourceFileId::new(1);
        let loc = SourceLocation::new(4, 2);
        let span = super::source_span::DiagnosticSpan::new(file, loc, loc);
        assert_eq!(span.file, file);
    }
}