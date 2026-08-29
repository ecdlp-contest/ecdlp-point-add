# Gate-efficient 1,482-qubit Qarton contest-ABI point addition

## AI Model / Harness

This candidate was prepared with Anthropic Claude Opus 5 (1M context) running in
Claude Code, the terminal coding agent, driven interactively from the contest
repository root. No separate reasoning-effort setting was exposed to the task,
so none is claimed.

The work used the repository's own trusted pipeline unchanged: the pinned Rust
toolchain, the untrusted `build_circuit` stage, and the trusted `eval_circuit`
evaluator invoked through `./ecdlp.js run`. Circuit generation used Python
3.13 with Qarton 1.0.0 in an isolated, repository-local virtual environment.

The submitted Rust module is generated output. Neither it nor its compressed
QPRT payload contains a validation nonce, a sampled-input table, a target-side
correction, or any evaluator-derived patch. The operation stream is a frozen
byte artifact committed together with its SHA-256.

## Summary

The accepted 1,283-qubit frontier scores 3,210,284,110. It is a *hybrid*
inversion: a gate-efficient binary-GCD value walk paired with a
space-efficient coefficient replay. That mix minimises qubit count, but the
contest score is a product, `peak qubits x average executed Toffoli`, and the
product does not reward qubits alone.

This submission runs the **gate-efficient** implementation on *both* halves of
the inversion. It spends 199 more qubits (1,482 against 1,283) to remove
541,075 average executed Toffolis (1,961,095 against 2,502,170). The product
falls to **2,906,342,790**, an improvement of **9.47%**.

The frozen operation stream passed 102,400 deterministic local shots with zero
classical mismatches, zero phase-garbage batches, and zero ancilla-garbage
batches.

Three other routes were measured and closed before settling on this one, and
they are worth recording because they look attractive and are not:

- **Toffoli depth is not an independent lever.** `write_score` defines
  `toffoli_depth = toffoli` because the emitted stream is strictly sequential,
  so the score reduces exactly to `qubits x toffoli`. Parallelising cannot help.
- **Qubit re-indexing yields nothing.** The evaluator scores peak qubits as
  `max(qubit_index) + 1`, an index-space measure, so any slack between that and
  the true simultaneous-live count would be free score. Measured on the frontier
  stream with plain interval liveness and again with reset-aware liveness that
  splits each qubit's usage at `Hmr`/`R` events (705,165 segments over 1,283
  qubits): the workspace interval-graph optimum is 771, plus 512 pinned output
  qubits, exactly 1,283. Slack zero. The circuit sits at its full width for
  95.9% of the stream.
- **Peephole cancellation yields nothing.** Zero adjacent identical Toffoli
  pairs, and zero cancellable pairs within a 64-operation commutation window
  over a 400,000-Toffoli sample.

## Method

### Source and contest ABI

The construction is Andre Schrottenloher's windowed affine point addition,
arXiv:2606.02235, from the public `ec-point-addition` implementation at commit
`9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`, built with Qarton 1.0.0.

The upstream primitive adds a point selected from a compile-time window by a
quantum selector; its signature is `(selector[1], AffPoint[513])`. The contest
instead requires an unconditional addition of a runtime-classical point with
exactly four registers. Two changes achieve that, and nothing else in the
schedule moves:

1. The selector becomes an internal ancilla pinned to `|1>`. Every
   selector-derived control (`creg_is_not_0`) then evaluates true, so the
   arithmetic runs unconditionally while the surrounding upstream schedule is
   preserved verbatim.

2. The four window lookups load from the classical Q registers instead of a
   baked-in constant: one X per coordinate wire, conditioned on the matching
   classical bit. The load is involutory, so applying the same conditioned X
   gates again unloads the word, returns the ancilla to `|0>`, and leaves Q
   untouched. Runtime `3*Q.x` is built from one temporary Q.x word and two
   modular additions.

Register layout in the emitted stream, verified by scanning the artifact:
qubits `[0,256)` are P.x and `[256,512)` are P.y; classical bits `[0,256)` are
Q.x and `[256,512)` are Q.y. Every `Hmr` writes into `[512,768)`, so the
preserved Q registers are never touched by a measurement.

### Choice of inversion mix

`IPModMul` threads a single `gate_efficient` flag into both halves of the
inversion, `ToBitVector` (the value walk) and `ApplyBitVector` (the coefficient
replay). Splitting that flag and measuring all four mixes at the same safety
margins gives, in the paper signature:

| value walk | coefficient replay | qubits | CCX | product |
| --- | --- | ---: | ---: | ---: |
| gate | gate | 1,483 | 1,994,490 | 2.958e9 |
| space | gate | 1,483 | 2,084,218 | 3.091e9 |
| space | space | 1,235 | 2,587,660 | 3.196e9 |
| gate | space | 1,285 | 2,497,932 | 3.210e9 |

The last row reproduces the accepted frontier. This submission takes the first.

### Safety margins

The GCD walk is heuristic: it runs a fixed round count and fixed register
padding, and fails if either is exceeded. `ITERATIONS_VAR` sets the round
margin and `U_PAD_VAR` the padding margin; `gcd.py` couples them through
`n - iterations*0.5*1.415 + u_padding >= 0`.

Upstream defaults (2.4 and 2.3) are tuned for the paper's own success target,
not for a 102,400-shot gate. An independent Monte-Carlo of 60,000 GCD runs
re-derived the distributions rather than trusting the published constants:
iterations have mean 361.82 against the heuristic `1.413n = 361.728` and
standard deviation 9.524 against `0.6*sqrt(n) = 9.600`; padding has mean 8.09,
standard deviation 4.25, and a clean exponential tail with log-survival slope
-0.4441 per bit. Extrapolating that fitted tail, the upstream default
`U_PAD_VAR = 2.3` carries roughly a 34% chance of at least one failure across
102,400 shots.

This submission uses `ITERATIONS_VAR = 4.0` (426 rounds, 710-bit dialog, the
same round margin as the accepted frontier) and `U_PAD_VAR = 3.5`, whose fitted
padding-overflow probability is about 8.6e-10 per shot, or roughly 1e-4 across
the full gate.

### Lowering

The contest operation set has no H gate. Qarton emits H in exactly two shapes,
and both are absorbed by the QPRT lowering:

- X-basis measurement, `h(q); msr(q,c)`, folds into a single `Hmr(q,c)`;
- a borrowed `|->` phase ancilla from `MCXWithBorrowedBits`, where `x(q); h(q)`
  prepares the state, a `ccx(a,b,q)` kicks a phase back, and `h(q); x(q)`
  releases it. The bracket gates are dropped and the Toffoli becomes `cz(a,b)`,
  carrying its classical controls.

Any other shape raises rather than being silently mistranslated. The emitter
was validated by regenerating the upstream paper-signature circuit and
reproducing the census this repository documented for it exactly: 1,443 qubits,
256 classical bits, 10,515,375 gates, 1,842,771 CCX, 857,383 Hmr, 854,058 CZ.

This package also drops three QPRT payloads the active build never reads, and
their wrapper modules, keeping the shared replay decoder. That is packaging
hygiene only: the emitted stream, and so `ops.bin` and its SHA-256, are
identical with or without them.

## Result

Measured by the repository's trusted evaluator through `./ecdlp.js run`, with
the contestant stage sandboxed as usual:

| Metric | Value |
| --- | ---: |
| Score | **2,906,342,790** |
| Peak logical qubits | 1,482 |
| Average executed Toffoli | 1,961,095.319 |
| Average executed Toffoli depth | 1,961,095 |
| Average executed Clifford | 8,277,783.561 |
| Emitted operations | 11,771,529 |
| Validation | 102,400 / 102,400 |
| Classical mismatches | 0 |
| Phase-garbage batches | 0 |
| Ancilla-garbage batches | 0 |
| `ops.bin` SHA-256 | `c308973b390574348da2048180853ab07fd13b026b8fa6e81878c66b92458443` |

Against the prior accepted frontier at 3,210,284,110, this is a reduction of
303,941,320, or 9.47%.

Before lowering, Qarton's own `auto_test` checked the adapted circuit against a
direct evaluation of the affine chord map on 10 random inputs, confirming both
output coordinates and the preservation of Q.

## Caveat and what is left

The GCD margins are heuristic, not proofs. The reported failure probabilities
come from a fitted exponential tail on 60,000 samples extrapolated well beyond
the sampled range; they are evidence, not guarantees. The trusted server draws
a fresh private seed, and that evaluation remains authoritative.

`TRUNCATE`, `ITER_CAN_BE_Q`, `PADDING`, and `PADDING2` are left at their
upstream values. They are separate approximation sources with their own failure
probabilities, and they were not independently characterised here; the frontier
appears to use the same defaults.

The remaining headroom is visible but unclaimed. Lowering `U_PAD_VAR` to 3.0
would score near 2.83e9 at roughly a 0.3% rejection risk, which was judged a
bad trade. More interestingly, the circuit runs a fixed 426 rounds on every
input while the walk converges after 362 on average, and only 5.5% of emitted
Toffolis are classically conditioned; making the trailing rounds skippable
against a converged flag would cut the *average executed* count substantially
without touching worst-case correctness or qubit count.

## Credit

Circuit construction is derived from Andre Schrottenloher's
`ec-point-addition` at commit `9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`
(AGPL-3.0), built with Qarton 1.0.0. The contest-ABI adaptation, the QPRT
lowering and emitter, the margin analysis, and the inversion-mix selection are
this submission's own work.

## References

- Andre Schrottenloher, *Optimized Point Addition Circuits for Elliptic Curve
  Discrete Logarithms*, arXiv:2606.02235, https://eprint.iacr.org/2026/1128.pdf
- `ec-point-addition`, https://gitlab.inria.fr/capsule/qarton-projects/ec-point-addition
- Khattar, Shutty, Gidney et al., *Verifiable quantum advantage via optimized
  DQI circuits*, arXiv:2510.10967 (the "dialog" GCD representation used here).
