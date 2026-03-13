//! Merkle module.
//!
//! Purpose:
//! - Canonical hashing and proof generation/verification.
//!
//! Invariants:
//! - Stable leaf encoding.
//! - Deterministic leaf ordering.

mod hash;
pub mod leaves;
pub mod proof;
mod tree;
#[cfg(test)]
mod tests;

pub use leaves::canonical_leaves_from_state;
pub use proof::{generate_inclusion_proof, MerkleProof};
pub use tree::{compute_root, verify_inclusion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleRoot(pub [u8; 32]);
