use crate::state::ShadowState;

const LEAF_MAGIC: &[u8; 4] = b"HLLF";
const LEAF_VERSION: u16 = 1;

const LEAF_STATE_HEADER: u8 = 1;
const LEAF_BID_LEVEL: u8 = 2;
const LEAF_ASK_LEVEL: u8 = 3;

fn write_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

fn write_u16_le(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_u32_le(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_u64_le(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_i64_le(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_optional_u64_le(out: &mut Vec<u8>, value: Option<u64>) {
    match value {
        Some(v) => {
            write_u8(out, 1);
            write_u64_le(out, v);
        }
        None => write_u8(out, 0),
    }
}

fn write_optional_i64_le(out: &mut Vec<u8>, value: Option<i64>) {
    match value {
        Some(v) => {
            write_u8(out, 1);
            write_i64_le(out, v);
        }
        None => write_u8(out, 0),
    }
}

fn header_leaf(state: &ShadowState) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(LEAF_MAGIC);
    write_u16_le(&mut out, LEAF_VERSION);
    write_u8(&mut out, LEAF_STATE_HEADER);
    write_u32_le(&mut out, state.tracked_market_id.as_asset_id());
    write_optional_u64_le(&mut out, state.last_sequence);
    write_optional_i64_le(&mut out, state.last_funding_rate.map(|v| v.0));
    write_i64_le(&mut out, state.cumulative_trade_quantity.0);
    out
}

fn level_leaf(tag: u8, price: i64, quantity: i64) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(LEAF_MAGIC);
    write_u16_le(&mut out, LEAF_VERSION);
    write_u8(&mut out, tag);
    write_i64_le(&mut out, price);
    write_i64_le(&mut out, quantity);
    out
}

pub fn canonical_leaves_from_state(state: &ShadowState) -> Vec<Vec<u8>> {
    let mut leaves = Vec::new();

    leaves.push(header_leaf(state));

    for (price, quantity) in state.iter_bids_asc() {
        leaves.push(level_leaf(LEAF_BID_LEVEL, price.0, quantity.0));
    }

    for (price, quantity) in state.iter_asks_asc() {
        leaves.push(level_leaf(LEAF_ASK_LEVEL, price.0, quantity.0));
    }

    leaves
}
