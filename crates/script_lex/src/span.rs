//! Source positions for Phase 1 frontend diagnostics.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §3 (source text model)

/// Zero-based UTF-8 byte offsets into a source buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[must_use]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn empty(at: u32) -> Self {
        Self {
            start: at,
            end: at,
        }
    }
}

/// One-based line/column (column counts Unicode scalar values from line start).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineCol {
    pub line: u32,
    pub column: u32,
}

impl LineCol {
    #[must_use]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Map a UTF-8 byte offset to 1-based line/column (SPEC-P1 diagnostics support).
///
/// Line terminators: LF, CR, CRLF (CRLF advances one line, not two).
#[must_use]
pub fn line_col_at(source: &str, byte_offset: u32) -> LineCol {
    let end = (byte_offset as usize).min(source.len());
    let mut line = 1u32;
    let mut col = 1u32;
    let bytes = source.as_bytes();
    let mut i = 0usize;
    while i < end {
        let b = bytes[i];
        if b == b'\r' {
            line = line.saturating_add(1);
            col = 1;
            i += 1;
            if i < end && bytes[i] == b'\n' {
                i += 1;
            }
            continue;
        }
        if b == b'\n' {
            line = line.saturating_add(1);
            col = 1;
            i += 1;
            continue;
        }
        // Advance one scalar value for column.
        let ch = source[i..].chars().next().unwrap_or('\0');
        i += ch.len_utf8();
        if i <= end {
            col = col.saturating_add(1);
        }
    }
    LineCol::new(line, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_basic() {
        let src = "ab\nc";
        assert_eq!(line_col_at(src, 0), LineCol::new(1, 1));
        assert_eq!(line_col_at(src, 2), LineCol::new(1, 3)); // at '\n'
        assert_eq!(line_col_at(src, 3), LineCol::new(2, 1));
    }

    #[test]
    fn line_col_crlf() {
        let src = "a\r\nb";
        assert_eq!(line_col_at(src, 3), LineCol::new(2, 1));
    }
}
