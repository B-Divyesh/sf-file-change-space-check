import { writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright";

const [url, evidenceDirectory] = process.argv.slice(2);
if (!url || !evidenceDirectory) throw new Error("URL and evidence directory are required");

const browser = await chromium.launch();
const errors = [];
const views = [];
try {
  for (const viewport of [{ name: "desktop", width: 1366, height: 900 }, { name: "phone", width: 390, height: 844 }]) {
    const context = await browser.newContext({ viewport });
    const page = await context.newPage();
    page.on("pageerror", (error) => errors.push(String(error).slice(0, 300)));
    page.on("console", (message) => { if (message.type() === "error") errors.push(message.text().slice(0, 300)); });
    const started = Date.now();
    const response = await page.goto(url, { waitUntil: "networkidle", timeout: 60_000 });
    await page.screenshot({ path: resolve(evidenceDirectory, `screenshot-${viewport.name}.png`), fullPage: true });
    views.push({
      name: viewport.name,
      status: response?.status(),
      loadMs: Date.now() - started,
      title: await page.title(),
      lang: await page.locator("html").getAttribute("lang"),
      h1: await page.locator("h1").count(),
      main: await page.locator("main").count(),
      imagesMissingAlt: await page.locator("img:not([alt])").count(),
      unlabeledButtons: await page.locator("button:not([aria-label])").evaluateAll((buttons) => buttons.filter((button) => !(button.textContent ?? "").trim()).length),
      horizontalOverflow: await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth),
    });
    await context.close();
  }
} finally {
  await browser.close();
}
const report = { url, errors, views };
await writeFile(resolve(evidenceDirectory, "verify.json"), `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify(report));
if (errors.length || views.some((view) => view.status !== 200 || !view.title || view.lang !== "en" || view.h1 !== 1 || view.main !== 1 || view.imagesMissingAlt || view.unlabeledButtons || view.horizontalOverflow)) process.exitCode = 1;
