use super::ShadowState;

const CODEC_MAGIC: &[u8; 4] = b"SHST";
const CODEC_VERSION: u16 = 1;

const FIELD_TRACKED_MARKET_ID: u8 = 1;
const FIELD_LAST_SEQUENCE: u8 = 2;
const FIELD_BIDS: u8 = 3;
const FIELD_ASKS: u8 = 4;
const FIELD_LAST_FUNDING_RATE: u8 = 5;
const FIELD_CUMULATIVE_TRADE_QUANTITY: u8 = 6;

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

pub fn serialize_state(state: &ShadowState) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend_from_slice(CODEC_MAGIC);
    write_u16_le(&mut out, CODEC_VERSION);

    write_u8(&mut out, FIELD_TRACKED_MARKET_ID);
    write_u32_le(&mut out, state.tracked_market_id.as_asset_id());

    write_u8(&mut out, FIELD_LAST_SEQUENCE);
    write_optional_u64_le(&mut out, state.last_sequence);

    write_u8(&mut out, FIELD_BIDS);
    write_u32_le(&mut out, state.bids.len() as u32);
    for (price, quantity) in state.iter_bids_asc() {
        write_i64_le(&mut out, price.0);
        write_i64_le(&mut out, quantity.0);
    }

    write_u8(&mut out, FIELD_ASKS);
    write_u32_le(&mut out, state.asks.len() as u32);
    for (price, quantity) in state.iter_asks_asc() {
        write_i64_le(&mut out, price.0);
        write_i64_le(&mut out, quantity.0);
    }

    write_u8(&mut out, FIELD_LAST_FUNDING_RATE);
    write_optional_i64_le(&mut out, state.last_funding_rate.map(|v| v.0));

    write_u8(&mut out, FIELD_CUMULATIVE_TRADE_QUANTITY);
    write_i64_le(&mut out, state.cumulative_trade_quantity.0);

    out
}
