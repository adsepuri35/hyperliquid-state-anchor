# Architecture Overview

This document defines strict separation of concerns for deterministic verifiable state commitments.

## Rust Modules

- `ingestion/`
  - Purpose: fetch/normalize Hyperliquid events.
  - Must not: apply business/state transitions.

- `state/`
  - Purpose: deterministic state transitions only.
  - Must: enforce sequence monotonicity and fixed-point/integer arithmetic.

- `merkle/`
  - Purpose: canonical hashing, root construction, proof generation/verification.
  - Must: preserve deterministic leaf ordering and stable encoding.

- `epoch/`
  - Purpose: epoch boundary policy and root freeze/finalization.
  - Must: enforce monotonic epoch progression.

- `publisher/`
  - Purpose: submit finalized roots on-chain.
  - Must not: embed state transition logic.

- `db/`
  - Purpose: persistence and replay data access.
  - Must: support ordered replay reads and append-style event storage.

## On-Chain Contract Scope

The contract should remain minimal and include only:

- root storage per epoch
- epoch monotonicity checks
- replay/overwrite protections
- proof verification hooks (as needed by MVP flow)

It should not include full exchange business logic.
