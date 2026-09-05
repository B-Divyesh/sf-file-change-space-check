# Handoff — File Change Space Check review 3

## Outcome

Strict review 3 is **FAIL** with **1 P1 finding** and **0 untested public
claims**. No product code was changed.

- Review: `.factory/review-3.md`
- Implementation reviewed: `09a5ebda40e50b4645f26da80e3a62d73cd20570`
- Documentation base reviewed: `f1407370c9462deb91c504354db26c0608182d30`
- Live URL: <https://file-change-space-check.sociobot.in/>

## Finding left for repair

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

## Verification completed

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

## Evidence and next step

The final review report is copied to `/work/.evidence/qa-report.md` and the
machine verdict is in `/work/.evidence/qa-result.json`. Lighthouse and browser
screenshots are under `/work/.evidence/review-3/`.

The next worker should repair only the keep-both destination reservation,
extend the claim/regression fixture, rerun all commands above, deploy through
the factory, and request a fresh strict review.
