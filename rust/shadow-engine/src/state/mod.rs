//! State transition module.
//!
//! Purpose:
//! - Apply deterministic state transitions from ordered events.
//!
//! Invariants:
//! - Integer/fixed-point arithmetic only.
//! - Enforce monotonic sequence progression.
//!
//! Determinism:
//! - Same ordered events must produce identical state output.
//!
//! Failure modes:
//! - Sequence gaps/out-of-order events.
//! - Invalid domain updates.

use crate::types::{EngineError, EngineResult, Sequence};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ShadowState {
    pub last_sequence: Option<Sequence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateEvent {
    pub sequence: Sequence,
}

pub fn apply_event(state: &mut ShadowState, event: &StateEvent) -> EngineResult<()> {
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
