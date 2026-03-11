use std::collections::BTreeMap;

use crate::types::{FixedI64, Sequence};

use super::MarketId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowState {
    pub tracked_market_id: MarketId,
    pub last_sequence: Option<Sequence>,
    pub bids: BTreeMap<FixedI64, FixedI64>,
    pub asks: BTreeMap<FixedI64, FixedI64>,
    pub last_funding_rate: Option<FixedI64>,
    pub cumulative_trade_quantity: FixedI64,
}

impl ShadowState {
    pub fn new(tracked_market_id: MarketId) -> Self {
        Self {
            tracked_market_id,
            last_sequence: None,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_funding_rate: None,
            cumulative_trade_quantity: FixedI64(0),
        }
    }
}
