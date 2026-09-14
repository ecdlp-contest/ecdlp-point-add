# Generated arithmetic point addition

## AI Model/Harness

GPT-6 produced and reviewed this implementation. The Rust arithmetic routines were generated from an upstream formal/source representation and were never edited by hand. The submitted routines perform arithmetic circuit construction and emit operations when executed. Reviewed width and layout constants remain data. The delivery contains the generated implementation and a public explanation of its algorithm, measurements and limitations.

Formal provenance is scoped. Kernel-checked contracts cover selected signed arithmetic, phase cancellation and scheduling identities. A separate complete operation comparison binds source execution to native construction. These are distinct evidence layers: neither a local identity nor a successful sample establishes universal correctness of the entire quantum channel. The submission makes an approximate, artifact-specific claim.

## Summary

The circuit adds a quantum point P to a classical point Q on secp256k1. Local stock measurements are 1,299 qubits, 1,039,062 rounded executed Toffolis, 1,039,062 in the benchmark Toffoli-depth metric, and score 1,349,741,538. This frozen circuit passed 102,400 stock shots and a fresh locally seeded 1,024,000-shot diagnostic, both using 20 CPU workers. Both observed zero classical mismatches, zero phase-garbage batches and zero ancilla-garbage batches.

The accepted parent used 1,299 qubits and had a trusted score of 1,395,267,591. The new candidate retains its arithmetic construction architecture while changing signed capacities, fixed traversal depth, carry-comparison width and replay scheduling. Its own local measurements satisfy the requested limit of fewer than 1,300 qubits and a score below 1,350,000,000. Server-private evaluation remains pending at submission time; the parent's acceptance is not evidence about these changed bytes.

## Method

The field modulus is p = 2^256 - 2^32 - 977. On the supported ordinary affine domain, the slope is (Py - Qy)/(Px - Qx). The new x coordinate is the slope squared minus Px minus Qx, and the new y coordinate follows from the slope and the old-to-new x difference. The circuit combines reversible coordinate translations, division, squaring and multiplication with explicit cleanup. The unchanged benchmark defines reference behavior and the tested input domain.

Division and multiplication use an alternating signed binary GCD recurrence with 750 fixed rounds and coefficient replay. After the initial lift, an odd residual y is replaced by (y + k*x)/2 for another odd residual x and k in {-1,+1}. Low bits determine the decision so that the next residual is odd. The inverse reconstructs the preceding residual from its quotient, the other residual and the retained decision. Both traversal directions, retained flags, and measurement phase corrections are explicit arithmetic construction routines.

The signed capacity profile starts from per-round signed requirements accumulated in earlier factor experiments and failure-derived point trajectories. It unions those requirements with the recorded envelope of a separate approximately 4.19-million-factor cohort, then adds three bits of margin. Where the requirement plus this margin is at most96 bits, it adds one further bit throughout that region. Both complete controller trajectories from the newly retained failure are included in the requirement union. The profile also intersects the initial capacity with a backward envelope conditional on unit termination at round 750. These bounds have separate premises. In particular, a recorded factor envelope is empirical, and changing the endpoint does not inherit a convergence theorem or a previous native validation result.

Widths may shrink and regrow. Each signed transition requires the input residuals, doubled intermediate and signed output to fit their declared frames. Truncating a bit without that support would lose information needed by reversal. A conditional representation theorem establishes encode/decode round trips within a signed radius, and a backward recurrence bounds predecessor magnitude when the endpoint and intervening equations hold. These identities explain the representation discipline; they do not establish support for every possible circuit input.

Before native generation, the complete changed source was replayed on 23 distinct retained point inputs across 64 measurement lanes. This cohort includes earlier natural failures and capacity witnesses. Independent affine reference results, phase, ancilla cleanup and signed walk transitions all passed. The regression observer was also exercised with deliberate nonzero phase and dirty temporary state. These are targeted regressions, not an independent random accuracy estimate and not part of the million-shot count.

An earlier candidate passed its stock cohort but failed the fresh million-shot diagnostic at shot479228, with one classical mismatch, zero phase-garbage batches and one ancilla-garbage batch. Complete source replay identified the first signed-width loss at round562: the result needed73 bits but the profile allocated72. Both controllers still reached unit endpoints by round750. The representation was then revised upstream using both full trajectories and a broader small-residual margin, affecting80 recorded requirement rows. The failing bytes were retained and were not submitted; this delivery requires its own new stock and fresh million-shot results.

The raw residual walk and coefficient replay have separate state. The new division checkpoints are rounds 17 and 728; multiplication uses rounds 354 and 728. Contiguous replay groups are refitted to the changed capacities and available scratch. The cost model includes retained history, live coefficients, raw forward and inverse work, prefix replay and terminal cleanup. Every replay price is checked against complete source execution, and the optimized total must equal the predicted full-circuit change.

Logical control-lifetime checks require each ordinary retained decision to be created, consumed once by coefficient replay, and released only after use. Both forward and reverse schedules pass, and deliberate use-before-create cases are detected. A separate conditional commutation identity permits independent raw and coefficient actions to change order when their ownership and shared-control premises hold. This logical scheduling evidence remains distinct from a universal theorem about the physical allocator.

Compact arithmetic providers depend on available scratch wires. Rolling carry layouts and deferred phase recovery retain the information needed by later consumers. The ordinary chunk carry-comparison cap is reduced from 36 bits to 33 bits, while the 68-bit correction window is retained. Strictly different retained prefixes suffice to decide the corresponding full unsigned order; equal-prefix cases retain an explicit support obligation. Deferred carry erasure must recover the same predicate attached to the same measurement outcome. All such construction obligations are consumed by completion in this generated instance.

The complete source estimate is 1,039,064 expected Toffolis under its measurement-guard model. This is distinct from rounded stock execution measurements and emitted gate count. All complete operations, including guards, measurements, inverse work and cleanup, are included in the source/native binding. The score reported here comes from the stock evaluator, not from the source estimate or an isolated arithmetic subtotal.

## Result

| Quantity | Local result |
|---|---:|
| Peak logical qubits | 1,299 |
| Emitted CCX/CCZ operations | 1,119,996 |
| Rounded executed Toffolis | 1,039,062 |
| Benchmark Toffoli-depth metric | 1,039,062 |
| Stock score | 1,349,741,538 |
| Complete serialized operations | 21,244,660 |
| Stock shots on 20 workers | 102,400 |
| Stock classical / phase / ancilla errors | 0 / 0 / 0 |
| Fresh diagnostic shots on 20 workers | 1,024,000 |
| Diagnostic classical / phase / ancilla errors | 0 / 0 / 0 |

The frozen native circuit commitment is SHA-256 e655053780df7969e6d723b3fd5b1312c2cf7c71b9b6194552c09aa0e8b962a7. Two source-to-Rust generations agree byte for byte. Two executions of the native builder emit identical bytes. Decompression and comparison of every serialized operation reproduce the complete source operation digest, including classical guards and measurements. The implementation constructs arithmetic; it does not decode or embed a stored whole-circuit schedule.

The stock evaluator, simulator, reference arithmetic, dependencies and scoring formula are unchanged. Rust 1.93.0 compiled the native delivery. The supplemental diagnostic uses the existing diagnostic evaluator, with increased shot count, failure-input logging and progress reporting; its simulator and validity checks match stock. A fresh local seed was fixed before the diagnostic, without favorable-seed retries. Its resource statistics do not replace the packaged stock score, and its cohort is separate from server-private testing.

## Caveat and what is left

This remains approximate. Named bad events include signed-capacity overflow, failure to converge within 750 rounds, disagreement of a shortened comparison predicate, correction-window support failure, and exceptional affine inputs outside the declared support. Zero sampled errors do not prove a uniform failure probability. Universal channel refinement, allocator correctness and complete caller-level instantiation of every conditional theorem remain open.

The local resource target is met by this exact generated artifact and its own tests. It does not imply that every narrower profile, shorter traversal or regrouping works. Further changes require a new source epoch, deterministic regeneration, resource measurement and fresh accuracy validation. Host compilation and simulation time are experiment costs; faster simulation itself does not reduce logical qubits or target-circuit gates.
