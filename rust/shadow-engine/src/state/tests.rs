use sha3::{Digest, Keccak256};

use crate::state::{state_commitment_hash, serialize_state, MarketId, ShadowState};

fn hash_with_schema_prefix(state: &ShadowState, schema_version: u16) -> [u8; 32] {
    let serialized = serialize_state(state);
    let mut hasher = Keccak256::new();
    hasher.update(schema_version.to_le_bytes());
    hasher.update(serialized);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

#[test]
fn serialize_state_snapshot_empty_state() {
    let state = ShadowState::new(MarketId::new(7));
    let encoded = serialize_state(&state);

    let expected = vec![
        83, 72, 83, 84, // "SHST"
        1, 0, // codec version
        1, 7, 0, 0, 0, // tracked market id
        2, 0, // last sequence = None
        3, 0, 0, 0, 0, // bids len = 0
        4, 0, 0, 0, 0, // asks len = 0
        5, 0, // last funding = None
        6, 0, 0, 0, 0, 0, 0, 0, 0, // cumulative trade quantity = 0
    ];

    assert_eq!(encoded, expected);
}

#[test]
fn commitment_hash_is_deterministic_for_same_state() {
    let state = ShadowState::new(MarketId::new(3));
    let h1 = state_commitment_hash(&state);
    let h2 = state_commitment_hash(&state);

    assert_eq!(h1, h2);
}

#[test]
fn commitment_hash_changes_when_schema_version_changes() {
    let state = ShadowState::new(MarketId::new(3));
    let hash_v1 = hash_with_schema_prefix(&state, 1);
    let hash_v2 = hash_with_schema_prefix(&state, 2);

    assert_ne!(hash_v1, hash_v2);
}
