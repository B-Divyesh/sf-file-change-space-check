# Independent verification 2 — FAIL

**Verified:** 2026-08-28 UTC  
**Candidate:** `9e99be7b34c44f954f73d4e5411bac12adcb3923`  
**Live URL:** <https://file-change-space-check.sociobot.in/>  
**Verdict:** **FAIL — do not release this candidate.**

This is a fresh verification from a clean, detached checkout. It supersedes
the earlier deployment-only result: the live deployment is healthy and
byte-matches this candidate. The failures are reproducible product and
verification-harness defects.

## P1 release blockers

### Symlinked destination ancestry bypasses source containment

The CLI rejects a lexical destination inside the source, but accepts a path
that enters the source through a symlinked directory ancestor:

```sh
mkdir -p /tmp/fcsc-edge/source/nested /tmp/fcsc-edge/destination
truncate -s 4096 /tmp/fcsc-edge/source/item.bin
ln -s /tmp/fcsc-edge/source /tmp/fcsc-edge/linked-parent
fcsc /tmp/fcsc-edge/source /tmp/fcsc-edge/linked-parent/new-subdir \
  --policy overwrite --json
```

This exits `0` and emits `sufficient`, with a `create-directory` action at
`/tmp/fcsc-edge/linked-parent/new-subdir` and a copy beneath it. That physical
destination is inside `/tmp/fcsc-edge/source`. It bypasses the tool's own
containment guard and produces an unsafe/confusing real-copy plan. The direct
lexical control correctly exits `1` with `destination cannot be inside the
source tree`.

### Invalid input returns the documented insufficient-space code

README promises exit `1` for invalid input and exit `2` for insufficient
space. The production binary returns `2` for a Clap parse error:

```sh
fcsc SOURCE DESTINATION --policy not-a-policy --json
# exit 2; error: invalid value 'not-a-policy' for '--policy <POLICY>'
```

An actually insufficient expanded sparse file independently returns `2`; thus
automation cannot distinguish bad invocation from a capacity failure.

### Dark mode has four serious Axe contrast findings

Fresh Axe 4.13 Playwright scans at 390×844 dark mode report four serious
`color-contrast` violations on proof-strip labels `01` through `04`. Each is
`#c8f04a` on `#f7f1df`, **1.16:1** at 13 px versus the 4.5:1 requirement. The
same result occurs on the live URL.

## Clean-checkout quality evidence

Ran after `npm ci` from the detached candidate:

```sh
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
```

- `npm test` passed: 7 Rust unit tests, 3 CLI integration tests, 1 doctest,
  4 site tests, and the exact production build.
- The repeat build, formatting check, and strict Clippy check passed.
- `cargo package` passed package verification. A clean consumer unpacked the
  `.crate`, installed it with `cargo install --path`, and successfully emitted
  a JSON plan with its installed `fcsc`.
- `npm audit --omit=dev` found 0 vulnerabilities. No separate repository
  TypeScript type-check or lint script/configuration exists.

## CLI end-to-end evidence

A fixture with a nested new file, a `report.txt` conflict, and existing
`report (copy 1).txt` produced the expected policy-specific plans:

| Policy | Actions |
| --- | --- |
| overwrite | create directory; copy new file; overwrite `report.txt` |
| skip | create directory; copy new file; skip `report.txt` |
| keep-both | create directory; copy new file; copy `report (copy 2).txt` |

All three `--no-space-check` runs emitted manifests and exited `3`; source and
destination metadata were unchanged before/after. Two identical keep-both
runs produced identical manifests. An empty source to a new destination
emitted only `create-directory` and exited `0`. An oversized sparse source
with `--sparse expand` emitted `insufficient` and exited `2` before any action.
`--help` is useful and `--json` is machine-readable.

## Site, privacy, PWA, performance

- Desktop 1440×1000 and 390×844 mobile were exercised by keyboard and visual
  inspection. There is no horizontal overflow. First Tab reaches the skip
  link with a visible 3 px outline. Radio selection, invalid `-1` free space,
  announced error/`aria-busy`, and recovery at `22 GB` were tested.
- Light-mode Axe on `/`, `/privacy/`, and `/terms/` has no serious/critical
  finding. No page errors or console errors occurred. Reduced-motion emulation
  removes entrance movement; no looping/flashing was observed.
- Recorded desktop/mobile runtime requests use only the page origin. There are
  no remote fonts/scripts, and the CLI has no networking code. Live policies:
  restrictive same-origin CSP, `Referrer-Policy: no-referrer`, `nosniff`, and
  restrictive permissions policy. Privacy and terms pages are present.
- After service-worker readiness, offline home reload returned cached content
  with status `200`. Static review confirms `skipWaiting`, `clients.claim`,
  and re-precaching for updates.
- Budgets: initial JS 3,872 bytes; CSS 14,339 bytes; hero 172,482 bytes. A
  mobile Lighthouse run (bundled Chromium, screenshot disabled) scored 97
  performance, 100 accessibility, 100 best practices, 92 SEO; FCP 1.1 s, LCP
  2.0 s, CLS 0, TBT 170 ms. The SEO deduction is robots.txt, not a stated
  release threshold.

## Fresh deployment verification

HTTPS now validates: subject `CN=file-change-space-check.sociobot.in`,
GeoTrust issuer, and curl reports `SSL certificate verify ok`. Root, privacy,
terms, service worker, Linux download, and hero return `200`.

SHA-256 content comparisons match local candidate output for `index.html`,
privacy/terms HTML, `sw.js`, `mark.svg`, hero WebP, hashed CSS and JS, and
`downloads/fcsc-linux-x86_64` (719,072 bytes). Live hashed assets are
immutable for one year, the download is cached for a day, and HTML/service
worker use short revalidated caching.

## P2 verification-harness defect

`npm run audit:a11y` fails in a clean checkout because
`scripts/a11y-audit.mjs` writes `.factory/evidence/axe.json` without creating
`.factory/evidence/`:

```text
ENOENT: no such file or directory, open
'/work/repo/.factory/evidence/axe.json'
```

Independent Axe runs supplied the evidence above. Fix the script to create its
output directory (or avoid source-tree test output), then rerun verification.

## Required fixes before release

1. Resolve destination ancestry before containment checks and add a
   symlink-ancestor regression test.
2. Return exit `1` for Clap parse errors, or revise the public exit-code
   contract without colliding with insufficient space.
3. Correct the proof-strip dark colors and rerun dark 390 px Axe.
4. Make `npm run audit:a11y` clean-clone-safe.
