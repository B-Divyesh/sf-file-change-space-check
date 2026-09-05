import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { chmod, lstat, mkdir, mkdtemp, readFile, readdir, rm, stat, truncate, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, join, relative } from "node:path";
import test from "node:test";
import { chromium } from "playwright";
import { ensureBuild, paths, startSiteServer } from "./site-server.mjs";

await ensureBuild();

async function fixture(prefix = "fcsc-claim-") {
  const root = await mkdtemp(join(tmpdir(), prefix));
  const source = join(root, "source");
  const destination = join(root, "destination");
  await mkdir(source);
  await mkdir(destination);
  return { root, source, destination };
}

function runCli(args, options = {}) {
  return spawnSync(paths.binary, args, { encoding: "utf8", ...options });
}

function parsedPlan(sample, policy = "overwrite") {
  const result = runCli([sample.source, sample.destination, "--policy", policy, "--json", "--no-space-check"]);
  assert.equal(result.status, 3, result.stderr);
  return { result, value: JSON.parse(result.stdout) };
}

async function treeSnapshot(root) {
  const entries = [];
  async function visit(directory) {
    for (const name of (await readdir(directory)).sort()) {
      const path = join(directory, name);
      const metadata = await lstat(path);
      const record = { path: relative(root, path), mode: metadata.mode, size: metadata.size, type: metadata.isDirectory() ? "directory" : "file" };
      if (metadata.isDirectory()) await visit(path);
      else record.sha256 = createHash("sha256").update(await readFile(path)).digest("hex");
      entries.push(record);
    }
  }
  await visit(root);
  return entries.sort((left, right) => left.path.localeCompare(right.path));
}

function demoRootFromJson(value) {
  return join(value.source, "..");
}

test("@claim:demo-sandbox the CLI demo uses a new temporary sample", async () => {
  const cwd = await mkdtemp(join(tmpdir(), "fcsc-demo-cwd-"));
  try {
    const before = await readdir(cwd);
    const result = runCli(["--demo", "--json"], { cwd });
    assert.equal(result.status, 0, result.stderr);
    const value = JSON.parse(result.stdout);
    const root = demoRootFromJson(value);
    assert.ok(root.startsWith(tmpdir()));
    assert.equal(value.summary.conflicts, 1);
    assert.equal(value.actions.length, 6);
    assert.equal(await stat(join(root, "demo-manifest.json")).then((item) => item.isFile()), true);
    assert.deepEqual(await readdir(cwd), before);
    await rm(root, { recursive: true });
  } finally {
    await rm(cwd, { recursive: true });
  }
});

test("@claim:read-only-plan planning leaves source and destination unchanged", async () => {
  const sample = await fixture();
  try {
    await mkdir(join(sample.source, "archive"));
    await writeFile(join(sample.source, "archive/new.txt"), "new file");
    await writeFile(join(sample.source, "conflict.txt"), "new conflict");
    await writeFile(join(sample.destination, "conflict.txt"), "old conflict");
    const before = await treeSnapshot(sample.root);
    const { value } = parsedPlan(sample, "overwrite");
    assert.equal(value.summary.conflicts, 1);
    assert.deepEqual(await treeSnapshot(sample.root), before);
  } finally {
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:metadata-only regular file contents are not required", async () => {
  const sample = await fixture();
  const locked = join(sample.source, "locked-video.mov");
  try {
    await writeFile(locked, "contents cannot be opened by the test user");
    await chmod(sample.root, 0o755);
    await chmod(sample.source, 0o755);
    await chmod(sample.destination, 0o755);
    await chmod(locked, 0o000);
    const args = [sample.source, sample.destination, "--policy", "overwrite", "--json", "--no-space-check"];
    const result = process.getuid?.() === 0
      ? spawnSync("setpriv", ["--reuid=65534", "--regid=65534", "--clear-groups", paths.binary, ...args], { encoding: "utf8" })
      : runCli(args);
    assert.equal(result.status, 3, result.stderr);
    const value = JSON.parse(result.stdout);
    assert.equal(value.summary.files_scanned, 1);
    assert.equal(value.actions[0].operation, "copy");
  } finally {
    await chmod(locked, 0o600).catch(() => {});
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:conflict-policies overwrite, skip, and keep-both change the plan", async () => {
  const sample = await fixture();
  try {
    await writeFile(join(sample.source, "photo.raw"), "new photo");
    await writeFile(join(sample.destination, "photo.raw"), "old photo");
    await writeFile(join(sample.destination, "photo (copy 1).raw"), "older photo");
    const overwrite = parsedPlan(sample, "overwrite").value.actions[0];
    const skip = parsedPlan(sample, "skip").value.actions[0];
    const keep = parsedPlan(sample, "keep-both").value.actions[0];
    assert.equal(overwrite.operation, "overwrite");
    assert.equal(skip.operation, "skip");
    assert.equal(keep.operation, "copy");
    assert.equal(basename(keep.destination), "photo (copy 2).raw");
  } finally {
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:sparse-bounds sparse files produce lower and upper allocation bounds", async () => {
  const sample = await fixture();
  try {
    const sparse = join(sample.source, "disk.img");
    await writeFile(sparse, "header");
    await truncate(sparse, 64 * 1024 * 1024);
    const { value } = parsedPlan(sample);
    assert.ok(value.summary.write_bytes_lower < value.summary.write_bytes_upper);
    assert.ok(value.summary.write_bytes_upper >= 64 * 1024 * 1024);
  } finally {
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:deterministic-json unchanged inputs produce byte-identical JSON", async () => {
  const sample = await fixture();
  try {
    await writeFile(join(sample.source, "z.txt"), "z");
    await writeFile(join(sample.source, "a.txt"), "a");
    const first = parsedPlan(sample).result.stdout;
    const second = parsedPlan(sample).result.stdout;
    assert.equal(first, second);
    assert.ok(first.indexOf("a.txt") < first.indexOf("z.txt"));
  } finally {
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:upper-bound-verdict the safer sparse upper bound controls the verdict", () => {
  const result = spawnSync("cargo", ["test", "tests::upper_bound_controls_space_verdict", "--", "--exact"], { cwd: paths.repository, encoding: "utf8" });
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
  assert.match(result.stdout, /1 passed/);
});

test("@claim:json-manifest JSON output and manifest files contain the same plan", async () => {
  const sample = await fixture();
  try {
    await writeFile(join(sample.source, "notes.txt"), "notes");
    const manifest = join(sample.root, "plan.json");
    const result = runCli([sample.source, sample.destination, "--policy", "overwrite", "--json", "--no-space-check", "--manifest", manifest]);
    assert.equal(result.status, 3, result.stderr);
    const stdout = JSON.parse(result.stdout);
    const saved = JSON.parse(await readFile(manifest, "utf8"));
    assert.deepEqual(saved, stdout);
    assert.equal(saved.schema_version, 1);
  } finally {
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:exit-codes exit codes distinguish safe, invalid, insufficient, and unchecked plans", async () => {
  const sample = await fixture();
  try {
    await writeFile(join(sample.source, "small.bin"), "small");
    assert.equal(runCli([sample.source, sample.destination, "--policy", "overwrite"]).status, 0);
    assert.equal(runCli([sample.source, sample.destination, "--policy", "invalid"]).status, 1);
    assert.equal(runCli([sample.source, sample.destination, "--policy", "overwrite", "--no-space-check"]).status, 3);
    await truncate(join(sample.source, "small.bin"), 1024 * 1024 * 1024 * 1024);
    assert.equal(runCli([sample.source, sample.destination, "--policy", "overwrite", "--sparse", "expand"]).status, 2);
  } finally {
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:source-install Rust 1.85 installs and runs fcsc from a clean consumer prefix", async () => {
  const root = await mkdtemp(join(tmpdir(), "fcsc-consumer-"));
  try {
    const install = spawnSync("cargo", ["+1.85.0", "install", "--path", paths.repository, "--root", root, "--locked"], { encoding: "utf8" });
    assert.equal(install.status, 0, `${install.stdout}\n${install.stderr}`);
    const version = spawnSync(join(root, "bin/fcsc"), ["--version"], { encoding: "utf8" });
    assert.equal(version.status, 0, version.stderr);
    assert.equal(version.stdout.trim(), "fcsc 0.1.0");
  } finally {
    await rm(root, { recursive: true });
  }
});

test("@claim:node-build-minimum Node 20.19 produces the static site", async () => {
  const npx = process.platform === "win32" ? "npx.cmd" : "npx";
  const build = spawnSync(npx, ["--yes", "--package=node@20.19.0", "node", "scripts/build-site.mjs"], {
    cwd: paths.repository,
    encoding: "utf8",
  });
  assert.equal(build.status, 0, `${build.stdout}\n${build.stderr}`);
  assert.equal(await stat(join(paths.root, "index.html")).then((item) => item.isFile()), true);
  assert.equal(await stat(join(paths.root, "sw.js")).then((item) => item.isFile()), true);
});

test("@claim:cli-local-only the CLI completes without opening a network socket", async () => {
  const root = await mkdtemp(join(tmpdir(), "fcsc-network-"));
  try {
    const guard = join(root, "network-guard.so");
    const compile = spawnSync("rustc", [join(paths.repository, "tests/fixtures/network_guard.rs"), "--edition=2024", "--crate-type", "cdylib", "-O", "-o", guard], { encoding: "utf8" });
    assert.equal(compile.status, 0, compile.stderr);
    const marker = join(root, "socket-attempted");
    const result = runCli(["--demo", "--json"], { env: { ...process.env, LD_PRELOAD: guard, FCSC_NETWORK_ATTEMPT_LOG: marker } });
    assert.equal(result.status, 0, result.stderr);
    const value = JSON.parse(result.stdout);
    await assert.rejects(stat(marker), { code: "ENOENT" });
    await rm(demoRootFromJson(value), { recursive: true });
  } finally {
    await rm(root, { recursive: true });
  }
});

test("@claim:browser-demo the fixed browser sample is populated, resettable, and unsaved", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();
    await page.goto(`${site.origin}/demo/`);
    assert.equal(await page.getByText("Demo — sample data, nothing is saved", { exact: true }).count(), 1);
    assert.equal(await page.locator("#action-list li").count(), 6);
    assert.match(await page.locator("#action-list").innerText(), /field-laptop\.img/);
    assert.equal(await page.locator('input[type="file"]').count(), 0);
    await page.locator('input[name="policy"][value="keep-both"]').check();
    assert.match(await page.locator("#action-list").innerText(), /photos \(copy 1\)\.raw/);
    await page.getByRole("button", { name: "Reset demo" }).click();
    assert.equal(await page.locator('input[name="policy"][value="overwrite"]').isChecked(), true);
    assert.deepEqual(await page.evaluate(async () => ({ local: Object.keys(localStorage), session: Object.keys(sessionStorage), databases: (await indexedDB.databases()).map((item) => item.name) })), { local: [], session: [], databases: [] });
    await page.evaluate(() => scrollTo(0, document.body.scrollHeight));
    assert.equal(await page.getByText("Demo — sample data, nothing is saved", { exact: true }).isVisible(), true);
    await context.close();
  } finally {
    await browser.close();
    await site.close();
  }
});

test("@claim:site-privacy the full demo flow sends only same-origin requests and sets no cookies", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const context = await browser.newContext();
    const page = await context.newPage();
    const requests = [];
    page.on("request", (request) => requests.push(request.url()));
    await page.goto(site.origin, { waitUntil: "networkidle" });
    await page.getByRole("link", { name: "Try it with sample data" }).click();
    await page.locator('input[name="policy"][value="skip"]').check();
    await page.locator("#free-space").fill("22");
    assert.ok(requests.length > 0);
    assert.deepEqual([...new Set(requests.map((url) => new URL(url).origin))], [site.origin]);
    assert.deepEqual(await context.cookies(), []);
    await context.close();
  } finally {
    await browser.close();
    await site.close();
  }
});

test("@claim:offline-demo the demo reloads offline after the first visit", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.goto(`${site.origin}/demo/`, { waitUntil: "networkidle" });
    await page.evaluate(() => navigator.serviceWorker.ready);
    await page.reload({ waitUntil: "networkidle" });
    assert.equal(await page.evaluate(() => Boolean(navigator.serviceWorker.controller)), true);
    await context.setOffline(true);
    await page.reload({ waitUntil: "domcontentloaded" });
    assert.equal(await page.title(), "Demo — File Change Space Check");
    assert.equal(await page.locator("#action-list li").count(), 6);
    assert.match(await page.locator("#network-state").innerText(), /Offline/);
    await context.close();
  } finally {
    await browser.close();
    await site.close();
  }
});

test("@claim:non-executable-manifest saving a manifest does not execute file actions", async () => {
  const sample = await fixture();
  try {
    await writeFile(join(sample.source, "copy-me.txt"), "source");
    const before = await treeSnapshot(sample.destination);
    const manifest = join(sample.root, "manifest.json");
    const result = runCli([sample.source, sample.destination, "--policy", "overwrite", "--no-space-check", "--manifest", manifest]);
    assert.equal(result.status, 3, result.stderr);
    assert.deepEqual(await treeSnapshot(sample.destination), before);
    assert.equal((await stat(manifest)).mode & 0o111, 0);
    assert.equal((JSON.parse(await readFile(manifest, "utf8"))).actions[0].operation, "copy");
  } finally {
    await rm(sample.root, { recursive: true });
  }
});

test("@claim:estimate-within-two-percent dense copy estimates stay within two percent of allocation", () => {
  const result = spawnSync("cargo", ["test", "tests::estimates_match_actual_dense_copy_for_every_policy", "--", "--exact"], { cwd: paths.repository, encoding: "utf8" });
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
  assert.match(result.stdout, /1 passed/);
});

test("@claim:linux-download the staged Linux download is the tested release binary", () => {
  const download = join(paths.root, "downloads/fcsc-linux-x86_64");
  const local = readFile(paths.binary);
  const staged = readFile(download);
  return Promise.all([local, staged]).then(([localBytes, stagedBytes]) => {
    assert.deepEqual(stagedBytes, localBytes);
    const result = spawnSync(download, ["--version"], { encoding: "utf8" });
    assert.equal(result.status, 0, result.stderr);
    assert.equal(result.stdout.trim(), "fcsc 0.1.0");
  });
});
