use crate::merkle::{
    canonical_leaves_from_state, compute_root, generate_inclusion_proof, verify_inclusion,
};
use crate::state::{MarketId, ShadowState};
use crate::types::FixedI64;

fn sample_state() -> ShadowState {
    let mut state = ShadowState::new(MarketId::new(0));
    state.last_sequence = Some(3);
    state.bids.insert(FixedI64(100_000), FixedI64(2));
    state.bids.insert(FixedI64(99_500), FixedI64(1));
    state.asks.insert(FixedI64(100_500), FixedI64(3));
    state.last_funding_rate = Some(FixedI64(10));
    state.cumulative_trade_quantity = FixedI64(7);
    state
}

#[test]
fn merkle_root_is_reproducible() {
    let leaves = canonical_leaves_from_state(&sample_state());
    let r1 = compute_root(&leaves).expect("root");
    let r2 = compute_root(&leaves).expect("root");
    assert_eq!(r1, r2);
}

#[test]
fn inclusion_proof_verifies_for_valid_leaf() {
    let leaves = canonical_leaves_from_state(&sample_state());
    let root = compute_root(&leaves).expect("root");
    let proof = generate_inclusion_proof(&leaves, 1).expect("proof");
    let ok = verify_inclusion(&root, &leaves[1], &proof).expect("verify");
    assert!(ok);
}

#[test]
fn inclusion_proof_rejects_tampered_leaf_and_sibling() {
    let leaves = canonical_leaves_from_state(&sample_state());
    let root = compute_root(&leaves).expect("root");
    let proof = generate_inclusion_proof(&leaves, 1).expect("proof");

    let mut bad_leaf = leaves[1].clone();
    bad_leaf[0] ^= 0x01;
    let ok_leaf = verify_inclusion(&root, &bad_leaf, &proof).expect("verify");
    assert!(!ok_leaf);

    let mut bad_proof = proof.clone();
    bad_proof.siblings[0][0] ^= 0x01;
    let ok_sibling = verify_inclusion(&root, &leaves[1], &bad_proof).expect("verify");
    assert!(!ok_sibling);
}

#[test]
fn inclusion_proof_rejects_tampered_index() {
    let leaves = canonical_leaves_from_state(&sample_state());
    let root = compute_root(&leaves).expect("root");
    let proof = generate_inclusion_proof(&leaves, 1).expect("proof");

    let mut bad_proof = proof.clone();
    bad_proof.leaf_index = 0;

    let ok = verify_inclusion(&root, &leaves[1], &bad_proof).expect("verify");
    assert!(!ok);
}
