# Qarton classical-Q point addition

## AI Model / Harness

This candidate was implemented by OpenAI Codex based on GPT-5 in the Codex
desktop application. The work used the repository's autoresearch isolation
contract, a per-run Git worktree, a per-run Cargo target, Python/Qarton 1.0.0 for
source extraction, WSL2 Ubuntu 24.04 for Rust builds, and the unchanged trusted
`build_circuit` and `eval_circuit` binaries. The application did not expose a
separate user-facing reasoning-effort label, so none is invented here.

## Summary

The candidate is a direct operation-stream Rust port of André Schrottenloher's
Qarton secp256k1 point-addition circuit, adapted from its fixed-point one-bit
window interface to the contest's arbitrary classical-Q ABI. The source anchor
is `ec-point-addition` commit
`9b23c9170a636a7097a02afb3a3d6cbb6425c9f4` with Qarton 1.0.0.

The circuit exposes the required four registers: quantum `P.x[256]`, quantum
`P.y[256]`, classical `Q.x[256]`, and classical `Q.y[256]`. It overwrites P with
P+Q and preserves Q. Fixed lookups become reversible classical loads. The
precomputed `3*Q.x` lookup becomes three modular additions from one recyclable
loaded Q.x word. Q.x is not retained across the inverse, avoiding roughly 256
additional peak qubits.

The first 2.4-sigma adapter reproduced the low resource envelope at 1,441 qubits
and 1,804,545 Qarton-weighted CCX, but it was invalid: the trusted 102,400-shot
run found three classical failures, one phase-garbage batch, and three
ancilla-garbage batches. Qarton's source describes the 2.4-sigma inversion
profile as allowing approximately one failure per 10,000--20,000 inversions.
That result is retained as negative evidence and is not presented as valid.

The active candidate raises both inversion safety margins to 6.0. The unchanged
trusted evaluator passed all 102,400 deterministic local shots with zero
classical, phase, or ancilla failures. It uses 1,536 qubits, averages
2,241,864.716 executed Toffolis, and has rounded score 3,443,504,640. The
95-qubit increase is not Rust overhead: it is already present in Qarton's
allocation and comes from the longer, more heavily padded 6-sigma inverse.

## Method

### Classical-Q source adaptation

The Python reference circuit has two quantum `ModIntType(p)` registers followed
by two classical 256-bit registers. A Q coordinate is loaded into a clean quantum
word by applying an X conditioned on each corresponding classical bit. Applying
the same gates again clears that word. This preserves Q and adds no Toffoli cost
to the lookup itself.

The first stage loads Q.x and Q.y, performs the original coordinate subtractions,
and unloads both words before the in-place modular division. This preserves the
paper circuit's important workspace lifetime: no 256-bit Q word is live across
the inverse.

The second fixed lookup originally loaded the compile-time constant `3*Q.x`.
Runtime Q prevents that precomputation. The adapter loads Q.x once, performs
three complete modular additions into the original accumulator, and unloads
Q.x. This costs two additional modular additions but reuses the same lifetime.

The controlled square-add stage was originally enabled when the selector was
nonzero. The adapter binds its control to a temporary `|1>` qubit and clears it.
The following controlled negation becomes unconditional. Finally, Q.x and Q.y
are loaded again for the output-coordinate stage and uncomputed immediately.

### Reliability hardening

The paper repository sets `ITERATIONS_VAR = 2.4` and `U_PAD_VAR = 2.3` for the
binary-GCD schedule. Its source explicitly calls this an approximate choice with
expected failures. A contest candidate cannot accept that failure model because
every one of 102,400 fresh-seeded shots must pass.

The active generation profile sets both margins to 6.0 before constructing the
memoized inverse circuits. This increases the Qarton allocation from 1,441 to
1,536 qubits and the weighted CCX estimate from 1,804,545 to 2,242,731. The
active stream is a reliability-hardened derivative; it does not claim to
reproduce the paper's exact resource row.

The delta occurs before the language boundary: Python/Qarton allocates 1,441
wires at 2.4 sigma and 1,536 at 6 sigma. `ITERATIONS_VAR` lengthens the fixed
inverse schedule and `U_PAD_VAR` widens its signed coefficient workspace. Rust
preserves those assigned wire indices; decoding and compression add no qubits.

### Operation-preserving Rust replay

The fully decomposed, classical-last Qarton circuit is streamed into a canonical
IR. The 6-sigma source stream SHA-256 is
`2edf1a06c4c285b20fce52eef6c28783ccb9563a0f4bd4dd241d9bc144031f2e`.
Every operation is lowered to the contest vocabulary and encoded as a compact
versioned replay. The lowered-record SHA-256 is
`97c0329cdb77b80458972682a56dadd83565037cdd1dc04648b49e13c1cb8dda`.

All 1,097,858 adjacent Qarton `H; MSR` pairs become HMR. Remaining Hadamards
belong to measurement-uncompute phase workspaces: an ancilla is prepared as
`|->`, receives a CCX on its target, and is unprepared. The lowering replaces
that exact phase-kickback gadget with CZ. One classical control uses
`c_condition`; two controls additionally use the trusted condition stack.

The replay is zstd-compressed and decoded by Rust into validated `Op` values.
The decoder checks magic, qubit count, classical-bit count, gate count, record
boundaries, operand aliases through `Op::validate`, and trailing bytes. Tests
traverse the complete fixed-point and classical-Q replays and check gate totals.

## Result

The default build emits the reliability-hardened adapter without an environment
switch. Rebuilding after activation produced the same evaluated `ops.bin`, with
SHA-256
`713dfb1c5c0ff91381afc4b4d765a9b54a85e82278c89ba56e64228bf534de03`.

The unchanged trusted evaluator reported:

- 102,400 of 102,400 deterministic commitment-only shots passed;
- zero classical mismatches;
- zero phase-garbage batches;
- zero ancilla-garbage batches;
- 1,536 peak logical qubits;
- 2,241,864.716 average executed Toffolis;
- 9,526,511.625 average executed Clifford operations;
- 13,764,960 emitted operations; and
- rounded balanced score 3,443,504,640.

The full locked offline release test suite also passed. It includes complete
decoder tests for the paper-signature replay and the active classical-Q replay.

## Reduction opportunities

The following are prospective optimizations, not claims about the validated
artifact. Each would require regenerating the source and lowered hashes and
rerunning the complete trusted gate.

1. **Tune the margins independently.** Both parameters moved directly to 6.0.
   Sweeping `ITERATIONS_VAR` and `U_PAD_VAR` separately may recover inverse
   rounds, coefficient width, and part of the 95-qubit delta. Any smaller choice
   needs a justified tail bound plus independent-seed testing; a local pass alone
   does not prove the failure probability is acceptable.
2. **Use a reversible overflow path.** A narrower common coefficient register
   plus an overflow flag and exact uncomputed fallback could avoid sizing every
   input for the 6-sigma tail. Simple truncation is invalid and would recreate
   the rejected 2.4-sigma failures.
3. **Optimize runtime `3*Q.x`.** A secp256k1-specialized modular doubling plus
   one addition, or a fused shift/add with pseudo-Mersenne correction, may beat
   three generic modular additions. Schedule its scratch outside the inverse
   peak so a gate saving does not increase qubits.
4. **Improve source-level lifetimes.** Reuse coefficient/comparison scratch
   across mutually exclusive binary-GCD phases, shrink active widths over the
   schedule, and cancel compute/uncompute pairs before allocation. Serialization
   alone cannot reduce the declared peak.
5. **Extend verified lowering peepholes.** Beyond the existing HMR and CZ
   rewrites, look for cancellable classical loads, condition scopes, and inverse
   gate pairs across block boundaries. Require local equivalence checks and full
   trusted validation for every rewrite.

## Credit and references

The point-addition architecture, optimized modular arithmetic, bounded binary-GCD
schedule, and original Qarton implementation are the work of André
Schrottenloher. This submission is a Rust operation-stream port and contest-ABI
adaptation of that work; it does not claim authorship of the underlying paper
construction or Python algorithms.

- André Schrottenloher, *Optimized Point Addition Circuits for Shor's Algorithm*,
  [arXiv:2606.02235](https://arxiv.org/abs/2606.02235).
- André Schrottenloher's Python/Qarton implementation,
  [`ec-point-addition`](https://gitlab.inria.fr/capsule/qarton-projects/ec-point-addition),
  source commit `9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`.
- Qarton 1.0.0, used to construct, decompose, allocate, and resource-count the
  source circuit before deterministic lowering to the Rust replay.

## Caveat and what is left

The successful campaign used the deterministic local commitment-only seed. It
is strong stream-qualified evidence but not a server-seeded receipt and not an
all-input proof. The 6-sigma parameters make the bounded-GCD failure mechanism
substantially less likely; they do not prove the bound can never be exceeded.

The benchmark excludes infinity and equal-x inputs. The specialization depends
on that domain and is not a complete elliptic-curve addition formula outside it.

Any Python, Qarton, parameter, lowering, or allocator change requires a new
source hash, lowered hash, Rust asset, and complete trusted evaluation. The
1,441-qubit approximate stream must not replace the active asset merely to claim
a smaller score.
