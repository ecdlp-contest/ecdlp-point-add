# Generated arithmetic point addition

## AI Model/Harness

GPT-6 produced and reviewed this implementation. The arithmetic Rust routines were generated from an upstream formal/source representation and were never edited by hand. The submitted routines execute arithmetic construction to emit the circuit operations. The submission includes the implementation and its public explanation. It excludes private research infrastructure and source-generation inputs.

The formal provenance is scoped. Kernel-checked contracts support selected arithmetic identities and transformations; a separate complete operation comparison binds the source execution to the generated native construction. Neither those contracts nor the sampled validation establish universal correctness of the entire quantum channel. This submission makes an approximate, artifact-specific claim.

## Summary

The circuit adds a quantum point P to a classical point Q on secp256k1 under the benchmark point-add interface. Its local stock measurements are 1,299 qubits, 1,074,110 rounded executed Toffolis, 1,074,110 in the benchmark Toffoli-depth metric, and score 1,395,268,890. The same frozen circuit passed 102,400 stock shots and a fresh locally seeded 1,024,000-shot diagnostic, each using 20 CPU workers. Both observed zero classical mismatches, zero phase-garbage batches and zero ancilla-garbage batches.

The previous accepted 1,337-qubit route has a recorded trusted score of 1,428,858,585. This candidate's measurements are local results pending the server's private validation. A previous 1,299-qubit submission failed server validation despite passing its local cohort. Its acceptance is not claimed or inherited here. That history motivated retaining failure-derived point cases, explicitly checking signed interface capacity, and requiring a new post-fix million-shot audit of the actual regenerated circuit.

## Method

The field modulus is p = 2^256 - 2^32 - 977. On the supported ordinary affine domain, the slope is (Py - Qy)/(Px - Qx), the result's x coordinate is slope squared minus Px minus Qx, and the result's y coordinate follows from the slope and the difference between the old and new x coordinates. The construction combines reversible coordinate translations, modular division, squaring and multiplication with explicit cleanup of temporary state. The benchmark determines the reference point-add behavior and tested inputs.

Division and multiplication use an alternating signed binary GCD recurrence with a fixed 760-round schedule and coefficient replay. Following the initial lift, an odd residual y is replaced by (y + k*x)/2 for an odd residual x and k in {-1,+1}. The decision derives from the low bits so that the next residual is odd. The inverse reconstructs the previous residual from the quotient, the other residual and the retained decision. Both traversal directions are built explicitly; flags and measurement phase corrections are part of the generated arithmetic.

The signed width profile is an observed support envelope with a uniform eight-bit margin. It combines previously measured per-round requirements with retained point regression trajectories. Both the division and multiplication factors of each retained point case contribute complete trajectories. Three additional capacity witnesses discovered in a separate factor cohort are also included. The profile intersects these empirical requirements with the existing initial capacity and a conditional backward bound derived from unit termination at the fixed endpoint. These intersections have explicit premises; they do not turn an observed envelope into a uniform failure-probability theorem.

A narrower profile failed a retained point regression during preparation of this submission. Its signed result did not fit at round 248, so that candidate was rejected before native submission. The replacement changes the upstream capacity profile across the complete affected trajectories, then regenerates arithmetic. There is no input-specific branch and no edit to generated gates. All 232 retained full point regression lanes passed on the replacement, checking the independent affine reference, relative phase, temporary-state cleanup and signed walk behavior.

The preceding four-bit-margin native candidate passed stock testing but failed its fresh million-shot audit at shot 59,375, with one classical mismatch and one ancilla-garbage batch. Exact source replay reproduced its first signed overflow at round 51 across 64 measurement lanes; the failing division trajectory needed up to two extra bits. The new profile incorporates both controller trajectories and uses a uniform eight-bit margin. It passes that retained case and a separate scalar screen of 4,193,658 fresh factors with zero uniform width or convergence failures. Scalar screening is not counted as point-circuit shots, and the failed native candidate is not submitted.

The circuit separates the raw GCD state from coefficient replay. Contiguous replay groups and division/multiplication checkpoints are selected with a complete cost model that includes the retained decision history, live coefficients, raw forward and inverse traversals, prefix replay and terminal cleanup. The division checkpoints are rounds 17 and 742; the multiplication checkpoints are rounds 321 and 742. Grouping changes the time at which replay consumes retained controls, so workspace and reverse order must be accounted for together. Cost savings are checked against full source execution rather than inferred from a single isolated adder.

Compact replay providers are chosen according to the available scratch wires. Rolling carry layouts and deferred carry phase recovery reduce repeated work while retaining the information required by later consumers. A carry can be retired only when the construction preserves its recovery condition and the corresponding measurement phase. The source construction verifies that all deferred obligations have been consumed at completion. Native operation equality and sampled phase/cleanup checks supply additional evidence about this generated instance.

The existing 36-bit comparison predicate and 68-bit correction window remain part of the approximate arithmetic. They retain their support obligations. The coordinate and square interfaces are preserved while this candidate changes the signed width and replay arrangement. Its complete source estimate is 1,074,108 expected Toffolis under the measurement-guard cost model. Expected source cost and rounded stock execution measurements are different quantities; the score above uses the stock result.

## Result

| Quantity | Local result |
|---|---:|
| Peak logical qubits | 1,299 |
| Emitted CCX/CCZ operations | 1,171,372 |
| Rounded executed Toffolis | 1,074,110 |
| Benchmark Toffoli-depth metric | 1,074,110 |
| Stock score | 1,395,268,890 |
| Complete serialized operations | 22,757,961 |
| Stock shots on 20 workers | 102,400 |
| Stock classical / phase / ancilla errors | 0 / 0 / 0 |
| Fresh diagnostic shots on 20 workers | 1,024,000 |
| Diagnostic classical / phase / ancilla errors | 0 / 0 / 0 |

The frozen native circuit commitment is SHA-256 d912955fa7aaa2932461b721390e4ca9a43addab075703995552bee8548e0d22. Two independent executions of the generated builder emit identical bytes. Decompression and comparison of every serialized operation reproduce the complete source operation digest, including classical guards and measurements. Two source-to-Rust generations also agree byte for byte. The delivery contains arithmetic routines and reviewed width/layout data; it does not decode or embed a stored whole-circuit schedule.

The stock evaluator, simulator, reference arithmetic, benchmark dependencies and scoring formula were preserved. Rust 1.93.0 compiled the native delivery. The supplemental audit uses the existing diagnostic evaluator, whose changes relative to stock are the increased shot count, failure-input logging and progress reporting. Its simulator and validity checks match stock. A fresh local validation seed was fixed before that audit, with no favorable-seed retries. This audit is separate from the stock cohort and is not the server's private cohort. Its resource statistics do not replace the packaged stock score.

For reproduction, execute the generated arithmetic through the benchmark's standard build and emission process, then run the unchanged stock evaluator. Compare the circuit commitment before interpreting resource or accuracy differences. Host compilation and simulation time are experiment costs; faster simulation does not reduce logical qubits or target-circuit gates.

## Caveat and what is left

This remains approximate. The signed capacity envelope, fixed-round convergence, comparison-prefix and correction-window support, and exceptional affine inputs have separate obligations. Zero sampled errors do not prove a uniform failure bound or correctness on every input. Selected formal identities, source construction checks and complete operation equality remain separate from universal allocator and channel refinement proofs. Server acceptance is pending at submission time.

The below-1.35-billion research target is still open. At 1,299 qubits it requires at most 1,039,260 rounded Toffolis under the current metric. This candidate improves the measured rung while retaining the explicit distinction between a lower local score and a complete formal correctness claim. Further work must reduce complete replay or GCD cost without discarding signed information or shifting uncounted work into cleanup.
