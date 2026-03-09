//! Publisher module.
//!
//! Purpose:
//! - Submit finalized roots to the on-chain commitment contract.
//!
//! Invariants:
//! - No state transition logic in this module.
//! - Idempotent publish semantics by epoch.
//!
//! Determinism:
//! - Off-chain state/hash must already be finalized before submission.
//!
//! Failure modes:
//! - RPC submission failures.
//! - Nonce/replay/contract rejections.

use crate::epoch::FinalizedEpoch;
use crate::types::{EngineError, EngineResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishReceipt {
    pub tx_hash: [u8; 32],
}

pub trait RootPublisher {
    fn publish(&self, epoch: &FinalizedEpoch) -> EngineResult<PublishReceipt>;
}

pub fn publish_once<P: RootPublisher>(
    publisher: &P,
    epoch: &FinalizedEpoch,
) -> EngineResult<PublishReceipt> {
    if epoch.epoch_id == 0 {
        return Err(EngineError::InvalidInput("epoch_id must start from 1"));
    }
    publisher.publish(epoch)
}
