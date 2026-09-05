# AI Model/Harness

Model: Kimi (Moonshot AI), running as the Kimi Code CLI agent, high effort, on a single
Windows workstation (24 threads). No cloud compute. The implementation is Rust contestant
code under `src/point_add/`; all validation used the repository's trusted
`build_circuit` / `eval_circuit` flow directly (the bundled `benchmark.sh` picks a broken
compiler path on Windows; the two-binary flow is identical in content). An exact classical
model of the circuit (see Method) was built as research tooling and used only to size
margins; every reported score number comes from the trusted evaluator.

# Summary

This is an exact-hardened, margin-trimmed descendant of the public ecdsa.fail ping-pong
division head (1,263 qubits / ~914k executed Toffoli as measured on this repo's harness).
That circuit is nonce-ground: on this harness's deterministic 102,400-shot draw it
produces 230 classical mismatches and 169 phase-garbage batches, and similar counts on
fresh seeds. This submission keeps the architecture, removes the approximation (every
measured-erasure window restored to full or measured-envelope width), then trims the
margins back toward the measured envelope under continuous re-verification.

The frozen submitted circuit (baked defaults, no environment dependence):

- 1,385 qubits;
- 16,744,146 emitted operations;
- 1,163,310.460 average executed Toffolis on the deterministic 102,400-shot draw;
- score 1,611,184,350 (= 1,385 x 1,163,310; the op stream is serial, so executed
  Toffoli depth equals executed Toffoli count);
- `ops.bin` SHA-256 `c5c98a6a50d1d745c29ada53b09c949936d3fc3887d8188774b14af2a95b7399`.

That is 34.2% below the previous leader's 2,448,636,680.

Fresh-draw evidence (the property this contest actually gates on): zero classical
mismatches, zero phase-garbage batches, zero ancilla-garbage batches on the deterministic
draw plus 8 independent fresh `ECDLP_VALIDATION_SEED` draws (8 x 102,400 shots) of the
final stream, and the same on intermediate trims. Additionally, an exact classical model
of the circuit (validated shot-for-shot against `eval_circuit` on four separate eval
outcomes, including two single-shot failure identifications) predicts zero failures and
zero phase-residual events on 60 independent fresh 102,400-shot draws (6.1 million shots)
of the final configuration, and the same at each kept trim.

# Method

## Architecture (unchanged from the public lineage)

Affine point add P += Q with Q classical. The coordinate shell (modular subtract of Q,
times-three add, Solinas square, final reconstruction) surrounds two ping-pong
traversals: a signed binary-GCD-style value walk on (p, denominator) records one sign
qubit per round; a coefficient replay consumes that tape once; a reverse walk restores
the denominator and clears the tape. The divide traversal computes
lambda = (Qy - Py)/(Qx - Px) in place; the multiply traversal applies
lambda * (Px - x3). The replay is interleaved with the walk at checkpoints R1/R2 so the
replay executes while the tape is short; the peak is set by tape + two 256-bit
coefficient registers + walk registers + a headroom-scheduled carry ladder.

## Exact-hardening

The public head ships every guard as a finite truncation and grinds the Fiat-Shamir
nonce until one draw passes. Each truncation is a knob with a measured failure rate
(lambda, expected failing shots per draw). The failure channels were identified and
closed as follows:

1. Measured-erasure windows restored. `REPLAY_CHUNK_COMPARE` 21 -> 96 (full chunk: a
   tie now means equality; the scheduled chunk widths are <= 64 so this is exact),
   `REPLAY_FLAG_COMPARE` 20 -> 56, `REPLAY_FOLD_WINDOW` (+MUL) 53 -> 80,
   `ENDPOINT_FOLD_WINDOW` 18 -> 64 (full 64-bit slice), shell fold window `LSBS`
   53 -> 96, shell reduction compares `TLM_MSBS` 19 -> 256 (full width), square
   overflow compare 24 -> 64 and pad guard 24 -> 48. `TLM_COUT_ERASE_CAP` was verified
   dead in this composition (disabling it is byte-identical).
2. Walk convergence depth. At the shipped 696/694 rounds the walk non-convergence tail
   dominates (lambda ~ 100 per 102,400-shot draw).
3. Width-schedule guard band. The sampled per-round width schedule is fitted; rare
   shots need up to schedule+4 bits (worst case over 2.46M traversal samples).
   A uniform `SCHED_BIAS` closes that population and is peak-qubit-neutral.

## Margin trims (this revision)

With the envelope measured, margins were trimmed back under continuous verification
(each step: trusted deterministic + fresh-seed evals, plus 60 fresh 102,400-shot draws
under the exact classical model, all zero before the trim was kept):

- ROUNDS 816 -> 790 both traversals: worst observed convergence need is 747 rounds
  (over 2.46M traversal samples; 733 in the 790-round schedule's own envelope scan),
  so 790 keeps >= 43 rounds of margin. -72.6M score.
- SCHED_BIAS 8 -> 6: the worst observed width need (schedule+6 in the 790-round
  rescale) sits at slack 0 and survives by the walk's self-healing; a failure requires
  a never-observed need of schedule+7. -12.0M.
- REPLAY_FOLD_WINDOW 96 -> 80: the fold carry ladder sets the replay cell's width, so
  this also frees 16 qubits. Residual: carry escape needs 47 consecutive ones past the
  33-bit addend, ~2e-9 per draw across all folds. -54.3M.
- REPLAY_FLAG_COMPARE 64 -> 56: phase-only repair; a mis-repair needs a 56-bit tie,
  ~6e-9 per draw. -8.7M.
- REPLAY_CHUNK_COMPARE reviewed: no-op at 64 vs 96 (scheduled chunks are <= 64 bits
  wide, so both mean full-chunk); left at 96.
- Stream-level passes (ported from the lanetight tree: adjacent identical-CCX
  cancellation and commuting-window cancellation, hardened to compare the per-op
  classical condition bit) were evaluated and REMOVED: an exact-rule scan shows zero
  cancelable pairs in this stream even at window 100,000 (the lineage's internal
  constant-propagation already eliminated them), and maximal-run XOR factoring finds no
  factorable runs (99.998% of CCX ops are isolated singletons). No stream pass ships.

Executed Toffoli moved from ~914k (ported head) to 1,163,310; the peak from 1,263 to
1,385 qubits.

## Validation methodology

Because any knob change re-rolls the Fiat-Shamir commitment, per-draw failure rates were
measured against fresh `ECDLP_VALIDATION_SEED` values. The exact classical model (a
rebinding of the bit-exact walk/replay model to this repo's v3 draw derivation) was
checked against `eval_circuit` on: the baseline port (predicted 203 classical + 47
phase events vs eval's 206 + 164 batches), two mid-hardening configs where it predicted
the unique failing shot index exactly (shots 72706 and 19958, both confirmed), and the
final configuration (predicted 0, eval confirms 0). The two residual channels (walk
convergence, width slack) were measured over millions of samples with the same model,
which is what sets the round count, bias, and window floors above.

# Result

| metric | value |
|---|---|
| qubits | 1,385 |
| avg executed Toffoli | 1,163,310.460 |
| executed Toffoli depth | 1,163,310 (serial stream) |
| score | 1,611,184,350 |
| emitted ops | 16,744,146 |
| ops.bin SHA-256 | c5c98a6a50d1d745c29ada53b09c949936d3fc3887d8188774b14af2a95b7399 |

Validation: deterministic 102,400-shot draw 0/0/0; 8 fresh-seed 102,400-shot draws all
0/0/0 on the final stream (plus per-trim fresh draws, all clean); exact classical model
60 fresh draws (6.1M shots) zero failures, zero phase-residual events at the final
configuration.

## Caveat and what is left

This is "exact-enough", not a machine-checked all-input proof. Two channels remain
probabilistic in principle: walk convergence beyond 790 rounds (no worst-case round
bound is proven for this signed walk; measured tail puts this at ~1e-9 per draw or
below) and width-schedule excess beyond the +6-bit guard band (~1e-5 per draw by
extrapolation of the measured envelope; 0 observed in 6.1M+ shots). The window channels
are either exactly closed or bounded at ~1e-9 per draw. All bounds are direct
measurements of the circuit's own classical semantics, not analogy. A proven
convergence bound or a certificate-pinned width schedule would make the circuit exact
outright.

## Credit

The ping-pong division architecture, interleaved replay, width schedule, and tuning
knobs come from the public ecdsa.fail contest lineage (the `8510360` head and its
published memory notes). The exact classical walk model was ported from the same
lineage's public prefilter and revalidated on this harness. The cancellation-pass rule
was ported from the lanetight tree (found zero-yield here and not shipped). The
hardening, envelope measurement, margin trims, and this repo's integration are new.
