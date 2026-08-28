# Independent verification 3 — PASS

**Verified:** 2026-08-28 UTC  
**Candidate:** `c5fcfd5412ac8bff83f2134f4f7f0e7a861be9e8`  
**Live URL:** <https://file-change-space-check.sociobot.in/>  
**Verdict:** **PASS — candidate is release-ready.**

Fresh detached worktree `/tmp/fcsc-qa-c5fcfd5`, checked out at the exact
candidate SHA. Dependencies were installed with a clean `npm ci`. No product
code was modified during verification. This supersedes the previous FAIL
reports.

## Quality gates

All passed:

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
```

- `npm test` passed 8 Rust library tests, 4 CLI integration tests, 1 doctest,
  6 site tests, and its production build.
- The separate exact production build created `dist/downloads/fcsc` and
  `dist/site/`. Formatting and strict Clippy passed. There is no separate
  TypeScript typecheck/lint configuration; Vite compiled TypeScript as part
  of the production build.
- `cargo package` created and verified the 0.1.0 crate. `npm audit --omit=dev`
  reported zero vulnerabilities.
- The clean-checkout accessibility audit created its own evidence directory
  and scanned `/`, `/privacy/`, and `/terms/` at 390 x 844 in light and dark
  schemes: 0 violations, including 0 serious/critical.

## CLI end-to-end evidence

Independent temporary fixtures showed no source/destination metadata changes
before versus after planning.

| Policy | Exit | Result |
| --- | ---: | --- |
| `overwrite --no-space-check` | 3 | nested new file and symlink copied; `report.txt` overwritten; 12,288 B upper write and 4,096 B reclaimable |
| `skip --no-space-check` | 3 | same new entries; conflict skipped; 8,192 B upper write |
| `keep-both --no-space-check` | 3 | conflict deterministically mapped to `report (copy 2).txt`; 12,288 B upper write |

Two unchanged keep-both runs emitted byte-identical manifests. `--manifest`
produced schema-1 JSON and human output said that no files were changed. A
single source file mapped correctly to `DESTINATION/<source-file-name>`.

Boundary and recovery cases passed:

- `--help` exits 0; invalid policy and missing source exit 1 with actionable
  stderr and no JSON stdout.
- A destination file exits 1. Both a direct inside-source destination and a
  path reaching source through a symlinked ancestor exit 1 with `destination
  cannot be inside the source tree`.
- A 12 GiB sparse source with `--sparse expand` against 9,961,594,880 free
  bytes emitted a schema-1 `insufficient` manifest, 12,884,901,888 B upper
  write, and exit 2 before execution.

For the brief's 2% estimate measure, a 4 MiB new file and 3 MiB conflict
(over a 1 MiB destination version) were planned and then copied using each
corresponding real policy operation. Estimated net upper and actual regular
file allocation delta were exact:

| Policy | Estimated net upper | Actual delta | Difference |
| --- | ---: | ---: | ---: |
| overwrite | 6,291,456 B | 6,291,456 B | 0% |
| skip | 4,194,304 B | 4,194,304 B | 0% |
| keep-both | 7,340,032 B | 7,340,032 B | 0% |

The packaged crate was installed into a clean consumer prefix with `cargo
install --path target/package/file-change-space-check-0.1.0 --root ...
--locked`. Its installed `fcsc` emitted an unchecked schema-1 JSON copy plan
and exit 3 as documented.

## Website, accessibility, PWA, and privacy

Independent Playwright checks of the built site found no console or page
errors at desktop 1440 x 1000 or mobile 390 x 844.

- First Tab reaches a visible skip link. Keyboard Space selects the skip
  policy. `-1` sets `aria-invalid=true`, an actionable error, and busy state;
  changing to `22` recovers to “Safe to start” and updates the action row.
- Mobile has one `h1`, one `main`, no horizontal overflow, and tested primary
  touch targets at least 44 px tall. Dark canvas is explicit. Reduced motion
  produces 0.01 ms animation/transition durations and `scroll-behavior: auto`.
- A temporary-server service-worker update test confirmed control, offline
  reload (`200`, correct title, one `h1`), then update from `sw-v1` to
  `sw-v2` after `registration.update()`. The version marker was injected only
  by test infrastructure, never product files.
- Local and live Axe scans (3 pages x light/dark at 390 px) both returned 0
  violations and 0 serious/critical findings.
- Live runtime requests used only the product origin. Source review found no
  CLI networking, telemetry, cookies, remote fonts, or third-party runtime
  scripts. Privacy and terms pages are present and accurate.

Production asset sizes: JS 3,872 B; CSS 14,391 B; hero WebP 172,482 B. Fresh
mobile Lighthouse JSON reported Performance 98, Accessibility 100, Best
Practices 100, SEO 92; FCP 1.1 s, LCP 1.7 s, CLS 0, TBT 140 ms. The Lighthouse
process logged a Playwright-Chromium tab shutdown error after it wrote this
complete warning-free report; this was not a page error.

## Live deployment parity and policies

HTTPS requests returned 200 for `/`, `/privacy/`, `/terms/`, `/sw.js`,
`/downloads/fcsc-linux-x86_64`, and the hero. SHA-256 matches the clean build
for landing and legal HTML, service worker, Linux binary, and hero WebP. The
720,712-byte live binary matches `dist/downloads/fcsc`.

TLS validates for `CN=file-change-space-check.sociobot.in` (GeoTrust TLS RSA
CA G1). Headers include same-origin restrictive CSP, `Referrer-Policy:
no-referrer`, `X-Content-Type-Options: nosniff`, and restrictive
camera/microphone/geolocation Permissions Policy. Hashed JS is immutable for
one year, the download is cached one day, and HTML/service worker revalidate
quickly.

## Defects by severity

- **P0:** none observed.
- **P1:** none observed.
- **P2:** none observed.
- **P3:** none observed.
