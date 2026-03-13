//! Merkle module.
//!
//! Purpose:
//! - Canonical hashing and proof generation/verification.
//!
//! Invariants:
//! - Stable leaf encoding.
//! - Deterministic leaf ordering.
//!
//! Determinism:
//! - Root must be reproducible for identical canonical input.
//!
//! Failure modes:
//! - Invalid proof shape.
//! - Unsupported hash/schema configuration.

use crate::types::{EngineError, EngineResult};
use sha3::{Digest, Keccak256};

pub mod leaves;

pub use leaves::canonical_leaves_from_state;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleRoot(pub [u8; 32]);

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

fn hash_leaf(leaf: &[u8]) -> [u8; 32] {
    keccak256(&[MERKLE_LEAF_DOMAIN, leaf])
}

fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    keccak256(&[MERKLE_NODE_DOMAIN, left, right])
}

pub fn compute_root(canonical_leaves: &[Vec<u8>]) -> EngineResult<MerkleRoot> {
    if canonical_leaves.is_empty() {
        return Ok(MerkleRoot(keccak256(&[MERKLE_EMPTY_DOMAIN])));
    }

    let mut level: Vec<[u8; 32]> = canonical_leaves
        .iter()
        .map(|leaf| hash_leaf(leaf))
        .collect();

    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i < level.len() {
            let left = level[i];
            let right = if i + 1 < level.len() {
                level[i + 1]
            } else {
                // Duplicate last node for odd levels.
                level[i]
            };
            next.push(hash_node(&left, &right));
            i += 2;
        }
        level = next;
    }

    Ok(MerkleRoot(level[0]))
}

pub fn verify_inclusion(
    _root: &MerkleRoot,
    _leaf: &[u8],
    _proof: &[Vec<u8>],
    _index: u64,
) -> EngineResult<bool> {
    Err(EngineError::NotImplemented("merkle proof verification"))
}
