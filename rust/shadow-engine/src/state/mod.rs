//! Deterministic state transition module.

pub mod codec;
pub mod digest;
pub mod events;
pub mod hash;
pub mod replay;
pub mod snapshot;
pub mod transition;

use crate::types::{EngineError, EngineResult};
pub use codec::serialize_state;
pub use events::{AggressorSide, EventKind, MarketId, Side, StateEvent, TimestampMs};
pub use hash::state_commitment_hash;
pub use replay::apply_events;
pub use snapshot::ShadowState;
use transition::apply_event_kind;

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

    apply_event_kind(state, &event.kind)?;
    state.last_sequence = Some(event.sequence);
    Ok(())
}
