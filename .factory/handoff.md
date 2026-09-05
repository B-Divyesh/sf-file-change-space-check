# Handoff — File Change Space Check v0.1.0

## Outcome

Strict review 2 is resolved. The product is a free, read-only CLI for people
who need to estimate free space and conflicts before copying a large local
folder. The first action on the site is **Try it with sample data**.

Independent verification 5 passed on 2026-09-05 with zero findings and zero
untested claims. Its full report is in `.factory/verification-5.md`.

- Implementation and deployed SHA: `09a5ebda40e50b4645f26da80e3a62d73cd20570`
- Earlier review/documentation SHA: `0e652897c3218ce215a2436b66999c0448b815f4`
- This handoff is a later documentation-only commit and does not change the
  deployed image.
- Independent verification 5 reviewed documentation SHA
  `172eb9ce12c43bc2486ab091f4306021a5d12bdf` against that deployed
  implementation.
- Live site: <https://file-change-space-check.sociobot.in/>
- Browser sample: <https://file-change-space-check.sociobot.in/demo/>

## Repairs made

1. Rust 1.85 source installs now work. The incompatible let-chain was replaced
   with stable nested conditionals. The source-install claim performs a real
   fresh consumer install with exactly Rust 1.85.0 and executes `fcsc --version`.
2. The documented Node floor is now Node.js 20.19+ in the README,
   `package.json`, and lockfile. A new claim runs the complete site build with
   exactly Node 20.19.0 and checks its static output.
3. Phone headers retain the three direct route links on every route. Header and
   footer route links are now at least 44×44 CSS px on both 390 px phone and
   1440 px desktop layouts.
4. The regression browser test covers header/footer visibility, target sizes,
   all main routes, the designed 404, and horizontal overflow. It observes
   rendered dimensions instead of asserting CSS source text.
5. The claims registry has 18 independently runnable outcome checks. The two
   version-boundary public statements are now covered.

## Earlier finding disposition

| Finding | Current disposition |
| --- | --- |
| Rust 1.85 cannot compile | Fixed; `cargo +1.85.0 test --locked --all-targets`, doctests, and a clean consumer `cargo +1.85.0 install` pass. |
| Node 20.0 advertised too broadly | Fixed honestly by publishing the Vite-compatible Node 20.19+ floor and testing the exact boundary. |
| Phone header removes navigation | Fixed; direct links remain visible and keyboard reachable at 390 px on home, demo, privacy, terms, and 404. |
| Phone footer targets are 15 px high | Fixed; all route links measure at least 44×44 px. Desktop header links were also enlarged to meet the shared target standard. |
| Untested Rust and Node public claims | Fixed; the Rust boundary is included in `source-install`, and `node-build-minimum` is a new declared claim. |
| Symlink containment, invalid exit code, dark contrast, demo, privacy, offline, metadata, discovery, and 404 findings from earlier reviews | Remain fixed; current clean and live checks passed. |

## Verification

Independent verification 5 repeated the full clean-clone and live checks. It
installed the packaged crate into a new consumer prefix, ran every one of the
18 claim commands separately, exercised normal, invalid, boundary, recovery,
and symlink-containment paths, and found no defect. Fresh live phone and desktop
contexts also passed demo/reset/isolation, route, keyboard, offline/update,
privacy, link, header, 404, and parity checks. Ten live Axe scans found zero
violations. Lighthouse scored 100 for performance, accessibility, best
practices, and SEO, with FCP 1.0 s, LCP 1.7 s, TBT 30 ms, and CLS 0.

A fresh remote clone at the implementation SHA ran after `npm ci` and the
documented Rust 1.85 toolchain installation:

```sh
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
npm audit --omit=dev
```

All commands passed. `npm test` passed 8 Rust library tests, 7 CLI integration
tests, 1 doctest, 8 browser/site tests, and all 18 aggregate claim tests.
The accessibility audit scanned home, demo, privacy, terms, and the designed
404 in light and dark mode (10 scans, zero violations). `cargo package`
verified the crate. The production asset sizes are 4.10 kB JS, 18.75 kB CSS,
and 172,482 bytes for the original hero image.

Every one of the 18 `test` commands in `.factory/claims.json` was then run
separately from that same clean checkout. All passed. This includes exact Rust
1.85 source installation, exact Node 20.19 static build, the CLI and browser
demos, sparse boundaries, invalid/recovery paths, privacy, offline reload,
and Linux binary parity.

A separately packaged crate was installed into a new consumer root. Its
installed `fcsc --demo --policy keep-both --json` emitted schema 1, one
conflict, six actions, and `photos (copy 1).raw`.

## Live verification

The static deployment completed on the existing `sf-file-change-space-check`
resource without changing its durable configuration.

- The live home, service worker, and Linux artifact byte-match the final build.
  SHA-256 values are respectively
  `cfa9bc18d3f8c84a5437a99a065805aefcb0a4719d8edd44f4604f32eebc680c`,
  `8d92bcbb6b3e02731a797a44020849e32c8b7eef59256a50df4c1f6b4760a47f`, and
  `2baa828d15ca9d61251ef86cd83046d2315dc91bd5623523a70d24d12699d6da`.
- Fresh desktop and phone contexts show the job, audience, and **Try it with
  sample data** before scrolling. They have no console errors or horizontal
  overflow, and their header/footer route links measure at least 44×44 px.
- The live sample starts with six realistic actions and its persistent
  **Demo — sample data, nothing is saved** label. Keyboard keep-both changes
  the conflict output, `-1` reports an error, `22` recovers to **SAFE TO
  START**, reset restores the sample, and **Start for real** removes demo mode.
  Browser storage, cookies, and external requests remain empty/absent.
- A dedicated controlled context reloaded `/demo/` offline with the right
  title and six sample actions.
- Live Axe scanned five routes in light and dark mode at 390×844: 10 scans,
  zero violations. Home, demo, privacy, terms, robots, and sitemap return 200;
  unknown paths return the designed HTTP 404. CSP, HSTS, no-referrer, nosniff,
  and restrictive permissions headers are live.
- Lighthouse wrote a complete mobile report: Performance 100, Accessibility
  100, Best Practices 100, SEO 92; FCP 1.1 s, LCP 1.7 s, CLS 0, TBT 50 ms.
  The Lighthouse runner reported a Chromium tab-shutdown symptom after it had
  written the complete report. Independent live page contexts had no console
  or page errors.

## Run, verify, and deploy

```sh
rustup toolchain install 1.85.0 --profile minimal
npm ci
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
scripts/verify-url.sh https://file-change-space-check.sociobot.in/ .factory/evidence/url
/opt/fleet/lib/deploy-static.sh file-change-space-check /work/repo/dist/site
```

The catalog description is a verb-first 68-byte line in
`.factory/catalog-description.txt` and was copied to
`/work/.evidence/catalog-description.txt`.

## Remaining limits

- The crate is not published on crates.io. The documented source install and
  tested Linux x86-64 download work; registry publication is an owner action.
- Windows support is not implemented. The allocation checks use Unix metadata.
- Hard links, reflinks, compression, quotas, and reserved filesystem space are
  filesystem-specific and intentionally not modeled.
- The product is free, so there is no billing offer or billing-registration
  dependency.
