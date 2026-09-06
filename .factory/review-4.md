# Check space before copying files — strict review 4

Reviewed 6 September 2026 against <https://file-change-space-check.sociobot.in/>.

## Verdict

**PASS — 0 findings and 0 untested public claims.**

- Implementation candidate: `5e868b086921f520442e85431cfb98cc092e48d0`
- Documentation commit reviewed: `a988b9cdb523f409c34f399468c15affdcec67bf`
- P0: 0; P1: 0; P2: 0; P3: 0; untested public claims: 0

The documentation commit differs from the implementation candidate only in
`.factory/handoff.md` and `.factory/verification-6.md`. No product code was
changed for this review.

## Job, audience, and first action

The job is to estimate free space and conflicts before copying, extracting, or
reorganising a large local folder. It is for people moving large folders who
need an estimate before a long copy starts.

Fresh 1440×1000 desktop and 390×844 phone contexts opened at scroll position
zero. Both showed **Check space before you copy files**, the intended audience,
and **Try it with sample data** before scrolling. The phone action was a
366×48 px target; the three visible phone navigation links were each at least
44 px high. There were no page or console errors. The inspected phone capture
is `live/phone-home.png` in the evidence directory.

## Demo and browser checks

One click opened `/demo/` in both fresh contexts. It had the route title, the
persistent **Demo — sample data, nothing is saved** label, and six realistic
manifest actions: archive directory and files, project notes, a sparse disk
image, and the `photos.raw` conflict.

- Keyboard Space selected keep-both and showed `photos (copy 1).raw`.
- `-1` free space set `aria-invalid="true"` and announced the 0–9,999 MiB
  error. Entering `22` recovered to the safe result.
- **Reset demo** restored overwrite and 16 MiB. The banner remained visible
  after scrolling. **Start for real** returned to `/#install` with no banner.
- The fresh browser data stores were empty: localStorage, sessionStorage,
  IndexedDB, and cookies. The full flow made only same-origin requests.
- After the service worker controlled a new phone context, `/demo/` reloaded
  offline with its title, six actions, and **Offline · sample works**.

Live Axe scanned home, demo, privacy, terms, and the designed unknown-route
page in light and dark colour schemes: 10 scans and zero violations. Keyboard
focus began at the skip link; reduced motion set transitions to `0.00001s` and
scroll behaviour to `auto`. The supplied URL verifier passed fresh desktop and
phone checks for status, title, language, main landmark, image alternatives,
button names, console errors, and horizontal overflow.

## Routes, privacy, and release parity

Home, demo, privacy, and terms returned 200 with their own titles, one `h1`,
and a `main` landmark. `/does-not-exist-review4` deliberately returned the
designed 404 document with a return path; its expected HTTP 404 is not a
defect. All discovered links resolved: product routes and download returned
200; the repository and issue links on GitHub returned 200. The self-link on
the designed 404 intentionally retained its deliberate 404 response.

The live response has the declared CSP including `frame-ancestors 'none'`,
HSTS, `no-referrer`, `nosniff`, and restrictive device permissions. Runtime
checks found no cookies, analytics, remote fonts, or third-party code.

Fifteen live files byte-match a new production build of the implementation:
five HTML documents, service worker, discovery documents, marks, images,
hashed JavaScript and CSS, and the Linux binary. The matching binary SHA-256:

```text
86539a4f2a5457ab1e41ac6f9d764d0dfd9f547684cc484fa2718687d9586687
```

## Clean checkout and installed artifact

A new remote clone at the documentation SHA installed Node dependencies and
Rust 1.85.0. These commands passed:

```text
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

`npm test` passed 9 library tests, 7 CLI integration tests, 1 doctest, 8 site
tests, and 18 aggregate claim tests. The local accessibility audit reported
10 pages with zero violations. The build created `dist/`; its initial
JavaScript is 4.10 kB and CSS is 18.75 kB before gzip.

The verified package installed into a new Rust 1.85 consumer prefix and ran as
`fcsc 0.1.0`. The installed command produced an unchecked normal JSON plan
(exit 3), rejected an invalid policy with an actionable exit 1, flagged a
1 TiB expanded sparse input as insufficient (exit 2), and ran the bundled
keep-both demo (exit 0, six actions). Source and destination hashes were the
same before and after planning.

## Claims

Every command in `.factory/claims.json` was run separately from the clean
checkout. Each selected exactly one tagged sandbox test and passed:

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

The landing page, demo, privacy and terms pages, README, CLI help, and demo
guide were cross-checked with this registry. Their reliance claims about the
sample, metadata-only read-only planning, policies, sparse bounds, manifest,
exit codes, supported toolchains, local operation, privacy, offline use,
estimate accuracy, and Linux binary are covered. The static free/open-source
statement is consistent with the MIT-licensed source and absence of any payment
or account path. No missing, false, incomplete, or untested reliance claim was
found.

## Earlier findings and their current disposition

| Earlier finding | Current disposition |
| --- | --- |
| Missing deployment or TLS; live/local mismatch | Fixed. HTTPS and 15 live artifacts match the candidate build. |
| Symlinked destination-inside-source plan | Fixed. Library and consumer paths reject it with exit 1. |
| Invalid input used the insufficient-space code | Fixed. Invalid is 1, insufficient is 2, unchecked is 3. |
| Dark-mode contrast and clean accessibility audit failures | Fixed. Local and live light/dark Axe are clean; the clean audit completes. |
| Missing CLI/browser demo and false crates.io command | Fixed. Both isolated demos work; only source and tested direct download paths are advertised. |
| Missing registry or untested claims | Fixed. All 18 commands passed separately; no unlisted reliance claim remains. |
| Missing routes, legal/discovery files, metadata, 404, copy audit, or URL verifier | Fixed. Required resources, metadata, verifier, links, and deliberate 404 all pass. |
| First screen was unclear or lacked a sample action | Fixed. Fresh desktop and phone first screens state the job, audience, and action plainly. |
| Rust 1.85 or Node minimum claims were false | Fixed. Exact Rust 1.85 and Node 20.19 claim tests pass. |
| Phone navigation hidden or targets too small | Fixed. Three header links remain visible and all measured targets meet 44 px. |
| Keep-both assigned two entries to one destination | Fixed. The repair claim and installed-artifact paths pass; prior collision coverage remains green. |
| Earlier Lighthouse shutdown note | No page error reproduced. Byte-identical live output preserves the prior successful performance evidence. |

## Evidence and limits

Evidence is under `/work/.evidence/review-4/`: separate claim logs, clean
quality-gate logs, package-consumer output, parity hashes, URL checks, link
checks, screenshots, and live browser JSON. This report is also copied to
`/work/.evidence/qa-report.md`; the matching machine result is
`/work/.evidence/qa-result.json`.

The documented limits remain: the crate is not published to crates.io, Windows
support is not offered, and hard links, reflinks, compression, quotas, and
reserved filesystem space are outside the estimator model. These are disclosed
limits, not defects in the reviewed Linux product.

## Final result

**PASS — 0 findings and 0 untested public claims.**
