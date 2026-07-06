//! Content digest for cache identity and plan binding.

/// Content digest for cache identity and SIR binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Digest(pub u64);