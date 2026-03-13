use sha3::{Digest, Keccak256};

const MERKLE_EMPTY_DOMAIN: &[u8] = b"HL_MERKLE_EMPTY_V1";
const MERKLE_LEAF_DOMAIN: &[u8] = b"HL_MERKLE_LEAF_V1";
const MERKLE_NODE_DOMAIN: &[u8] = b"HL_MERKLE_NODE_V1";

fn keccak256(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    for part in parts {
        hasher.update(part);
    }
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

pub(crate) fn empty_root_hash() -> [u8; 32] {
    keccak256(&[MERKLE_EMPTY_DOMAIN])
}

pub(crate) fn hash_leaf(leaf: &[u8]) -> [u8; 32] {
    keccak256(&[MERKLE_LEAF_DOMAIN, leaf])
}

pub(crate) fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    keccak256(&[MERKLE_NODE_DOMAIN, left, right])
}

pub(crate) fn leaf_hashes(canonical_leaves: &[Vec<u8>]) -> Vec<[u8; 32]> {
    canonical_leaves.iter().map(|leaf| hash_leaf(leaf)).collect()
}
