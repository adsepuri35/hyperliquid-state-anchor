//! Shared domain types used across module boundaries.
//!
//! Keep this file small and explicit. Consensus-relevant types should remain
//! integer/fixed-point based and versioned.

use core::fmt;

/// Monotonic identifier for finalized epochs.
pub type EpochId = u64;

/// Monotonic sequence number for input events.
pub type Sequence = u64;

/// Fixed-point numeric value wrapper.
///
/// Value semantics:
/// - Stored as an integer.
/// - Interpretation of scale is handled by the owning type/schema.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FixedI64(pub i64);

impl fmt::Debug for FixedI64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FixedI64({})", self.0)
    }
}

/// Canonical schema version to include in hash preimages.
pub const SCHEMA_VERSION: u16 = 1;

/// Crate-wide error type for scaffolded module APIs.
#[derive(Debug)]
pub enum EngineError {
    InvalidInput(&'static str),
    NotImplemented(&'static str),
}

pub type EngineResult<T> = Result<T, EngineError>;
