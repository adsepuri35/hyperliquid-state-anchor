# AGENTS.md

## Project: Hyperliquid Verifiable State Commitment System

### Purpose

This repository implements a deterministic off-chain Hyperliquid (HL) state replicator that periodically commits cryptographic state roots to HyperEVM. The system enables verifiable proofs of HL state without storing full exchange data on-chain.

The system has two primary components:

1. **Rust Shadow Engine (off-chain)**
2. **Solidity Commitment Contract (on-chain via Foundry)**

Agents must respect strict determinism and separation of concerns.

---

## Architectural Principles

### 1. Determinism is Mandatory

- Same ordered input events must always produce identical state hashes.
- No floating-point arithmetic.
- All numeric values must use fixed-point integers.
- All maps must be iterated in sorted order before hashing.
- Serialization format must be stable and versioned.

If a change affects hashing or state encoding, it must increment the schema version.

---

### 2. Clear Module Boundaries

Do not mix responsibilities.

**Rust modules:**
- `ingestion/` → Hyperliquid data fetching only
- `state/` → deterministic state transitions only
- `merkle/` → hashing + proof generation only
- `epoch/` → scheduling + root freeze logic
- `publisher/` → on-chain submission only
- `db/` → persistence only

**Contracts** must not contain business logic beyond:
- Root storage
- Verification
- Optional dispute mechanics

---

### 3. Minimal On-Chain Data

On-chain storage must only include:
- `epoch_id`
- `root_hash`
- metadata (timestamp, publisher, optional bond)

Never store full market data on-chain.

---

## Coding Standards

### Rust

- Use explicit types.
- Avoid unwrap() in production paths.
- No nondeterministic randomness.
- No HashMap iteration without sorting keys.
- All serialization must be explicit (no implicit serde defaults for hashing).

**Testing requirements:**
- Deterministic replay tests
- Restart consistency tests
- Hash equality across independent runs

---

### Solidity

- Contracts must be minimal and gas-conscious.
- All root updates must enforce monotonic epoch progression.
- Use Foundry for testing.
- Include fuzz tests for proof verification.
- No upgradeable proxies for MVP.

---

## Development Roadmap Guidance

Agents should follow this order:

1. Implement deterministic state engine (single market MVP).
2. Add canonical serialization.
3. Implement Merkle root computation.
4. Build minimal commitment contract.
5. Add root publisher from Rust to contract.
6. Implement inclusion proof verification.
7. Add challenge / bond mechanism (optional advanced phase).

Do not implement advanced fraud games before deterministic correctness is proven.

---

## Database Guidelines

- Use Postgres.
- Persist raw events for replay.
- Persist computed epoch roots.
- Maintain snapshot checkpoints for fast recovery.
- WAL-style append logging preferred.

---

## Performance Considerations

Agents should measure and document:
- Root computation latency
- Memory usage
- Proof generation time
- On-chain verification gas cost

Performance benchmarks should be included in `/docs/`.

---

## Security Model

Assume:
- Malicious publisher
- Event gaps
- Out-of-order messages
- Contract replay attempts

Code must include:
- Sequence enforcement
- Epoch monotonicity
- Explicit schema version hashing
- Input validation

---

## Testing Requirements

Minimum test coverage must include:
- Deterministic state hash reproducibility
- Merkle proof validity tests
- Contract replay protection tests
- Root overwrite rejection tests
- Restart recovery hash equality

---

## Prohibited Changes

Agents must NOT:
- Introduce floating-point arithmetic
- Change hashing algorithm without version bump
- Mix state logic into publisher module
- Store large state blobs on-chain
- Remove deterministic ordering guarantees

---

## Versioning

All state hashes must include:
hash = H(schema_version || serialized_state)


If serialization or state layout changes, increment `schema_version`.

---

## Documentation Expectations

Each major module must include:
- Purpose
- Invariants
- Determinism guarantees
- Failure modes

README must explain:
- What is stored on-chain
- What is NOT stored on-chain
- How proofs work at a high level

---

## Long-Term Extensions (Optional)

- Multi-relayer quorum
- Bonded root submission
- Interactive dispute protocol
- Incremental Merkle updates
- Multi-market parallelization

These are advanced and should only be attempted after MVP stability.

---

## Summary for Agents

This is not a trading bot.  
This is not a DeFi app.  
This is infrastructure.

**Prioritize:**
- Determinism
- Correctness
- Reproducibility
- Clear module separation
- Minimal on-chain footprint

All changes must preserve verifiability and state integrity.
