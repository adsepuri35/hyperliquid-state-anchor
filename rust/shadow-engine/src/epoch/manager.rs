use crate::types::{EngineError, EngineResult, EpochId};

use super::FixedTimeEpochPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpochAdvance {
    pub previous_epoch_id: EpochId,
    pub new_epoch_id: EpochId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochManager {
    pub policy: FixedTimeEpochPolicy,
    pub current_epoch_id: Option<EpochId>,
    pub last_finalized_epoch_id: Option<EpochId>,
    pub last_event_timestamp_ms: Option<u64>,
}

impl EpochManager {
    pub fn new(policy: FixedTimeEpochPolicy) -> Self {
        Self {
            policy,
            current_epoch_id: None,
            last_finalized_epoch_id: None,
            last_event_timestamp_ms: None,
        }
    }

    pub fn observe_event_timestamp(
        &mut self,
        timestamp_ms: u64,
    ) -> EngineResult<Option<EpochAdvance>> {
        if let Some(last_ts) = self.last_event_timestamp_ms {
            if timestamp_ms < last_ts {
                return Err(EngineError::InvalidInput("non-monotonic event timestamp"));
            }
        }

        let event_epoch_id = self.policy.epoch_id_for_timestamp_ms(timestamp_ms)?;
        self.last_event_timestamp_ms = Some(timestamp_ms);

        match self.current_epoch_id {
            None => {
                self.current_epoch_id = Some(event_epoch_id);
                Ok(None)
            }
            Some(current) if event_epoch_id == current => Ok(None),
            Some(current) if event_epoch_id > current => {
                self.current_epoch_id = Some(event_epoch_id);
                Ok(Some(EpochAdvance {
                    previous_epoch_id: current,
                    new_epoch_id: event_epoch_id,
                }))
            }
            Some(_) => Err(EngineError::InvalidInput("non-monotonic epoch progression")),
        }
    }

    pub fn mark_finalized(&mut self, epoch_id: EpochId) -> EngineResult<()> {
        if epoch_id == 0 {
            return Err(EngineError::InvalidInput("epoch_id must start at 1"));
        }

        if let Some(last) = self.last_finalized_epoch_id {
            if epoch_id <= last {
                return Err(EngineError::InvalidInput(
                    "epoch finalization must be strictly monotonic",
                ));
            }
            if epoch_id != last + 1 {
                return Err(EngineError::InvalidInput(
                    "cannot skip epoch during finalization",
                ));
            }
        }

        if let Some(current) = self.current_epoch_id {
            if epoch_id >= current {
                return Err(EngineError::InvalidInput(
                    "cannot finalize current or future epoch",
                ));
            }
        }

        self.last_finalized_epoch_id = Some(epoch_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::EpochManager;
    use crate::epoch::FixedTimeEpochPolicy;

    #[test]
    fn tracks_epoch_advancement_from_event_timestamps() {
        let policy = FixedTimeEpochPolicy::new(0, 1_000).expect("policy");
        let mut manager = EpochManager::new(policy);

        assert!(manager.observe_event_timestamp(100).expect("ok").is_none());
        assert!(manager.observe_event_timestamp(900).expect("ok").is_none());

        let adv = manager
            .observe_event_timestamp(1_000)
            .expect("ok")
            .expect("advance");
        assert_eq!(adv.previous_epoch_id, 1);
        assert_eq!(adv.new_epoch_id, 2);
    }

    #[test]
    fn rejects_duplicate_or_non_monotonic_finalization() {
        let policy = FixedTimeEpochPolicy::new(0, 1_000).expect("policy");
        let mut manager = EpochManager::new(policy);

        manager.observe_event_timestamp(2_100).expect("observe");
        manager.mark_finalized(1).expect("finalize 1");

        assert!(manager.mark_finalized(1).is_err());
        assert!(manager.mark_finalized(3).is_err());
        manager.mark_finalized(2).expect("finalize 2");
        assert!(manager.mark_finalized(3).is_err());
    }
}
