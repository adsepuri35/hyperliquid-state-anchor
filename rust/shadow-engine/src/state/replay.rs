use crate::types::EngineResult;

use super::{apply_event, ShadowState, StateEvent};

pub fn apply_events(state: &mut ShadowState, events: &[StateEvent]) -> EngineResult<()> {
    let mut prev_key = None;
    for event in events {
        let key = event.ordering_key();
        if let Some(prev) = prev_key {
            if key <= prev {
                return Err(crate::types::EngineError::InvalidInput(
                    "events are not strictly ordered by canonical key",
                ));
            }
        }
        prev_key = Some(key);
        apply_event(state, event)?;
    }
    Ok(())
}
