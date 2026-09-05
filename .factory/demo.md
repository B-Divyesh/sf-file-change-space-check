# Demo sandbox

## CLI entry point

Run:

```sh
fcsc --demo
```

The command creates a fresh directory named `fcsc-demo-<pid>-<nonce>` under the
operating system's temporary directory. It creates only sample files there,
runs the real planner, and writes `demo-manifest.json` inside that directory.
It prints both paths. It does not read or write the caller's file trees.

Use `--policy overwrite`, `--policy skip`, or `--policy keep-both` to change the
sample conflict result. `--sparse` changes its allocation assumption. Add
`--json` for machine-readable output. Run the command again to reset into a new
sandbox. Remove a printed sandbox when it is no longer needed.

The sample shape is also documented under `examples/demo/`. It contains a
media archive, a sparse disk image, and an existing `photos.raw` conflict.

## Browser entry point

Open <https://file-change-space-check.sociobot.in/demo/> or select **Try it with
sample data** on the first screen.

The page displays **Demo — sample data, nothing is saved** while demo mode is
active. **Reset demo** restores overwrite policy and 16 MiB of free space.
**Start for real** opens the installation section.

The browser sample stays in memory. It creates no localStorage, sessionStorage,
IndexedDB, OPFS, cookie, or server record. It never asks for a local file. The
browser values use the same sample names and allocation shape as the CLI demo.
