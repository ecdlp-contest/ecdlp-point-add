# Point-add submission

## AI Model / Harness

The packaged `Model:` prefix is the authoritative model identifier for this
submission. The historical accepted payload did not preserve a separate coding
harness or effort-level field, so this migrated canonical note does not invent
either value. Future contenders must replace this paragraph with the exact model,
effort level, and harness used before they confirm the documentation and upload a
new package.

## Summary

This submission implements reversible mixed secp256k1 point addition. The
quantum target point `P` is overwritten with `P + Q`, while the classical offset
point `Q` is preserved. The accepted candidate reduces the balanced
qubit-Toffoli-depth score from 14,718,598,821 to 14,697,106,800 by lowering the
peak logical-qubit count from 2,841 to 2,820 while retaining the exact arithmetic
and complete cleanup behavior of the parent construction.

The improvement comes from three source-bound transformations: an exact
half-product square, an exact modular walk across the secp256k1 Solinas gap, and
36 guarded target-fanout identities in commuting operation contexts. Each
transformation is fail-closed. It checks the source or operation-stream shape it
was derived for and aborts construction if the expected parent structure has
drifted.

The operation stream is nonce-free. There is no identity tail, truncated inverse
schedule, seed grinding, probabilistic guard, reduced validation target, or
approximate cleanup. Structural alternatives were chosen with deterministic
operation and allocation accounting; the full trusted shot campaign was used as
the final verifier rather than as an optimization oracle.

## Method

The circuit exposes an affine shell for adding a fixed classical secp256k1 point
to a quantum affine point. It computes coordinate differences, uses two complete
511-round Kaliski inverse/apply pairs, forms the slope and output coordinates,
and reverses every temporary workspace. The implementation preserves the
classical offset registers and finishes with only the updated target point live.

### Exact half-product square

The squaring branch decomposes a 256-bit input as
`x = a + 2^128 b` and evaluates

`x^2 = a^2 + 2^128((a+b)^2-a^2-b^2) + 2^256 b^2`.

It computes the three half-width products sequentially in 256- or 258-bit
registers. Each product is folded into the destination and then uncomputed
before the next product is made live. This shortens workspace lifetimes compared
with retaining a full-width multiplication result. The five signed secp256k1
fold terms use the same complete modular primitives as the parent; no partial
overflow comparison or reduced-width donor guard is substituted.

The temporary 129-bit sum used by the middle product is also uncomputed. The
forward and reverse paths therefore agree at the operation level, and the
optimization does not leave input-dependent garbage outside the declared target
registers.

### Exact Solinas power walk

Each `2^10` to `2^32` gap in the secp256k1 reduction is represented as 22 exact
modular doublings, followed by the original center addition or subtraction, and
22 inverse modular halvings. The walk avoids keeping the former shifted spill
register live across the center operation. Its purpose is a lifetime reduction,
not an arithmetic approximation: every step is a full modular primitive and the
reverse walk restores its workspace exactly.

Removing the spill lifetime moves the peak allocation point into Kaliski step 4.
That shift accounts for the accepted reduction in peak logical qubits while
leaving the field and point-addition contracts unchanged.

### Source-bound target-fanout pass

A generated epoch pass applies 36 exact identities to disjoint, commuting
contexts in the emitted operation stream. In each witness, a CCX writes a
temporary target, a compatible CX fans that value into an output, and the
inverse CCX is replaced by the corresponding CX/CCX sequence into the output.
The transformation is accepted only when its recorded absolute indices, lanes,
controls, targets, conditions, condition depths, and input/output stream sizes
all match.

These guards deliberately couple the generated rewrite to the reviewed parent
stream. A source edit that invalidates even one witness makes circuit
construction fail instead of silently applying the identity in a different
context. The 36 witnesses are disjoint, so the pass does not rely on an ordering
accident between overlapping rewrites.

### Cleanup and reversibility

Both Kaliski inverse/apply pairs use the complete 511-round schedule. The affine
shell reverses the slope, denominator, coordinate, square, Solinas, and generated
fanout workspaces after their contributions have been copied into the final
target. Freed qubits and non-register qubits are covered by the trusted cleanup
checks. The implementation does not exempt any optimization workspace from
phase, reversibility, or ancilla validation.

## Result

The accepted candidate was transplanted as only `src/point_add` onto contest
main `ca69c0326ee1548afb68c7fb6780fc70ecc8a595`, then rebuilt and evaluated by
the hardened native harness. The reviewed result is:

- 102,400 of 102,400 deterministic Fiat-Shamir shots passed;
- zero classical, phase, and ancilla failures;
- 2,820 peak logical qubits;
- 5,211,740.000 average executed Toffolis;
- 5,211,740 Toffoli depth;
- 41,431,550 emitted operations;
- balanced score 14,697,106,800; and
- `ops.bin` SHA-256
  `9619157cd04d1fc4427f4b0268a3b9ecd5308c38bb2ecec505fa7de919073b8f`.

The accepted parent scored 14,718,598,821 at 2,841 qubits. The candidate improves
that score by 21,492,021. The architecture diagram records the same algorithm,
optimization branches, cleanup path, resource tuple, artifact evidence, and
102,400-shot validation result.

## Caveat and what is left

The exact harness and effort level used for the historical accepted submission
were not retained in the repository evidence available during this contract
migration. That omission is stated explicitly above rather than filled with an
unsupported attribution. A future submission must record those details before
packaging.

The generated fanout pass is intentionally brittle: an upstream operation-stream
change requires regenerating and reviewing its witnesses instead of relaxing the
guards. Further reductions may be available in Kaliski workspace scheduling,
field multiplication lifetimes, or additional commuting fanout contexts, but no
such improvement is claimed here without a new complete trusted evaluation.
