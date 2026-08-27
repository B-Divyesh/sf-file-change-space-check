# Independent verification — FAIL

**Candidate:** `9e99be7b34c44f954f73d4e5411bac12adcb3923`  
**Verified:** 2026-08-27  
**Production URL:** `https://file-change-space-check.sociobot.in`

## Verdict

**FAIL. Do not release this candidate.** The public deployment is unavailable
at its required hostname, and the CLI has a source/destination-overlap safety
hole when the destination reaches the source through a symlink. The dark web
treatment also has serious accessibility violations.

## Release blockers

### P0 — production hostname is not a working deployment

Fresh HTTPS verification of the required URL fails certificate hostname
validation. The presented certificate is for
`*.msha-slice-7-eus2-0-ase.p.azurewebsites.net`, with no
`file-change-space-check.sociobot.in` SAN. A normal `curl` fails with:

```text
SSL: no alternative certificate subject name matches target host name
'file-change-space-check.sociobot.in'
```

Using `curl -k` only for diagnosis showed `/`, `/downloads/fcsc-linux-x86_64`,
`/privacy/`, and `/sw.js` returning `404 Site Not Found` (2,667 bytes).
`/terms/` and a hashed JS asset happen to return 200, so this is a partial,
not usable, deployment. The diagnostic download body does not equal the local
candidate binary: local SHA-256
`a8f6c85276f40ff3bedbdb275d2f99fd83666f4325b128b6ab61e82f9084ea8b`,
live response SHA-256
`1e0878f232e32cf44e87ba00bd6957c1ebdfc9bc7c1c0a1389f8c62e6ae3311a`.

### P1 — destination-inside-source check is bypassable through a symlink

The planner only compares lexical destination and canonical source paths. It
therefore accepts a destination that resolves into the source tree via a
symlink, instead of refusing the recursive/invalid plan.

Reproduction from the built release binary:

```sh
mkdir linked-source
printf x > linked-source/file.txt
ln -s linked-source source-alias
fcsc linked-source source-alias/inside --policy overwrite --json --no-space-check
```

It exits `3` with an unchecked manifest, including these actions, rather than
an input error:

```text
create-directory /tmp/.../source-alias/inside
copy             /tmp/.../source-alias/inside/file.txt
```

If a user follows this plan with a copy tool, the destination becomes part of
the scanned source tree; the estimate is no longer a valid preflight and can
lead to recursive copying. Resolve destination symlinks/existing ancestors
before checking whether it is contained by the canonical source.

### P1 — dark theme has serious color-contrast failures

On the production-built site at 390 px with `prefers-color-scheme: dark`, axe
4.13 reports four serious `color-contrast` violations. The `01`, `02`, `03`,
and `04` labels in the planning strip are `#c8f04a` on `#f7f1df`: 1.16:1,
below the required 4.5:1 for 13 px text. Light mode and the legal pages are
clean, but the required dark treatment is not accessible.

## Other defects

### P2 — the supplied accessibility audit cannot run from a clean checkout

`npm run audit:a11y` exits 1 after a successful scan because
`scripts/a11y-audit.mjs` writes
`.factory/evidence/axe.json` without creating `.factory/evidence/`:

```text
ENOENT: no such file or directory, open '.../.factory/evidence/axe.json'
```

After creating that directory in the disposable verification clone, the same
script reported 0 violations on `/`, `/privacy/`, and `/terms/`. The script
must create its own output directory so the documented verification command
works from a clean clone.

### P3 — documented invalid-input exit code disagrees with the CLI

README promises exit code `1` for invalid input. `fcsc SOURCE DEST --policy
nope` instead exits `2` (Clap usage error). Either document the distinct usage
error code or map it to the promised contract and test it.

## Successful local evidence

The following was run from a new detached clone at the candidate commit after
`npm ci` (Node 22.23.2, npm 10.9.8, Rust 1.98.0):

- `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings`
  passed.
- `cargo test --all-targets` passed: 7 library tests and 3 CLI integration
  tests. `cargo test --doc` passed its public API doctest.
- `npm test` passed: the Rust suite, 4 site tests, and the exact release build.
  A separate `npm run build` passed and produced `dist/`.
- `cargo package --allow-dirty` packaged and verified the crate. The resulting
  `.crate` was unpacked into a clean consumer directory, installed with
  `cargo install --path … --root …`, and its installed `fcsc` emitted a valid
  schema-1 JSON manifest for a fixture. `fcsc --version` reported `0.1.0`.
- Normal CLI fixtures exercised `overwrite`, `skip`, and `keep-both`, nested
  directories, file/directory type conflicts, empty/no-space-check plans,
  sparse `auto`/`preserve`/`expand`, JSON plus `--manifest`, invalid source,
  same source/destination, and lexical destination-inside-source recovery.
  Source and destination snapshots were unchanged by planning. On a 64 MiB
  `/dev/shm` destination, an 80 MiB sparse file was flagged insufficient by
  `auto` and `expand` (exit 2); `preserve` reported its intentional zero-byte
  lower allocation.
- Privacy inspection and browser request capture found no telemetry, cookies,
  remote fonts, or third-party runtime requests. The CLI dependency tree has
  no networking crate. `npm audit --omit=dev` found 0 vulnerabilities.
- Local production-preview browser checks at 1440 px and 390×844 had no page
  or console errors and no horizontal overflow. Keyboard Tab reaches the
  visible 3 px skip-link focus ring; simulator validation announces an invalid
  value and recovers to “Safe to start”; reduced motion resolves transitions
  and animation to `0.00001s`.
- PWA verification on the production build: `sw.js` registered, controlled the
  page, created `fcsc-shell-v1`, and served `/` successfully after the browser
  was set offline. The worker uses `skipWaiting` and `clients.claim`.
- Axe on the default/light production preview found 0 violations on `/`,
  `/privacy/`, and `/terms/`; dark-mode results are the P1 failure above.
- Lighthouse against the local production preview (mobile) scored Performance
  99, Accessibility 100, Best Practices 100, SEO 92; FCP 1.0 s, LCP 2.1 s,
  TBT 20 ms, CLS 0. Initial static transfer is 200,821 bytes (196.11 KiB):
  3,872-byte JS, 14,339-byte CSS, and 172,482-byte hero image, within the
  stated 200 KiB initial-JS and 50 KiB CSS budgets.
- The deploy config specifies immutable caching for `/assets/*`, one-day
  download caching, CSP, `Referrer-Policy: no-referrer`, `nosniff`, and a
  restrictive permissions policy. A live asset did expose those headers, but
  the root document is unavailable, so deployment response policy cannot pass
  as a whole.

## Required next verification

1. Fix the hostname binding/certificate and deploy a complete site root,
   download, legal pages, and service worker; rerun verified HTTPS checks
   without `-k` and compare the deployed binary hash to the candidate.
2. Canonicalize/resolve the destination's existing ancestor before rejecting
   source-contained destinations; add a regression test for the symlink case.
3. Correct dark-proof-strip contrast, make `audit:a11y` create its evidence
   directory, and rerun axe in both light and dark modes.
4. Reconcile and test the documented invalid-input exit-code contract.
