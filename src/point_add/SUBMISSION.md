# AI Model/Harness

Model: GPT-6. Harness: Codex. This submission was developed with AI assistance.
No numerical effort setting is claimed. This report describes the fixed circuit,
its source-level construction, measured resources, and approximation limits.

# Summary

This candidate implements mixed affine secp256k1 point addition with 1,337 peak
logical qubits. The local 102,400-shot result is 1,068,707 rounded executed
Toffolis and 1,068,707 rounded Toffoli depth, giving a score of 1,428,861,259.
The local stock evaluation and a 1,024,000-shot diagnostic audit both found
zero classical, phase, and ancilla failures on the same fixed operation stream.
Independent server evaluation remains a separate requirement.

The selected predecessor uses 1,367 qubits and has a local score of
1,598,666,857. This child saves 30 qubits and 169,805,598 score units, a local
score reduction of approximately 10.62%. The change is made in the formal
representation and its source-level generators. The submitted circuit is
generated from that representation and checked against the sealed stream.

The arithmetic remains approximate. Finite iteration schedules, signed-width
assumptions, correction windows and measured comparison prefixes impose named
conditions. Successful shots provide evidence about the frozen circuit; they
do not prove correctness for every input or establish a uniform failure bound.

# Method

## Affine schedule

Let P=(x,y) be the quantum point and Q=(u,v) the preserved classical point. All
field operations use p=2^256-2^32-977. The public interface consists of two
256-bit quantum coordinates and two 256-bit classical coordinates. On supported
nonexceptional inputs, the slope is lambda=(y-v)/(x-u), and the answer obeys
x'=lambda^2-x-u and y'=lambda*(x-x')-y.

The reversible circuit first subtracts u from x and v from y. Division turns
the second coordinate into the slope. The first coordinate then adds 3u and
subtracts the slope square, producing x+2u-lambda^2=u-x'. Multiplication applies
the slope to that value. Subtracting v gives lambda*(u-x')-v=y', and a final
reverse subtraction produces x'. Keeping these intermediates in the coordinate
registers avoids a third persistent field register. The outer coordinate and
square blocks are retained from the selected predecessor.

## Signed walk and coefficient replay

Division and multiplication use 760 scheduled signed ping-pong rounds. Their
forward walks, coefficient application, and inverse cleanup are distinct
stages. The two campaigns retain different initial conditions and history
recovery schedules. Their finite round budget is part of the approximation
contract; it is not a universal convergence theorem.

The ordinary coefficient fold uses a 68-bit correction window and a 36-bit
phase-comparison prefix limit. Initial and special operations retain their
separate wider windows. The fixed schedule determines these choices before
evaluation. No runtime search or input-dependent circuit selection occurs.
The source representation records carry capacity, input and output widths,
and retained high information at the boundaries where replay changes shape.

A preceding width-trimming experiment passed the smaller local cohort but
failed the deeper audit. Diagnosis found that a high bit discarded at a replay
boundary could still carry information required by a later operation. The
current construction preserves that information in its representation. The
repair was made upstream and a new circuit was generated and frozen. The failed
candidate's passing receipts are not used as evidence for this submission.

For a raw half step, output capacity follows the retained raw word rather than
an optimistic estimate of its numerical width. For supported even inputs, the
post-half physical capacity is the preceding raw capacity minus one. Temporary
arithmetic width is chosen from the maximum of the input width, previous output
width, and new output width plus its required carry. This avoids automatically
adding an extra wire to whichever width happens to be largest, while preserving
the actual carry obligation. These identities are applied with explicit local
preconditions; the complete circuit is measured after composition.

Temporary measured values retain their matching phase corrections. A carry
that seems redundant for classical arithmetic may still influence phase, so
it cannot be deleted solely because a value test passes. Measurement consumers,
inverse replay, and release of temporary wires are checked separately. The
source-level changes preserve the existing outer arithmetic blocks and the
fixed measurement semantics of the generated operation schedule.

## Square and resource accounting

The modular square retains the recursive 128-bit split and sparse-prime
reduction. Low-half, high-half, and sum-square terms are combined with cleanup
after their consumers finish. Its 96-bit correction windows, 64-bit measured
comparison prefixes, and 48-bit shifted-term guards remain finite assumptions.
This child does not claim that retaining that block removes its boundary cases.

The source census gives expected executed Toffoli contributions of 507,453.5
for division, 507,817 for multiplication, 51,449.5 for the square, and 1,987.5 for
coordinate operations. Their sum is 1,068,707.5. Fractional values reflect
measurement-conditioned execution. They differ from the 1,141,863 emitted
CCX/CCZ operations and from the total count of all operation types. The stock
evaluator's measured mean and rounded score are reported separately below.

## Generation and delivery

The Rust circuit implementation is generated from a formal-level specification
and intermediate representation. The generated Rust was not manually edited.
The compact submission adapter is also generated from the specified operation
format. Arithmetic changes occur in the formal source layer and are regenerated;
generated candidate Rust is never used as an editable arithmetic template.

The submission contains a fixed compact operation schedule and a decoder.
Lossless decompression and per-plane byte prefix sums modulo 256 restore the
byte planes. Their inverse permutation reconstructs the intermediate records, then
the decoder expands their gates and ordered classical guards. This changes
storage only. It does not inspect evaluation inputs, choose measurement seeds,
perform host-side point addition, or download circuit data at runtime.

Full inverse-permutation equality checks the compressed schedule. Two native
emissions from the delivery build match the sealed operation stream byte
for byte. The stock builder and scorer remain unchanged. A fresh full stock
evaluation of the delivery build also passes all 102,400 shots. Source reproduction,
local semantic contracts, bounded checks, owner traces, and shot results are
separate evidence layers. No whole-circuit universal quantum-channel proof is
claimed by combining those layers.

# Result

| Metric | Local result |
|---|---:|
| Peak logical qubits | 1,337 |
| All emitted operations | 21,213,277 |
| Emitted CCX plus CCZ | 1,141,863 |
| Mean executed Toffolis, stock cohort | 1,068,707.4720507814 |
| Rounded executed Toffolis | 1,068,707 |
| Rounded Toffoli depth | 1,068,707 |
| Contest score | 1,428,861,259 |
| Stock validation shots | 102,400 |
| Diagnostic audit shots | 1,024,000 |
| Workers per evaluation | 20 |
| Classical / phase / ancilla failures, each run | 0 / 0 / 0 |

The operation-stream SHA-256 is
`877af1c65265b5cdda96694366d5e2b3d4162a4f9972bbe641bda6071c9392e2`.
The unrounded product of qubits and the stock mean is 1,428,861,890.1318946.
That diagnostic quantity differs from the submitted score, which uses rounded
resource metrics. The deterministic source expectation also differs slightly
from the finite cohort's measurement-conditioned mean.

The fixed circuit passed local stages of 64, 512, 1,024, 2,048, and 9,024 shots,
then the full stock cohort and the deeper diagnostic audit. The deep audit used
the same immutable operation bytes. Its local cohort overlaps the stock cohort;
the counts must not be added as independent evidence. Repeated stock evaluation
on a delivery decoder likewise establishes reproduction, not a new independent
statistical cohort. The server's undisclosed inputs and seed remain separate.

# Caveat and remaining work

Named failure conditions include equal input x coordinates, zero multiplication
factors, insufficient signed convergence, signed-width overflow, terminal
support failures, correction-window disagreement, comparison-prefix phase
disagreement, shifted-square overflow, final-negation disagreement, and
noncanonical zero. Local contracts have explicit assumptions. Preserved
structured boundary diagnostics outside those conditions include failures in
arithmetic components; passing random shots does not erase those limitations.

The corrected retained-high representation addresses the diagnosed truncation
mechanism and passes the new audit. It does not prove that every remaining
approximation is harmless. Universal channel and allocator correctness and
uniform probability bounds remain open. Broader domains, reversible fallbacks,
or further gate reductions would require changed formal artifacts, regeneration,
and a new fixed circuit with its own evaluation evidence.

# Credit

The selected predecessor reconstructs the public signed ping-pong architecture
of Vasily Gnuchev's 1,385-qubit entry, submission `sub_mtozpobm_tncua8`, associated
with public commit `83d48b960749bd3a05c43cda72ff40282e64a7d4`. This descent also
investigates donor techniques from the public ECDSA Fail work, including Teddy's
commit `897dda2b0cf267151ecd973252d2a5078cbf1b63`. Architectural ideas are reused;
donor validation receipts do not certify this generated child. The reversible
point-addition interface and measurement-aware evaluation contract come from
the ECDLP point-add contest and its acknowledged ECDSA Fail lineage.
