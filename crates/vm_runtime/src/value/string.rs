//! String length and slice semantics.
//!
//! Spec: `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md` §10–§13

use vm_core::error::registry::RuntimeErrorCode;

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Unicode scalar count (not byte length).
#[must_use]
pub fn string_scalar_len(value: &str) -> usize {
    value.chars().count()
}

/// Slice a string by scalar indices `[start, end)`.
pub fn string_slice(value: &str, start: i64, end: i64) -> RuntimeResult<String> {
    if start < 0 || end < 0 {
        return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }
    if start > end {
        return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }

    let len = string_scalar_len(value) as i64;
    if start > len || end > len {
        return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }

    let start_u = usize::try_from(start).expect("non-negative start fits usize");
    let end_u = usize::try_from(end).expect("non-negative end fits usize");

    let result: String = value.chars().skip(start_u).take(end_u - start_u).collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_len_counts_scalars_not_bytes() {
        assert_eq!(string_scalar_len("a🙂b"), 3);
    }

    #[test]
    fn string_slice_preserves_scalar_sequence() {
        let slice = string_slice("a🙂b", 1, 2).expect("slice");
        assert_eq!(slice, "🙂");
    }

    #[test]
    fn negative_slice_bound_raises_index_error() {
        let err = string_slice("abc", -1, 2).unwrap_err();
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }

    #[test]
    fn out_of_range_slice_raises_index_error() {
        let err = string_slice("abc", 0, 4).unwrap_err();
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }

    #[test]
    fn start_greater_than_end_raises_index_error() {
        let err = string_slice("abc", 2, 1).unwrap_err();
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }
}