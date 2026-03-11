# TODO - Hyperliquid Verifiable State Commitment System

This checklist tracks implementation progress against `AGENTS.md`.

## Current Status

- [x] Repository scaffold created
- [x] Rust workspace initialized (`rust/`)
- [x] Placeholder Rust binary exists (`rust/shadow-engine/src/main.rs`)
- [ ] Deterministic state replication implemented
- [ ] Merkle commitments implemented
- [ ] On-chain commitment contract implemented
- [ ] End-to-end root publication flow implemented
- [ ] Determinism/security/performance test coverage implemented

## Phase 0 - Project Setup and Structure

- [x] Create module layout in Rust crate:
  - [x] `ingestion/` (HL data fetching only)
  - [x] `state/` (deterministic state transitions only)
  - [x] `merkle/` (hashing + proof generation only)
  - [x] `epoch/` (scheduling + root freeze logic)
  - [x] `publisher/` (on-chain submission only)
  - [x] `db/` (persistence only)
- [x] Define shared crate-wide types for:
  - [x] Event sequence identifiers
  - [x] Fixed-point numeric wrappers
  - [x] Epoch/root metadata
  - [x] Error enums (no `unwrap()` in production paths)
- [x] Expand `README.md` with:
  - [x] What is stored on-chain
  - [x] What is not stored on-chain
  - [x] High-level proof flow
- [x] Add `/docs/` directory scaffold

## Phase 1 - Deterministic State Engine (Single-Market MVP)

- [x] Define canonical event model for single market:
  - [x] Book/trade/update event schema
  - [x] Deterministic ordering fields
  - [x] Sequence gap detection fields
- [x] Implement deterministic transition logic in `state/`:
  - [x] Pure transition function(s)
  - [x] Explicit integer/fixed-point math only
  - [x] Reject/handle out-of-order events
  - [x] Enforce sequence monotonicity
- [x] Ensure deterministic map/set handling:
  - [x] Use ordered data structures where needed
  - [x] Sort keys before serialization/hash input
- [x] Add replay runner:
  - [x] Apply ordered raw events -> resulting state
  - [x] Deterministic state hash output for same input

## Phase 2 - Canonical Serialization + Schema Versioning

- [x] Define explicit canonical serialization format (no implicit serde defaults for hashing)
- [x] Introduce `schema_version` constant and wire it into hash preimage:
  - [x] `hash = H(schema_version || serialized_state)`
- [x] Add versioning policy doc in `/docs/`:
  - [x] When version bump is required
  - [x] Backward compatibility assumptions
- [x] Add tests:
  - [x] Serialization stability snapshot tests
  - [x] Hash changes when schema version increments
  - [x] Hash stays identical across independent runs for same schema/input

## Phase 3 - Merkle Commitments and Proofs

- [ ] Implement `merkle/` root construction over canonicalized leaves
- [ ] Define leaf encoding format and domain separators
- [ ] Implement inclusion proof generation
- [ ] Implement proof verification library (Rust side)
- [ ] Add tests:
  - [ ] Root reproducibility
  - [ ] Valid proof acceptance
  - [ ] Invalid proof rejection (tampered leaf/path/index)

## Phase 4 - Epoch Logic and Root Freeze

- [ ] Implement `epoch/` scheduler policy:
  - [ ] Epoch boundary definition
  - [ ] Freeze/finalize state at epoch close
- [ ] Persist epoch artifacts:
  - [ ] `epoch_id`
  - [ ] `root_hash`
  - [ ] Timestamp + publisher metadata
- [ ] Enforce epoch monotonicity in off-chain logic
- [ ] Add tests:
  - [ ] No duplicate epoch commitment records
  - [ ] No epoch rollback/overwrite

## Phase 5 - Database and Replay Persistence (Postgres)

- [ ] Implement Postgres schema and migrations:
  - [ ] Raw events table (append-only/WAL-style)
  - [ ] Epoch roots table
  - [ ] Snapshot checkpoints table
- [ ] Implement `db/` adapters:
  - [ ] Insert raw events
  - [ ] Read ordered event streams for replay
  - [ ] Save/load snapshots
  - [ ] Save/load finalized roots
- [ ] Add restart/recovery flow:
  - [ ] Recover from latest snapshot + tail replay
- [ ] Add tests:
  - [ ] Restart recovery hash equality
  - [ ] Replay from genesis equals replay from checkpoint

## Phase 6 - Solidity Commitment Contract (Foundry)

- [ ] Scaffold Foundry project in `contracts/`
- [ ] Implement minimal commitment contract:
  - [ ] Store `epoch_id`, `root_hash`, minimal metadata
  - [ ] Enforce strictly increasing epoch updates
  - [ ] Prevent replay/root overwrite
  - [ ] Keep business logic minimal
- [ ] Add proof verification interface/hook (as needed by MVP proof flow)
- [ ] Add tests (Foundry):
  - [ ] Epoch monotonicity enforcement
  - [ ] Replay protection
  - [ ] Root overwrite rejection
  - [ ] Fuzz tests for proof verification paths

## Phase 7 - Publisher Integration (Rust -> HyperEVM)

- [ ] Implement `publisher/`:
  - [ ] Read finalized roots from DB
  - [ ] Submit transaction to commitment contract
  - [ ] Record tx hash/status
- [ ] Add idempotency and retry policy:
  - [ ] Safe retries without duplicate logical commits
  - [ ] Backoff + failure classification
- [ ] Add tests:
  - [ ] Submission success path
  - [ ] Reorg/retry handling assumptions
  - [ ] Duplicate publish prevention

## Phase 8 - End-to-End Verification Flow

- [ ] Build E2E flow:
  - [ ] Ingest sample events
  - [ ] Compute deterministic state + root
  - [ ] Publish root on-chain
  - [ ] Generate and verify inclusion proof
- [ ] Add integration tests for full pipeline
- [ ] Document operator runbook:
  - [ ] Startup
  - [ ] Replay/recovery
  - [ ] Incident handling (gaps/out-of-order events)

## Security and Correctness Checklist

- [ ] No floating-point usage anywhere
- [ ] No nondeterministic randomness in consensus-affecting paths
- [ ] No unsorted `HashMap` iteration in hash-critical paths
- [ ] Explicit schema version included in all state-hash preimages
- [ ] Input validation for malformed/out-of-order/gapped events
- [ ] Sequence enforcement at ingestion + transition boundary
- [ ] Contract-level replay attempt rejection

## Testing Matrix (Minimum Required)

- [ ] Deterministic replay tests
- [ ] Restart consistency tests
- [ ] Hash equality across independent runs
- [ ] Merkle proof validity/invalidity tests
- [ ] Contract replay protection tests
- [ ] Root overwrite rejection tests

## Performance and Benchmarking (`/docs/`)

- [ ] Measure and document:
  - [ ] Root computation latency
  - [ ] Memory usage during replay and root build
  - [ ] Proof generation time
  - [ ] On-chain verification gas cost
- [ ] Add reproducible benchmark scripts/commands

## Optional Advanced Phase (Only After MVP Stability)

- [ ] Bonded root submission
- [ ] Challenge/dispute protocol
- [ ] Multi-relayer quorum
- [ ] Incremental Merkle updates
- [ ] Multi-market parallelization

## Definition of MVP Done

- [ ] Deterministic single-market replay produces stable roots
- [ ] Canonical serialization versioned and tested
- [ ] Merkle proofs generated and verified
- [ ] Minimal on-chain contract stores epoch root metadata only
- [ ] Off-chain publisher submits monotonic epoch roots
- [ ] Required deterministic/security/recovery tests pass
- [ ] README/docs explain architecture and guarantees clearly
