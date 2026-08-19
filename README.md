# secp256k1 Point-Addition Circuit Contest

Build the lowest-score reversible circuit for adding a quantum secp256k1 point
to a classical secp256k1 point. The score is:

```text
round(average executed CCX + CCZ) * peak logical qubits
```

Lower is better. The native benchmark name remains `ecadd-challenge-test`; the
`ecdlp-contest` server routes its package to track `point-add-secp256k1-v1`.

This repository follows the ECDSA Fail trust boundary and the
`ecdlp-5-bit-contest` package/CLI workflow:

- contestant code is everything under `src/point_add/`;
- `build_circuit` is untrusted and emits `ops.bin` from contestant code;
- `eval_circuit` is trusted, does not import contestant code, and validates
  102,400 Fiat-Shamir shots in 100 waves of 1,024;
- `ecdlp package` archives only the native manifest `editablePaths` and wraps
  the native score/artifact with model attribution and a public note;
- the `ecdlp-contest` trusted worker overlays the archive on a clean baseline,
  reruns the benchmark, and promotes only a reproduced passing submission.

## Circuit contract

The circuit exposes four 256-wire registers:

| Register | Wire type | Contract |
| --- | --- | --- |
| 0: `target_x` | qubit | affine input `P.x`, overwritten with `(P + Q).x` |
| 1: `target_y` | qubit | affine input `P.y`, overwritten with `(P + Q).y` |
| 2: `offset_x` | classical bit | affine input `Q.x`, preserved |
| 3: `offset_y` | classical bit | affine input `Q.y`, preserved |

The curve is `y^2 = x^3 + 7 mod p`, where
`p = 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f`.
The deterministic generator excludes infinity and same-x exceptional cases.

A successful native ECDSA Fail run enforces:

- classical correctness on 102,400 self-seeded shots;
- reversibility and cleanup of freed workspace;
- phase cleanliness;
- forward/reverse identity.

## Nonce-free Kaliski/apply baseline

The checked-in default is a correctness-first affine point-add circuit adapted
from public ECDSA Fail commit
`da90a484510cf223b525ea84b3f2da9bccd6f8b3`. It stays close to the ordinary
Kaliski/apply construction: both inversion pairs use the conservative
`2 * 256 - 1 = 511` schedule, with explicit forward, multiply/apply, scale,
cleanup, and backward blocks. It has no identity tail, nonce, or seed-grinding
parameter.

The detailed reversible block schedule is documented in
`src/point_add/memory/NONCE_FREE_POINT_ADD_BLOCKS.md`.

The parallel v3 trusted evaluator measured:

| Metric | Baseline |
| --- | ---: |
| Validation | 102,400/102,400; zero classical, phase, and ancilla failures |
| Average executed CCX+CCZ | 5,180,786.000 |
| Peak logical qubits | 2,841 |
| Emitted operations | 40,922,100 |
| Score | 14,718,613,026 |
| `ops.bin` SHA-256 | `962f5c2c1e7a8f3fe4c65230af910e638870d914c50452ef71c7fa197e9e51a5` |
| Trusted evaluation | 16 workers × 64 bit-sliced shots; 4m 11.70s measured on Apple M1 |

### Validation-shot acknowledgment

The increase from 9,024 to 102,400 routine validation shots acknowledges
[Craig Gidney's critique](https://x.com/CraigGidney/status/2088847417022034364?s=20)
that a low-shot heuristic can miss sparse circuit errors and understate the
Toffoli cost of retries. Gidney specifically proposed 10 million shots and
retry-rate-aware scoring. The contest's 102,400-shot gate is an evenly batched
per-submission step in that direction and can scale by full 1,024-shot waves
for deeper audits.

### Secret validation seed

Shot count alone does not stop *grinding*. Because the trusted evaluator is
deterministic and can be run offline, a contestant could otherwise search — using
free nonce entropy — for a knowingly-incorrect but cheaper circuit that happens to
pass the particular cases a predictable seed selects, then submit it with
certainty.

To close this, the trusted reproduction worker draws a **fresh secret seed for
each submission, after the submitted `ops.bin` is locked**, and mixes it into the
Fiat-Shamir derivation of both the validation inputs and the per-shot measurement
randomness (via `ECDLP_VALIDATION_SEED`, folded into the SHAKE256 domains). The
102,400 cases a submission is tested against are therefore unpredictable at
submission time, so a sparse-error circuit cannot be pre-selected to pass them.

The **mechanism is public; the seed value is not disclosed in advance** — only the
scheme is (Kerckhoffs's principle), and a distinct seed is drawn per submission.
Contestant runs supply no seed and stay fully deterministic, so the checked-in
baseline reproduces its published score byte-for-byte and local development and
self-verification are unaffected; contestants simply cannot know which cases the
trusted rerun will draw. Ranking uses the trusted rerun's reproduced score and
metrics, not self-reported values.

After a submission is accepted, **its seed is published in the trusted-worker
report** so that anyone can replay the exact 102,400-case validation — check out
the accepted commit, rebuild, and rerun the evaluator with that
`ECDLP_VALIDATION_SEED` — and independently reproduce the ranked score.
Disclosing a locked submission's seed cannot aid grinding (the artifact is already
fixed), and every submission draws an independent seed, so a past seed reveals
nothing about a future one. Because the seed changes only which inputs are
validated (never the emitted circuit), the `ops.bin` commitment — and, for a
correct data-independent circuit, the score — remain exactly reproducible across
seeds.

## Architecture diagram contract

Every submission must update the Mermaid network at:

```text
src/point_add/architecture.mmd
```

The diagram must be valid UTF-8, at most 1 MiB, and start with `flowchart` or
`graph`. It must contain exactly one root with the exact label below, plus
exactly one `Algorithm` and one `Optimization` node:

```mermaid
flowchart TD
  Target["Target primitive: quantum P plus classical Q on secp256k1"]
  Algorithm["Algorithm"]
  Optimization["Optimization"]

  Target --> Algorithm
  Target --> Optimization

  Algorithm --> AlgorithmDetail["Explain the submitted reversible point-add construction"]
  Optimization --> OptimizationDetail["Explain the submitted score improvements and tradeoffs"]
```

The target must be the only root in the connected network and have no incoming
edge. Both explanation branches must have at least one outgoing edge to a
sub-child node. Add further nodes and cross-links to explain the submitted
algorithm, arithmetic blocks, cleanup, optimizations, metrics, and validation
evidence as a network—not merely as three labels.

`ecdlp package` commits the diagram path, byte length, and SHA-256 in submission
metadata. Local validation and the server both compare that commitment with the
diagram inside `submission.tar.gz`.

The trusted evaluator defaults to `ECDLP_EVAL_THREADS=16`. Every worker runs
64 bit-sliced shots, so 102,400 shots form exactly 100 full 1,024-shot waves.
Inputs and measurement randomness are derived from separate artifact-bound
SHAKE256 domains, including the batch index, so results do not depend on thread
scheduling.

## Local workflow

```bash
./ecdlp.js setup
./ecdlp.js preflight
./ecdlp.js run --note "describe the candidate"
```

The checked-in baseline passes the final command and writes the native minimal
`score.json`. A package is eligible for promotion only after the server's
trusted worker independently reproduces that pass:

```bash
./ecdlp.js package \
  --note-file src/point_add/SUBMISSION.md \
  --model "model name"
./ecdlp.js validate
```

Then authenticate and submit to the public contest server:

```bash
./ecdlp.js login <api-key>
./ecdlp.js submit --source-url https://github.com/<org>/<repo>/pull/<id> --watch
```

The CLI rejects equal-or-worse submissions against the current accepted
leaderboard. Server receipt is not promotion: ranked visibility follows only
after the trusted worker reproduces the archive and accepts it.

## Editable and trusted files

Edit only:

```text
src/point_add/
```

Keep the following trusted contract files unchanged when comparing or
submitting candidates:

```text
src/bin/build_circuit.rs
src/bin/eval_circuit.rs
src/circuit.rs
src/sim.rs
src/weierstrass_elliptic_curve.rs
Cargo.toml
Cargo.lock
rust-toolchain
benchmark.json
```

Keep `src/point_add/SUBMISSION.md` and `src/point_add/architecture.mmd`
synchronized with every submitted circuit. The note records the candidate and
evidence; the Mermaid network is a required package artifact.

## Generated files

`ops.bin`, `score.json`, `dist/`, and `.workspace/` are generated and ignored.
`results.tsv` is the append-only native benchmark history. Do not hand-edit any
of them.

## Attribution

The public challenge harness and baseline arithmetic come from
[ecdsafail/ecdsafail-challenge](https://github.com/ecdsafail/ecdsafail-challenge).
See `NOTICE` for the CC BY 4.0 attribution of reused ZKP-ECC files.
