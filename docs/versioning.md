# State Hash Versioning

This project versions state commitments with:

`hash = H(schema_version || serialized_state)`

## When `schema_version` Must Increase

Increase `schema_version` for any change that can alter commitment bytes, including:

- field add/remove/rename in canonical state
- field order changes in canonical serialization
- numeric encoding changes (width, signedness, endian)
- map/list ordering rule changes
- hash preimage layout changes
- hash algorithm changes

## Backward Compatibility

- Different schema versions are expected to produce different hashes for the same logical state.
- Old and new schema versions are both valid historically, but not interchangeable.
- Verifiers must know which schema version was used for a given commitment.

## Practical Rule

If a node running old code and a node running new code could produce different serialized bytes from the same replayed events, bump `schema_version`.
