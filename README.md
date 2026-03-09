# Hyperliquid State Anchor

Deterministic infrastructure for replicating Hyperliquid state off-chain and anchoring periodic cryptographic commitments on HyperEVM.

## What This System Does

- Replays ordered Hyperliquid events in a deterministic Rust shadow engine.
- Computes canonical state commitments (Merkle roots).
- Publishes epoch roots to an on-chain Solidity commitment contract.
- Enables verifiable inclusion proofs against committed roots.

## What Is Stored On-Chain

Only minimal commitment data:

- `epoch_id`
- `root_hash`
- metadata (for example timestamp/publisher, depending on contract fields)

## What Is Not Stored On-Chain

- Full order book or trade history
- Full replicated exchange state
- Raw event payloads used for off-chain replay
- Large historical blobs

## High-Level Proof Flow

1. Ingestion collects ordered Hyperliquid events.
2. State engine deterministically applies transitions using fixed-point/integer math only.
3. Canonical serialization is produced and versioned (`schema_version`).
4. Merkle root is computed for the finalized epoch state.
5. Root is submitted to the commitment contract on HyperEVM.
6. A verifier checks an inclusion proof against the on-chain root.

## Repository Layout

- `rust/` - shadow engine and deterministic state logic
- `contracts/` - Solidity commitment contract (Foundry)
- `docs/` - architecture, invariants, and benchmark documentation
- `docker/`, `scripts/` - local tooling scaffolding

## Determinism Rules (Non-Negotiable)

- Same ordered inputs must produce identical hashes.
- No floating-point arithmetic in consensus-relevant logic.
- No unsorted map iteration in hash-critical paths.
- Hash preimage includes schema version:
  - `hash = H(schema_version || serialized_state)`

## Status

Current implementation is in scaffold stage. See `TODO.md` for phased build-out.
