# AI Model/Harness

Model: GPT-6. Harness: Codex. This submission was developed with AI assistance.
No numerical effort setting is claimed. The report below describes the circuit,
its measured behavior, its limitations, and the public architecture it builds on.

# Summary

This candidate implements mixed affine secp256k1 point addition with 1,367 peak
logical qubits. The local 102,400-shot result is 1,169,471 rounded executed
Toffolis and 1,169,471 rounded Toffoli depth, giving a score of 1,598,666,857.
The local evaluation found no classical, phase, or ancilla failures. These are
local measurements; acceptance requires the contest's independent server run.

The architectural reference is the public 1,385-qubit signed ping-pong entry by
Vasily Gnuchev, published under the GitHub username gnuchev. Its ranked score is
1,611,199,585. This candidate reduces peak width by 18 qubits while allowing a
small increase in executed arithmetic. The resulting local score is about
0.778% lower. Qubit count alone is not the objective: the extra carry work is
useful only because the combined qubit and executed-resource score improves.

The arithmetic is approximate. Finite iteration schedules, signed widths,
correction windows and measured comparison prefixes carry explicit limitations.
The successful local sample is evidence about one fixed circuit, not a proof
that every possible input is handled correctly. No private-seed success is
claimed in this note before the server has evaluated the submission.

# Method

## Affine computation

Let P=(x,y) be the quantum point and Q=(u,v) the classical point, with field
arithmetic modulo p=2^256-2^32-977. The four 256-bit interfaces are quantum x,
quantum y, classical u and classical v. The classical point is preserved. For
the supported nonexceptional inputs, the secant slope is
lambda=(y-v)/(x-u), and the result is x'=lambda^2-x-u,
y'=lambda*(x-x')-y.

The reversible schedule first subtracts u from x and v from y. A divide
campaign turns the second quantum coordinate into the slope while retaining
the denominator needed by the inverse replay. The first coordinate then adds
3u and subtracts the slope square, yielding x+2u-lambda^2, which equals u-x'.
The multiply campaign multiplies the slope by that value. Subtracting v gives
lambda*(u-x')-v, which is y' on the same secant line. A final reverse
subtraction of u produces x'. This ordering keeps the intermediate values in
the two existing coordinate registers instead of allocating a third persistent
field register.

## Signed ping-pong division and multiplication

Both arithmetic campaigns use 790 scheduled signed rounds. The walk alternates
the roles of its working registers and records the decisions needed for later
replay. Forward computation, application to the target, and inverse cleanup
are distinct parts of the schedule. The narrowing signed-width table and the
termination conditions are assumptions of the approximate construction; the
fixed round count is not asserted to establish universal convergence.

Initial rounds use their known starting values to combine initialization with
the first walk operations. Recoverable history information is erased when the
retained state determines it, then reconstructed at the point required by
reverse replay. The divide and multiply campaigns retain their respective
sign-erasure and parity-recovery arrangements. The implementation does not
replace a signed-walk width schedule with an unsigned inversion schedule.

The width reduction comes from scheduling carry workspace and modular
correction banks so that storage can be reused across nonoverlapping stages.
Split carry layouts preserve the arithmetic correction windows. At each
composition boundary, temporary storage must be released consistently before
the next arithmetic stage uses those wires. Reducing the declared width by
discarding live state would not be a valid optimization.

Measured temporary values require their matching phase corrections. A temporary
AND or carry that looks classically redundant cannot simply be removed when it
still determines a measurement correction. The carry predicates, measurement
outcomes and correction gates remain paired through forward and inverse
execution. This distinction is essential because ordinary value tests alone
would miss phase errors.

## Squaring and modular correction

The modular square uses a recursive square construction with a 128-bit split.
Low-half, high-half and sum-square terms are combined in place, and temporary
product information is uncomputed after its consumers have finished. Reduction
uses the sparse constant 2^32+977 from the secp256k1 prime. The square component
uses 96-bit correction windows, 64-bit measured-overflow comparison prefixes,
and 48-bit guards for shifted terms. The finite windows are part of its
approximation boundary, not a claim of full-width arithmetic equivalence.

The expected executed Toffoli budget is 558,065 for division, 557,969 for
multiplication, 51,449.5 for the square, and 1,987.5 for the remaining coordinate
operations. These contributions sum to 1,169,471. Fractional expectations arise
from measurement-conditioned execution. They should not be confused with the
1,303,454 emitted CCX/CCZ operations or with the total count of all operation
types. The measured whole-circuit mean is reported separately below.

## Fixed construction and reproducibility

The Rust circuit implementation is generated from a formal-level specification
and intermediate representation. The generated Rust was not manually edited.
The compact submission adapter is also generated from the specified operation
format; it does not introduce hand-edited arithmetic. Changes are made upstream
and regenerated, with the resulting circuit checked against the frozen stream.

The submission contains a deterministic compact operation schedule and its
decoder. Lossless decompression restores that schedule, after which the
decoder expands the operations and ordered classical guards. This is a storage
choice only: it does not inspect evaluation inputs, choose a measurement seed,
alter the tested circuit, or perform host-side point addition for the evaluator.
There is no external runtime download or adjustable search parameter.

The delivery format is checked by decompression equality and by complete
native operation-stream equality. Independent emissions reproduce the same
stream. Arithmetic, gate order, register declarations, measurement identifiers
and conditional operations are fixed before evaluation. The trusted builder
and scorer are unchanged, and the scorer runs separately from the circuit
construction. Source generation and testing do not make the entire circuit a
universally proved quantum channel; that stronger claim is not made here.

# Result

| Metric | Local result |
|---|---:|
| Peak logical qubits | 1,367 |
| All emitted operations | 26,207,369 |
| Emitted CCX plus CCZ | 1,303,454 |
| Mean executed Toffolis | 1,169,470.5893945312 |
| Rounded executed Toffolis | 1,169,471 |
| Rounded Toffoli depth | 1,169,471 |
| Contest score | 1,598,666,857 |
| Local validation shots | 102,400 |
| Local evaluation workers | 20 |
| Classical / phase / ancilla failures | 0 / 0 / 0 |

The reviewed operation-stream SHA-256 is
`762f4fcc75f7d3509b5c39c8e4020a2890056f469f98df623fd53c24119dc61d`.
The score uses the contest's rounded resource metrics. The unrounded product
of qubits and mean executed Toffolis is 1,598,666,295.7023242; it is a diagnostic
quantity rather than the submitted integer score.

The fixed stream passed local stages of 64, 512, 1,024, 2,048 and 9,024 shots,
followed by the full 102,400-shot gate. A fresh evaluation of the final delivery
build also passes all 102,400 shots with the same result and matching stream.
Repeated runs with the same circuit-derived local seed demonstrate
reproducibility; they are not independent random cohorts. In particular, the
shot counts of such repeats must not be added together as a larger independent
statistical test. The server's undisclosed seed remains a separate admission
test and may expose failures absent from the local cohort.

# Caveat and what is left

The named failure conditions include equal input x coordinates, zero factors
in a multiply campaign, insufficient signed convergence, signed-width overflow,
terminal-support assumptions, modular correction-window disagreement,
comparison-prefix phase disagreement, shifted-square guard overflow,
final-negation window disagreement and noncanonical representations of zero.
The contest input generator excludes infinity and same-x exceptional pairs;
that exclusion does not remove the other approximation risks.

Structured boundary diagnostics have exposed value, phase or scratch failures
outside the asserted component conditions, including factor-288 and modular
square boundary cases. Those results are retained as limitations rather than
hidden by the successful random sample. No uniform failure-probability bound
is claimed, and local success does not justify describing the route as exact.

Further work would establish broader supported domains or reversible fallbacks
for the finite-window failures, and reduce arithmetic while preserving the
same cleanup and phase obligations. Any such change would require a newly
frozen stream and new evaluation evidence. This submission asks the server to
evaluate the fixed circuit described above; it does not claim success for an
unbuilt follow-up optimization.

# Credit

The signed ping-pong architecture is adapted from Vasily Gnuchev's public
1,385-qubit submission, identifier `sub_mtozpobm_tncua8`, at public commit
`83d48b960749bd3a05c43cda72ff40282e64a7d4`. That entry provides the architectural
reference. Its validation result is not reused as evidence for this candidate.
The reversible point-addition interface and measurement-aware testing contract
come from the ECDLP point-add contest and its acknowledged ECDSA Fail lineage.
