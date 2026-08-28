import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import test from "node:test";

const html = await readFile(new URL("./index.html", import.meta.url), "utf8");
const css = await readFile(new URL("./src/style.css", import.meta.url), "utf8");
const a11yAudit = await readFile(new URL("../scripts/a11y-audit.mjs", import.meta.url), "utf8");

test("landing page has required accessible landmarks", () => {
  assert.match(html, /<html lang="en">/);
  assert.equal((html.match(/<h1[ >]/g) ?? []).length, 1);
  assert.equal((html.match(/<main[ >]/g) ?? []).length, 1);
  assert.match(html, /class="skip-link"/);
  assert.match(html, /alt="[^"]+"/);
  assert.match(html, /aria-live="polite"/);
});

test("visual system includes focus, mobile, dark, and reduced-motion treatments", () => {
  assert.match(css, /:focus-visible/);
  assert.match(css, /max-width: 600px/);
  assert.match(css, /prefers-color-scheme: dark/);
  assert.match(css, /prefers-reduced-motion: reduce/);
});

test("dark proof-strip labels use a high-contrast lime-on-dark pairing", () => {
  assert.match(css, /\.proof-strip b \{ background: #141612; color: var\(--lime\); \}/);
});

test("accessibility audit creates its own evidence directory", () => {
  assert.match(a11yAudit, /for \(const colorScheme of \["light", "dark"\]\)/);
  assert.match(a11yAudit, /mkdir\(evidenceDirectory, \{ recursive: true \}\)/);
  assert.match(a11yAudit, /writeFile\(resolve\(evidenceDirectory, "axe\.json"\)/);
});

test("hero stays inside the image budget", async () => {
  const hero = await stat(new URL("./public/assets/allocation-ledger.webp", import.meta.url));
  assert.ok(hero.size <= 300_000, `hero is ${hero.size} bytes`);
});

test("there are no third-party runtime origins", () => {
  assert.doesNotMatch(html, /(?:src|href)="https?:\/\/(?!github\.com)/);
});
