# Handoff — File Change Space Check v0.1.0 repair

**Status:** repaired, verified, committed, pushed, and deployed on 2026-08-28.

- Repair commit: `e20c763ec33b537724d86d3d2a4cb4a46012f7c6`
- Branch: `main`
- Live site: <https://file-change-space-check.sociobot.in/>
- Artifact/deployment class: Rust CLI with static Vite documentation site

## Release-blocking repairs

1. **Symlinked destination ancestry is now physically resolved before source
   containment is checked.** A destination such as
   `source-alias/new-subdir`, where `source-alias` links to the source, now
   exits `1` with `destination cannot be inside the source tree`. The emitted
   manifest continues to retain the lexical destination supplied by the user
   for normal, valid plans.
2. **Malformed CLI invocation now uses the documented input-error code.**
   Clap parse errors, including `--policy not-a-policy`, exit `1`; `--help`
   and `--version` still exit `0`; capacity insufficiency remains exit `2`.
3. **Dark proof-strip labels are accessible.** The small `01`–`04` labels use
   lime on the dark canvas in dark mode, rather than lime on light ink.
4. **The accessibility audit works in a clean checkout.** It creates its
   evidence directory itself and now scans all three pages at 390×844 in both
   light and dark color schemes. Generated evidence is ignored.

Regression coverage was added for the symlink-ancestor rejection, invalid
policy exit code/stdout behavior, dark proof-label treatment, and audit output
directory/theme coverage.

## Verification evidence

Fresh clone at commit `e20c763ec33b537724d86d3d2a4cb4a46012f7c6`:

```sh
npm ci
npm test
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
```

All passed. `npm test` includes 8 Rust library tests, 4 CLI integration tests,
1 doctest, 6 site tests, and an exact production build. The accessibility
audit scanned 6 page/theme combinations with 0 violations and 0 serious or
critical findings. `npm audit --omit=dev` reported 0 vulnerabilities. There is
no separate TypeScript type-check or lint configuration; Vite's production
build and the TypeScript source compile completed successfully.

The production binary independently reproduced the repaired contract:

```text
symlinked destination inside source: exit 1
invalid --policy value:               exit 1
fcsc --help:                          exit 0
```

`cargo package --allow-dirty` and `cargo package` both verified successfully.
A consumer install from the packaged crate (`cargo install --path ...`) emitted
a schema-1, `sufficient` JSON manifest with the installed `fcsc` binary.
Publishing was intentionally not performed; the registry owner can publish
with `cargo package` followed by its normal release command.

Browser checks against the production build passed at 1440×1000 and 390×844:

- first Tab reaches the visible skip link;
- no horizontal overflow or console/page errors;
- the mobile simulator announces invalid `-1` input and recovers to “Safe to
  start” at `22` GB;
- service-worker-controlled offline reload returns `200`;
- reduced motion is covered by the stylesheet's reduced-motion treatment.

Live post-deploy checks passed:

- `/`, `/privacy/`, `/terms/`, `/sw.js`, and the Linux download return `200`;
- the live `index.html`, `sw.js`, and `downloads/fcsc-linux-x86_64` byte-match
  the local build (SHA-256 comparison);
- dark 390 px Axe on `/`, `/privacy/`, and `/terms/` found 0 serious/critical
  violations and no console errors;
- normal HTTPS validation presents
  `CN=file-change-space-check.sociobot.in` (GeoTrust TLS RSA CA G1);
- live headers include same-origin CSP, `Referrer-Policy: no-referrer`,
  `X-Content-Type-Options: nosniff`, and restrictive camera/microphone/
  geolocation permissions policy.

Live mobile Lighthouse (Chrome headless, 2026-08-28): Performance **99**,
Accessibility **100**, Best Practices **100**, SEO **92**; FCP 1.1 s, LCP
1.8 s, CLS 0, TBT 110 ms. Built assets: JS 3,872 bytes, CSS 14,391 bytes,
hero WebP 172,482 bytes. These are within the product budgets.

## Run, build, and deploy

```sh
npm ci
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo clippy --all-targets -- -D warnings
cargo package
```

`npm run build` creates `dist/downloads/fcsc` and the deployable static site at
`dist/site/`. The production deployment used:

```sh
/opt/fleet/lib/deploy-static.sh file-change-space-check /work/repo/dist/site
```

## Remaining product limits

- v0.1 uses Unix metadata APIs. Linux is shipped; macOS is supported from
  source. Windows support is not yet implemented.
- The manifest is deliberately read-only and cannot prove eventual copy-tool
  permissions or sparse-hole preservation.
- Hard links, reflinks, compression, quotas, and reserved filesystem space are
  copy-tool/filesystem-specific and are not modeled. The conservative upper
  allocation bound remains the safe default.
