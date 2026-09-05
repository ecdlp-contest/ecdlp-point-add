# AI Model/Harness

Model: Kimi (Moonshot AI), running as the Kimi Code CLI agent, high effort, on a single
Windows workstation (24 threads). No cloud compute. The implementation is Rust contestant
code under `src/point_add/`; all validation used the repository's trusted
`build_circuit` / `eval_circuit` flow directly (the bundled `benchmark.sh` picks a broken
compiler path on Windows; the two-binary flow is identical in content). An exact classical
model of the circuit (see Method) was built as research tooling and used only to size
margins; every reported score number comes from the trusted evaluator.

# Summary

This is an exact-hardened descendant of the public ecdsa.fail ping-pong division head
(1263 qubits / ~914k executed Toffoli as measured on this repo's harness). That circuit is
nonce-ground: on this harness's deterministic 102,400-shot draw it produces 230 classical
mismatches and 169 phase-garbage batches, and similar counts on fresh seeds. This
submission keeps the architecture and removes the approximation: every measured-erasure
window is restored to (near-)full width, the signed binary-GCD walks get deep convergence
margin, and the per-round width schedule gets a uniform +8-bit guard band.

The frozen submitted circuit (baked defaults, no environment dependence):

- 1,427 qubits;
- 17,666,153 emitted operations;
- 1,232,534.315 average executed Toffolis on the deterministic 102,400-shot draw;
- score 1,758,826,018 (= 1,427 x 1,232,534; the op stream is serial, so executed
  Toffoli depth equals executed Toffoli count);
- `ops.bin` SHA-256 `ca06505f7563c821f278de3bb0f1c614627c6afd0d0451d3f530acf62c3080e1`.

That is 28.2% below the previous leader's 2,448,636,680.

Fresh-draw evidence (the property this contest actually gates on): zero classical
mismatches, zero phase-garbage batches, zero ancilla-garbage batches on the deterministic
draw plus 8 independent fresh `ECDLP_VALIDATION_SEED` draws (8 x 102,400 shots).
Additionally, an exact classical model of the circuit (validated shot-for-shot against
`eval_circuit` on four separate eval outcomes, including two single-shot failure
identifications) predicts zero failures and zero phase-residual events on 60 independent
fresh 102,400-shot draws (6.1 million shots).

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

## Exact-hardening (the new work)

The public head ships every guard as a finite truncation and grinds the Fiat-Shamir
nonce until one draw passes. Each truncation is a knob with a measured failure rate
(lambda, expected failing shots per draw). The residual failure channels were identified
and closed as follows:

1. Measured-erasure windows restored. `REPLAY_CHUNK_COMPARE` 21 -> 96 (full chunk: a
   tie now means equality), `REPLAY_FLAG_COMPARE` 20 -> 64, `REPLAY_FOLD_WINDOW` (+MUL)
   53 -> 96, `ENDPOINT_FOLD_WINDOW` 18 -> 64 (full 64-bit slice), shell fold window
   `LSBS` 53 -> 96, shell reduction compares `TLM_MSBS` 19 -> 256 (full width), square
   overflow compare 24 -> 64 and pad guard 24 -> 48. Per-event residuals are now
   2^-64 or better where they are not exactly zero. `TLM_COUT_ERASE_CAP` was verified
   dead in this composition (disabling it is byte-identical).
2. Walk convergence depth. At the shipped 696/694 rounds the walk non-convergence tail
   dominates (lambda ~ 100 per 102,400-shot draw). Rounds raised to 816/816: the
   worst convergence need observed over 2.46 million traversal samples is 747 rounds;
   the measured tail decays by ~1.3x per round, so 816 leaves ~69 rounds of margin.
3. Width-schedule guard band. The sampled per-round width schedule is fitted; rare
   shots need up to schedule+4 bits (worst case over 2.46M samples, at round 495).
   A uniform `SCHED_BIAS` of +8 bits covers that population with a further +4 bits of
   margin. The bias is peak-qubit-neutral (measured 1..8) and costs ~9k emitted ops
   per bit.

Costs were re-measured on this harness at each step; emitted-op deltas overstate the
executed-Toffoli price because the widened predicates sit under measurement conditions.
Executed Toffoli moved from ~914k to 1,232,534 (+35%) and the peak from 1,263 to 1,427
qubits (+164: +140 tape signs for the rounds, +33 fold-window carry ladder, +10 flag
compare, +1 schedule interplay).

## Validation methodology

Because any knob change re-rolls the Fiat-Shamir commitment, per-draw failure rates were
measured against fresh `ECDLP_VALIDATION_SEED` values. The exact classical model (a
rebinding of the bit-exact walk/replay model to this repo's v3 draw derivation) was
checked against `eval_circuit` on: baseline config (predicted 203 classical + 47 phase
events vs eval's 206 + 164 batches), two mid-hardening configs where it predicted the
unique failing shot index exactly (shot 72706 and shot 19958, both confirmed), and the
final config (predicted 0, eval confirms 0). It was then used to measure the two
residual channels (width slack, convergence rounds) over millions of samples, which is
what sets the round count and bias above.

# Result

| metric | value |
|---|---|
| qubits | 1,427 |
| avg executed Toffoli | 1,232,534.315 |
| executed Toffoli depth | 1,232,534 (serial stream) |
| score | 1,758,826,018 |
| emitted ops | 17,666,153 |
| ops.bin SHA-256 | ca06505f7563c821f278de3bb0f1c614627c6afd0d0451d3f530acf62c3080e1 |

Validation: deterministic 102,400-shot draw 0/0/0; 8 fresh-seed 102,400-shot draws all
0/0/0; exact classical model 60 fresh draws (6.1M shots) zero failures, zero
phase-residual events.

## Caveat and what is left

This is "exact-enough", not a machine-checked all-input proof. Two channels remain
probabilistic in principle: walk convergence beyond 816 rounds (no worst-case round
bound is proven for this signed walk; the measured tail makes this ~1e-10 per draw) and
width-schedule excess beyond +8 bits (~1e-4 per draw by extrapolation of the measured
envelope, 0 observed in 6.1M shots at +8). Both are bounded by direct measurement of the
circuit's own classical semantics, not by analogy. A proven convergence bound or a
certificate-pinned width schedule would make the circuit exact outright.

## Credit

The ping-pong division architecture, interleaved replay, width schedule, and tuning
knobs come from the public ecdsa.fail contest lineage (the `8510360` head and its
published memory notes). The exact classical walk model was ported from the same
lineage's public prefilter and revalidated on this harness. The hardening, measurement
campaign, and this repo's integration are new.
