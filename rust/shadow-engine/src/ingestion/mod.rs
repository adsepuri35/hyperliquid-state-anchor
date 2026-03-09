//! Ingestion module.
//!
//! Purpose:
//! - Fetch and normalize Hyperliquid input events.
//!
//! Invariants:
//! - No state transition logic here.
//! - Preserve source ordering metadata.
//!
//! Determinism:
//! - Output events must include explicit ordering keys.
//!
//! Failure modes:
//! - Transport errors.
//! - Missing or malformed event fields.

use crate::types::{EngineError, EngineResult, Sequence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestedEvent {
    pub sequence: Sequence,
    pub payload: Vec<u8>,
}

pub trait EventSource {
    fn fetch_next(&mut self) -> EngineResult<Option<IngestedEvent>>;
}

pub fn validate_event(event: &IngestedEvent) -> EngineResult<()> {
    if event.payload.is_empty() {
        return Err(EngineError::InvalidInput("empty event payload"));
    }
    Ok(())
}
