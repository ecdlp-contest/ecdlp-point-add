# AI Model/Harness

Model: GPT-5.6 Sol with xhigh reasoning.

The implementation is Rust generated from a fixed source-level circuit description and
tested with the repository-provided evaluator. The submitted directory contains the Rust
implementation, its required action table, this note, and the architecture diagram. The
derivation below describes only public and submission-relevant facts.

# Summary

This submission is a 1,240-qubit descendant of the admitted 1,283-qubit
Schrottenloher/Qarton hybrid route. It keeps that line's compact reversible quotient and
product walks while independently applying the useful exact-rewrite ideas visible in the
public donor commit `d7d0995`. The donor did not provide the source-level authority for
this route, so its generated circuit and evidence were not inherited. The ideas were
re-derived, lowered to a new Rust source epoch, and evaluated again from new operation
bytes.

The frozen submitted circuit has:

- 1,240 qubits;
- 2,114,708 emitted CCX+CCZ gates;
- 11,541,887 emitted operations;
- 569,051 classical bookkeeping bits; and
- 1,974,705.988 average executed Toffolis over the final 1,024,000-shot local audit.

That audit used 20 evaluator workers and returned 0 classical mismatches, 0 phase-garbage
batches, and 0 ancilla-garbage batches. Its projected score is approximately
2,448,635,425. The final contest-CLI 102,400-shot run measured 1,974,707.404 average
executed Toffolis and a local score of 2,448,636,680. The emitted nonlinear-gate count is
0.70% above 2.100 million, which is the small reliability cost of retaining the uniform
cleanup margin while remaining below 1,250 qubits.

This is still an approximate route. Fixed-width guards leave explicitly bounded bad events;
finite random testing does not prove their universal absence. The server's private-seed
evaluation is therefore new evidence about this exact submitted stream, not a result
inherited from a parent, donor, or local run.

# Method

## Route derivation

The semantic starting point was the admitted 1,283-qubit hybrid route. Its reversible point
addition is organized around two compact 270-row binary-GCD-style walks: one for a quotient
factor and one for a product factor. Each walk stores an encoded row description, replays it
in reverse, and cleans the temporary state. Low-space coordinate subtraction, multiplication
by three, modular squaring, and affine reconstruction surround the two walks.

The public `d7d0995` submission showed that exact adjacent cancellation, compact fanout, and
generated block organization can reduce the cost of this family without changing the point
function. Those were used as design ideas only. The descendant was derived from the
1,283-qubit route's source semantics, and every row table, correction policy, Rust module,
operation stream, resource count, and shot result was generated afresh.

An earlier 1,227-qubit child was extremely close but failed the server's private seed at one
shot. That failure invalidated the epoch; a prior clean local run could not rescue it. The
route was therefore investigated by descending again from the 1,283-qubit authority rather
than editing the failed Rust or special-casing its input.

Several fresh children narrowed the failure family. The immediate 1,240-qubit predecessor
had 2,105,826 emitted CCX+CCZ gates and passed a local 102,400-shot run under one seed, but a
different deterministic commitment exposed one classical mismatch and one phase-garbage
batch. A complete 1,024,000-shot census of that same immutable stream, distributed over 20
workers, found four classical mismatches and two phase-garbage batches. One point was both
classically wrong and phase-dirty, leaving five distinct weak inputs.

## Why the failures were related

The five inputs were replayed through exact integer versions of the quotient and product
walks. Four classical failures first chose the wrong K2 row branch. Their failing rows and
required most-significant comparison widths were:

| Walk | Row | Old width | First sufficient width |
|---|---:|---:|---:|
| product | 181 | 59 | 60 |
| product | 167 | 60 | 62 |
| product | 157 | 62 | 64 |
| quotient | 168 | 59 | 61 |

In each case the retained prefixes tied even though the complete values were ordered. The
wrong branch then propagated through later arithmetic, so downstream value differences were
symptoms rather than separate bugs. All four exact factors still converged to the intended
terminal state within 270 rows when the exact comparison was used. This identified one
shared cause: the generated row comparator could discard the first distinguishing high bit.

The remaining point was different but nearby in structure. It had no incorrect K2 branch and
no row-width overrun. Its first phase error occurred in the controlled-add cleanup cell at
product row 60. For that cell the post-add word and addend first differ at absolute bit 230,
so the top 25 bits tie while the top 26 bits determine the correct cleanup predicate. An
isolated reversible replay confirmed that width 25 left phase garbage and widths 26 and 27
cleaned it. Changing the pseudo-Mersenne add fold width did not affect this point, ruling out
the fold as its cause.

Thus the failures form two related truncation classes, not five unrelated point exceptions:
K2 row decisions with a short high prefix, and controlled-add phase cleanup with a short
high prefix. Both compare an exact relation after retaining too little boundary information.

## Uniform source policy

The repair is a static operation-class policy rather than a list of observed rows:

- every K2 row comparison retains at least the top 64 active bits, while preserving any
  inherited row that was already wider;
- every controlled-add cleanup comparison retains the top 32 bits;
- controlled-add, controlled-subtract, and fused-scale pseudo-Mersenne folds retain 60, 62,
  and 63 bits respectively; and
- forward and reverse operations use the same selected widths.

The row rule is `max(inherited width, min(active width, 64))`. It changed 183 generated rows,
not merely the four rows seen in the census. The 32-bit cleanup rule likewise applies to the
complete controlled-add operation class, not only row 60. No input coordinate, factor,
random seed, shot number, expected answer, failure label, or evaluator state enters policy
selection.

For secp256k1, `p = 2^256 - 2^32 - 977`, so corrections use
`2^256 = 2^32 + 977 (mod p)`. The finite fold widths are valid when the selected low window
contains the correction carry or borrow. The high-prefix comparisons are valid when their
selected prefix contains the highest differing bit. These premises were checked for every
known counterexample and retained as explicit bad events for unobserved inputs.

The cleanup choice of 32 bits is six bits beyond the observed minimum of 26. Under the
stated heuristic that an unresolved high-prefix equality is no more likely than a uniform
prefix tie, a union bound can estimate residual risk, but that distribution assumption is
not presented as an all-input proof. The stronger width was retained because it costs no
additional peak qubits in this allocation and only a small number of gates.

## Generated implementation and cleanup discipline

The static row table and operation-class policy were set before Rust generation. The Rust in
this submission is the deterministic lowering result and was not edited afterward to make
the observed points pass. A new immutable source epoch produced a new operation stream; its
resource census and all evaluations refer to that same stream.

Each forward row has a matching reverse row. Encoded block slots are acquired, populated,
consumed during replay, and released in a fixed order. Coordinate shells similarly return
their work state after transferring the intended field result. Validation therefore treats
three properties independently: reference-coordinate equality, absence of residual phase,
and clean ancillas. Coordinate agreement alone would not establish a usable quantum circuit.

# Result

Before random testing, the five exact weak inputs from the rejected predecessor were replayed
against the fresh frozen stream. All five returned the correct coordinates with zero phase
and ancilla garbage.

The ordered local ladder on the same operation bytes was:

| Shots | Classical mismatches | Phase-garbage batches | Ancilla-garbage batches |
|---:|---:|---:|---:|
| 64 | 0 | 0 | 0 |
| 512 | 0 | 0 | 0 |
| 1,024 | 0 | 0 | 0 |
| 9,024 | 0 | 0 | 0 |
| 102,400 | 0 | 0 | 0 |
| 1,024,000 | 0 | 0 | 0 |

The final 1,024,000-shot audit used exactly 20 workers with 64 shots per worker batch. It used
the deterministic commitment-only local seed and the frozen stream identified by the
submission build. No Rust source, action mask, allocation policy, or operation byte changed
between resource measurement, exact weak-point replay, the ladder, and the deep audit.

The result supports the derivation hypothesis: all four observed classical failures close
under the uniform 64-bit row-comparison floor, and the phase-only failure closes under the
uniform 32-bit controlled-add cleanup prefix. The child retains the compact 1,240-qubit
allocation while paying 8,882 additional emitted CCX+CCZ gates over the rejected predecessor.

The server still validates with a private random seed. A server pass would admit this exact
child. A server failure would identify a remaining named finite-prefix or finite-fold bad
event and would invalidate the submitted epoch; it would not justify input-specific patching
or retroactively validate either the parent or donor bytes.
