//! Epoch module.
//!
//! Purpose:
//! - Scheduling and root freeze/finalization.
//!
//! Invariants:
//! - Epoch IDs are strictly monotonic.
//! - Freeze point is explicit and reproducible.
//!
//! Determinism:
//! - Identical event/time boundary policy yields same epoch segmentation.
//!
//! Failure modes:
//! - Attempted epoch rollback.
//! - Duplicate finalization.

use crate::types::{EngineError, EngineResult, EpochId};

pub mod policy;
pub mod manager;

pub use manager::{EpochAdvance, EpochManager};
pub use policy::FixedTimeEpochPolicy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedEpoch {
    pub epoch_id: EpochId,
    pub root_hash: [u8; 32],
}

pub fn validate_next_epoch(last: Option<EpochId>, next: EpochId) -> EngineResult<()> {
    if let Some(current) = last {
        if next <= current {
            return Err(EngineError::InvalidInput("non-monotonic epoch"));
        }
    }
    Ok(())
}
