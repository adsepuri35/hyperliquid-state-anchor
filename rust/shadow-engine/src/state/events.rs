//! Canonical event model for single-market deterministic replay.

use crate::types::{FixedI64, Sequence};

pub type TimestampMs = u64;
pub type AssetId = u32;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MarketId(AssetId);

impl MarketId {
    pub fn new(asset_id: AssetId) -> Self {
        Self(asset_id)
    }

    pub fn as_asset_id(&self) -> AssetId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AggressorSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    BookLevelUpsert {
        side: Side,
        price: FixedI64,
        quantity: FixedI64,
    },
    BookLevelDelete {
        side: Side,
        price: FixedI64,
    },
    Trade {
        trade_id: u64,
        price: FixedI64,
        quantity: FixedI64,
        aggressor: AggressorSide,
    },
    FundingUpdate {
        funding_rate: FixedI64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateEvent {
    pub sequence: Sequence,
    pub timestamp_ms: TimestampMs,
    pub market_id: MarketId,
    pub kind: EventKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderingKey {
    pub sequence: Sequence,
    pub timestamp_ms: TimestampMs,
}

impl StateEvent {
    pub fn ordering_key(&self) -> OrderingKey {
        OrderingKey {
            sequence: self.sequence,
            timestamp_ms: self.timestamp_ms,
        }
    }
}
