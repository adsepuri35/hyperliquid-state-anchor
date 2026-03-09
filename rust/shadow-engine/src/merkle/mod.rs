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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleRoot(pub [u8; 32]);

pub fn compute_root(_canonical_leaves: &[Vec<u8>]) -> EngineResult<MerkleRoot> {
    Err(EngineError::NotImplemented("merkle root computation"))
}

pub fn verify_inclusion(
    _root: &MerkleRoot,
    _leaf: &[u8],
    _proof: &[Vec<u8>],
    _index: u64,
) -> EngineResult<bool> {
    Err(EngineError::NotImplemented("merkle proof verification"))
}
