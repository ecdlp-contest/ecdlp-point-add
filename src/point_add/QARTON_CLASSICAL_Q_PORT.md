# Qarton classical-Q point-add port

## Scope

This is an operation-preserving Rust replay derived from André Schrottenloher's
Qarton point-addition implementation at `ec-point-addition` commit
`9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`. The paper/repository primitive adds
either zero or one fixed point selected by a quantum bit. The contest instead
requires an unconditional addition of an arbitrary runtime-classical point `Q`.

Primary sources: [arXiv:2606.02235](https://arxiv.org/abs/2606.02235) and the
[`ec-point-addition` Python/Qarton repository](https://gitlab.inria.fr/capsule/qarton-projects/ec-point-addition).
The Rust port and contest adaptation do not claim authorship of the underlying
paper construction or Python algorithms.

The source-level adapter exposes exactly four registers, in contest order:

1. quantum `P.x[256]`;
2. quantum `P.y[256]`;
3. classical preserved `Q.x[256]`;
4. classical preserved `Q.y[256]`.

## Adaptation

The selector is specialized to true and finite-point flags are removed because
the trusted generator excludes infinity and equal-x exceptional inputs.

Each fixed full-point lookup is replaced by allocating `Q.x/Q.y` quantum words,
loading them with classically conditioned X gates, using the original modular
arithmetic, and applying the same conditioned X gates to clear the words. These
loads are Clifford operations and preserve the classical registers.

The fixed `3*Q.x` lookup cannot be precomputed for runtime Q. Instead, `Q.x` is
loaded once, added to the accumulator three times modulo secp256k1's field prime,
and unloaded. It is deliberately not retained across the modular inverse, which
would increase the peak by approximately 256 qubits.

The decomposed Qarton stream is lowered as follows:

- adjacent `H; MSR` becomes the contest's HMR operation;
- Qarton's measurement-uncompute phase workspace is eliminated exactly by
  replacing a CCX into `|->` with its equivalent CZ phase correction;
- one or two Qarton classical controls become `c_condition` and, for two
  controls, the trusted condition stack.

The canonical 6-sigma source stream SHA-256 is
`2edf1a06c4c285b20fce52eef6c28783ccb9563a0f4bd4dd241d9bc144031f2e`.
The lowered-record SHA-256 is
`97c0329cdb77b80458972682a56dadd83565037cdd1dc04648b49e13c1cb8dda`.

## Reliability profiles

Qarton's paper parameters use `ITERATIONS_VAR = U_PAD_VAR = 2.4`. The source
describes this as an approximate profile with roughly one failure per
10,000--20,000 inversions. The classical-Q adapter at those settings uses 1,441
qubits and reports 1,804,545 probability-weighted CCX. Trusted evaluation found
three classical failures, one phase-garbage batch, and three ancilla-garbage
batches in 102,400 deterministic local shots. It is not a valid contest result.

The active replay sets both safety margins to 6.0. It uses 1,536 qubits and its
Qarton reporter gives 2,242,731 probability-weighted CCX. This profile has a
separate stream and must not be described as reproducing the paper's exact
resource row.

## Trusted validation

The unchanged build stage emitted 13,764,960 operations. The unchanged trusted
evaluator loaded the artifact with four ABI registers and reported:

| Metric | Result |
| --- | ---: |
| Shots | 102,400 |
| Classical mismatches | 0 |
| Phase-garbage batches | 0 |
| Ancilla-garbage batches | 0 |
| Qubits | 1,536 |
| Average executed Toffoli | 2,241,864.716 |
| Average executed Clifford | 9,526,511.625 |
| Rounded contest score | 3,443,504,640 |

This is deterministic commitment-only local evidence. It is not a fresh
server-seeded receipt or an all-input correctness proof. No package, upload, or
submission was performed.
