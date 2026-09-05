# Handoff — File Change Space Check v0.1.0

## Strict review 2 outcome

The fresh strict review is **FAIL** with 4 findings and 2 untested public
claims. The full record is [`.factory/review-2.md`](review-2.md). No product
code was changed during review.

- Implementation and deployment SHA: `a78cad718840554b319e2acc1c243d086a080e42`
- Prior failing review SHA: `5dd443f4ec863fc0bd4b3c1c5af7e253a711da18`
- Live URL: <https://file-change-space-check.sociobot.in/>
- Demo URL: <https://file-change-space-check.sociobot.in/demo>
- Artifact: Rust CLI plus static Vite documentation site

The final handoff and QA records are report-only changes after the deployed
implementation. Their documentation SHA is the commit containing this file;
the live site and binary remain the implementation SHA above.

Review 2 confirmed that the clean default-toolchain gates, all 17 declared
claim commands, installed CLI behavior, live sample, offline path, privacy,
routes, binary parity, and Lighthouse checks pass. It also found:

1. Rust 1.85 is advertised but cannot compile `src/main.rs`; Rust 1.88 passes.
2. Node 20+ is advertised, but Node 20.0 cannot run the site build; Node 20.19
   passes and matches locked Vite's engine requirement.
3. The 390 px header hides all navigation without a replacement menu.
4. Phone footer navigation targets are only 15 px high, below the 44 px
   product requirement.

The Rust and Node boundary statements are not validly covered by the claims
registry, giving an untested-claim count of 2. Product work should correct the
two minimums and their boundary tests, restore phone header navigation, and
enlarge footer hit areas before another verification.

## Independent verification 4

Verification 4 reviewed implementation `a78cad718840554b319e2acc1c243d086a080e42`
and documentation `3f0dfd58c9b4af63082cab377fcd2ab8df0ca332` from a new remote
clone. It is **PASS** with zero findings and zero untested claims. The full
record is [`.factory/verification-4.md`](verification-4.md).

The verifier independently ran every documented quality command, all 17
declared claim commands separately, a packaged-crate consumer install, fresh
desktop and phone live flows, live privacy/offline/accessibility/route checks,
and live binary parity. The local release and hosted Linux binary matched at
SHA-256 `2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da`.
Fresh mobile Lighthouse recorded Performance 100, Accessibility 100, and Best
Practices 100. Its Chromium process logged the known tab-shutdown symptom only
after producing the complete report; no product page console errors occurred.

## What changed

1. `fcsc --demo` creates a bundled media-archive sample under a new temporary
   directory. It runs the real planner, writes a JSON manifest inside that
   sandbox, and prints both paths. Policy and JSON modes work in the sandbox.
2. `/demo` and `/demo/` open a populated policy simulator. Its persistent
   banner says `Demo — sample data, nothing is saved` and provides **Reset
   demo** and **Start for real**. Browser demo state stays in memory.
3. The landing page names the job and audience before scrolling. Its first
   action is **Try it with sample data**. Slogan headings were replaced with
   plain task language.
4. The failing crates.io command was removed. The page and README document a
   tested GitHub checkout plus `cargo install --path . --locked`, and the site
   still serves the tested Linux binary.
5. `.factory/claims.json` registers 17 public claims. Every entry has one
   outcome test tagged `@claim:<id>` and a command that passes independently.
6. Real `/demo`, `/privacy`, `/terms`, `robots.txt`, `sitemap.xml`, and designed
   HTTP 404 responses are deployed. Each page has its own title, one `h1`,
   canonical URL, social metadata, icon, header, footer, and return path.
7. The project supplies `scripts/verify-url.sh`, TypeScript checking,
   behavioral route tests, a copy audit, demo documentation, and a verb-first
   catalog description.

## Review 1 finding disposition

| Finding | Disposition |
| --- | --- |
| P1-1: no CLI/browser demo sandbox | Fixed. CLI and browser demo paths use isolated sample data and have reset/start guidance. |
| P1-2: false registry install | Fixed. The unavailable registry command is explicitly rejected in README; source and download paths pass. |
| P1-3: claims absent | Fixed. All 17 declared commands pass separately from a clean checkout. |
| P1-4: demo, discovery, and 404 routes broken | Fixed live. `/demo`, robots, and sitemap return 200; unknown routes return the designed page with HTTP 404. |
| P2-1: first-screen and slogan copy | Fixed. Job, audience, first action, three facts, and copy audit now meet the plain-words contract. |
| P2-2: metadata missing | Fixed on all routes, with a 1200×630 derived social image and 180×180 touch icon. |
| P2-3: URL verifier missing | Fixed. `scripts/verify-url.sh URL DIR` records desktop/phone screenshots and structural results. |

All earlier findings remain fixed: destination symlink containment, invalid
input exit code, dark-mode contrast, clean-checkout Axe output, TLS, and live
deployment parity were retested.

## Clean-checkout verification

A fresh remote clone at the implementation SHA used the documented setup. All
commands passed:

```sh
npm ci
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
```

`npm test` passed 8 Rust unit tests, 7 CLI integration tests, 1 doctest, 7
browser/site tests, and 17 aggregate claim tests. TypeScript passed strict
checking. Axe checked five routes at 390×844 in light and dark modes: 10 page
runs, zero violations. The package contains the CLI sources, CLI tests, docs,
license, and bundled example files.

Every `test` value in `.factory/claims.json` was then run separately from that
clean checkout. All 17 commands passed. Coverage includes normal, invalid,
insufficient, unchecked, sparse, policy, deterministic, offline, privacy,
keyboard, reset, and recovery paths.

A separate clean consumer installed the packaged crate and ran `fcsc 0.1.0`.
Its `fcsc --demo --policy keep-both --json` returned one conflict and six
actions. A fresh GitHub clone also completed the exact documented source
installation flow.

## Live verification

The durable static deployment completed successfully on the existing
`sf-file-change-space-check` resource. No other service or infrastructure was
read or changed.

- Fresh 1440×1000 and 390×844 browsers showed the job, audience, and sample
  action before scrolling, with no horizontal overflow or console errors.
- `/demo` loaded directly with six realistic actions. Keep-both output,
  22 MiB recovery, reset, the persistent banner, and **Start for real** passed.
- Browser storage stayed empty. All observed requests were same-origin and no
  cookies were set.
- A dedicated context reloaded `/demo/` offline with the correct title and six
  actions after service-worker control.
- Live Axe checked five routes in both themes with zero violations.
- Privacy and terms return 200 with route-specific titles. Unknown paths return
  HTTP 404 with `Page not found — File Change Space Check`.
- `robots.txt` and `sitemap.xml` return 200. GitHub repository and issue links
  return 200.
- The live Linux download byte-matches the local release at SHA-256
  `2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da`.
  It reports `fcsc 0.1.0` and its demo passes.
- CSP, HSTS, `no-referrer`, `nosniff`, and restrictive permissions headers are
  live.

Live Lighthouse wrote a complete report: Performance 100, Accessibility 100,
Best Practices 100, SEO 92; FCP 0.9 s, LCP 1.7 s, CLS 0, TBT 0 ms. Chromium
reported its known tab-shutdown crash after writing the report. SEO lost eight
points because that interrupted run said it could not download `robots.txt`;
direct cold requests returned the valid file with HTTP 200.

Production assets are 4,098 bytes JavaScript, 18,278 bytes CSS, and 172,482
bytes for the hero image. They remain inside all declared budgets.

## Run and deploy

```sh
npm ci
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
scripts/verify-url.sh http://127.0.0.1:4173/ .factory/evidence/url
```

`npm run build` writes `dist/downloads/fcsc` and `dist/site/`. Deployment uses:

```sh
/opt/fleet/lib/deploy-static.sh file-change-space-check /work/repo/dist/site
```

## Remaining limits

- The crate is not published on crates.io. The documented source install and
  hosted Linux download work; registry publication remains an owner action.
- Windows is not implemented. The planner uses Unix allocation metadata.
- Hard links, reflinks, compression, quotas, and reserved filesystem space are
  filesystem-specific and are not modeled.
- Demo CLI sandboxes remain in the operating system's temporary directory for
  inspection. Their printed paths can be removed after use.
