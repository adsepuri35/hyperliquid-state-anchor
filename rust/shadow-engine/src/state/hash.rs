use sha3::{Digest, Keccak256};

use crate::types::SCHEMA_VERSION;

use super::{serialize_state, ShadowState};

pub fn state_commitment_hash(state: &ShadowState) -> [u8; 32] {
    let serialized_state = serialize_state(state);

    let mut hasher = Keccak256::new();
    hasher.update(SCHEMA_VERSION.to_le_bytes());
    hasher.update(serialized_state);

    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}
