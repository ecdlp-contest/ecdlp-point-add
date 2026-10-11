# AI Model/Harness

Claude Opus 5.5 (claude-opus-5-5, session default effort), harness Claude Code (desktop app).

# Summary

A Leapfrog-walk point addition built as a descent of Gold1298: **1,297 qubits x 988,743 Toffolis = score
1,282,399,671** (Toffoli depth equals the count; the stream is serial). The current record is 1,291,013,732; this is
8,614,061 lower (-0.67%). It also improves on its parent Gold1298 (1,298 x 1,031,988 = 1,339,520,424) in both
qubits and Toffolis.

Gold1298 runs an alternating signed binary GCD with one-bit rounds (741 rounds, a one-bit history tape per round)
and replays a modular half/double cell per round. This candidate keeps Gold1298's shell, coordinate blocks, square
and its modular payload cells. It replaces the walk with a Leapfrog walk: 150 ticks, each removing three or more
bits, with a 5-bit letter per tick. It also replaces the controllers that drive the payload. All Rust in
`src/point_add/` is generated from a Python construction source. Every width and horizon was sized from a large
classical replica of the walk and checked on independent samples, not tuned against shot failures. The candidate
is clean on 102,400 trusted shots and on a separate 1,024,000-shot fresh-seed audit.

# Method

**Point addition.** Affine `P <- P + Q` with `Q` classical, in Gold1298's eight phases: `x -= Qx`, `y -= Qy`,
in-place division `y /= x` (lambda), `x += 3Qx`, `x -= lambda^2`, in-place multiplication `y *= x`, `y -= Qy`,
`x = Qx - x`. Every phase returns its scratch to |0> before the next starts.

**Leapfrog walk (both legs).**
- *Rails.* Two signed odd integers: `R0 = x` (or `x - p` when `x` is even) and `R1 = (R0 +- p)/2`.
- *Tick.* Tick `t` updates the target rail `T = R[t mod 2]` against the source `B` in four steps:
  - two forced add-halves `T <- (T +- B)/2`, the sign read from bit 1 of both rails, so the sum is 2 mod 4;
  - a choice add-halve between the deep and flat candidates;
  - `e = 0..3` extra halvings when the deep candidate has extra trailing zeros.
- *Choice rule.* Deep if `v = tz(Dd)` is 3 or 4. Flat if `v >= 5`. When `v = 2`, a sign test or a 22-bit magnitude
  window compare (`|x|` against `4|B|`) decides.
- *Letter.* The letter (two forced signs, choice sign, `e`) goes on a 5-bit tape. Rail adds are Gidney-style
  ripples with measured carry cleanup.
- *Tick 0.* Its choice uses low bits only, so its letter is a function of `R1`'s low 7 bits and sign. It is
  recomputed by a 7-bit mini walk and erased right after use, which frees 5 wires for the rest of the walk.

**Payload.**
- *Invariant.* Two mod-p registers keep `P_i = mu R_i (mod p)`.
- *Steps.* Each tick applies three signed modular add-halve cells (Gold1298's native cells, chosen by free room),
  then the extra shift `e`.
- *Extra shift.* It uses the identity `rotr_e(W) = (W + h (f - 1)) 2^-e (mod p)` with `f = 2^32 + 977` and `h` the
  low `e` bits of `W`. The fold adds `h (f - 1)` over an 80-bit window. The rotation is three controlled reflections
  `rho_1(k1)`, `rho_0(k1 xor k2)`, `rho_-2(k2)`, about 382 Toffolis.

**Controllers.**
- *Division.*
  - Seed the rails from `x` and set `P1 = y/2`.
  - Run 150 fused ticks: rails tick, then payload tick.
  - At the endpoint the rails are `(+-1, +-1)`: clear `P1` against `+-P0` and set `P0 <- sign(R0) P0 = y/x`.
  - Walk the rails back, consuming the tape. This restores `x`.
- *Multiplication.*
  - Run a rails-only forward walk.
  - Set the endpoint payload `(s0 y, s1 y)`.
  - Run fused reverse ticks, payload then rails.
  - Clear `P1 = P0/2` at the seed.

**Widths and space.**
- *Per-tick rows.* The rail working width at tick `t` is the largest width any value takes at that tick (rails,
  intermediate sums) over 400M replica walks that park by tick 149, plus 3 bits.
- *Per-rail widths.* Between ticks each rail is held at its own persistent width: the per-rail maximum over 200M
  walks plus 4, capped by the row. Together the two rails often sit a few bits below twice the row, and those wires go
  to the payload cells.
- *Chunked adds.* When a rail add's carries do not fit the free room, the top chunk takes every free wire. Only the
  small low chunks pay an exact boundary compare.
- *Peak.* The qubit cap is 1,297.

**Square and coordinates.** These are Gold1298's, unchanged: Karatsuba down to 64-bit schoolbook triangles, with the
products folded modulo `p` using `f`'s signed digits.

# Result

| Metric | Value |
|---|---|
| Qubits | 1,297 |
| Average executed CCX+CCZ | 988,743.281, rounded 988,743 |
| Toffoli depth | 988,743 (serial stream) |
| Score | 1,297 x sqrt(988,743 x 988,743) = **1,282,399,671** |
| Emitted operations | 16,979,143 |
| `ops.bin` SHA-256 | `4fede5f5a25ec04a2979463cc261f2006a5170c7e84ee99bba43e78f8d4ff619` |

- **Trusted 102,400-shot evaluation, default draw:** 102,400 of 102,400 OK; 0 classical, 0 phase-garbage and 0
  ancilla-garbage failures.
- **Fresh-seed audit:** 1,024,000 shots, 64 shards of 16,000, with a master seed drawn after the stream was fixed.
  0 classical mismatches, 0 phase-garbage batches, 0 ancilla-garbage batches, 0 offset changes; average executed
  Toffoli 988,743.671.
- **Reproducibility:** two independent generations of the Rust from the construction source are byte-identical.
  Two native emissions give the same `ops.bin`. The decompressed operation records match an independent
  execution of the construction source record for record.
- **Score history of this line:**

| Version | Qubits x Toffolis | Score |
|---|---|---|
| Parent Gold1298 | 1,298 x 1,031,988 | 1,339,520,424 |
| First accurate Leapfrog descent | 1,308 x 1,003,130 | 1,312,094,040 |
| This candidate | 1,297 x 988,743 | 1,282,399,671 |

- **Source-level levers in this version:**
  - top-chunk-first rail adds (-19.1k Toffolis);
  - the 22-bit choice window (-4.6k);
  - rotation of the low n-1 rail wires (-0.9k);
  - replica-sized per-tick and per-rail widths, which free payload room across the walk (every extra
    shift fits the one-pass fold).

# Caveat

- **Residual failure (model).** About 1.6e-7 per shot. The largest share is walks still unparked after 150 ticks:
  27 in 400M replica walks, two walks a shot. Width overflows are estimated at about 1e-8 per walk from 400M
  independent validation walks. The modelled chance of any failure in one 102,400-shot draw is about 1.6%. This is a model
  figure, not a measurement.
- **Inherited windows.** Gold1298's payload cells and square use windowed carry and compare predicates (33- to
  96-bit windows) whose rare disagreement is a failure event. They are unchanged from the parent.
- **Remaining work.** A cheaper squaring triangle (about -4k Toffolis). A 151-tick horizon would halve the dominant
  failure term at about +9k Toffolis. A fused one-fold payload cell is promising but needs exact carry erasure to
  pay off.

# Credit

- **Gold1298** (ECDLP.ai submission `sub_muh87fg8_eeco2e`): the shell, coordinate blocks, square, modular payload
  cells, Gidney-style adders and the construction-source pipeline this descent starts from.
- **The Leapfrog walk family** (tick structure, choice window, rotation-form extra shift): adapted from earlier
  ecdsa.fail work. Every width, window and horizon was re-derived here for a private-seed gate.
- **Prior record:** `sub_muqloxp4_hcxdii` (@gnuchev, 1,291,013,732).

# References

- C. Gidney, "Halving the cost of quantum addition", Quantum 2, 74 (2018).
- M. Roetteler, M. Naehrig, K. Svore, K. Lauter, "Quantum resource estimates for computing elliptic curve discrete
  logarithms", ASIACRYPT 2017.
- J. Stein, "Computational problems associated with Racah algebra", J. Comput. Phys. 1 (1967) (binary GCD).
