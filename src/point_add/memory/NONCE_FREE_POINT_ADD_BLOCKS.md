# Nonce-free Kaliski/apply point-add baseline

The default circuit is built directly by `src/point_add/mod.rs`. It contains no
terminal identity pairs, tail rewrite, nonce, or seed-grinding parameter. The
trusted ECDSA Fail evaluator therefore derives its Fiat-Shamir inputs only from
the reversible arithmetic stream.

Both inverse applications use the conservative Kaliski bound
`2N - 1 = 511` for `N = 256`. This correctness-first schedule avoids the
truncated-walk failures seen in shorter experimental schedules under the
102,400-shot court.

## Circuit blocks

| Order | Phase label | Reversible transformation |
| --- | --- | --- |
| 1 | `coordinate_differences` | `dx = Px - Qx`; `dy = Py - Qy` |
| 2 | `pair1_kaliski_forward` | compute the raw Kaliski inverse of `dx` with 511 steps |
| 3 | `pair1_mul1` | multiply/apply `dy * inverse(dx)` into `lambda` |
| 4 | `pair1_halve` | remove the raw inverse's `2^511` scale |
| 5 | `pair1_mul2` / `pair1_kaliski_backward` | clear `dy` and uncompute the first inverse workspace |
| 6 | `x_coordinate_and_pair2_denominator` | form `Rx` and then `Qx - Rx` |
| 7 | `mul3_between_pair` | form the numerator needed to uncompute `lambda` |
| 8 | `pair2_kaliski_forward` | compute the raw inverse of `Qx - Rx` with 511 steps |
| 9 | `pair2_double` | align `lambda` with the raw inverse scale |
| 10 | `pair2_mul` | multiply/apply the inverse into `lambda` |
| 11 | `pair2_cleanup` / `pair2_kaliski_backward` | form `Ry`, clear `lambda`, and uncompute inverse workspace |
| 12 | `final_coordinate_restore` | restore `Rx` from `Qx - Rx` and free `lambda` |

The phase labels are build diagnostics and emit no gates. Ranking comes only
from the trusted evaluator.

## Trusted result

| Metric | Value |
| --- | ---: |
| Shots | 102,400/102,400 |
| Classical mismatches | 0 |
| Phase-garbage batches | 0 |
| Ancilla-garbage batches | 0 |
| Average executed CCX+CCZ | 5,180,786.000 |
| Peak logical qubits | 2,841 |
| Emitted operations | 40,922,100 |
| Score | 14,718,613,026 |
| `ops.bin` SHA-256 | `962f5c2c1e7a8f3fe4c65230af910e638870d914c50452ef71c7fa197e9e51a5` |
| Evaluation layout | 100 waves × 16 workers × 64 bit-sliced shots |
| Measured wall time | 4m 11.70s on Apple M1 |

This result was produced by the unchanged native `benchmark.sh` contract. The
server must reproduce it before promotion.
