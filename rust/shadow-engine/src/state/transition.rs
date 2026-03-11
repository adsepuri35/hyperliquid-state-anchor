use std::collections::BTreeMap;

use crate::types::{EngineError, EngineResult, FixedI64};

use super::{AggressorSide, EventKind, ShadowState, Side};

fn side_levels_mut(state: &mut ShadowState, side: Side) -> &mut BTreeMap<FixedI64, FixedI64> {
    match side {
        Side::Bid => &mut state.bids,
        Side::Ask => &mut state.asks,
    }
}

fn apply_book_level_upsert(
    state: &mut ShadowState,
    side: Side,
    price: FixedI64,
    quantity: FixedI64,
) -> EngineResult<()> {
    if price.0 <= 0 {
        return Err(EngineError::InvalidInput("book level price must be positive"));
    }
    if quantity.0 <= 0 {
        return Err(EngineError::InvalidInput(
            "book level quantity must be positive",
        ));
    }

    side_levels_mut(state, side).insert(price, quantity);
    Ok(())
}

fn apply_book_level_delete(
    state: &mut ShadowState,
    side: Side,
    price: FixedI64,
) -> EngineResult<()> {
    if price.0 <= 0 {
        return Err(EngineError::InvalidInput(
            "book level delete price must be positive",
        ));
    }
    side_levels_mut(state, side).remove(&price);
    Ok(())
}

fn apply_trade(
    state: &mut ShadowState,
    _trade_id: u64,
    price: FixedI64,
    quantity: FixedI64,
    _aggressor: AggressorSide,
) -> EngineResult<()> {
    if price.0 <= 0 {
        return Err(EngineError::InvalidInput("trade price must be positive"));
    }
    if quantity.0 <= 0 {
        return Err(EngineError::InvalidInput("trade quantity must be positive"));
    }

    let next = state
        .cumulative_trade_quantity
        .0
        .checked_add(quantity.0)
        .ok_or(EngineError::InvalidInput("trade quantity overflow"))?;
    state.cumulative_trade_quantity = FixedI64(next);
    Ok(())
}

fn apply_funding_update(state: &mut ShadowState, funding_rate: FixedI64) {
    state.last_funding_rate = Some(funding_rate);
}

pub fn apply_event_kind(state: &mut ShadowState, kind: &EventKind) -> EngineResult<()> {
    match kind {
        EventKind::BookLevelUpsert {side, price, quantity} => apply_book_level_upsert(state, *side, *price, *quantity),
        EventKind::BookLevelDelete { side, price } => {
            apply_book_level_delete(state, *side, *price)
        }
        EventKind::Trade {trade_id, price, quantity, aggressor} => apply_trade(state, *trade_id, *price, *quantity, *aggressor),
        EventKind::FundingUpdate { funding_rate } => {
            apply_funding_update(state, *funding_rate);
            Ok(())
        }
    }
}
