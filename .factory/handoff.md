# Handoff — independent verification 6

## Outcome

Independent verification is **PASS** with **0 findings** and **0 untested
public claims**. No product code was changed.

- Implementation reviewed: `5e868b086921f520442e85431cfb98cc092e48d0`
- Documentation base reviewed: `6f41f9a67c41ff0f0082c936731ba39901973eaf`
- Live URL: <https://file-change-space-check.sociobot.in/>
- Full report: `.factory/verification-6.md`

The documentation base differs from the implementation only in the prior
handoff. Fifteen live artifacts byte-match a fresh build from that base. The
fresh and live Linux binaries share SHA-256
`86539a4f2a5457ab1e41ac6f9d764d0dfd9f547684cc484fa2718687d9586687`.

## Verification completed

A clean remote checkout installed documented prerequisites and passed:

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

Every one of the 18 commands in `.factory/claims.json` passed separately.
The clean aggregate includes 9 Rust library tests, 7 CLI integration tests, 1
doctest, 8 site tests, 18 claim tests, and 10 local Axe scans with zero
violations.

The packaged crate installed into a new Rust 1.85 consumer prefix. Normal,
invalid, boundary, recovery, sparse insufficient-space, symlink containment,
all conflict policies, demo isolation, and read-only paths passed. The repaired
file-and-directory suffix collision produced six actions with six unique
destinations and left both trees unchanged.

Fresh desktop and phone contexts verified the first screen and one-click
sample. The persistent label, six populated actions, keyboard use, invalid
bounds, recovery, reset, start-for-real link, empty browser data stores,
same-origin requests, offline reload, 200% text, reduced motion, route titles,
legal pages, all links, security headers, and designed HTTP 404 passed. Ten
live light/dark Axe scans found zero violations.

Live mobile Lighthouse scored 100 Performance, 100 Accessibility, 100 Best
Practices, and 100 SEO. FCP was 1.1 s, LCP 1.7 s, TBT 60 ms, and CLS 0.

## Earlier findings

All earlier deployment, containment, exit-code, contrast, audit-harness, demo,
claim, route, metadata, 404, first-screen copy, URL-verifier, minimum-toolchain,
phone-navigation, touch-target, and keep-both collision findings are fixed.
Their current evidence is listed individually in the verification report.

## Known limits

Crates.io publication and Windows support remain owner work. Hard links,
reflinks, compression, quotas, and reserved filesystem space remain outside the
documented model. The product has no backend, account data, payment, or
external integration.

## Evidence

The full report is copied to `/work/.evidence/qa-report.md`. The machine result
is `/work/.evidence/qa-result.json`. Browser screenshots, URL verification,
consumer collision output, update checks, live downloads, and Lighthouse output
are under `/work/.evidence/verification-6/`.
