# Handoff — File Change Space Check repair 4

## Outcome

The keep-both collision is repaired and deployed.

- Implementation SHA: `5e868b086921f520442e85431cfb98cc092e48d0`
- Documentation: this handoff is a later documentation-only update; it does
  not alter that implementation.
- Live URL: <https://file-change-space-check.sociobot.in/>
- Deployment: static site deployed from the matching `dist/site` build on
  2026-09-05 UTC.

## Repair

The planner reserves every emitted action destination. Before processing a
source directory, it also reserves every ordinary child destination. A
keep-both name now avoids existing paths, earlier actions, and later sibling
source names that already look like generated copy names.

The `conflict-policies` claim and a Rust library test cover both the reported
file collision (`photo.jpg` plus `photo (copy 1).jpg`) and the corresponding
directory collision. They assert unique manifest destinations, preservation of
both source entries, and the next free deterministic suffix.

## Verification

A clean detached worktree at `5e868b0` ran `npm ci` and passed:

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

That includes 9 Rust library tests, 7 CLI integration tests, 1 doctest, 8
site tests, 18 aggregate claim tests, package verification, and 10 local
light/dark Axe scans with zero violations. Every one of the 18 commands in
`.factory/claims.json` also passed separately from that clean worktree.

The packaged crate installed into a fresh consumer prefix. The installed
`fcsc 0.1.0` planned the repaired file-and-directory collision fixture with
exit 3, six actions, and six unique destinations.

The staged release binary and live Linux download match:

```text
86539a4f2a5457ab1e41ac6f9d764d0dfd9f547684cc484fa2718687d9586687
```

After deployment, the supplied URL verifier passed fresh desktop and phone
contexts. Both saw the headline, audience, and **Try it with sample data**
before scrolling. The live demo showed six actions, its persistent sample
label, keyboard keep-both selection, invalid-value recovery, reset, and empty
browser storage/cookies. A separate context reloaded the demo offline.

Home, demo, privacy, terms, robots, sitemap, the Linux download, and all 13
linked destinations returned successfully. The unknown route deliberately
returns the designed HTTP 404 with its own title, `h1`, and return links. Live
Playwright Axe scans across these five pages in light and dark mode found zero
violations.

Live mobile Lighthouse wrote a complete report: Performance 100,
Accessibility 100, Best Practices 100, SEO 92; FCP 1.0 s, LCP 1.7 s, TBT 0
ms, CLS 0. Lighthouse 13 marked the otherwise valid `robots.txt` audit as
invalid; direct HTTPS inspection returned standard `User-agent`, `Allow`, and
sitemap directives. Chromium logged a tab-shutdown message only after the
complete report was written; fresh page loads had no console or page errors.

## Earlier findings

The prior TLS/deployment, symlink containment, exit-code, dark contrast,
accessibility-audit, demo, claims, metadata/404, plain-copy, phone navigation,
touch-target, Rust 1.85, and Node 20.19 findings remain fixed and are covered
by the current clean and live checks. Review 3's keep-both finding is now
covered by library, claim, package-consumer, and deployed-artifact checks.

## Remaining limits

Crates.io publication and Windows support remain owner work. Hard links,
reflinks, compression, quotas, and reserved filesystem space are outside the
estimator's model. This product has no backend, account data, paid offer, or
external integration.

## Evidence

Browser screenshots, URL verification JSON, Lighthouse JSON, and the catalog
description are under `/work/.evidence/repair-4/` and
`/work/.evidence/catalog-description.txt`.

## Superseded review 3 record

### Outcome

Strict review 3 is **FAIL** with **1 P1 finding** and **0 untested public
claims**. No product code was changed.

- Review: `.factory/review-3.md`
- Implementation reviewed: `09a5ebda40e50b4645f26da80e3a62d73cd20570`
- Documentation base reviewed: `f1407370c9462deb91c504354db26c0608182d30`
- Live URL: <https://file-change-space-check.sociobot.in/>

### Finding left for repair

Keep-both can emit two actions with the same destination when the source
already includes the generated alternate name:

```text
source/photo.jpg
source/photo (copy 1).jpg
destination/photo.jpg
```

Both planned actions target `destination/photo (copy 1).jpg`. The planner's
generated-path reservation does not include ordinary planned destinations.
This breaks the keep-both policy and makes the action manifest unsafe for a
consumer to apply as written.

Reserve every planned destination and make alternate-name selection avoid the
complete set of existing and planned paths. Add regression tests for source
file and directory names that collide with generated keep-both suffixes.

### Verification completed

A fresh remote clone at `f140737` installed Node dependencies and Rust 1.85.0.
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

All 18 declared claim commands were also run separately and exited
successfully. The `conflict-policies` fixture does not cover the failing source
name collision, so its pass does not clear the P1 finding.

The packaged crate was installed into a new consumer prefix. Version/help,
overwrite, skip, ordinary keep-both, unchanged trees, invalid input, missing
source, recovery, sparse insufficient space, symlink containment, and the
temporary CLI demo behaved as documented. The new collision fixture reproduced
the finding with the installed artifact.

Fresh live desktop and phone checks covered the first screen, one-click sample,
persistent sample label, reset, start-for-real, populated output, invalid and
recovery values, keyboard use, focus, 200% text, reduced motion, storage,
requests, offline reload/update, route titles, legal pages, links, headers, and
the deliberate 404. Ten light/dark live Axe scans had zero violations.
Lighthouse scored 100/100/100/100 with FCP 0.97 s, LCP 1.65 s, TBT 44 ms, and
CLS 0.

Eleven live artifacts byte-match the clean implementation build. The live
Linux binary, packaged binary, and release build share SHA-256
`2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da`.

### Evidence and next step

The final review report is copied to `/work/.evidence/qa-report.md` and the
machine verdict is in `/work/.evidence/qa-result.json`. Lighthouse and browser
screenshots are under `/work/.evidence/review-3/`.

The next worker should repair only the keep-both destination reservation,
extend the claim/regression fixture, rerun all commands above, deploy through
the factory, and request a fresh strict review.
