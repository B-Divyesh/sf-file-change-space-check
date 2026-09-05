# Check space before copying files — independent verification 5

**Verified:** 2026-09-05 UTC  
**Implementation candidate:** `09a5ebda40e50b4645f26da80e3a62d73cd20570`  
**Documentation reviewed:** `172eb9ce12c43bc2486ab091f4306021a5d12bdf`  
**Live URL:** <https://file-change-space-check.sociobot.in/>

## Verdict

**PASS.** There are **zero findings** at every severity and **zero untested
public claims**. The live product byte-matches the implementation candidate.
The later documentation commit changes only `.factory/handoff.md`.

## Job, audience, and first action

- **Job:** estimate free space and conflicts before copying, extracting, or
  reorganising a large local folder.
- **Audience:** people moving large folders who need an estimate before a long
  copy starts.
- **First action:** **Try it with sample data**. Fresh 1440×1000 desktop and
  390×844 phone contexts showed the action, the job, the audience, and three
  short facts before scrolling. The action says it opens a finished plan in
  one click.

## Live sample and data isolation

The first action opened `/demo/` with the title **Demo — File Change Space
Check**. The page immediately showed six actions for a media archive, including
`photos.raw` and the sparse `field-laptop.img`. The persistent label read
**Demo — sample data, nothing is saved**.

- Keyboard Space selected keep-both and changed the conflict destination to
  `photos (copy 1).raw`.
- Free space `-1` set `aria-invalid="true"` and announced “Enter free space
  from 0 to 9,999 MiB.” Entering `22` recovered to **SAFE TO START**.
- **Reset demo** restored overwrite, 16 MiB, and the original sample.
  **Start for real** returned to `/#install` and removed demo mode.
- The label stayed visible after scrolling. The demo offered no local-file
  input and created no localStorage, sessionStorage, IndexedDB, or cookies.
- Every captured request stayed on the product origin. No unexpected console
  or page errors occurred. The demo therefore made no change to real data.

The CLI sample provided the second isolation check. The installed
`fcsc --demo --policy keep-both --json` created a new temporary sandbox,
returned schema 1 with one conflict and six actions, and left its caller
directory unchanged.

## Installed CLI and edge paths

`cargo package` was installed into a new consumer prefix. The resulting binary
reported `fcsc 0.1.0`; its help listed the documented demo, policy, sparse,
JSON, manifest, and space-check options.

| Path | Result |
| --- | --- |
| overwrite | exit 3; create directory, copy new file, overwrite conflict |
| skip | exit 3; create directory, copy new file, skip conflict |
| keep-both | exit 3; chose `report (copy 2).txt` when suffix 1 existed |
| unchanged input trees | source and destination hashes and metadata matched before and after all plans |
| invalid policy | exit 1 with the invalid value in the error |
| missing source | exit 1 with the unreadable path and operating-system error |
| recovery after invalid input | valid JSON plan returned schema 1 and exit 3 |
| destination entering source through a symlink | exit 1 with `destination cannot be inside the source tree` |
| 1 TiB expanded sparse file | exit 2 with `insufficient`; upper write bound exceeded reported free space |

The clean suites also covered an empty source, deterministic sorting, saved
manifests, sparse lower and upper bounds, and dense estimates under all three
policies. The dense estimates remained within the public 2% limit.

## Clean-checkout gates

A new remote clone at documentation SHA `172eb9c` was used. It contains
implementation SHA `09a5ebd` as an ancestor and had no tracked changes after
verification. After `npm ci` and installing the documented Rust 1.85.0
toolchain, these commands passed:

```sh
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
cargo +1.85.0 test --locked --all-targets
cargo +1.85.0 test --locked --doc
```

`npm test` passed 8 Rust library tests, 7 CLI integration tests, 1 doctest,
8 browser/site tests, and all 18 aggregate claim tests. The separate build
wrote `dist/downloads/fcsc` and `dist/site/`. The accessibility command ran 10
page/theme scans with zero violations. Packaging, formatting, strict Clippy,
and the production dependency audit passed. The exact Node boundary reported
`v20.19.0`, and its claim performed the production site build.

Production sizes were 4,098 bytes of JavaScript, 18,746 bytes of CSS, and
172,482 bytes for the hero image. They are below the product budgets.

## Declared claims

Every `test` value in `.factory/claims.json` was run separately from the clean
clone. Each command selected one tagged outcome test and passed.

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
| `source-install` | PASS with Rust 1.85.0 |
| `node-build-minimum` | PASS with Node 20.19.0 |
| `cli-local-only` | PASS |
| `browser-demo` | PASS |
| `site-privacy` | PASS |
| `offline-demo` | PASS |
| `non-executable-manifest` | PASS |
| `estimate-within-two-percent` | PASS |
| `linux-download` | PASS |

The live pages and README were cross-checked against the registry. Statements
about the demo, read-only and metadata-only planning, policies, sparse bounds,
deterministic JSON, manifests, exit codes, supported toolchains, local-only
operation, privacy, offline use, estimate accuracy, and the Linux download are
covered. No missing, false, incomplete, or untested public claim was found.

## Live routes, accessibility, privacy, and performance

- Home, demo, privacy, terms, robots, and sitemap returned 200. An unknown
  path deliberately returned the designed HTTP 404 with its own title, one
  `h1`, one `main`, and working return links. The browser's expected 404
  resource message was not treated as a product error.
- Every route had the expected title. Canonical, Open Graph, Twitter, touch
  icon, language, landmarks, and image alternatives matched the built files.
  All internal links, the download, and both GitHub links resolved.
- Fresh phone and desktop checks found no horizontal overflow or unexpected
  console errors. Phone and desktop header and footer route links were visible
  and at least 44×44 CSS pixels on all five tested routes.
- The first Tab reached the visible skip link with a 3 px cobalt outline. All
  demo inputs had labels, invalid input was described by a `role="alert"`, the
  result used a polite live region, and keyboard traversal reached every
  control and the footer without a trap. A 200% root-text check kept the page
  at 390 CSS pixels without horizontal overflow or lost controls.
- Reduced motion changed animation and transition duration to `0.00001s` and
  smooth scrolling to `auto`. Nothing loops or flashes.
- Live Axe covered home, demo, privacy, terms, and the designed 404 in light
  and dark mode at 390×844: 10 scans, zero violations.
- A dedicated service-worker context reloaded `/demo/` offline with HTTP 200,
  the right title, six actions, and **Offline · sample works**. An online update
  check left one activated worker and no waiting version.
- Runtime requests were same-origin, with no cookies, analytics, remote fonts,
  or third-party code. CSP, HSTS, `no-referrer`, `nosniff`, and restrictive
  camera, microphone, and geolocation policies were present.
- Fresh mobile Lighthouse scored Performance 100, Accessibility 100, Best
  Practices 100, and SEO 100. FCP was 1.0 s, LCP 1.7 s, TBT 30 ms, and CLS 0.
  The run completed without warnings.

This product has no backend. Tenant isolation, server restart persistence,
health endpoints, and HTTP 429 handling do not apply. The deterministic,
metadata-only task has no useful AI step to add.

## Live candidate parity

The live home, demo, privacy, terms, designed 404, service worker, hero, hashed
JavaScript, hashed CSS, and Linux download were byte-identical to the clean
candidate build. Key SHA-256 values were:

| Artifact | SHA-256 |
| --- | --- |
| home | `cfa9bc18d3f8c84a5437a99a065805aefcb0a4719d8edd44f4604f32eebc680c` |
| service worker | `8d92bcbb6b3e02731a797a44020849e32c8b7eef59256a50df4c1f6b4760a47f` |
| Linux binary | `2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da` |

Normal HTTPS validation succeeded. This proves the live runtime is the
implementation candidate; the later report-only commit does not require a new
product image.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Missing TLS, partial deployment, or live/local mismatch | Fixed. HTTPS validates and ten live artifacts byte-match the clean candidate. |
| Destination-inside-source through a symlink | Fixed. The installed package rejects it with exit 1. |
| Invalid input shared the insufficient-space exit | Fixed. Invalid is 1, insufficient is 2, and unchecked is 3. |
| Dark proof-strip contrast | Fixed. Live and local light/dark Axe scans have zero violations. |
| Clean accessibility audit could not create evidence | Fixed. The command passes from the new clone. |
| Missing CLI/browser demo and false crates.io install | Fixed. Both demos work; source and direct-download paths are tested; the unavailable registry command is not advertised. |
| Missing claims registry or untested public claims | Fixed. All 18 commands pass separately; no unlisted claim was found. |
| Missing demo, legal, discovery, metadata, or 404 structure | Fixed. Required routes, metadata, discovery files, and the deliberate 404 pass live. |
| First screen did not state the job, audience, and sample action plainly | Fixed on fresh phone and desktop screens. The current headings use plain task words. |
| Missing URL verifier | Fixed. The supplied verifier passed both live viewports. |
| Rust 1.85 source install failed | Fixed. Exact-toolchain tests, doctest, and clean consumer install pass. |
| Node 20 minimum was too broad | Fixed. Public minimum is 20.19+, and exact Node 20.19.0 builds the site. |
| Phone header navigation was hidden | Fixed. Three direct links remain visible and keyboard reachable. |
| Header or footer links were below 44×44 px | Fixed. Rendered targets meet the minimum on phone and desktop. |
| Lighthouse Chromium shutdown note | Not reproduced. The fresh 100/100/100/100 report has no warning. |

## Remaining limits

Crates.io publication and Windows support remain documented owner work. Hard
links, reflinks, compression, quotas, and reserved filesystem space remain
explicitly outside the estimator's model. These limits do not contradict a
public claim or break the tested Linux product.

## Finding count

- P0: 0
- P1: 0
- P2: 0
- P3: 0
- Untested claims: 0

**Final verdict: PASS.**
