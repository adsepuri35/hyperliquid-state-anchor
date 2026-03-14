use crate::merkle::{canonical_leaves_from_state, compute_root};
use crate::state::ShadowState;
use crate::types::{EngineError, EngineResult, EpochId};

use super::artifact::FinalizedEpochRecord;

pub fn finalize_epoch(
    epoch_id: EpochId,
    state: &ShadowState,
    finalized_at_ms: u64,
    publisher: Option<String>,
) -> EngineResult<FinalizedEpochRecord> {
    if epoch_id == 0 {
        return Err(EngineError::InvalidInput("epoch_id must start at 1"));
    }

    let leaves = canonical_leaves_from_state(state);
    let root = compute_root(&leaves)?;

    Ok(FinalizedEpochRecord {
        epoch_id,
        root_hash: root.0,
        finalized_at_ms,
        publisher,
    })
}

#[cfg(test)]
mod tests {
    use super::finalize_epoch;
    use crate::state::{MarketId, ShadowState};
    use crate::types::FixedI64;

    fn sample_state() -> ShadowState {
        let mut state = ShadowState::new(MarketId::new(0));
        state.last_sequence = Some(2);
        state.bids.insert(FixedI64(100_000), FixedI64(1));
        state.asks.insert(FixedI64(100_500), FixedI64(2));
        state
    }

    #[test]
    fn finalize_epoch_is_deterministic_for_same_input() {
        let state = sample_state();
        let a = finalize_epoch(1, &state, 1_000, None).expect("artifact");
        let b = finalize_epoch(1, &state, 1_000, None).expect("artifact");
        assert_eq!(a.root_hash, b.root_hash);
    }

    #[test]
    fn finalize_epoch_sets_artifact_fields() {
        let state = sample_state();
        let artifact = finalize_epoch(2, &state, 2_000, Some("relayer-a".to_string()))
            .expect("artifact");
        assert_eq!(artifact.epoch_id, 2);
        assert_eq!(artifact.finalized_at_ms, 2_000);
        assert_eq!(artifact.publisher.as_deref(), Some("relayer-a"));
    }

    #[test]
    fn finalize_epoch_rejects_zero_epoch_id() {
        let state = sample_state();
        assert!(finalize_epoch(0, &state, 1_000, None).is_err());
    }
}
