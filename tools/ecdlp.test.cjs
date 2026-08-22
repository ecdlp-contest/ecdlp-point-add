const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

test("package enforces the 5-10 KiB section contract and submit requires the contender agent's verbal check", (t) => {
  const root = path.resolve(__dirname, "..");
  const workspace = path.join(root, ".workspace");
  fs.mkdirSync(workspace, { recursive: true });
  const fixture = fs.mkdtempSync(path.join(workspace, "ecdlp-submit-docs-test-"));
  t.after(() => fs.rmSync(fixture, { recursive: true, force: true }));
  const pointAdd = path.join(fixture, "src", "point_add");
  fs.mkdirSync(pointAdd, { recursive: true });

  fs.writeFileSync(path.join(fixture, "ops.bin"), "operation stream");
  fs.writeFileSync(path.join(fixture, "score.json"), `${JSON.stringify({ score: 20, metrics: { toffoli: 10, qubits: 2 } }, null, 2)}\n`);
  fs.writeFileSync(path.join(fixture, "benchmark.json"), `${JSON.stringify({
    schemaVersion: 1,
    name: "ecdlp-point-add",
    editablePaths: ["src/point_add"],
    scorePath: "score.json",
  }, null, 2)}\n`);
  fs.writeFileSync(path.join(pointAdd, "mod.rs"), "pub fn build() {}\n");
  fs.writeFileSync(path.join(pointAdd, "architecture.mmd"), `flowchart TD
  Target["Target primitive: quantum P plus classical Q on secp256k1"]
  Algorithm["Algorithm"]
  Optimization["Optimization"]
  Target --> Algorithm
  Target --> Optimization
  Algorithm --> Detail["Reversible algorithm and cleanup"]
  Optimization --> Result["Measured optimization and validation evidence"]
`);

  const cli = path.join(root, "ecdlp.js");
  const run = (...args) => spawnSync(process.execPath, [cli, ...args], {
    cwd: fixture,
    encoding: "utf8",
    shell: false,
  });

  fs.writeFileSync(path.join(pointAdd, "SUBMISSION.md"), "# Too short\n");
  const tooShort = run("package", "--model", "Test Agent");
  assert.notEqual(tooShort.status, 0);
  assert.match(`${tooShort.stdout}\n${tooShort.stderr}`, /between 5120 and 10240 UTF-8 bytes/);

  fs.writeFileSync(path.join(pointAdd, "SUBMISSION.md"), `# Too long\n\n${"detail ".repeat(2000)}`);
  const tooLong = run("package", "--model", "Test Agent");
  assert.notEqual(tooLong.status, 0);
  assert.match(`${tooLong.stdout}\n${tooLong.stderr}`, /between 5120 and 10240 UTF-8 bytes/);

  const paragraph = "The contender agent explains the algorithm, implementation, optimization tradeoffs, failed routes, validation evidence, measured result, caveats, and next steps in relevant detail.\n";
  let note = `# Point-add submission

## AI Model / Harness

Test Agent running under the Codex app with its stated effort level.

## Summary

The candidate is expected to beat the previous baseline through a measured
reduction in the scored resources.

## Method

The submitted reversible construction changes the implementation while
preserving its cleanup and correctness contract.

## Results

The note reports the measured score, resource tuple, and validation result.

## Comments

`;
  while (Buffer.byteLength(`Model: Test Agent\n\n${note}`, "utf8") < 5300) note += paragraph;

  fs.writeFileSync(path.join(pointAdd, "SUBMISSION.md"), note.replace("## Method", "## Approach"));
  const missingMethod = run("package", "--model", "Test Agent");
  assert.notEqual(missingMethod.status, 0);
  assert.match(`${missingMethod.stdout}\n${missingMethod.stderr}`, /missing required Markdown section 'Method'/);

  const emptyMethodNote = note.replace(
    "The submitted reversible construction changes the implementation while\npreserving its cleanup and correctness contract.\n",
    ""
  );
  fs.writeFileSync(path.join(pointAdd, "SUBMISSION.md"), emptyMethodNote);
  const emptyMethod = run("package", "--model", "Test Agent");
  assert.notEqual(emptyMethod.status, 0);
  assert.match(`${emptyMethod.stdout}\n${emptyMethod.stderr}`, /required Markdown section 'Method' must not be empty/);

  fs.writeFileSync(path.join(pointAdd, "SUBMISSION.md"), note);

  const packaged = run("package", "--model", "Test Agent");
  assert.equal(packaged.status, 0, `${packaged.stdout}\n${packaged.stderr}`);
  const validated = run("validate");
  assert.equal(validated.status, 0, `${validated.stdout}\n${validated.stderr}`);
  assert.match(validated.stdout, /package_status: valid/);

  const blockedSubmit = run("submit");
  assert.notEqual(blockedSubmit.status, 0);
  assert.match(`${blockedSubmit.stdout}\n${blockedSubmit.stderr}`, /AGENT_ACTION_REQUIRED/);
  assert.match(`${blockedSubmit.stdout}\n${blockedSubmit.stderr}`, /explicitly tell the user/i);
  assert.match(`${blockedSubmit.stdout}\n${blockedSubmit.stderr}`, /--confirm-docs-truthful/);
});
