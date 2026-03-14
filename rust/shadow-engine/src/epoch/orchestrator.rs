use crate::state::ShadowState;
use crate::types::{EngineError, EngineResult};

use super::artifact::FinalizedEpochRecord;
use super::finalize::finalize_epoch;
use super::manager::EpochManager;
use super::store::EpochRecordStore;

pub struct EpochOrchestrator<S: EpochRecordStore> {
    pub manager: EpochManager,
    pub store: S,
    pub publisher: Option<String>,
}

impl<S: EpochRecordStore> EpochOrchestrator<S> {
    pub fn new(manager: EpochManager, store: S, publisher: Option<String>) -> Self {
        Self {
            manager,
            store,
            publisher,
        }
    }

    pub fn observe_event_and_maybe_finalize(
        &mut self,
        event_timestamp_ms: u64,
        state: &ShadowState,
    ) -> EngineResult<Option<FinalizedEpochRecord>> {
        let advance = self.manager.observe_event_timestamp(event_timestamp_ms)?;
        let Some(advance) = advance else {
            return Ok(None);
        };

        if self.store.has_epoch(advance.previous_epoch_id) {
            return Err(EngineError::InvalidInput(
                "epoch record already exists for finalized epoch",
            ));
        }

        let artifact = finalize_epoch(
            advance.previous_epoch_id,
            state,
            event_timestamp_ms,
            self.publisher.clone(),
        )?;

        self.manager.mark_finalized(advance.previous_epoch_id)?;
        self.store.insert_finalized_epoch(artifact.clone())?;

        Ok(Some(artifact))
    }
}
