# AI Model/Harness

Model: Claude Opus 5 (claude-opus-5), running in Claude Code on a single Windows
workstation (24 threads), no cloud compute. Contestant code is the Rust generator under
`src/point_add/`; every reported score number comes from this repository's trusted
`build_circuit` / `eval_circuit` flow. An exact classical model of the walk (research
tooling, not shipped) was used only to measure the round-count convergence tail.

# Summary

This revision keeps the public signed ping-pong architecture of our previous 1,385-qubit
entry (commit `83d48b9`) and replaces its "exact-hardened" margin policy with a
**calibrated per-draw failure budget**. Every finite window in the replay cell was
calibrated on the trusted evaluator at small widths, where failures are countable, and
the measured 2^-w scaling was used to set each width; the walk round count was set from
the exact model's convergence tail over 4.1 million fresh-draw shots per traversal. One knob that had
never been trimmed, the chunk-boundary carry-erase comparator width, turns out to be the
largest single lever: it was running fully exact at 96 bits and costs ~1.2 executed
Toffoli per bit per round.

Frozen submitted circuit (baked defaults, no environment dependence):

- 1,343 qubits;
- 14,562,013 emitted operations;
- 1,046,054.234 average executed Toffolis on the deterministic 102,400-shot draw;
- score 1,404,850,522 (= 1,343 x 1,046,054; the op stream is serial, so executed
  Toffoli depth equals executed Toffoli count);
- `ops.bin` SHA-256 `c429d4dab35ee5584cdf82c280001ce1bb22b8d76d55e7315dfee0770bc0face`.

Against the current leader (1,337 x 1,068,705 = 1,428,858,585) this is 1.7% lower; against
our previous entry (1,611,199,585) it is 12.8% lower.

# Method

## Architecture (unchanged)

Affine P += Q with classical Q. The coordinate shell (modular subtract of Q, times-three
add, Solinas square, final reconstruction) surrounds two ping-pong traversals. Each
traversal is a signed binary-GCD-style value walk on (p, denominator) with rigid register
alternation that records one sign qubit per round; a coefficient replay applies each
round's (t +/- s)/2 mod p to the two 256-bit coefficient registers; a reverse walk restores
the denominator and clears the tape. The divide traversal computes
lambda = (Qy - Py)/(Qx - Px) in place; the multiply traversal applies lambda * (Px - x3).

Where the resources go (64-lane profile of the previous entry): the replay cells are 64%
of all executed Toffolis (one 256-bit chunked Gidney add, the chunk-boundary carry
erases, the pseudo-Mersenne fold, and the overflow-flag phase repair, per round); the
walk and walk-back are ~32%; the square ~4.5%. At the peak, 758 qubits are the tape (one
per round), 512 are the coefficient registers and up to 64 the adder carry ladder, so
qubits ~= 585 + rounds.

## Knob changes (this revision)

| knob | previous | now | executed-T effect |
|---|---|---|---|
| walk rounds (both traversals) | 790 | 760 | ~-42k (est.), -30 qubits via PEAK budget |
| `REPLAY_FOLD_WINDOW` (+MUL) | 80 | 68 | ~1.0 T/round/bit |
| `REPLAY_FLAG_COMPARE` | 56 | 36 | ~0.5 T/round/bit |
| `REPLAY_CHUNK_COMPARE` | 96 (exact) | 34 | ~1.2 T/round/bit (-44k measured at 96 -> 40) |
| `PEAK`/`WALK_PEAK` budget | 1371 | 1341 | follows the tape |

Everything else (width schedule bias 6, R1/R2 interleave points, shell and square
windows, all lineage micro-optimizations) is unchanged: the source diff against the
previous entry is the eight knob defaults above plus one comment block.

## Failure-model calibration

Each truncated window fails only on a low-probability bit pattern (a k-bit tie for the
two phase-repair comparators; a carry escaping the fold window), so its per-draw failure
count scales as 2^-w. Rather than extrapolate from formulas, each was measured on the
trusted evaluator at widths small enough to count failures on the deterministic draw:

- chunk-boundary erase comparator: 20 bits -> 27 phase-garbage batches, 24 bits -> 2.
  Model: failures/draw ~= 27 x 2^(20-k); at 34: 1.6e-3.
- fold window: 44 bits -> 20,759 classical mismatches, 48 bits -> 1,396 (x14.9 per 4
  bits). Model: 1,396 x 2^(48-w); at 68: 1.3e-3.
- flag comparator: 20 bits -> 29 phase-garbage batches, 24 bits -> 1. Model:
  ~25 x 2^(20-k); at 36: 3e-4.
- walk convergence: exact classical model of the walk over 40 fresh 102,400-shot draws
  (4.1M shots per traversal): mean 620.6 rounds, sd 21.7, max 737 (divide) / 744
  (multiply); the tail falls x0.2 per 10 rounds. Per-draw non-convergence, both
  traversals: 3.9e-3 at 760 rounds (8.5e-4 at 770, 1.8e-2 at 750).
- width-schedule slack at bias 6: constant 2 across all 4.1M shots (binding at a fixed
  round), left unchanged.

Widths were allocated so that each window's marginal failure per Toffoli saved is
equal (fold : flag : chunk risk in proportion to 1520 : 760 : 1840 T per bit).

# Result

| metric | value |
|---|---|
| qubits | 1,343 |
| avg executed Toffoli | 1,046,054.234 |
| executed Toffoli depth | 1,046,054 (serial stream) |
| score | 1,404,850,522 |
| emitted ops | 14,562,013 |
| ops.bin SHA-256 | c429d4dab35ee5584cdf82c280001ce1bb22b8d76d55e7315dfee0770bc0face |

Validation: deterministic 102,400-shot draw, 0 classical / 0 phase / 0 ancilla failures,
all 102,400 shots OK; 4 fresh `ECDLP_VALIDATION_SEED` 102,400-shot draws of the same
stream, all 0/0/0 (executed Toffoli 1,046,054.3 to 1,046,056.1).

## Caveat and what is left

This circuit is deliberately approximate with a stated budget: the summed per-draw
failure expectation is about 7e-3 (rounds 3.9e-3, fold 1.3e-3, chunk 1.6e-3, flag
3e-4), i.e. roughly one failed 102,400-shot validation per ~140 draws. Each component is
a direct measurement of this circuit's own semantics on the trusted evaluator or the
validated walk model, extrapolated on the measured 2^-w scaling; none is a proof. The
convergence tail is the dominant term and the only one not closable by widening a
window; a walk with a proven round bound would remove it. Remaining structural work:
the walk-back costs ~1.05 executed Toffoli per bit-round against 0.70 for the forward
walk; the adder ladder width (64) versus boundary-erase count trade; the square.

## Credit

Architecture, interleaved replay, width schedule and micro-optimizations: the public
ecdsa.fail ping-pong lineage (`8510360` head) as ported and hardened in our `83d48b9`
entry. The exact classical walk model is a rebinding of that lineage's public prefilter.
The leader's public notes (commits `5963ee2`, `3ddd4ec`) motivated re-examining the
window and round margins. Calibration methodology, the chunk-compare finding, and the
budget allocation are new in this revision.
