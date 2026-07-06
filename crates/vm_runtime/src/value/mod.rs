//! Runtime value semantics.
//!
//! Spec: `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md`, `PHASE-3-VM-RUNTIME-ROUND1.md` §2

pub mod key;
pub mod string;

pub use key::{hash_key, keys_equal, value_to_key, EnumKey, FloatKey, ValueKey};
pub use string::{string_scalar_len, string_slice};