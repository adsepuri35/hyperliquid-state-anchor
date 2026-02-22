f serialization or state layout changes, increment `schema_version`.

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
