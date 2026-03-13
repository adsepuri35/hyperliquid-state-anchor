use crate::types::{EngineError, EngineResult};

use super::hash::{hash_node, leaf_hashes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleProof {
    pub leaf_index: u64,
    pub siblings: Vec<[u8; 32]>,
}

pub fn generate_inclusion_proof(
    canonical_leaves: &[Vec<u8>],
    leaf_index: u64,
) -> EngineResult<MerkleProof> {
    if canonical_leaves.is_empty() {
        return Err(EngineError::InvalidInput(
            "cannot generate proof for empty leaf set",
        ));
    }

    let leaf_count = canonical_leaves.len() as u64;
    if leaf_index >= leaf_count {
        return Err(EngineError::InvalidInput("leaf_index out of range"));
    }

    let mut level = leaf_hashes(canonical_leaves);
    let mut index = leaf_index as usize;
    let mut siblings = Vec::new();

    while level.len() > 1 {
        let sibling = if index % 2 == 0 {
            if index + 1 < level.len() {
                level[index + 1]
            } else {
                level[index]
            }
        } else {
            level[index - 1]
        };
        siblings.push(sibling);

        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i < level.len() {
            let left = level[i];
            let right = if i + 1 < level.len() {
                level[i + 1]
            } else {
                level[i]
            };
            next.push(hash_node(&left, &right));
            i += 2;
        }

        index /= 2;
        level = next;
    }

    Ok(MerkleProof {
        leaf_index,
        siblings,
    })
}
