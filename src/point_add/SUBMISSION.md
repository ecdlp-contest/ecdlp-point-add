# Contextual zero-control reduction for the 1,283-qubit point-add circuit

## AI Model/Harness

This candidate was developed with AI-assisted circuit analysis, transformation
review, deterministic generation, and evidence review. Model attribution is
supplied by the submission metadata. The submitted circuit is a deterministic
replay and has no runtime dependency on the development process.

## Summary

This submission is a narrow cleanup of the ranked 1,283-qubit point-add
construction. It keeps the same four-register affine point-add interface, the
same 1,283-qubit allocation, the same arithmetic schedule, and the same
measurement and phase-cleanup structure. The only semantic change is the
deletion of 152 Toffoli operations whose control state is provably zero at the
exact operation boundary.

For a Toffoli with controls `a` and `b` and target `t`, the computational action
is `t <- t XOR (a AND b)`. If either control is zero on every reachable branch
at that boundary, `a AND b` is zero and the target is unchanged. The operation
is therefore the identity and can be removed. This is a contextual identity,
not a decomposition of an unrestricted Toffoli into Clifford gates.

The cleanup reduces the emitted operation count from 12,377,821 to 12,377,669
and reduces the emitted CCX count by exactly 152. It does not change the CZ
count, register layout, number of classical bits, or peak logical qubits. Since
the removed operations are unconditional, the executed Toffoli reduction is
exactly 152 for every identical set of evaluation inputs and measurement
outcomes.

The current trusted local evaluator completed 102,400 shots with zero classical
mismatches, zero phase-garbage batches, and zero ancilla-garbage batches. The
measured score is 3,210,081,396, below the live ranked score of 3,210,285,393.

## Method

### Start from an immutable accurate operation stream

The optimization begins with the accepted 1,283-qubit operation stream. The
parent is treated as immutable. A separate child stream is derived so that the
parent result, the child transformation, and the child evaluation cannot be
silently mixed. The child retains the complete operation order except at the
152 explicitly classified identity sites.

The source-level stream contains quantum inputs, classical inputs, clean
workspace allocation, reversible gates, classically conditioned regions,
measure-and-reset operations, and register declarations. The analysis covers
the complete stream rather than a prefix, a sampled trace, or a search over
rendered source text.

### Track only facts strong enough to justify a deletion

Each quantum wire is assigned one of three abstract values at every operation
boundary:

- `zero`, when the wire is known to be in computational value zero;
- `one`, when the wire is known to be in computational value one;
- `unknown`, when neither constant fact is established.

The two quantum input coordinates begin unknown. Fresh workspace begins zero.
Reversible operations update this abstract state conservatively. A NOT gate
swaps zero and one. A controlled NOT updates a known target only when the
control fact is sufficient; otherwise the target becomes unknown. Swap moves
the two abstract values. A Toffoli receives the same conservative treatment.
Measurement boundaries and phase workspaces are handled according to their
declared lifecycle rather than guessed from sampled behavior.

When an operation is classically conditioned, the analysis joins the state in
which the operation executes with the state in which it does not execute. A
wire remains constant after that join only if both branches agree. This avoids
using a fact that holds on one classical branch as if it held globally.

### Bind the identity to exact operation sites

A candidate is accepted only when all of the following agree:

1. the operation is CCX;
2. its absolute position in the frozen source stream matches;
3. its ordered pair of controls and its target match;
4. at least one control is `zero` immediately before the operation;
5. the operation is unconditional at the source boundary;
6. deleting it leaves all quantum and classical state unchanged.

The transformation fails closed if any position, operation kind, operand, or
stream commitment changes. It does not perform a textual replacement over the
submitted Rust. It consumes the bound operation declarations and regenerates a
new compressed operation stream and a small deterministic replay entry point.

### Restricted gate-replacement policy

The broader cleanup policy recognizes only replacements justified by the wire
state at a particular operation boundary:

- `CCX(0,b;t)` or `CCX(a,0;t)` is deleted because its target cannot flip.
- `CCX(1,b;t)` reduces to `CX(b,t)`, with the symmetric rule for the other
  control.
- `CCZ(0,b,t)` is deleted, while `CCZ(1,b,t)` reduces to `CZ(b,t)`.
- A CCX whose target is maintained in the minus phase eigenstate can reduce to
  a CZ between its controls because the target flip becomes phase kickback.
- A final-use AND workspace may use measurement-based uncomputation only when
  both measurement outcomes, the correction phase, the unchanged controls,
  and the workspace release are all covered by the same lifetime condition.

These rules are contextual. They do not imply that a generic CCX or CCZ can be
expressed using only CX and CZ. In the submitted child, the only newly applied
rule is zero-control CCX deletion. The other rule families either were already
present in the parent or had no newly eligible site under the strict boundary
conditions.

### Keep other gate reductions separate

The parent already contains 1,734 sites where a Toffoli target is held in the
minus phase eigenstate and the operation has been lowered to the corresponding
CZ phase action. Those sites are counted as already realized parent behavior;
they are not counted again as wins in this submission.

The analysis also considered final-use AND workspaces that might admit
measurement-based uncomputation. No site satisfied the strict complete-lifetime
matcher, so none was changed. Constant-one controls and unrestricted CCZ gates
were likewise left unchanged. This conservative policy is important: an
unrestricted CCX or CCZ is not equivalent to a composition of only CX and CZ.

### Regenerate and freeze the child

The 152 accepted declarations are applied to the source operation stream. The
result is lowered and serialized deterministically. The submitted Rust only
decodes that frozen child stream and exposes the required four-register point
addition entry point. It does not choose transformations at runtime.

The emitted circuit declares two 256-qubit target coordinates followed by two
256-bit classical offset coordinates. The target coordinates are overwritten
with the affine sum. The offset coordinates are preserved. All other allocated
qubits must return to zero, and the final global phase must be clean.

The child was frozen before evaluation. The same frozen operation commitment
was used for resource accounting and the complete trusted local validation.

## Results

### Whole-stream resource census

| Metric | Ranked parent | This child | Delta |
| --- | ---: | ---: | ---: |
| Peak logical qubits | 1,283 | 1,283 | 0 |
| Emitted CCX | 2,572,427 | 2,572,275 | -152 |
| Emitted CZ | 577,162 | 577,162 | 0 |
| Emitted operations | 12,377,821 | 12,377,669 | -152 |

The operation stream also contains 768 classical bits. The reduction does not
increase the measurement count or alter the condition-stack structure.

### Trusted local evaluation

| Measurement | Result |
| --- | ---: |
| Validation shots | 102,400 |
| Classical mismatches | 0 |
| Phase-garbage batches | 0 |
| Ancilla-garbage batches | 0 |
| Average executed CCX+CCZ | 2,502,012.001 |
| Rounded executed Toffoli | 2,502,012 |
| Rounded Toffoli depth | 2,502,012 |
| Average executed Clifford | 8,022,076.793 |
| Peak logical qubits | 1,283 |
| Emitted operations | 12,377,669 |
| Balanced score | 3,210,081,396 |

The frozen operation artifact has SHA-256 commitment
`8afad3ff46beb059ba86fd7024be94fc0305638a92d584AC517CC8DCE510CE1E`.
The score follows the contest definition: peak qubits multiplied by the square
root of rounded executed Toffoli count times rounded Toffoli depth. In this
operation model the charged depth equals the executed Toffoli count, so the
score reduces to `1,283 * 2,502,012`.

The live ranked parent is displayed at score 3,210,285,393. This candidate's
local score is lower by 203,997. Shot-dependent averages vary slightly between
independent validation seeds, while the structural reduction of 152
unconditional CCX operations is seed-independent for identical cases.

## Caveat and what is left

The 102,400-shot result is strong evidence about the frozen submitted circuit,
but it is not a proof over every possible curve input. The server's private-seed
validation remains the promotion authority.

This optimization is intentionally small and conservative. It removes no
qubits and does not attempt a general replacement of nonlinear three-qubit
gates. Larger improvements would require new arithmetic structure, additional
state invariants, or safe final-use workspace transformations. Any such change
should be derived and evaluated as a new immutable child rather than modifying
this submitted stream after validation.

The public payload contains only the minimal generated replay implementation,
its embedded operation data, and the documentation required by the contest.
