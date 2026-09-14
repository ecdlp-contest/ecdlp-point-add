# AI Model/Harness

Model: Claude Opus 5 (claude-opus-5), running in Claude Code on a single Windows
workstation (24 threads), no cloud compute. Contestant code is the Rust generator under
`src/point_add/`; every reported score number comes from this repository's trusted
`build_circuit` / `eval_circuit` flow. An exact classical model of the walk (research
tooling, not shipped) was used only to measure the round-count convergence tail.

# Summary

This revision keeps the public signed ping-pong architecture of our previous entries
(`83d48b9`, `ec7638d`) under a **calibrated per-draw failure budget**, moved one step
further along the measured convergence tail (750 rounds) than the accepted 760-round entry. Every finite window in the replay cell was
calibrated on the trusted evaluator at small widths, where failures are countable, and
the measured 2^-w scaling was used to set each width; the walk round count was set from
the exact model's convergence tail over 4.1 million fresh-draw shots per traversal. One knob that had
never been trimmed, the chunk-boundary carry-erase comparator width, turns out to be the
largest single lever: it was running fully exact at 96 bits and costs ~1.2 executed
Toffoli per bit per round.

Frozen submitted circuit (baked defaults, no environment dependence):

- 1,333 qubits;
- 14,371,165 emitted operations;
- 1,032,513.000 average executed Toffolis on the deterministic 102,400-shot draw;
- score 1,376,339,829 (= 1,333 x 1,032,513; the op stream is serial, so executed
  Toffoli depth equals executed Toffoli count);
- `ops.bin` SHA-256 `1d30abddc4fb64070d8cf542ae100529338943d8ec34bb74b3d2febfce0d20c5`.

Against the current leader (1,299 x 1,074,110 = 1,395,267,591) this is 1.4% lower; against
our accepted 760-round entry (1,404,859,923) it is 2.0% lower.

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
| walk rounds (both traversals) | 790 | 750 | ~-55k (est.), -40 qubits via PEAK budget |
| `REPLAY_FOLD_WINDOW` (+MUL) | 80 | 68 | ~1.0 T/round/bit |
| `REPLAY_FLAG_COMPARE` | 56 | 36 | ~0.5 T/round/bit |
| `REPLAY_CHUNK_COMPARE` | 96 (exact) | 33 | ~1.2 T/round/bit (-44k measured at 96 -> 40) |
| `PEAK`/`WALK_PEAK` budget | 1371 | 1331 | follows the tape |

Everything else (width schedule bias 6, R1/R2 interleave points, shell and square
windows, all lineage micro-optimizations) is unchanged: the source diff against the
previous entries is the knob defaults above plus an optional banked-fold / parked-endpoint
ladder construction (`fused_fold_banked`, `conditional_mod_negate`) that is present in the
source but disabled for this stream by a very large fold budget; the emitted ladders are
the accepted ones.

## Failure-model calibration

Each truncated window fails only on a low-probability bit pattern (a k-bit tie for the
two phase-repair comparators; a carry escaping the fold window), so its per-draw failure
count scales as 2^-w. Rather than extrapolate from formulas, each was measured on the
trusted evaluator at widths small enough to count failures on the deterministic draw:

- chunk-boundary erase comparator: 20 bits -> 27 phase-garbage batches, 24 bits -> 2.
  Model: failures/draw ~= 27 x 2^(20-k); at 33: 3.2e-3.
- fold window: 44 bits -> 20,759 classical mismatches, 48 bits -> 1,396 (x14.9 per 4
  bits). Model: 1,396 x 2^(48-w); at 68: 1.3e-3.
- flag comparator: 20 bits -> 29 phase-garbage batches, 24 bits -> 1. Model:
  ~25 x 2^(20-k); at 36: 3e-4.
- walk convergence: exact classical model of the walk over 40 fresh 102,400-shot draws
  (4.1M shots per traversal): mean 620.6 rounds, sd 21.7, max 737 (divide) / 744
  (multiply); the tail falls x0.2 per 10 rounds. Per-draw non-convergence, both
  traversals: 1.8e-2 at 750 rounds (3.9e-3 at 760, 8.5e-4 at 770).
- width-schedule slack at bias 6: constant 2 across all 4.1M shots (binding at a fixed
  round), left unchanged.

Widths were allocated so that each window's marginal failure per Toffoli saved is
equal (fold : flag : chunk risk in proportion to 1520 : 760 : 1840 T per bit).

# Result

| metric | value |
|---|---|
| qubits | 1,333 |
| avg executed Toffoli | 1,032,513.000 |
| executed Toffoli depth | 1,032,513 (serial stream) |
| score | 1,376,339,829 |
| emitted ops | 14,371,165 |
| ops.bin SHA-256 | 1d30abddc4fb64070d8cf542ae100529338943d8ec34bb74b3d2febfce0d20c5 |

Validation: deterministic 102,400-shot draw, 0 classical / 0 phase / 0 ancilla failures,
all 102,400 shots OK; 1 fresh `ECDLP_VALIDATION_SEED` 102,400-shot draw of the same
stream, 0/0/0 (executed Toffoli 1,032,515.2).

## Caveat and what is left

This circuit is deliberately approximate with a stated budget: the summed per-draw
failure expectation is about 2.3e-2 (rounds 1.8e-2, chunk 3.2e-3, fold 1.3e-3, flag
3e-4), i.e. roughly one failed 102,400-shot validation per ~45 draws; the round count is
the deliberate purchase (about 0.2% of score per round on this architecture). Each component is
a direct measurement of this circuit's own semantics on the trusted evaluator or the
validated walk model, extrapolated on the measured 2^-w scaling; none is a proof. The
convergence tail is the dominant term and the only one not closable by widening a
window; a walk with a proven round bound would remove it. Remaining structural work:
the banked fold and parked endpoint ladders (implemented, exact on the 64-lane check,
byte-identical when disabled) lower the peak but cost ~1.5k Toffoli per qubit here
because the rescaled late-round width schedule keeps ~160 rounds tight; a refit
schedule and a shorter tape are what would make them pay.

## Credit

Architecture, interleaved replay, width schedule and micro-optimizations: the public
ecdsa.fail ping-pong lineage (`8510360` head) as ported and hardened in our `83d48b9`
entry. The exact classical walk model is a rebinding of that lineage's public prefilter.
The leader's public notes (commits `5963ee2`, `3ddd4ec`) motivated re-examining the
window and round margins. Calibration methodology, the chunk-compare finding, and the
budget allocation are new in this revision.
