import assert from "node:assert/strict";
import { readdir, stat } from "node:fs/promises";
import test from "node:test";
import { chromium } from "playwright";
import { paths, startSiteServer } from "../tests/site-server.mjs";

test("all public routes have their own accessible document", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    const expected = [
      ["/", "File Change Space Check — Plan local file copies", "Check space before you copy files"],
      ["/demo/", "Demo — File Change Space Check", "Test a copy plan with sample files"],
      ["/privacy/", "Privacy — File Change Space Check", "How we handle data"],
      ["/terms/", "Terms — File Change Space Check", "Terms for using the estimate"],
    ];
    for (const [route, title, heading] of expected) {
      const response = await page.goto(`${site.origin}${route}`);
      assert.equal(response?.status(), 200, route);
      assert.equal(await page.title(), title);
      assert.equal(await page.locator("h1").count(), 1);
      assert.equal(await page.locator("h1").innerText(), heading);
      assert.equal(await page.locator("main").count(), 1);
      assert.equal(await page.locator('link[rel="canonical"]').count(), 1);
      assert.equal(await page.locator('meta[property="og:image"]').count(), 1);
      assert.equal(await page.locator('meta[name="twitter:card"]').count(), 1);
      assert.equal(await page.locator('link[rel="apple-touch-icon"]').count(), 1);
    }
  } finally {
    await browser.close();
    await site.close();
  }
});

test("unknown paths return the designed 404 document", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    const response = await page.goto(`${site.origin}/a-missing-plan`);
    assert.equal(response?.status(), 404);
    assert.equal(await page.title(), "Page not found — File Change Space Check");
    assert.equal(await page.locator("h1").innerText(), "Page not found");
    assert.equal(await page.getByRole("link", { name: "Return home" }).getAttribute("href"), "/");
  } finally {
    await browser.close();
    await site.close();
  }
});

test("the first phone and desktop screens name the job, audience, and sample action", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    for (const viewport of [{ width: 390, height: 844 }, { width: 1440, height: 1000 }]) {
      const page = await browser.newPage({ viewport });
      await page.goto(site.origin);
      assert.equal(await page.locator("h1").innerText(), "Check space before you copy files");
      assert.match(await page.locator(".hero-lede").innerText(), /people moving large folders/i);
      const action = page.getByRole("link", { name: "Try it with sample data" });
      assert.equal(await action.getAttribute("href"), "/demo/");
      const box = await action.boundingBox();
      assert.ok(box && box.y + box.height <= viewport.height, "sample action is visible before scrolling");
      assert.equal(await page.locator(".trust-list li").count(), 3);
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth), true);
      await page.close();
    }
  } finally {
    await browser.close();
    await site.close();
  }
});

test("demo controls work by keyboard, report errors, recover, and reset", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ viewport: { width: 390, height: 844 } });
    await page.goto(`${site.origin}/demo/`);
    assert.equal(await page.locator("#action-list li").count(), 6);
    await page.locator('input[name="policy"][value="skip"]').focus();
    await page.keyboard.press("Space");
    assert.equal(await page.locator("#headroom-value").innerText(), "19.0 MiB");
    assert.match(await page.locator("#action-list").innerText(), /SKIP\s+photos\.raw/);
    await page.locator("#free-space").fill("-1");
    assert.equal(await page.locator("#free-space").getAttribute("aria-invalid"), "true");
    assert.equal(await page.locator("#free-error").innerText(), "Enter free space from 0 to 9,999 MiB.");
    await page.locator("#free-space").fill("22");
    assert.match(await page.locator("#result-status").innerText(), /SAFE TO START/);
    await page.getByRole("button", { name: "Reset demo" }).click();
    assert.equal(await page.locator("#free-space").inputValue(), "16");
    assert.equal(await page.locator('input[name="policy"][value="overwrite"]').isChecked(), true);
    assert.match(await page.locator("#demo-message").innerText(), /Demo reset/);
    assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth), true);
  } finally {
    await browser.close();
    await site.close();
  }
});

test("focus and reduced-motion behavior remain visible and stable", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ reducedMotion: "reduce" });
    await page.goto(site.origin);
    await page.keyboard.press("Tab");
    assert.equal(await page.locator(":focus").innerText(), "Skip to main content");
    const focus = await page.locator(":focus").evaluate((node) => getComputedStyle(node).outlineWidth);
    assert.equal(focus, "3px");
    const motion = await page.locator(".hero-copy").evaluate((node) => {
      const style = getComputedStyle(node);
      return { animation: style.animationDuration, transition: style.transitionDuration };
    });
    assert.ok(parseFloat(motion.animation) <= 0.01, motion.animation);
    assert.ok(parseFloat(motion.transition) <= 0.01, motion.transition);
  } finally {
    await browser.close();
    await site.close();
  }
});

test("robots, sitemap, and production assets meet their budgets", async () => {
  const site = await startSiteServer();
  try {
    const robots = await fetch(`${site.origin}/robots.txt`);
    assert.equal(robots.status, 200);
    assert.match(await robots.text(), /Sitemap: https:\/\/file-change-space-check\.sociobot\.in\/sitemap\.xml/);
    const sitemap = await fetch(`${site.origin}/sitemap.xml`);
    assert.equal(sitemap.status, 200);
    const xml = await sitemap.text();
    for (const path of ["/", "/demo/", "/privacy/", "/terms/"]) {
      assert.ok(xml.includes(`<loc>https://file-change-space-check.sociobot.in${path}</loc>`), path);
    }
    const assetNames = await readdir(`${paths.root}/assets`);
    const script = assetNames.find((name) => name.endsWith(".js"));
    const stylesheet = assetNames.find((name) => name.endsWith(".css"));
    assert.ok(script && stylesheet);
    const files = await Promise.all([
      stat(`${paths.root}/assets/${script}`),
      stat(`${paths.root}/assets/${stylesheet}`),
      stat(`${paths.root}/assets/allocation-ledger.webp`),
    ]);
    assert.ok(files[0].size <= 200_000, `JavaScript is ${files[0].size} bytes`);
    assert.ok(files[1].size <= 50_000, `CSS is ${files[1].size} bytes`);
    assert.ok(files[2].size <= 300_000, `hero is ${files[2].size} bytes`);
  } finally {
    await site.close();
  }
});

test("every internal link and download resolves", async () => {
  const site = await startSiteServer();
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    const targets = new Set();
    for (const route of ["/", "/demo/", "/privacy/", "/terms/", "/404.html"]) {
      await page.goto(`${site.origin}${route}`);
      for (const href of await page.locator("a[href]").evaluateAll((links) => links.map((link) => link.getAttribute("href")))) {
        if (href && !href.startsWith("http") && !href.startsWith("mailto:")) targets.add(new URL(href, site.origin).href);
      }
    }
    for (const target of targets) {
      const response = await fetch(target);
      assert.equal(response.status, 200, target);
    }
  } finally {
    await browser.close();
    await site.close();
  }
});
