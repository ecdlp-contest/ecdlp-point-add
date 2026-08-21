# Point-add submission

## Algorithm

This submission implements reversible mixed secp256k1 point addition: the
quantum target point `P` is overwritten with `P + Q`, while the classical
offset point `Q` is preserved. It uses affine addition with two conservative
Kaliski inverse/apply pairs and explicit uncomputation of every workspace.

The operation stream is nonce-free. Both inverse pairs use the full
`2 * 256 - 1 = 511` schedule; there is no identity tail, tail rewrite, or seed
grinding.

The affine schedule is generated from a reviewed algorithm profile. A second
generated module applies five fail-closed exact target-fanout rewrites to the
emitted parent stream:

```text
CCX(a, b, t); CX(t, u); CCX(a, b, t)
    =>
CX(t, u); CCX(a, b, u)
```

Each rewrite restores `t`, preserves the XOR applied to `u`, and removes one
CCX. The generated matcher binds all controls, targets, inline conditions,
condition-stack depths, and absolute parent-stream indices before rewriting.
Any source drift aborts circuit construction.

## Trusted candidate evidence

- 102,400/102,400 deterministic parallel Fiat-Shamir shots passed;
- zero classical, phase, and ancilla failures;
- 2,841 peak logical qubits;
- 5,180,781.000 average executed Toffolis;
- 40,922,095 emitted operations;
- score 14,718,598,821;
- `ops.bin` SHA-256
  `f778dad9a0065a52c3ed1bbaf92de0578c74579476a4a545ed807e1da0bbc74c`.

Relative to the clean baseline, this removes exactly five emitted and five
average executed Toffolis, keeps the 2,841-qubit peak, and lowers the score by
14,205. The transformation was selected from circuit structure before the
trusted run; the 102,400 shots were used only as a final verifier. There is no
seed tuning, width truncation, approximate tail, or identity nonce.

## Resubmission record

Resubmitted 2026-08-19 with an unchanged circuit after an independent local
rerun (Windows, 16 workers x 64 bit-sliced shots) reproduced the identical
artifact: 102,400/102,400 shots passed with zero classical, phase, and ancilla
failures; the same 5,180,786.000 average executed Toffolis, 2,841 peak logical
qubits, score 14,718,613,026, and byte-identical `ops.bin` SHA-256. The prior
pending submission was withdrawn because its trusted-worker run failed on
missing repository secrets before evaluation started.

## Prior baseline record

The accepted parent emitted 40,922,100 operations at 5,180,786 average executed
Toffolis, 2,841 peak qubits, and score 14,718,613,026. The generated affine
schedule reproduced that operation stream byte-for-byte before the exact
fanout postpass was activated.
