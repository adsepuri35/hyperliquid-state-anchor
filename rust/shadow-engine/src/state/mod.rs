//! Deterministic state transition module.

pub mod events;

use crate::types::{EngineError, EngineResult, Sequence};
pub use events::{AggressorSide, EventKind, MarketId, Side, StateEvent, TimestampMs};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowState {
    pub tracked_market_id: MarketId,
    pub last_sequence: Option<Sequence>,
}

impl ShadowState {
    pub fn new(tracked_market_id: MarketId) -> Self {
        Self {
            tracked_market_id,
            last_sequence: None,
        }
    }
}

pub fn apply_event(state: &mut ShadowState, event: &StateEvent) -> EngineResult<()> {
    if event.market_id != state.tracked_market_id {
        return Err(EngineError::InvalidInput(
            "event market does not match tracked market",
        ));
    }

    if let Some(last) = state.last_sequence {
        if event.sequence <= last {
            return Err(EngineError::InvalidInput("out-of-order sequence"));
        }
        if event.sequence != last + 1 {
            return Err(EngineError::InvalidInput("sequence gap"));
        }
    }

    state.last_sequence = Some(event.sequence);
    Ok(())
}
