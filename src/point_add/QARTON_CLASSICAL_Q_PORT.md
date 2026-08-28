# Qarton classical-Q point-add port

## Scope

The active module is a generated operation replay derived from André
Schrottenloher's Qarton point-add implementation at commit
`9b23c9170a636a7097a02afb3a3d6cbb6425c9f4`. It adapts the source's fixed-point
selector interface to the contest's unconditional arbitrary classical-Q ABI.

It declares quantum P.x and P.y followed by preserved classical Q.x and Q.y.
Q coordinates are copied into clean quantum words with classically conditioned
X operations and unloaded by applying the same operations again. Runtime
`3*Q.x` is implemented by three modular additions from one temporary Q.x word.

## Active profile

The active replay combines a gate-efficient binary-GCD value walk with a
space-efficient coefficient replay. Both inverse safety margins are 4.0, the
schedule has 426 fixed rounds, and the packed dialog uses 710 bits.

The source stream contains 12,904,572 decomposed operations and has SHA-256
`18a114949e77ad5f586f58523d8ba261616e0892568b29cfe2d80b69a05a4917`.
The generated wrapper and compressed QPRT payload are immutable outputs.

## Lowering

The target-neutral lowering maps adjacent Qarton `H; MSR` to HMR, replaces a
phase CCX into `|->` with CZ, preserves one classical control directly, and
represents two controls with the trusted condition stack. The accepted decoder
validates record boundaries and operands before producing contest `Op` values.

The QPRT SHA-256 is
`a4f6f22453d26643df63e977e79292068cc5b143f9705d585ae93bb3677354f7`.
The decoded operation stream SHA-256 is
`b9ab2819553e845a7aab7cf27c02fe4e280f71b11edcd40d26896414ddddac08`.

## Validation boundary

The exact frozen stream passed 102,400 deterministic local shots at `0/0/0`.
It measured 1,283 qubits, 2,502,170.191 average executed Toffolis and depth,
8,022,141.868 average Clifford operations, and score 3,210,284,110.

This is cohort evidence, not an all-input proof or server-private receipt. The
4.0-margin inverse and special-prime approximations remain subject to the
trusted server's fresh private-seed evaluation.
