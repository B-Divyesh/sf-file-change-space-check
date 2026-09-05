# Check space before copying files — strict review 3

**Reviewed:** 2026-09-05 UTC  
**Live URL:** <https://file-change-space-check.sociobot.in/>  
**Implementation candidate:** `09a5ebda40e50b4645f26da80e3a62d73cd20570`  
**Documentation commit reviewed:** `f1407370c9462deb91c504354db26c0608182d30`  
**Verdict:** **FAIL — 1 finding and 0 untested public claims.**

The live runtime byte-matches the implementation candidate. Commits after that
candidate change only `.factory` reports. This review changed no product code.

## Job, audience, and first action

- **Job:** estimate free space and conflicts before copying, extracting, or
  reorganising a large local folder.
- **Audience:** people moving large folders who need an estimate before a long
  copy starts.
- **First action:** **Try it with sample data**. Fresh 1440×1000 desktop and
  390×844 phone contexts showed the job, audience, action, and three short
  facts before scrolling. The title is **Check space before you copy files**.

## Finding

### P1-1 — keep-both can assign two source files to one destination

The installed artifact produces an invalid keep-both manifest when the source
already contains the alternate name that the planner generates. The smallest
fixture is:

```text
source/photo.jpg
source/photo (copy 1).jpg
destination/photo.jpg
```

Running the installed package with:

```sh
fcsc source destination --policy keep-both --json --no-space-check
```

exited 3 and returned two `copy` actions with the identical destination:

```json
{
  "path": "destination/photo (copy 1).jpg",
  "sources": ["photo (copy 1).jpg", "photo.jpg"],
  "operations": ["copy", "copy"]
}
```

Both input trees remained unchanged, so the read-only guarantee still holds.
The manifest does not preserve both source files, however. A consumer that
applies these actions cannot copy both values to one path without one replacing
the other. This breaks a core conflict policy and the product's main output.

The cause is that generated alternate paths are added to the planner's
`reserved` set, but ordinary planned copy destinations are not. Depending on
sort order, either action may be planned first; neither branch prevents the
duplicate destination.

The declared `conflict-policies` command passes because its fixture occupies
`photo (copy 1).raw` in the destination. It does not cover a source entry with
that generated name. This is one functional and false-claim finding, not an
untested-claim count: all 18 public claim ids have an executed declared command.

Required repair: reserve every planned destination, and make generated
keep-both names avoid both existing and already planned paths. Add regression
coverage for colliding source files and directories before re-review.

## Clean-checkout gates

A new remote clone at documentation SHA `f140737` was used. It contains
implementation SHA `09a5ebd` as an ancestor. The documented Rust 1.85.0
toolchain was installed before the version-bound tests.

These commands passed:

```sh
npm ci
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
wrote `dist/downloads/fcsc` and `dist/site/`. The accessibility audit ran 10
page/theme scans with zero violations. Packaging, formatting, strict Clippy,
the production dependency audit, and exact Rust 1.85 tests passed.

Production output contains 4,098 bytes of JavaScript, 18,746 bytes of CSS, and
a 172,482-byte hero image. These remain below the product budgets.

## Declared claims

Every `test` value in `.factory/claims.json` was run separately from the clean
clone. Each selected its tagged test and exited successfully.

| Claim | Declared command |
| --- | --- |
| `demo-sandbox` | PASS |
| `read-only-plan` | PASS |
| `metadata-only` | PASS |
| `conflict-policies` | PASS, but contradicted by P1-1's boundary fixture |
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

The landing, demo, legal pages, README, and CLI help were cross-checked against
the registry. No unlisted public claim was found. The untested-claim count is
therefore **0**. Passing declared commands do not override the observed false
keep-both result.

## Installed CLI and runtime paths

`cargo package` was installed into a new consumer prefix. The installed command
reported `fcsc 0.1.0`, and its help listed the documented demo, policy, sparse,
JSON, manifest, and space-check options.

| Path | Result |
| --- | --- |
| overwrite | exit 3; conflict action was `overwrite` |
| skip | exit 3; conflict action was `skip` |
| keep-both with occupied suffix | exit 3; selected `report (copy 2).txt` |
| source already contains generated suffix | **P1-1: two actions used one destination** |
| source and destination snapshots | unchanged after every plan |
| invalid policy | exit 1 with the invalid value and allowed choices |
| missing source | exit 1 with the path and operating-system error |
| recovery after invalid input | valid schema-1 JSON, exit 3 |
| destination entering source through symlink | exit 1 with the containment error |
| 1 TiB expanded sparse source | exit 2; upper write bound exceeded free space |
| bundled keep-both demo | exit 0; one conflict, six actions, fresh temp sandbox |

The bundled demo left its caller directory unchanged and wrote its manifest
only inside the printed temporary sandbox. The staged build, installed package,
and live Linux download shared SHA-256
`2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da`.

## Live sample, accessibility, privacy, and routes

- The one-click sample opened `/demo/` with six realistic media-archive
  actions and the persistent **Demo — sample data, nothing is saved** label.
- Keyboard arrows selected keep-both and changed the conflict destination to
  `photos (copy 1).raw`. `-1` produced an announced range error, zero showed
  **Do not start**, and `22` recovered to **Safe to start**. Reset restored
  overwrite and 16 MiB. **Start for real** returned to `/#install` without a
  demo label.
- The demo exposed no file input and created no localStorage, sessionStorage,
  IndexedDB, or cookies. Every captured request stayed on the product origin.
- The first Tab reached the skip link with a 3 px cobalt outline. Keyboard-only
  traversal reached the sample and the demo footer without a trap. Header and
  footer route links were visible and at least 44×44 px on all tested phone and
  desktop routes.
- At 200% root text size on a 390 px viewport, the demo had no horizontal
  overflow and retained its reset, start, and free-space controls.
- Reduced motion changed animation and transition duration to `0.00001s` and
  scrolling to `auto`. Nothing flashes or loops.
- Live Axe covered home, demo, privacy, terms, and the designed 404 in light
  and dark mode: 10 scans, zero violations. There were no unexpected console
  or page errors.
- Home, demo, privacy, terms, robots, sitemap, the download, and linked GitHub
  pages returned 200. An unknown path deliberately returned the designed HTTP
  404 with its own title, one `h1`, one `main`, and return links.
- A service-worker-controlled context reloaded `/demo/` offline with HTTP 200,
  the correct title, six actions, and **Offline · sample works**. An online
  update check left one activated worker and no waiting version.
- Live headers include HSTS, same-origin CSP with `frame-ancestors 'none'`,
  `no-referrer`, `nosniff`, and restrictive camera, microphone, and geolocation
  permissions.
- Fresh mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO. FCP was 0.97 s, LCP 1.65 s, TBT 44 ms, and CLS 0.

The live home, demo, privacy, terms, designed 404, service worker, original
art, hashed JavaScript, hashed CSS, and Linux binary were byte-identical to the
clean candidate build: 11 of 11 checked artifacts matched. Home and service
worker SHA-256 values were respectively
`cfa9bc18d3f8c84a5437a99a065805aefcb0a4719d8edd44f4604f32eebc680c`
and `8d92bcbb6b3e02731a797a44020849e32c8b7eef59256a50df4c1f6b4760a47f`.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Missing TLS, partial deployment, or live/local mismatch | Fixed. HTTPS validates and 11 live artifacts byte-match the candidate. |
| Destination-inside-source through a symlink | Fixed. The installed package rejects it with exit 1. |
| Invalid input shared the insufficient-space exit | Fixed. Invalid is 1, insufficient is 2, and unchecked is 3. |
| Dark proof-strip contrast | Fixed. Live and local light/dark Axe scans are clean. |
| Clean accessibility audit could not write evidence | Fixed. The clean command completes. |
| Missing CLI/browser demo and false crates.io install | Fixed. Both demos work, and only tested source/direct-download paths are advertised. |
| Missing claims registry or claim commands | Fixed structurally. All 18 commands pass; P1-1 exposes a new false boundary outside the keep-both fixture. |
| Missing demo, legal, discovery, metadata, or 404 structure | Fixed. All required routes and metadata pass live. |
| First screen did not plainly state the job, audience, and action | Fixed on fresh phone and desktop screens. |
| Missing URL verifier | Fixed. The supplied verifier passed both live viewports. |
| Rust 1.85 source install failed | Fixed. Exact-toolchain tests and the source-install claim pass. |
| Node minimum was too broad | Fixed. The stated 20.19 floor and exact-boundary claim pass. |
| Phone header navigation was hidden | Fixed. Three header links remain visible on every tested route. |
| Header or footer targets were below 44×44 px | Fixed. All measured targets meet the minimum. |
| Lighthouse Chromium shutdown warning | Not reproduced. The fresh report completed without warnings. |

This product has no backend, so tenant isolation, server restart persistence,
health endpoints, and HTTP 429 allowances do not apply. Its deterministic,
metadata-only job has no useful AI step to add.

## Counts and final verdict

- P0: 0
- P1: 1
- P2: 0
- P3: 0
- Total findings: 1
- Untested public claims: 0

**Final verdict: FAIL.** The duplicate-destination keep-both manifest must be
repaired and regression-tested before this product can pass strict review.
