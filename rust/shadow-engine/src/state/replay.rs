use crate::types::EngineResult;

use super::{apply_event, ShadowState, StateEvent};

pub fn apply_events(state: &mut ShadowState, events: &[StateEvent]) -> EngineResult<()> {
    for event in events {
        apply_event(state, event)?;
    }
    Ok(())
}
