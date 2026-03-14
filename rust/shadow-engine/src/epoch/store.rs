use std::collections::BTreeMap;

use crate::types::{EngineError, EngineResult, EpochId};

use super::artifact::FinalizedEpochRecord;

pub trait EpochRecordStore {
    fn has_epoch(&self, epoch_id: EpochId) -> bool;
    fn insert_finalized_epoch(&mut self, record: FinalizedEpochRecord) -> EngineResult<()>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryEpochRecordStore {
    records: BTreeMap<EpochId, FinalizedEpochRecord>,
}

impl InMemoryEpochRecordStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, epoch_id: EpochId) -> Option<&FinalizedEpochRecord> {
        self.records.get(&epoch_id)
    }
}

impl EpochRecordStore for InMemoryEpochRecordStore {
    fn has_epoch(&self, epoch_id: EpochId) -> bool {
        self.records.contains_key(&epoch_id)
    }

    fn insert_finalized_epoch(&mut self, record: FinalizedEpochRecord) -> EngineResult<()> {
        if self.records.contains_key(&record.epoch_id) {
            return Err(EngineError::InvalidInput(
                "duplicate epoch record insertion",
            ));
        }
        self.records.insert(record.epoch_id, record);
        Ok(())
    }
}
