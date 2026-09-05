import { spawnSync } from "node:child_process";

const tag = process.argv[2];
if (!tag || !/^@claim:[a-z0-9-]+$/.test(tag)) {
  console.error("usage: npm run claim -- @claim:<id>");
  process.exit(1);
}
const result = spawnSync(process.execPath, [
  "--test",
  "--test-concurrency=1",
  `--test-name-pattern=${tag}`,
  "tests/claims.test.mjs",
], { stdio: "inherit" });
process.exit(result.status ?? 1);
