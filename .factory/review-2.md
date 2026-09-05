# Check space before copying files — strict review 2

**Reviewed:** 2026-09-05 UTC  
**Live URL:** <https://file-change-space-check.sociobot.in/>  
**Implementation candidate:** `a78cad718840554b319e2acc1c243d086a080e42`  
**Documentation base:** `caa91a9070ea41efca246d847bfc2196a97675ca`  
**Verdict:** **FAIL — 4 findings and 2 untested public claims.**

The live runtime byte-matches the implementation candidate. Commits after that
candidate change only reports. This review did not change product code.

## Job, audience, and first action

- **Job:** estimate free space and conflicts before copying, extracting, or
  reorganising a large local folder.
- **Audience:** people moving large folders who need a space and conflict
  estimate before a long copy starts.
- **First action before scrolling:** **Try it with sample data**. Fresh
  1440×1000 desktop and 390×844 phone pages showed it beside “See a finished
  plan in one click.”

The first-screen title is **Check space before you copy files**. It names the
job directly. The page uses plain task headings and no metaphor headings.

## Findings

### P1-1 — the documented Rust 1.85 source install cannot build

The landing page and README say Rust 1.85 or newer is required. `Cargo.toml`
also declares `rust-version = "1.85"`. A clean locked test with the exact
minimum fails:

```text
cargo +1.85.0 test --locked
error[E0658]: `let` expressions in this position are unstable
  --> src/main.rs:127:8
  --> src/main.rs:128:12
```

The same locked suite passes on Rust 1.88.0. The user-facing source install is
therefore false for part of its stated supported range.

The declared `source-install` claim command passes because it uses the
worker's Rust 1.98.0 toolchain. It does not select or test Rust 1.85. This is
one incomplete public claim test and counts as one untested public claim.

### P2-1 — the documented Node 20 development minimum is false

README and `package.json` state Node.js 20+ or `>=20`. The clean site build
fails under Node 20.0.0:

```text
npx --yes -p node@20.0.0 node scripts/build-site.mjs
TypeError [ERR_INVALID_ARG_TYPE]: The "path" argument must be of type string.
```

The script uses `import.meta.dirname`, which is unavailable in Node 20.0. The
locked Vite 7.3.6 package also declares `^20.19.0 || >=22.12.0`. The same build
passes on Node 20.19.0. No claims entry tests the published Node minimum, so
this counts as the second untested public claim.

### P2-2 — phone headers remove all route navigation

At 390×844, every tested route hides `header nav` through the 900 px media
query. There are zero visible header links and no replacement menu button on
home, demo, privacy, terms, or the designed 404 page.

The required phone header is therefore not the standard wordmark plus route
navigation. Visitors must scroll to the footer to find the remaining route
links.

### P2-3 — phone footer navigation targets are only 15 px high

The only general route navigation left on the phone is below the 44×44 px
minimum target size. Measured live target rectangles include:

| Link | Live size at 390 px |
| --- | ---: |
| Demo | 31.2×15 px |
| Privacy | 54.6×15 px |
| Terms | 39×15 px |
| Home on `/demo/` | 31.2×15 px |

The links have visible focus styles and enough horizontal separation, but
their hit areas remain 15 px high. Axe does not report this project-specific
44 px requirement.

## Declared claims

All 17 commands in `.factory/claims.json` were run separately from a fresh
remote clone at documentation base `caa91a9`. Every declared command selected
its tagged test and passed:

| Claim | Result |
| --- | --- |
| `demo-sandbox` | PASS |
| `read-only-plan` | PASS |
| `metadata-only` | PASS |
| `conflict-policies` | PASS |
| `sparse-bounds` | PASS |
| `deterministic-json` | PASS |
| `upper-bound-verdict` | PASS |
| `json-manifest` | PASS |
| `exit-codes` | PASS |
| `source-install` | PASS on Rust 1.98; incomplete for the stated 1.85 minimum |
| `cli-local-only` | PASS |
| `browser-demo` | PASS |
| `site-privacy` | PASS |
| `offline-demo` | PASS |
| `non-executable-manifest` | PASS |
| `estimate-within-two-percent` | PASS |
| `linux-download` | PASS |

The Rust 1.85 and Node 20.0 minimum-version statements are public claims that
the declared commands do not test. Both fail at their published boundary.
The final untested-claim count is **2**.

## Clean-checkout quality gates

After the documented `npm ci`, these commands passed from the fresh clone:

```sh
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
```

`npm test` passed 8 library tests, 7 CLI integration tests, 1 doctest, 7 site
tests, and all 17 aggregate claim tests. The build passed strict TypeScript
checking and wrote `dist/downloads/fcsc` plus `dist/site/`. The accessibility
audit ran 10 phone page/theme scans with zero Axe violations. Packaging,
formatting, strict Clippy, and the production dependency audit passed.

The production build contains 4,098 bytes of JavaScript, 18,278 bytes of CSS,
and a 172,482-byte hero image. These remain within the declared budgets.

## Installed CLI behavior

A packaged crate was installed into a new consumer root. The installed command
reported `fcsc 0.1.0`.

- Overwrite, skip, and keep-both produced their expected conflict operations.
  All three unchecked JSON plans exited 3.
- Keep-both selected `report (copy 2).txt` when copy suffix 1 was occupied.
- Source and destination metadata snapshots were unchanged after all plans.
- An invalid policy and a missing source exited 1 with useful errors.
- A destination entering the source through a symlinked ancestor exited 1.
- A 1 TiB expanded sparse input against about 9.8 GB free exited 2 with an
  `insufficient` schema-1 manifest.
- `--demo --policy keep-both --json` exited 0, created a new temporary
  sandbox, wrote the same schema-1 manifest there, reported one conflict and
  six actions, and left its caller directory empty.
- `--demo --no-space-check` exited 1 with the intended incompatibility error.

The hosted Linux binary exactly matches the clean release:

```text
2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da
```

## Live sample, error, and recovery paths

Fresh desktop and phone contexts entered the sample from the landing page.
The resulting `/demo/` page had the route-specific title, six populated
actions, and the persistent **Demo — sample data, nothing is saved** label.

Keyboard Space selected keep-both and changed the conflict output to
`photos (copy 1).raw`. Free space `-1` set `aria-invalid="true"` and announced
“Enter free space from 0 to 9,999 MiB.” Changing it to 22 MiB recovered to
**Safe to start**. Reset restored overwrite, 16 MiB, and the original action.
The banner remained visible after scrolling.

**Start for real** returned to `/#install`, removed the demo banner, and left
no sample state behind. There was no local-file input. Local storage, session
storage, IndexedDB, and cookies stayed empty. All recorded requests were to
the product origin, and no page or console error occurred.

## Accessibility, offline use, routes, and privacy

- First Tab focused **Skip to main content** with a 3 px cobalt outline.
- Reduced-motion checks removed the entrance movement. There is no looping or
  flashing content.
- Live Axe covered home, demo, privacy, terms, and a designed unknown route in
  light and dark themes: 10 scans, zero violations.
- The service worker controlled a dedicated context using `fcsc-shell-v2`.
  Offline `/demo/` reload returned 200 with the right title, one `h1`, six
  actions, and the label **Offline · sample works**. A later online update
  check kept an active worker with no waiting stale version.
- Home, demo, privacy, terms, robots, and sitemap returned 200. An unknown path
  deliberately returned the designed HTTP 404 with one `h1` and return links.
- Every crawled internal link, download, GitHub repository link, and issue link
  returned 200.
- Each page has its own title, canonical URL, one `h1`, and header, navigation,
  main, and footer landmarks. The phone visibility defect is P2-2.
- Runtime requests were same-origin. No analytics, cookies, remote fonts, or
  third-party runtime scripts were observed.
- CSP restricts resources to self and sets `frame-ancestors 'none'`. HSTS,
  `no-referrer`, `nosniff`, and restrictive device permissions are live.

The live landing, demo, privacy, terms, 404 document, service worker, hero
image, and Linux binary byte-match the clean build of the implementation
candidate.

Fresh mobile Lighthouse scored Performance 100, Accessibility 100, Best
Practices 100, and SEO 100. FCP was 1.06 s, LCP 1.67 s, CLS 0, and TBT 0.5 ms.
The report completed without a runtime error.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Missing TLS or partial deployment | Fixed. HTTPS validates and the live files match the candidate. |
| Destination-inside-source through a symlink | Fixed. Unit, claim, and installed-binary checks reject it. |
| Invalid input used exit 2 | Fixed. Invalid input exits 1; insufficient space exits 2. |
| Dark proof-strip contrast | Fixed. Live dark Axe has zero violations. |
| Clean accessibility audit could not write evidence | Fixed. The clean command completes. |
| Missing CLI/browser demo and false crates.io command | Fixed. Both demos work and the unavailable registry command remains removed. |
| Missing claims registry | Fixed for 17 registered claims, but the two minimum-version claims above remain incomplete. |
| Missing demo, discovery, metadata, and 404 routes | Fixed live. |
| First-screen wording and copy audit | Fixed. The job, audience, and sample action are plain and visible before scrolling. |
| Missing URL verifier | Fixed. The supplied verifier passed desktop and phone. |
| Lighthouse shutdown note | Not reproduced. Lighthouse completed with no runtime error. |

This is a CLI plus static site. Backend tenant isolation, backend restart
persistence, health endpoints, and HTTP 429 allowances do not apply. No AI
step would improve the metadata-only preflight job; adding one would weaken
the local, deterministic design.

## Required changes

1. Either avoid the unsupported let-chain syntax or publish Rust 1.88 as the
   real minimum. Add a claim test that runs the exact minimum toolchain.
2. Publish Node 20.19+ as the development minimum and align `package.json`, or
   make the scripts work on the advertised Node 20.0 boundary. Test it.
3. Keep visible route navigation in the phone header, directly or through an
   accessible menu.
4. Give the phone footer navigation links at least 44×44 px hit areas.

Until those changes are implemented and verified, the product verdict is
**FAIL**.
