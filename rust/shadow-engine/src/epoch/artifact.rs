use crate::types::EpochId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedEpochRecord {
    pub epoch_id: EpochId,
    pub root_hash: [u8; 32],
    pub finalized_at_ms: u64,
    pub publisher: Option<String>,
}
