# File Change Space Check

`fcsc` is a read-only preflight planner for large local copies. It scans source
and destination metadata, applies an explicit conflict policy, checks the
destination's free space, and emits a deterministic action manifest before any
copy begins.

It is for people moving archives, project trees, or media collections who need
to know whether a copy will fit—and what conflicts will do to that answer.
`fcsc` never copies, deletes, renames, or changes permissions.

## Install

Download a release binary from
[file-change-space-check.sociobot.in](https://file-change-space-check.sociobot.in),
or build from source:

```sh
cargo install --path .
```

Version 0.1.0 supports Linux and macOS. Windows support is planned.

## Usage

Choose what should happen when a source-relative path already exists at the
destination:

```sh
# Human-readable preflight. Exit 0 means the upper-bound requirement fits.
fcsc ./camera-roll /mnt/archive --policy overwrite

# Machine-readable manifest, suitable for review or another tool.
fcsc ./camera-roll /mnt/archive --policy skip --json > plan.json

# Preserve both names deterministically: photo.jpg -> photo (copy 1).jpg
fcsc ./camera-roll /mnt/archive --policy keep-both --manifest plan.json
```

Options:

```text
Usage: fcsc [OPTIONS] --policy <POLICY> <SOURCE> <DESTINATION>

Arguments:
  <SOURCE>       File or directory to scan
  <DESTINATION>  Existing destination directory, or a path below one

Options:
      --policy <POLICY>      overwrite, skip, or keep-both
      --sparse <SPARSE>      auto, preserve, or expand [default: auto]
      --json                 Print the complete JSON manifest to stdout
      --manifest <FILE>      Also write the JSON manifest to this file
      --no-space-check       Plan even when destination free space is unavailable
  -h, --help                 Print help
  -V, --version              Print version
```

The source directory's *contents* are mapped into `DESTINATION`; a single source
file maps to `DESTINATION/<source filename>`. Actions are sorted by relative
source path, and keep-both suffixes are stable for an unchanged filesystem.

### Sparse files and headroom

Default `--sparse auto` reports a lower and upper allocation bound. The upper
bound assumes sparse holes expand and is used for the pass/fail decision. Use
`--sparse preserve` only when the eventual copy tool is known to preserve holes,
or `--sparse expand` to report the conservative value as exact.

For overwrite plans, required headroom assumes the new file is fully written
before the old path is reclaimed. This is intentionally stricter than comparing
only the final net change.

Exit codes are stable: `0` safe, `2` insufficient space, `3` space check skipped
or unavailable, `1` invalid input or scan failure. A manifest is still emitted
for exit codes 2 and 3.

## Develop and verify

Requirements: Rust 1.85+, Node.js 20+, and npm 10+.

```sh
npm install
npm test
npm run build
```

`npm test` runs Rust unit/integration tests, site tests, and the production site
build. `npm run build` creates the CLI at `dist/downloads/fcsc` and the deployable
site at `dist/site/index.html`. To make a registry-ready Rust package without
publishing, run `cargo package`.

The site uses no analytics, cookies, remote fonts, or third-party runtime code.

## License

MIT — see [LICENSE](LICENSE).
