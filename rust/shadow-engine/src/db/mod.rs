//! Database module.
//!
//! Purpose:
//! - Persistence and replay-oriented storage access only.
//!
//! Invariants:
//! - Append raw events for deterministic replay.
//! - Persist finalized epoch roots and checkpoints.
//!
//! Determinism:
//! - Reads must return explicitly ordered event streams.
//!
//! Failure modes:
//! - Connection/migration errors.
//! - Inconsistent ordering at query boundary.

use crate::ingestion::IngestedEvent;
use crate::types::{EngineError, EngineResult, EpochId, Sequence};

pub trait EventStore {
    fn append_raw_event(&self, _event: &IngestedEvent) -> EngineResult<()> {
        Err(EngineError::NotImplemented("append_raw_event"))
    }

    fn read_events_ordered(&self, _from_sequence: Sequence) -> EngineResult<Vec<IngestedEvent>> {
        Err(EngineError::NotImplemented("read_events_ordered"))
    }
}

pub trait RootStore {
    fn save_epoch_root(&self, _epoch_id: EpochId, _root_hash: [u8; 32]) -> EngineResult<()> {
        Err(EngineError::NotImplemented("save_epoch_root"))
    }
}
