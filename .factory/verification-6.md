# Check space before copying files — independent verification 6

Verified 5 September 2026 against
<https://file-change-space-check.sociobot.in/>.

## Verdict

**PASS.** There are **zero findings** at every severity and **zero untested
public claims**.

- Implementation reviewed: `5e868b086921f520442e85431cfb98cc092e48d0`
- Documentation reviewed: `6f41f9a67c41ff0f0082c936731ba39901973eaf`
- P0: 0
- P1: 0
- P2: 0
- P3: 0
- Untested claims: 0

The documentation commit changes only `.factory/handoff.md`. All product files
and the built runtime are from the implementation commit. No product code was
changed during this verification.

## Job, audience, and first action

Before scrolling, fresh 1440×1000 desktop and 390×844 phone contexts show:

- Job: **Check space before you copy files**.
- Audience: people moving large folders who need a space and conflict estimate
  before a long copy.
- First action: **Try it with sample data**, followed by “See a finished plan in
  one click.”

The same screen shows three short facts about metadata-only scanning, read-only
planning, price, and telemetry. The wording uses plain task terms and no mood or
metaphor headings.

## Clean-checkout gates

A fresh remote clone was detached at the documentation SHA. After `npm ci` and
installation of the documented Rust 1.85.0 toolchain, these commands passed:

```text
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

The aggregate suite passed 9 Rust library tests, 7 CLI integration tests, 1
doctest, 8 site tests, and 18 claim tests. Package verification passed. The
dependency audit found zero vulnerabilities. Ten local light/dark Axe scans
reported zero violations.

The production build created `dist/`. Its initial JavaScript is 4.10 kB and CSS
is 18.75 kB before gzip, below the product budgets.

## Declared claims

Every `test` command in `.factory/claims.json` was run separately from the fresh
checkout. All 18 passed:

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

The landing page, demo, privacy page, terms, README, demo documentation, and
CLI help were cross-checked against the registry. No missing, false, incomplete,
or unlisted public claim was found.

## Installed CLI and collision repair

`cargo package` produced a verified crate. That package installed with Rust
1.85.0 into a new consumer prefix and reported `fcsc 0.1.0`.

The installed artifact passed these independent paths:

- `--demo` created a new temporary sandbox on each run, wrote a manifest there,
  emitted six realistic actions, and left the caller directory unchanged.
- Overwrite, skip, and keep-both returned policy-specific plans.
- Invalid policy and missing source returned exit 1 with actionable errors.
- A 1 TiB expanded sparse fixture returned exit 2 for insufficient space.
- `--no-space-check` returned exit 3 with a manifest.
- A symlinked destination inside the source was rejected with exit 1.
- Source and destination hashes were unchanged after planning.

The repaired boundary fixture contained `photo.jpg`, `photo (copy 1).jpg`,
`album`, and `album (copy 1)` beside destination conflicts. The installed
package returned six actions and six unique destinations. It assigned the
conflicting originals to `photo (copy 2).jpg` and `album (copy 2)` while
preserving the ordinary sibling destinations.

## Live sample, recovery, and privacy

Fresh desktop and phone contexts entered the sample in one click. Both showed
the persistent **Demo — sample data, nothing is saved** label and six populated
manifest actions. Keyboard Space selected keep-both and produced
`photos (copy 1).raw`.

Values below 0 and above 9,999 displayed the bound error through the form's
alert. The accepted boundaries 0 and 9,999 produced the expected unsafe and
safe results. Entering 22 recovered to **Safe to start**. **Reset demo** restored
overwrite and 16 MiB, and **Start for real** opened the install section.

The label remained visible at the bottom of the sample. No file input exists.
The full flow left localStorage, sessionStorage, IndexedDB, and cookies empty.
Its requests were same-origin only. The service-worker cache contains public
site files, as disclosed on the privacy page; no user data is stored.

## Routes, accessibility, offline use, and performance

Home, demo, privacy, and terms returned HTTP 200 with their own titles, one
`h1`, `lang="en"`, main/header/navigation/footer structure, metadata, and return
paths. The unknown route deliberately returned HTTP 404 with the designed
title, heading, navigation, and links. Every discovered internal and external
link resolved successfully.

Keyboard focus starts on the skip link and has a 3 px visible outline. The
tested controls operate by keyboard. Navigation remains visible on phone, and
the local rendered-target suite confirms header and footer links are at least
44×44 px. Phone layouts and 200% text had no horizontal overflow. Reduced-motion
mode reduced animation and transition durations to 0.00001 seconds and disabled
smooth scrolling.

Ten live Axe scans covering all five documents in light and dark modes found
zero violations. Fresh successful pages logged no console or page errors. The
single network-console message from the deliberate unknown-route navigation was
the expected HTTP 404 and is not a defect.

After first load, a dedicated context reloaded `/demo/` offline with six
actions and the visible offline status. An update-path check seeded a stale
Cache API response; the visible Linux download still delivered the current
release hash. Live mobile Lighthouse scored Performance 100, Accessibility
100, Best Practices 100, and SEO 100. FCP was 1.1 s, LCP 1.7 s, TBT 60 ms, and
CLS 0.

This product has no backend, account, tenant, payment, or server-side product
state. Tenant isolation, restart persistence, backend health, and 429 behavior
therefore do not apply. An AI step would not improve this deterministic,
metadata-only safety calculation and is outside the researched job.

## Live candidate parity

Fifteen live artifacts byte-match the fresh implementation build, including
all HTML documents, JS, CSS, images, discovery files, service worker, and Linux
download. The fresh build and live binary share SHA-256:

```text
86539a4f2a5457ab1e41ac6f9d764d0dfd9f547684cc484fa2718687d9586687
```

The downloaded binary reports `fcsc 0.1.0`. HTTPS responses include the
declared CSP, HSTS, no-referrer policy, content-type protection, and permissions
policy.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Missing TLS, partial deployment, or live/local mismatch | Fixed. HTTPS is complete and 15 live artifacts match the clean build. |
| Destination-inside-source through a symlink | Fixed. The installed package rejects it with exit 1. |
| Invalid input shared the insufficient-space exit | Fixed. Invalid is 1, insufficient is 2, and unchecked is 3. |
| Dark proof-strip contrast | Fixed. Local and live dark-mode Axe scans are clean. |
| Clean accessibility audit could not create evidence | Fixed. The clean command completes with zero violations. |
| Missing CLI/browser demo and false crates.io install | Fixed. Both sample paths work; only source and tested direct-download installation are advertised. |
| Missing claims registry or untested public claims | Fixed. All 18 declared commands pass separately, and no unlisted claim was found. |
| Missing demo, legal, discovery, metadata, or 404 structure | Fixed. All required routes, metadata, discovery files, links, and the designed 404 pass live. |
| First screen did not state the job, audience, and action plainly | Fixed in fresh desktop and phone contexts. |
| Missing URL verifier | Fixed. The supplied verifier passes both live viewports. |
| Rust 1.85 source install failed | Fixed. Exact-toolchain tests, doctest, and consumer install pass. |
| Node minimum was too broad | Fixed. The stated Node 20.19 floor passes its claim. |
| Phone header navigation was hidden | Fixed. Three route links remain visible on phone. |
| Header or footer targets were below 44×44 px | Fixed. The rendered-target regression passes across every route and viewport. |
| Keep-both assigned two source entries to one destination | Fixed. Library, claim, package-consumer, and live-binary parity evidence all pass. |

## Evidence

Evidence is under `/work/.evidence/verification-6/`. It includes fresh desktop,
phone, and demo screenshots; live browser results; URL-verifier output; the
consumer collision manifest; update-path results; downloaded live artifacts;
and the Lighthouse JSON. This report is also copied to
`/work/.evidence/qa-report.md`, with the machine verdict in
`/work/.evidence/qa-result.json`.

## Final result

**PASS — 0 findings and 0 untested public claims.**
