# Square correction with direct reuse of the lowest carry

## AI Model/Harness

GPT-6 through Codex desktop inspected the parent, derived and reviewed the rewrite, and prepared the Rust change. The harness used the repository builder and unchanged evaluator, with independent Python gate-program checks. 

The parent is commit `b436000fe15aff093a9e1c653cc40a5cc1f0346a` in `ecdlp-contest/ecdlp-point-add`. Its arithmetic Rust was generated from an upstream formal/source representation. This candidate applies a manual, limited specialization to that generated Rust. The parent's statement that all emitted operations were compared with its upstream representation is not a binding proof for this modified artifact. Evidence for the change consists of the stated local contract, its algebraic and phase argument, independent gate-level tests, and the candidate-specific build and evaluation results below.

## Summary

The triangular-square correction recomputes a lowest-bit carry already available as the input's lowest bit. The replacement passes that wire to the higher-bit adder and uses one CNOT to write or clear the lowest product bit, removing one CCX and HMR per call.

For the unchanged recursion schedule, nine square leaves are computed and subsequently uncomputed. Independent full-stream counting confirms 18 fewer unconditional CCX operations and 18 fewer HMR operations. This structural difference does not imply that two finite-sample mean Toffoli measurements differ by exactly 18. Removing HMR changes the evaluator's subsequent random-stream alignment, and later classically conditioned gates can have different sample counts.

The only circuit-implementation change is `arithmetic_75_adjust` in `src/point_add/arithmetic.rs`; its pads and cleanup remain. Round counts, capacity profiles, fold and phase windows, recursion threshold, coordinate schedule and trusted evaluation code are unchanged. No validation seed or nonce was searched.

## Method

Write the leaf input width as m and its lowest bit as a. The correction addend is the existing 2m-bit register `wide = x + pads`, with concatenation in little-endian order. In particular, its numeric value has the form `wide = a + 2W`. The original implementation invokes a full-width add or subtract on the 2m-bit product. The replacement operates on `wide[1..]` and `product[1..]`, passes `x[0]` as the carry input of the existing `chunk_add`, and applies `CX(x[0], product[0])`.

There are two different input contracts. During forward construction the product's lowest bit is zero when subtraction begins. During inverse construction the product's lowest bit equals a when addition begins. These statements require clean workspace and the retained-product contract. Runtime allocation does not reset an arbitrary dirty qubit. Equivalence is not claimed on dirty allocations or arbitrary product low bits.

The forward contract follows from the square construction: the leaf product starts clean, triangular rows act only on slices starting at bit `2*i+1`, and `spread` calls `zero_low_add`, which skips bit zero. None of these operations changes the lowest product bit before the forward adjustment. The inverse contract follows from `product = x*x`, hence `product[0] = x[0]`. Recursive inverse construction first reverses the combination of subproducts and then uncomputes each leaf. It relies on the square consumers preserving the retained product and its input until that inverse. This is a local construction contract, not a new proof of the inherited whole-circuit workspace invariant on every benchmark input.

For the forward case let `product = 2P`. Then, modulo `2^(2m)`,

`2P - (a + 2W) = a + 2(P - W - a)`.

The low result is a, and the high result subtracts W with borrow a. The original subtractor is implemented by complementing the target, adding the source, then complementing the target again. The replacement keeps those complements on the high target and supplies carry a directly to the high adder. The CNOT changes the clean lowest product bit from zero to a.

For the inverse case let `product = a + 2P`. Then

`(a + 2P) + (a + 2W) = 2(P + W + a)`.

The low result is zero, and the high result receives carry a. The same high adder with carry input `x[0]` applies, followed by the CNOT that clears the lowest product bit. The addend remains 2m bits wide, including its pads; no range or approximation window is narrowed.

In the original forward low-bit carry step the relevant AND is `a AND 1`; in the inverse step it is `a AND a`. Either writes a to a clean carry wire. Higher stages only use that carry as a control, so using `x[0]` in its place preserves their action and leaves the input bit unchanged. HMR of the old carry contributes `(-1)^(r*a)` for outcome r. The conditional CZ has controls `(a,1)` forward or `(a,a)` inverse, contributing the same phase. They cancel for either outcome, which is otherwise unused. Deleting this block preserves the local quantum channel on the stated subspace, including inputs entangled with external registers. The square stage has no pending deferred carry phase.

The structural count uses the parent threshold of 128 and its strict `m < threshold` leaf test. Each of the two 128-bit products has leaves of widths 64, 64 and 65. The 129-bit sum product has leaves of widths 64, 65 and 66. Nine leaves, each adjusted once forward and once backward, give the measured structural reduction of 18 CCX. Shortening a local carry lifetime does not establish a lower global Q; measure the completed stream.

## Result

The unchanged parent was reproduced locally with the original evaluator on two predetermined cohorts of 102,400 shots each. Both passed. The published-server-seed reproduction had mean executed Toffoli count 1,031,988.115, rounded count 1,031,988, 1,298 qubits, and Q times rounded count 1,339,520,424. The independently precommitted fresh cohort had mean 1,031,986.502, rounded count 1,031,987, the same 1,298 qubits, and product 1,339,519,126. These are local baseline measurements; they are not new private-server results for the candidate.

The independent local kernel check passed 78,984 gate-program simulations. For adjustment widths m=1 through 5, it tested every input x and every product high word satisfying the applicable low-bit contract, for both add and subtract. For complete triangular squares at m=1 through 9, it tested every x in both forward construction from zero and inverse cleanup from the square. It checked the exact output, restoration of input bits, every scratch release, and all remaining scratch bits at completion.

The phase check represents each independent HMR outcome by a symbolic variable over GF(2). These local programs use outcomes only as single classical guards on CZ gates, so the phase expression is linear in those variables. Requiring its coefficient bitset to be zero checks all measurement assignments for each tested basis input, rather than sampling measurement branches. Each old/new pair also had exactly one fewer CCX and HMR per adjustment. These are independently transcribed Python gate programs based on the parent arithmetic and simulator semantics. They are not an exhaustive execution of the patched Rust, and the small-width tests are not a full-size point-add proof.

Candidate-specific artifact and evaluation results:

- Candidate commit or immutable source identity: arithmetic.rs SHA-256 `d94d8247509b54131a4247aa1cdbc75a052017704dd6b6eeb136dfd160d14685`.
- Emitted stream SHA-256: `89971d50f27ebb8f1f29e664cb5a6eeb2b9af069ae77f79e0053d17c22ab05f2`.
- Build and static stream audit: release build passed; 21,064,168 records, 36,270,492 bytes; static CCX 1,111,810 to 1,111,792, all 18 removed CCX unconditional; CCZ remains zero; Q remains 1,298.
- Fresh cohort, 102,400 shots: PASS; zero coordinate mismatches, phase-garbage batches or ancilla-garbage batches; Q1298, rounded T1031975, benchmark depth1031975, score1339503550.
- Stock commitment-derived cohort, 102,400 shots: PASS; zero coordinate mismatches, phase-garbage batches or ancilla-garbage batches; Q1298, rounded T1031981, benchmark depth1031981, score1339511338.
- Final Q times rounded mean Toffoli comparison: stock local score is 9086 below the published parent score1,339,520,424; this is a finite-sample comparison, separate from the structural18-CCX reduction.
- Private-server status: pending at submission; no candidate server acceptance is claimed.

## Caveat and what is left

The parent uses approximate bounded arithmetic. Keeping its parameters does not establish that every secp256k1 input remains inside every inherited signed capacity, carry window, or convergence bound. The local channel argument is conditional on clean scratch and the specified product relation. It does not turn finite validation into a universal correctness or allocation proof for the complete point-add circuit. The unchanged evaluator checks coordinates, phase and final ancillary storage; no additional full reverse pass was performed here. Its depth field equals the rounded Toffoli count, not an independently measured parallel depth. Acceptance requires the server rerun. Deleted measurements change RNG alignment. Changing the operation commitment also changes the sampled inputs; parent/candidate results are not a paired-input comparison.

## Credit

[Jackie Chia-Hsun Lee and the accepted parent contributors](https://github.com/ecdlp-contest/ecdlp-point-add/commit/b436000fe15aff093a9e1c653cc40a5cc1f0346a) supply the point-add architecture, signed arithmetic, measurement-assisted cleanup, square decomposition, compiler runtime, parameter profiles and evaluation machinery. The parent submission identifies its model as GPT-5.6-Sol (High); that attribution is reported from the parent note. This contribution is limited to the lowest-carry specialization and its new review and verification evidence. It does not claim to originate the inherited arithmetic or the general technique of reusing a known value in reversible computation.
