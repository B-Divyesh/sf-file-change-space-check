import AxeBuilder from "@axe-core/playwright";
import { chromium } from "playwright";
import { createServer } from "node:http";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { extname, resolve } from "node:path";

const root = resolve(import.meta.dirname, "../dist/site");
const types = { ".css": "text/css", ".html": "text/html", ".js": "text/javascript", ".svg": "image/svg+xml", ".webp": "image/webp" };
const server = createServer(async (request, response) => {
  const pathname = new URL(request.url ?? "/", "http://localhost").pathname;
  const relative = pathname === "/" ? "index.html" : pathname.endsWith("/") ? `${pathname.slice(1)}index.html` : pathname.slice(1);
  try {
    const body = await readFile(resolve(root, relative));
    response.writeHead(200, { "content-type": types[extname(relative)] ?? "application/octet-stream" });
    response.end(body);
  } catch {
    response.writeHead(404).end("Not found");
  }
});
await new Promise((resolveListen) => server.listen(0, "127.0.0.1", resolveListen));
const address = server.address();
if (!address || typeof address === "string") throw new Error("Could not start audit server");
const origin = `http://127.0.0.1:${address.port}`;
const browser = await chromium.launch();
const findings = [];
try {
  for (const colorScheme of ["light", "dark"]) {
    for (const path of ["/", "/privacy/", "/terms/"]) {
      const context = await browser.newContext({ colorScheme, viewport: { width: 390, height: 844 } });
      const page = await context.newPage();
      await page.goto(`${origin}${path}`, { waitUntil: "networkidle" });
      const result = await new AxeBuilder({ page }).analyze();
      findings.push({ path, colorScheme, violations: result.violations });
      await context.close();
    }
  }
} finally {
  await browser.close();
  server.close();
}
const evidenceDirectory = resolve(root, "../../.factory/evidence");
await mkdir(evidenceDirectory, { recursive: true });
await writeFile(resolve(evidenceDirectory, "axe.json"), `${JSON.stringify(findings, null, 2)}\n`);
const serious = findings.flatMap(({ path, violations }) => violations.filter(({ impact }) => impact === "serious" || impact === "critical").map(({ id, impact }) => ({ path, id, impact })));
console.log(JSON.stringify({ pages: findings.length, violations: findings.reduce((sum, page) => sum + page.violations.length, 0), serious }, null, 2));
if (serious.length > 0) process.exitCode = 1;
