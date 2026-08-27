# Handoff — File Change Space Check v0.1.0

## What shipped

- A read-only Rust CLI (`fcsc`) that plans a source tree into a destination,
  with required `overwrite`, `skip`, and `keep-both` conflict policies.
- Deterministic, versioned JSON manifests (`--json` and `--manifest`) plus a
  compact human report and stable exit codes: 0 safe, 1 input/scan error,
  2 insufficient space, 3 unchecked space.
- Lower/upper allocation bounds for sparse files, destination block-size and
  available-space checks, conservative pre-reclamation headroom, recursive type
  conflict handling, symlink metadata, and explicit special-file skips.
- A Vite/TypeScript documentation site at `dist/site` with a keyboard-friendly
  live policy simulator, install/download path, honest limits, offline cache,
  dark mode, 390 px layout, privacy page, and terms page.
- An original 1536×1024 allocation-ledger hero generated for this product and
  optimized to 172,482-byte WebP. Prompt and provenance are in
  `.factory/design.md`.
- README usage contract, MIT license, changelog, typed public Rust API with a
  compiling doctest, clean-clone scripts, and a registry-ready crate definition.

## Run and verify

```sh
npm install
npm test
npm run build
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm run audit:a11y
cargo clippy --all-targets -- -D warnings
cargo package
```

`npm run build` is the release command. It creates:

- `dist/downloads/fcsc` — stripped Linux x86_64 CLI
- `dist/site/index.html` — static deployment root
- `dist/site/downloads/fcsc-linux-x86_64` — website download

## Verification on 2026-08-27

- Rust: 6 unit tests, 3 CLI integration tests, and 1 doctest passed.
- Site: 4 structural/budget tests passed; `npm audit` reported 0 vulnerabilities.
- `cargo clippy --all-targets -- -D warnings`: passed.
- Playwright browser smoke test: no page/console errors; title, `lang`, one h1,
  main landmark, image alternatives, and button names present at desktop and
  390×844 mobile viewports.
- Axe 4.13 Playwright scan on `/`, `/privacy/`, and `/terms/`: 0 violations.
- Lighthouse 13 mobile/simulated throttling: Performance 100, Accessibility
  100, Best Practices 100, SEO 100; LCP 1.8 s, FCP 1.0 s, CLS 0, TBT 0 ms.
  INP is not available for a synthetic no-interaction run.
- Initial transfer: 197 KiB. Built assets: JS 3,872 bytes, CSS 14,339 bytes,
  hero WebP 172,482 bytes. CLI binary: 719,072 bytes.
- `cargo package`: verified locally; publishing intentionally not performed.

## Known gaps and next steps

- v0.1 targets Unix metadata APIs. Linux is shipped; macOS is supported from
  source but needs a factory-built release artifact. Windows support is next.
- Metadata cannot prove that the eventual copy command has write permission or
  will preserve sparse holes. The manifest calls both out, and `auto` uses the
  expanded upper bound for the verdict.
- Hard-link preservation, reflinks, compression, quotas, and reserved filesystem
  space are copy-tool/filesystem-specific and are not modeled. The conservative
  upper bound remains the safe default.
- The browser simulator is an explanatory fixed fixture, not a browser filesystem
  scanner. Real paths remain local to the CLI by design.
