# File Change Space Check — preflight copy-space review

**Review date:** 2026-09-05 UTC  
**Live URL:** <https://file-change-space-check.sociobot.in/>  
**Implementation candidate:** `e20c763ec33b537724d86d3d2a4cb4a46012f7c6`  
**Documentation commit reviewed:** `36ea2bdb539825c2e4273ead7dbad5aad2c87e2b`  
**Verdict:** **FAIL — do not declare this product PASS or release-ready.**

`e20c763` is the implementation candidate: `c5fcfd5` and `36ea2bd` only change `.factory` reports. The live landing, legal pages, Linux binary, and service worker byte-match a fresh production build of that candidate.

## Job, audience, and first action

- **Job:** estimate free space and conflicts before copying, extracting, or reorganising a large local folder.
- **Audience:** people moving archives, project trees, or media collections who need to know whether the operation will fit before starting it.
- **First live-screen action:** `Download for Linux`; the adjacent copied command is `cargo install file-change-space-check`. There is no `Try it with sample data` action before scrolling.

Fresh desktop (1440×1000) and phone (390×844) pages loaded at scroll position zero with no console or page errors. The first-screen screenshots showed the same missing sample action at both sizes.

## Findings

### P1-1 — the required one-click CLI demo sandbox does not exist

The CLI contract requires a bundled realistic sample, `fcsc --demo` (or equivalent) that runs in a temporary directory, a landing-page recording, and a `/demo` or `?demo=1` entry with a persistent `Demo — sample data, nothing is saved` label, `Reset demo`, and isolated demo storage.

None is present. The repository has no `examples/`, `.factory/demo.md`, or demo command. The downloaded live binary returns exit 1 for `--demo`:

```text
error: unexpected argument '--demo' found
```

`/demo` returns the normal landing title and HTML. The landing has only a fixed browser policy simulator behind a `Try a policy` anchor; it has no sample-entry action, persistent sample label, reset control, real-data boundary, or terminal recording of the real binary. It cannot meet the required one-click, no-real-data demo path.

### P1-2 — the prominent installation command is false

The first screen tells visitors to copy `cargo install file-change-space-check`. From a fresh consumer prefix this command exits 101:

```text
error: could not find `file-change-space-check` in registry `crates-io` with version `*`
```

The crate is ready to package but has not been published. The page must show a working download or source-install path until publication. A primary setup command that fails is a release blocker.

### P1-3 — claims registry and claim tests are absent; 14 public claims are untested

`.factory/claims.json` does not exist, so there are no declared claim commands to run from a clean checkout. This is not a zero-claim product. The landing, README, and privacy page make these 14 independently relyable claims without a tagged sandbox test:

1. The tool makes no file changes.
2. It scans metadata before a copy.
3. It does not read file contents by default.
4. It applies the selected conflict policy.
5. It budgets sparse-file uncertainty conservatively.
6. It emits deterministic JSON.
7. It has no telemetry.
8. It makes no network requests / is local-only.
9. The browser simulator uses a fixed sample and never accepts local files.
10. The site has no analytics.
11. The site has no cookies.
12. The site has no remote fonts or third-party runtime code.
13. The service worker lets the documentation and demo work offline.
14. The manifest is non-executable and the tool never transfers files.

Manual evidence supports several statements, but it does not replace one tagged clean-state test per public claim. The untested-claim count is **14**.

### P1-4 — required routes and 404 behaviour are not deployed

The required `/demo`, `robots.txt`, `sitemap.xml`, and designed 404 resources are missing. Requests to `/demo`, `/robots.txt`, `/sitemap.xml`, `/404`, `/404.html`, and `/does-not-exist` all return **200** with the landing page title and landing `<h1>`, because `navigationFallback` rewrites them to `index.html`. A deliberate HTTP 404 would have been acceptable; this is a missing required structure and a broken unknown-route experience. There is no route-specific Demo title or styled, useful 404 page.

### P2-1 — the first screen does not meet the plain-words entry contract

The live `<h1>` is `Know the copy fits. Before the first byte moves.` It does not plainly name the job, and the first screen does not name the intended person. Its first action is download/install, not the required sample action. Disallowed metaphor or slogan wording also appears in `One tree. Three very different answers.`, `Headroom, not wishful subtraction.`, `Put it before cp, not inside it.`, and `It plans the terrain. It does not drive.` The required `.factory/copy-audit.md` is absent.

### P2-2 — required metadata is missing from all site pages

Titles and descriptions exist, but the built site has no canonical links, Open Graph tags, Twitter card tags, or Apple touch icon. There is also no generated `robots.txt`, `sitemap.xml`, or `404.html`. This misses the required route metadata and discovery contract.

### P2-3 — the required URL verifier is not supplied

The attached accessibility baseline requires the worker's `verify-url.sh`. No such executable exists in the checkout, so that required check cannot be run. The project Axe audit ran successfully, but it is not the missing verifier.

## Checks that passed

### Clean checkout and build

A fresh detached worktree at `36ea2bd` used Node v22.23.2, npm 10.9.8, and Rust/Cargo 1.98.0. These commands passed:

```sh
npm ci
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
```

`npm test` passed 8 Rust library tests, 4 CLI integration tests, 1 doctest, 6 site tests, and the production build. The accessibility audit scanned home, privacy, and terms in light and dark at 390×844 with zero Axe violations. `cargo package` verified the crate and `npm audit --omit=dev` found zero vulnerabilities. The production build produced 3,872 B JavaScript, 14,391 B CSS, and a 172,482 B hero image.

The required claims commands could not be run because P1-3's registry is absent. No failing declared claim command was hidden.

### Installed artifact and CLI paths

The packaged crate installed into a fresh consumer prefix and reported `fcsc 0.1.0`. A separate download of the live Linux binary has SHA-256 `726bed7444b96cae4b1fe11b72f507439f6e9039d66fb2e96a53f7a549025528`, exactly matching `dist/downloads/fcsc`; it ran a real keep-both plan with exit 3, an unchecked schema-1 manifest, and four actions.

Normal, invalid, boundary, and recovery tests of the packaged binary passed:

| Case | Result |
| --- | --- |
| overwrite / skip / keep-both on nested realistic files | exit 3 unchecked manifests; expected action changes; source and destination snapshots unchanged |
| invalid policy | exit 1 and actionable stderr; no JSON stdout |
| missing source | exit 1 and actionable stderr |
| 256 MiB sparse source, `--sparse expand`, 64 MiB free | exit 2, `insufficient`, 268,435,456 B upper write |
| destination entering source through symlinked ancestor | exit 1, `destination cannot be inside the source tree` |

For dense 4 MiB-new / 3 MiB-conflict / 1 MiB-existing fixtures, estimated net upper change and actual file-allocation delta were exact for overwrite (6,291,456 B), skip (4,194,304 B), and keep-both (7,340,032 B): 0.00% difference, within the stated 2% goal.

### Live browser, privacy, accessibility, and performance

Fresh desktop and phone Playwright contexts exercised the live page. The fixed policy simulator populated realistic-looking output; keyboard selection worked with a 3 px focus outline; `-1` was announced as invalid; changing to `22` GB recovered to `SAFE TO START`; no horizontal overflow occurred at 390 px. Reduced motion resolved transitions/animation to 0.00001s and scrolling to `auto`.

Live Axe/Playwright scans of `/`, `/privacy/`, and `/terms/` in both light and dark schemes reported zero violations and zero console errors. All recorded runtime requests stayed on the product origin. After first load, the service worker controlled the page and an offline reload returned 200 with the expected title and one `<h1>`. Privacy and terms had their expected titles; all linked product/GitHub destinations returned 200.

Live Lighthouse wrote a complete report before Chromium reported a tab-shutdown crash: Performance 100, Accessibility 100, Best Practices 100, SEO 92; FCP 1.0 s, LCP 1.7 s, CLS 0, TBT 0 ms. This matches the earlier verifier environment symptom, not a page console error.

TLS validates for the live hostname. The root response is 200 with restrictive CSP, `Referrer-Policy: no-referrer`, `nosniff`, and restrictive camera/microphone/geolocation permissions policy. The live home, privacy, terms, service worker, and Linux artifact match the fresh build.

## Earlier findings and current disposition

| Earlier finding | Current disposition and evidence |
| --- | --- |
| Production TLS/partial deployment P0 | Fixed. Normal HTTPS root and required existing pages/download return 200; live build byte-matches candidate. |
| Symlinked destination-inside-source P1 | Fixed. A symlinked ancestor plan exits 1 with the containment error. |
| Invalid CLI input returned exit 2 P3 | Fixed. Invalid policy exits 1; insufficient space still exits 2. |
| Dark proof-strip contrast P1 | Fixed. Live dark Axe has zero violations. |
| `audit:a11y` clean-checkout P2 | Fixed. It created evidence and completed six page/theme scans. |
| Earlier Lighthouse Chromium shutdown note | Repeated only after a complete report; live page checks had no page/console error. |

The earlier findings are repaired, but the seven findings above mean the product is **FAIL**.

## Required next steps

1. Ship a real bundled `fcsc --demo` sample, isolated demo route/label/reset flow, demo documentation, and landing recording; do not touch real user data in that path.
2. Remove or replace the false registry install command until the crate is actually published.
3. Add `.factory/claims.json` and a clean-state tagged test for every public claim; either test or remove each reliance-worthy claim.
4. Add real `/demo`, `robots.txt`, `sitemap.xml`, and styled 404 responses; add all required per-route metadata.
5. Rewrite the first screen in plain words, add the copy audit, supply `verify-url.sh`, and repeat review from a clean checkout.

