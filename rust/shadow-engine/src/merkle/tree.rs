use crate::types::EngineResult;

use super::hash::{empty_root_hash, hash_leaf, hash_node, leaf_hashes};
use super::{MerkleProof, MerkleRoot};

pub fn compute_root(canonical_leaves: &[Vec<u8>]) -> EngineResult<MerkleRoot> {
    if canonical_leaves.is_empty() {
        return Ok(MerkleRoot(empty_root_hash()));
    }

    let mut level = leaf_hashes(canonical_leaves);

    while level.len() > 1 {
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
        level = next;
    }

    Ok(MerkleRoot(level[0]))
}

pub fn verify_inclusion(root: &MerkleRoot, leaf: &[u8], proof: &MerkleProof) -> EngineResult<bool> {
    let mut computed = hash_leaf(leaf);
    let mut index = proof.leaf_index;

    for sibling in &proof.siblings {
        computed = if index % 2 == 0 {
            hash_node(&computed, sibling)
        } else {
            hash_node(sibling, &computed)
        };
        index /= 2;
    }

    if index != 0 {
        return Ok(false);
    }

    Ok(computed == root.0)
}
