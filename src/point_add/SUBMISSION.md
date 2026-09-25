# Generated 1298-qubit arithmetic point addition

## AI Model/Harness

GPT-5.6-Sol (High) produced and reviewed this implementation. The Rust arithmetic routines were generated from an upstream formal/source representation and were not edited by hand. Executing those routines constructs the arithmetic circuit and emits its operations. The implementation does not decode a stored whole-circuit schedule or substitute a precomputed gate array for arithmetic construction. Reviewed constants describe widths, checkpoints and layouts used by the builders.

The evidence is deliberately separated by scope. Kernel-checked contracts cover selected signed-representation, prefix-ordering, resource and control-lifetime identities. Complete operation comparison binds the source execution to the native builder output. The trusted evaluator supplies resource measurements and sampled accuracy for the frozen circuit. These layers support an artifact-specific approximate claim; none is presented as a universal proof of the complete quantum channel.

## Summary

This circuit adds a quantum point P to a classical point Q on secp256k1. Its local trusted resource tuple is 1,298 peak logical qubits, 1,031,994 rounded executed Toffolis, 1,031,994 in the benchmark Toffoli-depth metric, and score 1,339,528,212. The score is 471,788 below 1,340,000,000.

The exact circuit passed the stock 102,400-shot evaluation and a separate fresh 1,024,000-shot diagnostic audit, both using 20 CPU workers. Each run observed zero classical mismatches, zero phase-garbage batches and zero ancilla-garbage batches. Server-private evaluation remains pending at submission time.

The construction descends from the accepted 1,299-qubit arithmetic-builder architecture. It reduces the fixed signed walk from 750 to 741 rounds and uses a pointwise capacity profile. An initial 741-round candidate passed the stock cohort but failed the fresh million-shot audit. The defect was structural: a signed multiplication target needed more bits than its frame supplied. Widening only the first failing row merely moved the first overflow to a later row. The submitted candidate instead repairs the complete observed failing trajectory, regenerates the builders, and repeats all binding and accuracy checks on the new bytes.

## Method

The field modulus is p = 2^256 - 2^32 - 977. On the supported ordinary affine domain, the slope is (Py - Qy)/(Px - Qx). The new x coordinate is the slope squared minus Px minus Qx, and the new y coordinate follows from the slope and the old-to-new x difference. Reversible coordinate translations, division, squaring and multiplication implement this calculation and restore temporary state. The unchanged benchmark defines the reference behavior and tested input domain.

Division and multiplication use an alternating signed binary-GCD recurrence with 741 fixed rounds and coefficient replay. After the initial lift, an odd residual y is replaced by (y + k*x)/2 for another odd residual x and a sign k in {-1,+1}. Low bits select k so that the next residual is odd. Reverse execution reconstructs the preceding residual from the quotient, the other residual and the retained decision. Forward traversal, reverse traversal, retained controls, coefficient replay, cleanup and measurement-phase correction are emitted by explicit arithmetic routines.

The signed registers follow a per-round capacity profile rather than retaining the initial width for all 741 rounds. Widths can shrink and later regrow because every transition must still contain its signed inputs, doubled intermediate and signed output. Removing a bit without that support would lose information needed by reversal. The representation contracts establish encode/decode round trips inside the declared signed radius and relate a valid successor to its predecessor. They do not assert that every possible benchmark input remains inside every chosen radius.

The rejected predecessor exposed that distinction. At diagnostic shot 86,868, the multiply walk reached round 152 with a 221-bit signed target in a 220-bit frame. A one-row repair made round 154 the next overflow. Replaying the entire failing trajectory identified nine required frame changes: rounds 152 and 154 use 221 bits; rounds 159 and 160 use 219; round 162 uses 218; rounds 163, 164 and 165 use 217, 216 and 217; and round 168 uses 215. These requirements were applied upstream to every forward, replay and cleanup use of the affected frames. The resulting complete source price, rather than an isolated patch estimate, is used here.

The fixed-depth choice also received a fresh scalar holdout containing 4,193,658 rows. It observed one capacity failure, and that row was also the sole terminal-convergence failure; no capacity failure occurred among rows that reached the required terminal state. This finite observation helps separate the chosen capacity envelope from the declared 741-round nonconvergence event. It is empirical evidence, not a uniform convergence or capacity theorem.

Raw residual traversal and coefficient replay have separate state and lifetimes. Division uses checkpoints at rounds 17 and 728; multiplication uses checkpoints at rounds 354 and 728. Contiguous replay groups are fitted to the declared capacities and available scratch. The complete tariff charges retained history, live coefficients, forward and inverse raw work, replay, modular correction, phase recovery and terminal cleanup. The expected source cost is 1,031,993 Toffolis under the measurement-guard model; the trusted evaluator's rounded executed value is 1,031,994.

Control-lifetime checking requires every ordinary retained decision to be created before use, consumed by its replay owner, and released only after its final consumer. Both traversal directions satisfy the clean schedule, while deliberately injected use-before-create and premature-release schedules fail. A conditional commutation contract permits independent raw and coefficient actions to change order only when ownership and shared-control premises hold. This establishes the scoped logical schedule; physical allocation for all possible executions remains a separate open obligation.

Compact modular arithmetic providers use rolling carries and deferred phase recovery to share scratch. The ordinary modular path retains its 68-bit correction window and 36-bit phase-prefix limit. A separate signed-depth selection uses a 34-bit retained-prefix condition: distinct high prefixes determine unsigned order, while equal prefixes remain an explicit support obligation. Square and coordinate layers also retain their declared finite correction and prefix conditions. The final operation stream includes all guards, measurements, inverse work and cleanup required by these choices.

Two independent source-to-Rust generations agree byte for byte. Two executions of the generated native builder emit identical circuit bytes. Decompressing and comparing all 21,064,276 serialized operation records reproduces the complete source operation stream, including classical guards and measurements. The generated Rust therefore remains an output of the source representation rather than an input to it.

The stock evaluator measured the frozen circuit over 102,400 deterministic commitment-derived shots. After that gate passed, a fresh locally seeded diagnostic evaluated 1,024,000 shots with the same simulator and validity checks, adding only a larger shot count, progress reporting and failure-input capture. The fresh seed was fixed before execution without favorable-seed retries. Diagnostic resource output does not replace the packaged stock score.

## Result

| Quantity | Local result |
|---|---:|
| Peak logical qubits | 1,298 |
| Expected source Toffolis | 1,031,993 |
| Rounded executed Toffolis | 1,031,994 |
| Benchmark Toffoli-depth metric | 1,031,994 |
| Stock score | 1,339,528,212 |
| Headroom below 1.34B | 471,788 |
| Complete serialized operations | 21,064,276 |
| Stock shots on 20 workers | 102,400 |
| Stock classical / phase / ancilla errors | 0 / 0 / 0 |
| Fresh diagnostic shots on 20 workers | 1,024,000 |
| Diagnostic classical / phase / ancilla errors | 0 / 0 / 0 |

The source interpreter and generated Rust construction agree over every serialized operation record. Repeated generation and repeated native emission are deterministic. The trusted evaluator, simulator, reference arithmetic, benchmark manifest, dependencies and scoring formula are unchanged.

## Caveat and what is left

This remains approximate. Named bad events include signed-capacity overflow outside the finite envelope, failure to converge within 741 rounds, disagreement or equality in shortened prefix predicates, modular-correction-window support failure, shifted-term guard failure, noncanonical intermediate results, and exceptional affine inputs outside the declared support. Zero sampled errors do not establish a uniform failure probability.

Universal channel refinement, universal physical-allocation refinement, a uniform convergence bound and caller-level discharge of every conditional support premise remain open. The local resource and sampled-accuracy claims apply only to this exact generated artifact. Any further width, schedule or gate change requires a new source epoch, deterministic regeneration, complete resource measurement and fresh validation.
