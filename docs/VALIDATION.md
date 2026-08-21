# Validation security and reproducibility

The point-add contest uses a 102,400-shot trusted validation gate together with
a private, per-submission validation seed. The shot count reduces the chance
that sparse errors escape testing; the private seed prevents a contestant from
pre-selecting a knowingly incorrect artifact for the evaluator's exact cases.

## Why 102,400 shots

The previous 9,024-shot routine gate was increased in acknowledgment of
[Craig Gidney's critique](https://x.com/CraigGidney/status/2088847417022034364?s=20)
that low-shot heuristics can miss sparse circuit errors and understate the
Toffoli cost of retries. Gidney proposed 10 million validation shots and
retry-rate-aware scoring.

The contest uses 102,400 shots as a stricter, routinely affordable
per-submission gate. They form exactly 100 complete waves of 1,024 shots:

```text
100 waves * 16 workers * 64 bit-sliced shots = 102,400 shots
```

The count can scale by complete 1,024-shot waves for deeper audits. It is not
presented as equivalent to a 10-million-shot audit or as a substitute for
retry-rate-aware analysis.

## Why the validation seed is private

A high shot count alone does not prevent grinding if every validation case is
known before submission. With a public deterministic seed and free nonce
entropy, a contestant could search for a cheaper, knowingly incorrect circuit
that happens to pass the predictable cases and submit it with certainty.

For each submission, the trusted reproduction worker therefore:

1. rebuilds `ops.bin` without credentials or a validation seed and verifies its
   size and SHA-256 against the submitted commitment;
2. only after that commitment check, draws a fresh private seed;
3. passes that seed only to the trusted evaluator and folds it into the
   artifact-bound SHAKE256 domains for validation inputs and per-shot measurement
   randomness;
4. runs the 102,400-shot trusted evaluation;
5. publishes the seed in the trusted-worker report after acceptance.

The mechanism is public; only the seed value is private before the artifact is
locked and evaluated. Each submission receives an independent seed, so a prior
seed does not reveal future validation cases.

## Local and accepted-result reproducibility

Contestant runs supply no private seed and remain deterministic. The checked-in
baseline therefore reproduces its published `ops.bin`, score, and metrics
byte-for-byte during ordinary local development.

After acceptance, anyone can replay the trusted validation by checking out the
accepted source, rebuilding the circuit, and rerunning the evaluator with the
published `ECDLP_VALIDATION_SEED`. Publishing the seed after evaluation cannot
aid grinding because the submitted artifact is already fixed.

The seed changes only which cases are validated; it never changes the emitted
circuit. For a correct data-independent circuit, the committed `ops.bin` and
its score are stable across validation seeds. Ranking uses the trusted worker's
reproduced score and metrics, not a contestant's self-reported result.
