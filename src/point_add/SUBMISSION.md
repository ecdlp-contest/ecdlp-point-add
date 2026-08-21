# Point-add submission

## Algorithm

This submission implements reversible mixed secp256k1 point addition. The
quantum target point `P` is overwritten with `P + Q`, while the classical
offset point `Q` is preserved. The affine shell computes coordinate
differences, uses two full 511-round Kaliski inverse/apply pairs, forms the
slope and output coordinates, and reverses all workspaces.

The operation stream is nonce-free. There is no identity tail, truncated
inverse schedule, seed grinding, probabilistic guard, or approximate cleanup.

## Exact generated optimizations

The Rust source applies three fail-closed, exact transformations to the clean
parent:

1. The squaring branch splits `x = a + 2^128 b` and uses
   `x^2 = a^2 + 2^128((a+b)^2-a^2-b^2) + 2^256 b^2`. It computes the three
   half products sequentially in 256/258-bit registers and uncomputes each.
2. Each secp256k1 Solinas `2^10` to `2^32` gap is implemented as 22 modular
   doubles, the original center add/subtract, and 22 inverse modular halves.
   This removes the former shift spill live range and moves the qubit peak.
3. A generated source-bound epoch pass applies 36 exact target-fanout
   identities across commuting contexts. It checks the expected input/output
   stream sizes, absolute indices, qubits, conditions, and condition depths;
   any parent drift aborts construction.

The square and Solinas transformations use full modular primitives. They do
not import reduced-width donor guards or partial overflow comparators.

## Trusted candidate evidence

The candidate was transplanted as only `src/point_add` onto contest main
`ca69c0326ee1548afb68c7fb6780fc70ecc8a595`, then rebuilt and evaluated by the
hardened native harness:

- 102,400/102,400 deterministic Fiat-Shamir shots passed;
- zero classical, phase, and ancilla failures;
- 2,820 peak logical qubits;
- 5,211,740.000 average executed Toffolis;
- 41,431,550 emitted operations;
- score 14,697,106,800;
- `ops.bin` SHA-256
  `9619157cd04d1fc4427f4b0268a3b9ecd5308c38bb2ecec505fa7de919073b8f`.

The accepted parent scored 14,718,598,821 at 2,841 qubits. This candidate
reduces the balanced score by 21,492,021. Structural alternatives were chosen
using deterministic operation/allocation accounting; the 102,400 shots were
used only as the final verifier, not as an optimization target.
