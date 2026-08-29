# Qarton contest-ABI point-add port

## Scope

The active module is a generated operation replay derived from André
Schrottenloher's Qarton point-add implementation at commit
`9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`, built with Qarton 1.0.0. It adapts
the source's fixed-window selector interface to the contest's unconditional
arbitrary classical-Q ABI.

It declares quantum P.x and P.y followed by preserved classical Q.x and Q.y.
Q coordinates are copied into clean quantum words with classically conditioned
X operations; applying the same operations again unloads the word and preserves
Q. Runtime `3*Q.x` is built from one temporary Q.x word and two modular
additions.

Register layout in the frozen stream: qubits `[0,256)` are P.x and `[256,512)`
are P.y; classical bits `[0,256)` are Q.x and `[256,512)` are Q.y. Every `Hmr`
writes into `[512,768)`, so the preserved Q registers are never touched by a
measurement.

## Active profile

Both halves of the inversion — the binary-GCD value walk (`ToBitVector`) and the
coefficient replay (`ApplyBitVector`) — use the gate-efficient implementation.
The safety margins are `ITERATIONS_VAR = 4.0` and `U_PAD_VAR = 3.5`, giving a
schedule of 426 fixed rounds and a 710-bit packed dialog.

This differs from the previously accepted 1,283-qubit route, which paired a
gate-efficient value walk with a space-efficient coefficient replay. That mix
minimises qubit count; the contest score is a product, and the all-gate-efficient
mix wins it.

## Lowering

The contest operation set has no H gate. Qarton emits H in exactly two shapes,
both absorbed by the lowering:

- X-basis measurement, `h(q); msr(q,c)`, folds into a single `Hmr(q,c)`;
- a borrowed `|->` phase ancilla, where `x(q); h(q)` prepares the state, a
  `ccx(a,b,q)` kicks a phase back, and `h(q); x(q)` releases it. The four
  bracket gates are dropped and the Toffoli becomes `cz(a,b)`, carrying its
  classical controls.

Any other shape is rejected rather than silently mistranslated. The emitter was
validated by regenerating the upstream paper-signature circuit and reproducing
its published census exactly: 1,443 qubits, 256 classical bits, 10,515,375
gates, 1,842,771 CCX, 857,383 Hmr, 854,058 CZ.

The QPRT stream SHA-256 is
`5751bc09399bcb3bb921519fb5e004d9f65958d8241e05c2b4662d59798920f3`.

## Validation boundary

The exact frozen stream passed 102,400 deterministic local shots at 0 classical
/ 0 phase / 0 ancilla failures. It measured 1,482 qubits, 1,961,095.319 average
executed Toffolis and depth, 8,277,783.561 average Clifford operations, and
score 2,906,342,790.

This is cohort evidence, not an all-input proof or a server-private receipt. The
GCD round and padding margins are heuristic, and the special-prime
approximations remain subject to the trusted server's fresh private-seed
evaluation.
