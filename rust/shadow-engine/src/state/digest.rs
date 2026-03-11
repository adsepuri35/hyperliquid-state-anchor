#![allow(dead_code)]

use crate::types::SCHEMA_VERSION;

use super::ShadowState;

const FNV64_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV64_PRIME: u64 = 0x100000001b3;

fn fnv1a_extend(state: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *state ^= u64::from(*byte);
        *state = state.wrapping_mul(FNV64_PRIME);
    }
}

// Debug-only checksum for local determinism checks.
// Not suitable for commitments or proof verification.
pub(crate) fn debug_state_digest64(state: &ShadowState) -> u64 {
    let mut hash = FNV64_OFFSET_BASIS;

    fnv1a_extend(&mut hash, b"schema_v");
    fnv1a_extend(&mut hash, &SCHEMA_VERSION.to_le_bytes());

    fnv1a_extend(&mut hash, b"market");
    fnv1a_extend(&mut hash, &state.tracked_market_id.as_asset_id().to_le_bytes());

    fnv1a_extend(&mut hash, b"last_seq");
    match state.last_sequence {
        Some(sequence) => {
            fnv1a_extend(&mut hash, &[1]);
            fnv1a_extend(&mut hash, &sequence.to_le_bytes());
        }
        None => fnv1a_extend(&mut hash, &[0]),
    }

    fnv1a_extend(&mut hash, b"bids");
    for (price, quantity) in state.iter_bids_asc() {
        fnv1a_extend(&mut hash, &price.0.to_le_bytes());
        fnv1a_extend(&mut hash, &quantity.0.to_le_bytes());
    }

    fnv1a_extend(&mut hash, b"asks");
    for (price, quantity) in state.iter_asks_asc() {
        fnv1a_extend(&mut hash, &price.0.to_le_bytes());
        fnv1a_extend(&mut hash, &quantity.0.to_le_bytes());
    }

    fnv1a_extend(&mut hash, b"funding");
    match state.last_funding_rate {
        Some(rate) => {
            fnv1a_extend(&mut hash, &[1]);
            fnv1a_extend(&mut hash, &rate.0.to_le_bytes());
        }
        None => fnv1a_extend(&mut hash, &[0]),
    }

    fnv1a_extend(&mut hash, b"cum_trade_qty");
    fnv1a_extend(&mut hash, &state.cumulative_trade_quantity.0.to_le_bytes());

    hash
}
