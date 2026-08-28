# Generated 1,283-qubit Qarton hybrid point addition

## AI Model / Harness

This candidate was prepared with OpenAI GPT-5.6 Sol in the Codex desktop
application. The reasoning-effort setting was not exposed to this task, so it
is not guessed. The work used the contest repository's isolated development
workflow, Python/Qarton 1.0.0 for circuit construction, and the repository-pinned
ECDLP CLI with the unchanged trusted Rust builder and evaluator.

The active Rust replay wrapper is generated output. It and its compressed QPRT
payload were produced deterministically from the pinned Qarton construction.
Neither contains a validation nonce, sampled-input table, target-side correction,
or evaluator-derived patch.

## Summary

This submission replaces the accepted 1,536-qubit Qarton replay with a hybrid
4.0-margin circuit. It retains the same four-register contest ABI and the same
reversible classical-Q adaptation of André Schrottenloher's point-addition
circuit. The inversion uses a gate-efficient shrinking binary-GCD value walk
and a space-efficient full-width coefficient replay.

The frozen operation stream passed 102,400 deterministic local shots with zero
classical mismatches, zero phase-garbage batches, and zero ancilla-garbage
batches. It measured 1,283 peak logical qubits, 2,502,170.191 average executed
Toffolis, 8,022,141.868 average executed Clifford operations, and 12,377,821
emitted operations. After contest rounding, the balanced score is
3,210,284,110.

The currently accepted 1,536-qubit route scores 3,443,495,424. The submitted
candidate reduces that by 233,211,314, about 6.77%. It exchanges roughly
260,305 additional average Toffolis for 253 fewer peak qubits, which improves
the balanced product.

## Method

### Source construction and contest ABI

The source anchor is André Schrottenloher, *Optimized Point Addition Circuits
for Shor's Algorithm*, arXiv:2606.02235, and its public
`ec-point-addition` Python/Qarton implementation at commit
`9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`.

The original Qarton primitive selects a fixed point with a quantum selector.
The contest instead requests unconditional addition of an arbitrary
runtime-classical point. The adapter exposes exactly four registers, in the
required order:

1. quantum `P.x[256]`;
2. quantum `P.y[256]`;
3. classical preserved `Q.x[256]`;
4. classical preserved `Q.y[256]`.

A classical Q coordinate is loaded into a clean quantum word with X operations
conditioned on the corresponding classical bits. Applying the same operations
after the arithmetic unloads the word and preserves Q. The first stage loads
Q.x and Q.y, performs the coordinate differences, and unloads both words before
the inverse. No 256-qubit Q register therefore remains live through the inverse
workspace peak.

The fixed source circuit precomputes `3*Q.x`. Runtime Q prevents that lookup, so
the adapter loads Q.x once, performs three modular additions into the original
accumulator, and unloads Q.x. The source selector is specialized to true, and
finite-point flags are specialized under the benchmark domain, which excludes
infinity and equal-x exceptional inputs. Q.x and Q.y are loaded again only for
the final coordinate stage and are uncomputed immediately.

### Hybrid inverse profile

The candidate splits the modular inverse into two independently selected
backends. The shrinking binary-GCD value walk uses the gate-efficient backend;
the full-width coefficient replay uses the space-efficient backend. Both the
fixed-iteration safety margin and signed-coefficient padding margin are 4.0.
The generated schedule has 426 fixed rounds and a 710-bit packed dialog.

This resource reduction occurs before the Rust language boundary. Qarton assigns
the 1,283-wire peak while constructing the circuit; decoding and compression do
not compress wire identifiers or change lifetimes. Compared with the accepted
6.0-margin route, the shorter inverse schedule and smaller signed coefficient
workspace reduce peak qubits but raise the probability of an approximation
boundary being reached.

### Deterministic operation lowering

The fully decomposed, classical-last Qarton circuit is streamed into a canonical
operation representation. A clean forward reconstruction emitted 12,904,572
source operations and reproduced the source-stream SHA-256:

`18a114949e77ad5f586f58523d8ba261616e0892568b29cfe2d80b69a05a4917`.

The classical-Q adapter performs five 256-bit coordinate loads and the five
matching unloads. Adjacent Q.x/Q.y copies appear as 512-operation runs, for
2,560 classically controlled X operations in total. Qarton AND gates are
lowered consistently to CCX at source generation time; this is not a later
change to the Rust target.

The source operations are lowered to the contest vocabulary and encoded in a
versioned QPRT replay:

- adjacent Qarton `H; MSR` becomes the contest HMR operation;
- a phase CCX targeting `|->` becomes the equivalent CZ correction;
- one classical control becomes `c_condition`;
- two classical controls additionally use the trusted condition stack; and
- register declarations and ordered operands are preserved.

The lowered-record SHA-256 is
`a35c38d09fe8777333b15cf93c05964a564544990f42f79a8b1feb21472bb888`.
The QPRT SHA-256 is
`a4f6f22453d26643df63e977e79292068cc5b143f9705d585ae93bb3677354f7`,
and the compressed payload SHA-256 is
`42c86d6903b66b7a6c3d16a8d01dad9f1b8962f18241666e872555358a2b1b7d`.

The public Rust wrapper contains the immutable census constants, payload
inclusion, decoder invocation, and complete-stream tests. The accepted
`qarton_fixed_port` decoder is unchanged. It checks replay magic, qubit and
classical-bit counts, record boundaries, operand aliases through
`Op::validate`, and trailing bytes before returning the operation vector.

### Reproducibility controls

The generator fixes the Qarton repository commit, Qarton version, source
parameters, operation ordering, and payload hashes. Rust is an output of that
pipeline. The evaluator operates only on `ops.bin`; it does not import the
Python construction or use submission-side testing data.

The local operation commitment is
`b9ab2819553e845a7aab7cf27c02fe4e280f71b11edcd40d26896414ddddac08`.
That commitment identifies the circuit evaluated for the metrics below. Any
source, parameter, lowering, or payload change must produce a new commitment
and undergo a complete evaluation again.

## Result

The frozen local evaluation reported:

| Metric | Result |
| --- | ---: |
| Shots | 102,400 |
| Classical mismatches | 0 |
| Phase-garbage batches | 0 |
| Ancilla-garbage batches | 0 |
| Peak logical qubits | 1,283 |
| Average executed Toffoli | 2,502,170.191 |
| Average executed Toffoli depth | 2,502,170.191 |
| Average executed Clifford | 8,022,141.868 |
| Emitted operations | 12,377,821 |
| Rounded balanced score | 3,210,284,110 |

The decoded stream contains 2,572,427 CCX operations, 704,099 HMR operations,
577,162 CZ operations, and 91,428 paired two-control condition scopes. The
trusted builder declares 1,028 register operations and preserves the four ABI
registers in their required order.

The submission is rebuilt and packaged by the current repository-pinned CLI.
Server receipt alone is not a ranked result: the trusted worker must reproduce
the submitted operation commitment and pass all 102,400 shots using a fresh
private seed.

## Caveat and what is left

The inverse schedule and several special-prime arithmetic paths use bounded
approximations. The clean local 102,400-shot cohort is strong evidence about
these exact bytes, but it does not prove every exceptional event unreachable
or predict the private validation seed. A server-seeded mismatch, phase event,
or dirty ancilla would invalidate this candidate despite its lower score.

If validation fails, the repair must occur in the source construction and
produce a new immutable operation stream. Patching Rust after observing the
evaluator or adding a nonce would not be an admissible correction.

The benchmark's finite, unequal-x input domain is part of the construction.
This is a mixed affine point-add primitive, not a complete exceptional-case
elliptic-curve group law or a complete implementation of Shor's algorithm.

## Credit and references

The point-addition architecture, modular arithmetic, bounded binary-GCD method,
and original Qarton implementation are André Schrottenloher's work. This
submission is a generated Rust replay and contest-ABI adaptation; it does not
claim authorship of the underlying algorithms.

- André Schrottenloher, *Optimized Point Addition Circuits for Shor's
  Algorithm*, [arXiv:2606.02235](https://arxiv.org/abs/2606.02235).
- André Schrottenloher's Python/Qarton implementation,
  [`ec-point-addition`](https://gitlab.inria.fr/capsule/qarton-projects/ec-point-addition),
  commit `9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`.
- Qarton 1.0.0, used for circuit construction, decomposition, allocation, and
  resource counting before deterministic Rust replay generation.
