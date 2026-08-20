# AGENTS.md — secp256k1 Point-Addition Circuit Contest

This repository is an optimization contest. Contestant code is limited to
`src/point_add/`; the benchmark, evaluator, manifest, toolchain, and other trusted
files define the scoring contract and must not be weakened or changed as part of
an optimization.

Read `README.md` and `benchmark.json` before changing contestant code. Use the
trusted benchmark to establish correctness and score. A generated or
hand-calculated score is not a verified result.

## Local autoresearch workspace

Autoresearch is expected. Maintain reproducible local research memory under
`.workspace/autoresearch/` instead of relying on chat history or leaving loose
files around the repository.

The entire `.workspace/` tree is ignored, machine-local working state. Never add,
force-add, commit, package, or submit anything under it. The research workspace
may contain contender-private ideas and outcomes; it is not an organizer-facing
deliverable.

### Bootstrap or resume

Before optimization research, create the following structure if it is missing.
Create missing files and directories without overwriting, truncating, or
reinitializing existing research memory.

```text
.workspace/autoresearch/
  ledger.jsonl
  current-best.json
  docs/
    STATE.md
    CODEMAP.md
    HYPOTHESES.md
    DECISIONS.md
    METHODS.md
    handoffs/
  tools/
    README.md
    scripts/
    analyzers/
    generators/
    tests/
    fixtures/
  claims/
  runs/
  worktrees/
  targets/
  cache/
  scratch/
```

Seed only missing documents. Their roles are:

- `docs/STATE.md`: the currently verified baseline, best candidate, active work,
  and blockers.
- `docs/CODEMAP.md`: navigation notes about relevant contestant source.
- `docs/HYPOTHESES.md`: prioritized, untested ideas.
- `docs/DECISIONS.md`: evidence-backed conclusions, including failed routes that
  should not be repeated unchanged.
- `docs/METHODS.md`: local measurement, comparison, validation, and retention
  conventions.
- `ledger.jsonl`: one compact terminal JSON record per experiment.
- `current-best.json`: a machine-readable pointer to the best fully validated
  local run.

Initialize `current-best.json` with null result fields when no candidate has been
locally reproduced by the trusted evaluator. README claims, estimates, partial
validation, and historical scores are not locally verified results.

When the workspace already exists, orient from it before proposing new work. Read
in this order:

1. `docs/STATE.md`
2. `current-best.json`
3. `docs/HYPOTHESES.md`
4. `docs/DECISIONS.md`
5. the latest records in `ledger.jsonl`
6. active entries under `claims/`

Do not recreate a workspace merely because it was produced by another agent or
session.

### Claims and parallel work

Before an expensive run, create `claims/<run-id>.json` containing the agent or
session identifier, hypothesis, owned source paths, intended resource use, and
start time. Do not duplicate an active claim. Claims coordinate agents sharing
one local workspace; they are not committed or synchronized through Git.

Remove or mark the claim complete when the run reaches a terminal state. An
abandoned claim should be annotated with the last known state and recovery steps.

### One experiment, one run ID

Every experiment receives a unique ID of this form:

```text
YYYYMMDDTHHMMSSZ-<short-slug>-<4-random-hex>
```

Create matching locations:

```text
runs/<run-id>/
worktrees/<run-id>/
targets/<run-id>/
```

Never reuse a run directory or silently change its hypothesis. Keep all files
specific to an experiment inside its run directory; do not create top-level
per-experiment folders such as `.workspace/logs-*` or
`.workspace/experiment-*`.

Each run directory should contain:

```text
manifest.json
hypothesis.md
source.patch
summary.md
logs/
  build.log
  evaluation.log
metrics/
  score.json
  result-row.tsv
  comparison.json
artifacts/
  index.json
```

Before computation, `manifest.json` must record:

- schema version, run ID, UTC creation time, and agent/session identifier;
- state `planned`;
- parent commit and exact contestant-source state;
- baseline run ID or an explicit statement that no verified baseline exists;
- one measurable hypothesis and one primary variable;
- acceptance criterion;
- planned commands and resource budget;
- related research-tool paths and hashes when they materially affect the result.

Run candidate builds and benchmarks in the matching isolated worktree. Use the
matching target directory as `CARGO_TARGET_DIR`. The benchmark writes fixed-name
outputs such as `ops.bin`, `score.json`, and `results.tsv`; isolation prevents
parallel candidates from overwriting or mixing those outputs.

### Repository-local build rule

All agent-initiated compilation, circuit emission, evaluation, build outputs,
and per-run build caches **must stay inside this repository folder**. On
Windows, executables built outside the checked-out repository may be denied by
filesystem policy or Windows Application Control even when compilation
succeeds. Read-only package-source caches managed by Cargo are not build-output
locations and need not be relocated.

- Start ordinary setup, preflight, build, and run commands from the repository
  root. For an experiment, start them from its matching repository-local
  worktree under `.workspace/autoresearch/worktrees/<run-id>/`.
- Set `CARGO_TARGET_DIR` to the matching absolute path under
  `.workspace/autoresearch/targets/<run-id>/`. Never point it at `%TEMP%`, the
  system temporary directory, a home-directory cache, another checkout, or any
  path outside this repository.
- Keep `ops.bin`, `score.json`, `results.tsv`, process logs, extracted sources,
  and temporary build products in the applicable worktree, target, run, or
  scratch directory under `.workspace/autoresearch/`.
- Before an expensive build, resolve and verify the repository root, worktree,
  and target paths. Abort if the worktree or target does not remain beneath the
  resolved repository root. Record the resolved working directory,
  `CARGO_TARGET_DIR`, and build platform in `manifest.json`.
- If a command is launched by a wrapper, task runner, PowerShell process, or
  shell script, set its working directory explicitly; do not rely on the
  caller's current directory.
- A dependency cache may seed a new target only when the cache is also inside
  this repository. The candidate crate must still be rebuilt in its matching
  target, and each experiment must retain its own worktree and target.
- If Windows blocks a newly built executable, do not copy the build outside the
  repository or attempt to weaken security policy. Close or annotate the run,
  then retry in a new run ID using repository-local paths. WSL is an acceptable
  fallback only when its worktree, target, and outputs map back into this same
  repository (for example under `/mnt/c/.../<repo>/.workspace/autoresearch/`).

PowerShell agents should establish the paths explicitly before invoking Cargo:

```powershell
$repoRoot = (git rev-parse --show-toplevel).Trim()
$runWorktree = Join-Path $repoRoot ".workspace\autoresearch\worktrees\<run-id>"
$runTarget = Join-Path $repoRoot ".workspace\autoresearch\targets\<run-id>"
$repoPrefix = [IO.Path]::GetFullPath($repoRoot).TrimEnd('\') + '\'
foreach ($candidatePath in @($runWorktree, $runTarget)) {
  $resolvedPath = [IO.Path]::GetFullPath($candidatePath)
  if (-not $resolvedPath.StartsWith($repoPrefix, [StringComparison]::OrdinalIgnoreCase)) {
    throw "Build path escapes repository: $resolvedPath"
  }
}
$env:CARGO_TARGET_DIR = $runTarget

Push-Location $runWorktree
try {
  cargo build --release --locked --offline
} finally {
  Pop-Location
}
```

For a non-experiment baseline run, the repository's own default `target/`
directory is acceptable because it is still inside the checkout.

Do not treat root `results.tsv` as the research ledger and never hand-edit a
generated benchmark output. Copy the applicable score and newly produced result
row into the run's `metrics/` directory.

### Research documents and tools

Put reusable research helpers in `.workspace/autoresearch/tools/`. Put a one-off
helper used by only one experiment in `runs/<run-id>/tools/`.

Research tools must:

- write generated output only inside the applicable run directory or `scratch/`;
- never fabricate or edit trusted scores;
- never weaken correctness, reversibility, cleanup, phase, or shot validation;
- never modify trusted benchmark files;
- record their path and SHA-256 in the run manifest when their behavior affects
  the interpretation or reproducibility of the result.

Do not put research notes or helper programs under `src/point_add/`. That entire
directory is submission-eligible. Only deliberately promoted circuit
implementation, its required architecture diagram, and its submission note
belong there.

Run summaries are the authoritative record of individual experiments. Global
documents are indexes and current-state memory, not substitutes for run evidence.
Keep verified facts separate from hypotheses and estimates.

### Close every run

Every run must end with exactly one verdict:

- `accepted`: correct and strictly improves the fully validated baseline;
- `rejected`: correct but equal, worse, or outside its acceptance criterion;
- `invalid`: compilation, classical correctness, reversibility, cleanup, phase,
  or other validation failure;
- `inconclusive`: incomplete, interrupted, noisy, or insufficiently validated.

Before closing a run:

1. Capture the exact contestant-source patch and final source state.
2. Preserve build and evaluation logs.
3. Record the complete score tuple, validation level, elapsed time, commands, and
   environment—not only the scalar score.
4. Record artifact paths, SHA-256 hashes, sizes, and retention decisions in
   `artifacts/index.json`.
5. Write `summary.md` with the verdict, evidence, interpretation, bounded negative
   findings, and best next action.
6. Append one terminal record to `ledger.jsonl`.
7. Update `docs/DECISIONS.md` when the run establishes a reusable conclusion.
8. Update `current-best.json` and `docs/STATE.md` only after a fully validated
   improvement.
9. Close the claim and write a handoff when unfinished work remains.

Large `ops.bin` files may be discarded after recording their hash and size unless
they are required to reproduce the current best candidate or diagnose a failure.
Preserve the source patch, manifest, metrics, summary, and artifact index for all
terminal runs, including rejected and invalid runs.

### Submission boundary

Before packaging or submitting, verify that the payload contains no autoresearch
documents, helper tools, logs, worktrees, target directories, caches, generated
artifacts, credentials, or machine-specific information. Only files allowed by
`benchmark.json` are contestant-editable and submission-eligible.
