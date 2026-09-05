# File Change Space Check

`fcsc` estimates free space and conflicts before a large local copy starts. It
is for people moving archives, project trees, media folders, or extracted data.

The CLI scans filesystem metadata and applies an overwrite, skip, or keep-both
policy. It prints a human result or a path-sorted JSON action manifest. It
never performs the planned copy.

## Try the bundled sample

Run one command without choosing local files:

```sh
fcsc --demo
```

The command creates a media-archive sample in a new temporary directory. It
prints the sandbox and manifest paths. Run it again to reset the sample. The
demo also accepts `--policy`, `--sparse`, and `--json`.

The browser version is at
[file-change-space-check.sociobot.in/demo/](https://file-change-space-check.sociobot.in/demo/).
It uses the same sample shape and stores no changes.

## Install

Download the tested Linux x86-64 binary from the
[product site](https://file-change-space-check.sociobot.in/#install). Or install
from a source checkout with Rust 1.85 or newer:

```sh
git clone https://github.com/B-Divyesh/sf-file-change-space-check.git
cd sf-file-change-space-check
cargo install --path . --locked
fcsc --version
```

The crate is ready to publish but is not on crates.io. Do not use
`cargo install file-change-space-check` yet.

## Plan a copy

Choose what happens when a relative path already exists:

```sh
# Human result. Exit 0 means the safer upper bound fits.
fcsc ./camera-roll /mnt/archive --policy overwrite

# JSON for another tool to inspect.
fcsc ./camera-roll /mnt/archive --policy skip --json > plan.json

# Keep both names and save the same manifest.
fcsc ./camera-roll /mnt/archive --policy keep-both --manifest plan.json
```

The source directory contents map into `DESTINATION`. A single source file maps
to `DESTINATION/<source filename>`. With `--no-space-check`, unchanged inputs
produce byte-identical JSON. A checked plan includes the current free-space
snapshot, which can change between runs.

```text
Usage: fcsc [OPTIONS] [SOURCE] [DESTINATION]

Arguments:
  [SOURCE]       File or directory to scan
  [DESTINATION]  Existing destination directory, or a path below one

Options:
      --demo                 Run the bundled sample in a temporary directory
      --policy <POLICY>      overwrite, skip, or keep-both
      --sparse <SPARSE>      auto, preserve, or expand [default: auto]
      --json                 Print the complete JSON manifest
      --manifest <FILE>      Also save the JSON manifest to this file
      --no-space-check       Skip the destination free-space check
  -h, --help                 Print help
  -V, --version              Print version
```

Exit codes are `0` safe, `1` invalid, `2` insufficient, and `3` unchecked. A
manifest is still available for insufficient and unchecked plans.

## Space and accuracy

The default sparse mode reports lower and upper allocation bounds. The upper
bound assumes sparse holes expand and controls the verdict.

Overwrite headroom budgets the new file before reclaiming the old path. This
is stricter than comparing only the final net change.

Dense fixture estimates are tested against real allocated-byte changes for all
three policies. Each result must stay within 2% of the measured change.

## Privacy and limits

The CLI reads metadata without requiring regular file contents. It changes no
source or destination files and opens no network socket.

The manifest is data, not an executable script. It cannot prove later write
permissions or exact copy-tool allocation.

The site uses no analytics, cookies, remote fonts, or third-party runtime code.
Its service worker caches public pages and the fixed demo for offline use.

## Develop and verify

Requirements are Rust 1.85+, Node.js 20.19+, npm 10+, and Chromium for browser
checks. Playwright 1.58.2 is pinned in `package-lock.json`. The verification
suite checks the Rust 1.85 boundary, so install it first when using rustup:

```sh
rustup toolchain install 1.85.0 --profile minimal
```

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

`npm test` builds the release, checks TypeScript, runs Rust tests, exercises
browser routes, and runs every claim sandbox. `npm run build` creates the CLI
at `dist/downloads/fcsc` and the static site at `dist/site/`.

Run one declared claim with `npm run claim -- @claim:<id>`. All public claims
and their commands are in [`.factory/claims.json`](.factory/claims.json).

Deploy the built site without changing infrastructure:

```sh
/opt/fleet/lib/deploy-static.sh file-change-space-check /work/repo/dist/site
```

## License

MIT. See [LICENSE](LICENSE).
