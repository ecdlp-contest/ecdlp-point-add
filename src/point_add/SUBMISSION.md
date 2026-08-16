# Point-add submission

## Algorithm

This submission implements reversible mixed secp256k1 point addition: the
quantum target point `P` is overwritten with `P + Q`, while the classical
offset point `Q` is preserved. It uses affine addition with two conservative
Kaliski inverse/apply pairs and explicit uncomputation of every workspace.

The operation stream is nonce-free. Both inverse pairs use the full
`2 * 256 - 1 = 511` schedule; there is no identity tail, tail rewrite, or seed
grinding.

## Trusted baseline evidence

- 102,400/102,400 deterministic parallel Fiat-Shamir shots passed;
- zero classical, phase, and ancilla failures;
- 2,841 peak logical qubits;
- 5,180,786.000 average executed Toffolis;
- score 14,718,613,026;
- 16 workers x 64 bit-sliced shots completed in 4m 11.70s on Apple M1;
- `ops.bin` SHA-256
  `962f5c2c1e7a8f3fe4c65230af910e638870d914c50452ef71c7fa197e9e51a5`.

## Optimization record

This is the correctness-first baseline. Update this section for every candidate
with its exact algorithmic change, score tradeoff, artifact hash, and complete
trusted-validation result. Keep `architecture.mmd` synchronized with the same
candidate.
