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

pub mod policy;
pub mod manager;
pub mod artifact;
pub mod finalize;
pub mod store;
pub mod orchestrator;

pub use artifact::FinalizedEpochRecord;
pub use finalize::finalize_epoch;
pub use manager::{EpochAdvance, EpochManager};
pub use orchestrator::EpochOrchestrator;
pub use policy::FixedTimeEpochPolicy;
pub use store::{EpochRecordStore, InMemoryEpochRecordStore};
