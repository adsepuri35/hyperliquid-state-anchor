use crate::types::{EngineError, EngineResult, EpochId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedTimeEpochPolicy {
    pub genesis_timestamp_ms: u64,
    pub epoch_duration_ms: u64,
}

impl FixedTimeEpochPolicy {
    pub fn new(genesis_timestamp_ms: u64, epoch_duration_ms: u64) -> EngineResult<Self> {
        if epoch_duration_ms <= 0 {
            return Err(EngineError::InvalidInput(
                "epoch_duration_ms must be greater than zero",
            ));
        }

        Ok(Self {
            genesis_timestamp_ms,
            epoch_duration_ms,
        })
    }

    pub fn epoch_id_for_timestamp_ms(&self, timestamp_ms: u64) -> EngineResult<EpochId> {
        if timestamp_ms < self.genesis_timestamp_ms {
            return Err(EngineError::InvalidInput(
                "timestamp earlier than epoch genesis",
            ));
        }

        let offset = timestamp_ms - self.genesis_timestamp_ms;
        Ok((offset / self.epoch_duration_ms) + 1)
    }

    pub fn next_epoch_boundary_ms(&self, epoch_id: EpochId) -> EngineResult<u64> {
        if epoch_id <= 0 {
            return Err(EngineError::InvalidInput("epoch_id must start at 1"));
        }

        let step = self
            .epoch_duration_ms
            .checked_mul(epoch_id)
            .ok_or(EngineError::InvalidInput("epoch boundary overflow"))?;
        self.genesis_timestamp_ms
            .checked_add(step)
            .ok_or(EngineError::InvalidInput("epoch boundary overflow"))
    }

    pub fn has_epoch_advanced(
        &self,
        previous_timestamp_ms: u64,
        new_timestamp_ms: u64,
    ) -> EngineResult<bool> {
        if new_timestamp_ms < previous_timestamp_ms {
            return Err(EngineError::InvalidInput("non-monotonic event timestamp"));
        }

        let previous_epoch = self.epoch_id_for_timestamp_ms(previous_timestamp_ms)?;
        let new_epoch = self.epoch_id_for_timestamp_ms(new_timestamp_ms)?;
        Ok(new_epoch > previous_epoch)
    }
}

#[cfg(test)]
mod tests {
    use super::FixedTimeEpochPolicy;

    #[test]
    fn maps_timestamps_to_epoch_ids() {
        let policy = FixedTimeEpochPolicy::new(1_000, 100).expect("policy");
        assert_eq!(policy.epoch_id_for_timestamp_ms(1_000).expect("epoch"), 1);
        assert_eq!(policy.epoch_id_for_timestamp_ms(1_099).expect("epoch"), 1);
        assert_eq!(policy.epoch_id_for_timestamp_ms(1_100).expect("epoch"), 2);
    }

    #[test]
    fn detects_epoch_advancement() {
        let policy = FixedTimeEpochPolicy::new(0, 1_000).expect("policy");
        assert!(!policy.has_epoch_advanced(100, 999).expect("check"));
        assert!(policy.has_epoch_advanced(999, 1_000).expect("check"));
    }
}
