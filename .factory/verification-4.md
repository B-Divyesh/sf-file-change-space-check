# Independent verification 4 — PASS

**Verified:** 2026-09-05 UTC  
**Implementation candidate:** `a78cad718840554b319e2acc1c243d086a080e42`  
**Documentation commit:** `3f0dfd58c9b4af63082cab377fcd2ab8df0ca332`  
**Live URL:** <https://file-change-space-check.sociobot.in/>

## Verdict

**PASS.** There are **zero findings** at every severity and **zero untested
public claims**. The live runtime matches the reviewed implementation. The
documentation-only commit is later than the deployed implementation and does
not change the product image.

## Job, audience, and first action

- **Job:** estimate free space and conflicts before copying, extracting, or
  reorganising a large local folder.
- **Audience:** people moving large folders who need an estimate before a long
  copy starts.
- **First action:** **Try it with sample data**. It was visible before scrolling
  in fresh 1440×1000 desktop and 390×844 phone contexts, alongside the plain
  explanation that it opens a finished sample plan in one click.

## Clean-checkout evidence

A new remote clone at documentation SHA `3f0dfd5` was checked to verify that it
contains implementation SHA `a78cad7` as an ancestor. After `npm ci`, every
documented quality command passed:

```sh
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
```

`npm test` built both artifacts, type-checked TypeScript, passed 8 Rust library
tests, 7 CLI integration tests, 1 doctest, 7 site tests, and 17 aggregate
claim tests. The standalone build wrote `dist/downloads/fcsc` and `dist/site/`.
The accessibility audit scanned five routes in both light and dark themes (10
page runs) with zero violations. `cargo package` verified the package and the
production dependency audit found zero vulnerabilities.

All 17 declared commands from `.factory/claims.json` were also run separately
from that clone. Each selected exactly its tagged outcome test and passed:

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
| `source-install` | PASS |
| `cli-local-only` | PASS |
| `browser-demo` | PASS |
| `site-privacy` | PASS |
| `offline-demo` | PASS |
| `non-executable-manifest` | PASS |
| `estimate-within-two-percent` | PASS |
| `linux-download` | PASS |

This leaves **0/17 untested claims**. Copy and documentation were cross-checked
against the registry: read-only planning, metadata-only scanning, policies,
sparse bounds, JSON manifests, exit codes, local-only operation, browser demo,
privacy, offline use, non-executable manifests, estimate accuracy, and the
Linux artifact each have a matching tested claim. The source requirement is
encoded by `rust-version = "1.85"`; `cargo package` and the clean source
installation completed successfully.

## CLI and consumer artifact

A fresh consumer root installed the verified package with:

```sh
cargo install --path target/package/file-change-space-check-0.1.0 --root <fresh-root> --locked
```

The installed `fcsc 0.1.0` then ran `--demo --policy keep-both --json`. It
created an isolated temporary sandbox and emitted a schema-1, unchecked
manifest with one conflict and six actions, including
`photos (copy 1).raw`. Normal, invalid, boundary, and recovery behavior is
covered independently by the CLI suite and claim commands: policy-specific
plans; invalid input exit 1; insufficient space exit 2; unchecked exit 3;
symlinked destination-inside-source rejection; sparse upper-bound verdict;
and no source or destination changes during planning.

The fresh local release and live download have the identical SHA-256:

```text
2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da
```

## Live product evidence

Fresh desktop and phone pages loaded with HTTP 200, the expected plain-language
title, `lang="en"`, exactly one `h1`, exactly one `main`, no missing image
alt text, no unlabeled buttons, no horizontal overflow, and no console errors.

Direct `/demo/` verification found the persistent **Demo — sample data,
nothing is saved** label, six realistic actions, and both **Reset demo** and
**Start for real**. Keyboard selection of keep-both changed the conflict to
`photos (copy 1).raw`. An invalid `-1` free-space value set `aria-invalid` and
announced the corrective error; `22` recovered to **SAFE TO START**; reset
restored overwrite and 16 MiB. No localStorage, sessionStorage, or cookies
were created, and the demo never provides a local-file input.

The full home-to-demo request capture contained only product-origin documents,
CSS, JavaScript, and the self-hosted image. It set no cookies. A dedicated
service-worker-controlled context reloaded `/demo/` offline with HTTP 200,
the correct title, and all six sample actions. Reduced-motion mode reduced
hero animation and transition durations to `0.00001s`; first Tab reached the
visible 3 px skip-link outline.

Live route checks passed for `/`, `/demo/`, `/privacy/`, `/terms/`,
`/robots.txt`, and `/sitemap.xml`. Route titles and canonical tags are correct;
the designed unknown-path response is intentionally HTTP 404 with title
`Page not found — File Change Space Check`, one `h1`, and return links. Every
internal link/download and both GitHub links returned 200. The deliberate 404
is expected behavior, not a defect.

Live Axe checks covered home, demo, privacy, terms, and the designed 404 at
390×844 in light and dark schemes: 10 scans, zero violations, zero
serious/critical issues. Fresh mobile Lighthouse recorded Performance 100,
Accessibility 100, and Best Practices 100 (FCP 1.0 s, LCP 1.7 s, CLS 0, TBT
0 ms). Chromium logged its known tab-shutdown crash only after Lighthouse had
written the complete report; independent page loads had no console errors.

The live response supplies HSTS, restrictive same-origin CSP with
`frame-ancestors 'none'`, `Referrer-Policy: no-referrer`, `nosniff`, and a
restrictive camera/microphone/geolocation permissions policy.

## Earlier findings disposition

| Earlier finding | Current disposition |
| --- | --- |
| Initial missing deployment/TLS and partial live output | Fixed; HTTPS validates and live runtime is complete. |
| Symlinked destination-inside-source plan | Fixed; regression unit test and clean suite pass. |
| Invalid input shared the insufficient-space exit | Fixed; invalid input returns documented exit 1. |
| Dark proof-strip contrast | Fixed; live Axe is clean in dark mode. |
| Clean-clone accessibility audit could not write evidence | Fixed; fresh audit completes. |
| Missing demo sandbox, false crates.io install, absent claims | Fixed; real CLI/browser demos, source/download install, and 17 claim tests pass. |
| Missing demo/discovery/404 routes, metadata, copy audit, URL verifier | Fixed; all routes, metadata, links, verifier, and plain first screen pass. |
| Lighthouse tab-shutdown note | Reproduced after a complete 100/100/100 report; it is a Chromium process shutdown symptom, not a page error. |

## Remaining product limits

Crates.io publication and Windows support remain owner work, as documented.
They do not contradict any public install claim or block this Linux-focused
read-only CLI. Filesystem-specific hard links, reflinks, compression, quotas,
and reserved space are deliberately outside the estimator's stated model.
