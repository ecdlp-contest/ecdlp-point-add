import assert from "node:assert/strict";
import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { after, test } from "node:test";
import { fileURLToPath } from "node:url";

import {
  assertArtifactCommitment,
  sanitizedChildEnv,
  validateArchiveEntries,
} from "./trusted-worker.mjs";

const ROOT_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const WORKSPACE_DIR = path.join(ROOT_DIR, ".workspace");
fs.mkdirSync(WORKSPACE_DIR, { recursive: true });
const TEST_DIR = fs.mkdtempSync(path.join(WORKSPACE_DIR, "trusted-worker-test-"));

after(() => fs.rmSync(TEST_DIR, { recursive: true, force: true }));

function makeArchive(name, entries) {
  const sourceDir = path.join(TEST_DIR, `${name}-source`);
  fs.mkdirSync(sourceDir, { recursive: true });
  for (const entry of entries) {
    const target = path.join(sourceDir, entry.path);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    if (entry.symlink) fs.symlinkSync(entry.symlink, target);
    else fs.writeFileSync(target, entry.contents ?? "fixture\n");
  }
  const archivePath = path.join(TEST_DIR, `${name}.tar.gz`);
  const roots = [...new Set(entries.map((entry) => entry.path.split("/")[0]))];
  const tar = spawnSync("tar", ["-czf", archivePath, "-C", sourceDir, ...roots], {
    encoding: "utf8",
    shell: false,
  });
  assert.equal(tar.status, 0, tar.stderr);
  return archivePath;
}

test("child environments retain build tools but remove credentials and seeds", () => {
  const env = sanitizedChildEnv({
    PATH: "/usr/bin:/bin",
    HOME: "/home/runner",
    ECDLP_TRUSTED_WORKER_TOKEN: "worker-secret",
    ECDLP_VALIDATION_SEED: "private-seed",
    GITHUB_TOKEN: "github-secret",
    ACTIONS_RUNTIME_TOKEN: "actions-secret",
  });

  assert.equal(env.PATH, "/usr/bin:/bin");
  assert.equal(env.HOME, "/home/runner");
  assert.equal(env.ECDLP_TRUSTED_WORKER_TOKEN, undefined);
  assert.equal(env.ECDLP_VALIDATION_SEED, undefined);
  assert.equal(env.GITHUB_TOKEN, undefined);
  assert.equal(env.ACTIONS_RUNTIME_TOKEN, undefined);
});

test("artifact commitment must match before trusted evaluation", () => {
  const artifactPath = path.join(TEST_DIR, "ops.bin");
  const artifact = Buffer.from("committed operation stream");
  fs.writeFileSync(artifactPath, artifact);
  const submission = {
    artifact_binary_sha256: crypto.createHash("sha256").update(artifact).digest("hex"),
    metrics: {
      artifact_binary_size_bytes: artifact.length,
      logical_qubits: 1,
    },
  };

  assert.doesNotThrow(() => assertArtifactCommitment(submission, artifactPath));
  assert.throws(
    () => assertArtifactCommitment({ ...submission, artifact_binary_sha256: "0".repeat(64) }, artifactPath),
    /artifact_binary_sha256 mismatch/,
  );
});

test("archive validation accepts only regular editable-path entries", () => {
  const manifest = { editablePaths: ["src/point_add"] };
  const valid = makeArchive("valid", [
    { path: "src/point_add/mod.rs", contents: "pub fn build() {}\n" },
  ]);
  assert.doesNotThrow(() => validateArchiveEntries(manifest, valid));

  const outside = makeArchive("outside", [
    { path: "src/point_add/mod.rs" },
    { path: "benchmark.json" },
  ]);
  assert.throws(() => validateArchiveEntries(manifest, outside), /outside editable paths/);

  const symlink = makeArchive("symlink", [
    { path: "src/point_add/mod.rs" },
    { path: "src/point_add/escape", symlink: "../../benchmark.json" },
  ]);
  assert.throws(() => validateArchiveEntries(manifest, symlink), /not a regular file or directory/);
});

test("worker orders artifact verification before private seed generation", () => {
  const worker = fs.readFileSync(path.join(ROOT_DIR, "tools", "trusted-worker.mjs"), "utf8");
  const processBody = worker.slice(
    worker.indexOf("async function processSubmission"),
    worker.indexOf("async function reportTrustedFailure"),
  );
  assert.ok(processBody.indexOf("assertArtifactCommitment(submission)") >= 0);
  assert.ok(
    processBody.indexOf("assertArtifactCommitment(submission)")
      < processBody.indexOf("crypto.randomBytes(32)"),
  );
});

test("runtime sandbox clears the environment and mounts an isolated proc", () => {
  const benchmark = fs.readFileSync(path.join(ROOT_DIR, "benchmark.sh"), "utf8");
  assert.match(benchmark, /--clearenv/);
  assert.match(benchmark, /--proc \/proc/);
  assert.doesNotMatch(benchmark, /--ro-bind \/proc \/proc/);
  assert.match(benchmark, /\/usr\/bin\/env -i/);
});
