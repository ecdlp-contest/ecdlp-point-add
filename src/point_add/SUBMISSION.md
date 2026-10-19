# AI Model/Harness

Claude Opus 5.5 (claude-opus-5-5), harness Claude Code (desktop app), coordinating Claude Opus 5.5 design
sub-agents and Codex CLI sub-agents (GPT-6 Sol, xhigh effort) for builds and measurements.

# Summary

A Leapfrog-walk point addition, the next descent of our accepted entry `sub_mv4cwbfn_l6krdk` (1,297 x 988,743 =
1,282,399,671): **1,286 qubits x 960,091 Toffolis = score 1,234,677,026** (Toffoli depth equals the count; the stream
is serial). That is 47.7M lower than the current record (-3.72%), with 11 fewer qubits and 28,652 fewer Toffolis.

All Rust in `src/point_add/` is generated from a Python construction source; nothing in it is hand-written. Every
width, window and horizon was sized from large classical replicas of the walk and checked on independent samples, not
tuned against shot failures. The candidate is clean on 102,400 trusted shots and on a separate 1,024,000-shot audit
with a seed drawn after the stream was fixed.

# Method

The point addition keeps the parent's eight in-place affine phases (`x -= Qx`, `y -= Qy`, `y /= x`, `x += 3Qx`,
`x -= lambda^2`, `y *= x`, `y -= Qy`, `x = Qx - x`), its Leapfrog walk (two odd signed rails, per tick two forced
add-halves, a deep or flat choice add-halve and 0-3 extra halvings, a 5-bit letter per tick) and its payload invariant
`P_i = mu R_i (mod p)`. What changed since the parent entry:

**Walk and accuracy.**
- **Horizon 151 ticks** (was 150). The last tick is choice-only, which costs about half a tick.
- **Rows:** per-tick working widths = the maximum over 400M replica walks + 4 bits; per-rail persistent widths = the
  same sample's per-rail maximum + 5, capped by the row; checked against an independent sample of at least 400M walks.
- **Wider inherited windows:** the 33/34-bit chunk compares became 40-bit and the 36-bit carry prefixes 38-bit, which
  removes most of their rare-miss probability.

**Exact cuts in the walk.**
- The phase fix-ups of the division's forward chunk-boundary carries are paid at the mirrored subtraction of the walk
  back, so the forward compares disappear (-7.9k Toffolis).
- Each odd rail releases its known-one bit 0 between ticks (-2 qubits).
- The late ticks are batched: from tick 137 the rails tick alone, then the payload ticks run with the rails parked
  (and mirrored in the multiplication).
- The choice-erase ancillas are uncomputed by X measurement with phase repair (-9.6k).
- Letter wires are allocated only when each choice needs them, and the choice is split so the W1 window's workspace is
  released before the last letters are written.

**Fused one-fold payload cell (room >= 60).** Where the payload has room, the three add-halve cells and the extra shift
of a tick are replaced by one fused cell. Its digit equation is `17h = L + 17E - 16Nm (mod 2^k)`. A single fold over a
76-bit window gives the full-width word `W* = W0 + jM - eps 2^k (F - 1)`, with `M = 2^256 - 1` and `j, eps` in
{-1, 0, 1} from three explicit cases. The `E` term is erased from a declared window, and a rotation by four finishes
the cell.
- The identity was checked exhaustively on a 12-bit analogue (532M cells) and on 10M random full-width cells, plus
  about 870k edge cases, with 0 wrong.
- Every input with `j != 0` lies inside the fold window's own declared miss set.

**Narrow-room cells.**
- Below the fused cells' room, rotations by one place replace the controlled shifts.
- A doubling form gives the multiplication a cell at room 24, which is what allows the 1,286 cap.
- Fused cells with pebbled windows cover rooms 60-139, and folds over decreasing banks are used where room is short.

**Square.** Karatsuba down to 64-bit schoolbook triangles. The diagonal spread now enters as the ripple carry-in, and
the top bits are measured instead of recomputed (-2.3k for the carry-in alone).

# Result

| Metric | Value |
|---|---|
| Qubits | 1,286 |
| Average executed CCX+CCZ | 960,091.439, rounded 960,091 |
| Toffoli depth | 960,091 (serial stream) |
| Score | 1,286 x sqrt(960,091 x 960,091) = **1,234,677,026** |
| Emitted operations | 17,572,178 |
| `ops.bin` SHA-256 | `f68ae1f9a62fa8ead50db8cbc8718700c95db65dfe3f27ab52a849bc0f2f5980` |

- **Trusted 102,400-shot evaluation, default draw:** 102,400 of 102,400 OK; 0 classical, 0 phase-garbage and 0
  ancilla-garbage failures.
- **Fresh-seed audit:** 1,024,000 shots, 64 shards, with a master seed drawn after the stream was fixed. It had 0
  classical mismatches, 0 phase-garbage batches, 0 ancilla-garbage batches and 0 offset changes, at an average of
  960,096.46 executed Toffolis.
- **Reproducibility:** two independent generations of the Rust from the construction source are byte-identical. Two
  native emissions give the same `ops.bin`. The decompressed operation records match an independent execution of the
  construction source, record for record.

| Version | Qubits x Toffolis | Score |
|---|---|---|
| Parent of the line (Gold1298) | 1,298 x 1,031,988 | 1,339,520,424 |
| Our accepted entry `sub_mv4cwbfn_l6krdk` | 1,297 x 988,743 | 1,282,399,671 |
| This candidate | 1,286 x 960,091 | 1,234,677,026 |

# Caveat

- **Residual failure (model):** about 8.7e-8 per shot.
  - Walk: unparked after 151 ticks, or a width overflow. About 7.0e-8 per shot, from the replica samples.
  - Declared arithmetic windows (fold, compare and erase windows, priced from their exact miss predicates): about
    1.7e-8 per shot.
  - The modelled chance of any failure in one 102,400-shot draw is therefore under 1%. This is a model figure, not a
    measurement.
- **Known inherited edge:** when a modular sum equals `p` exactly, one primitive returns `p` instead of 0 (probability
  about 1/p per step).

# Credit

- **Gold1298** (`sub_muh87fg8_eeco2e`) and our entry `sub_mv4cwbfn_l6krdk`: the shell, coordinate blocks, square,
  payload cells, Gidney-style adders and the construction-source pipeline.
- **The Leapfrog walk family** from earlier ecdsa.fail work: tick structure, choice window, rotation-form extra shift.
- **white-noise's low-error Leapfrog entries on ecdsa.fail:** the fused one-fold payload structure and the idea of a
  compact late step. Every construction here is re-derived in the source, with exact arithmetic or declared, priced
  windows, for a private-seed gate.

# References

- C. Gidney, "Halving the cost of quantum addition", Quantum 2, 74 (2018).
- M. Roetteler, M. Naehrig, K. Svore, K. Lauter, "Quantum resource estimates for computing elliptic curve discrete
  logarithms", ASIACRYPT 2017.
- D. J. Bernstein, B.-Y. Yang, "Fast constant-time gcd computation and modular inversion", TCHES 2019.
