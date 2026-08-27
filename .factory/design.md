# Visual thesis: allocation ledger

## Direction and rationale

The site uses a **neo-brutalist utility** language derived from disk labels,
terminal printouts, and graph-paper capacity calculations. Thick black rules
make boundaries unambiguous; exposed labels and offsets make the interface feel
like an inspectable tool, not a generic software landing page. Decoration earns
its place by explaining the product's central idea: source blocks, conflicts,
and available space are reconciled before action.

## Palette

Light mode is the primary, explicitly painted treatment.

| Token | Value | Use |
| --- | --- | --- |
| Paper | `#F5F0E3` | warm document background |
| Ink | `#171713` | text, rules, hard shadows |
| Sheet | `#FFFDF6` | working surfaces |
| Cobalt | `#165DFF` | actions and source-file blocks |
| Safety lime | `#C8F04A` | safe result and emphasis |
| Amber | `#F4B942` | uncertainty and conflict |
| Signal red | `#C8322B` | errors and insufficient space |
| Muted ink | `#5E5C52` | secondary copy (7.0:1 on paper) |

Dark treatment follows the user's preference with `#141612` canvas,
`#20231C` surfaces, `#F7F1DF` text, cobalt lightened to `#76A4FF`, and lime
held at `#C8F04A`. Status always includes an icon and words, never color alone.

## Typography and spacing

The display face is the self-hosted/system monospace stack `ui-monospace,
SFMono-Regular, Menlo, Consolas, monospace`, evoking manifests and byte counts.
Body copy uses the native sans stack `Inter, ui-sans-serif, system-ui` without a
network font request. The scale is 14 / 16 / 20 / 28 / clamp(42–76) px, with
tabular numerals for all capacity values. Body text is at least 16 px.

Spacing follows an 8 px base with 4 px half-steps. Rules are 2–3 px and hard
shadows use a consistent 6 px offset. Content is capped at 1180 px; long prose
stays between 55 and 72 characters. At 390 px, the proof ledger stacks, the
secondary decorative caption disappears, and actions become full-width.

## Interaction grammar

Controls depress into their hard shadow by 3 px, like a physical utility key.
Focused controls receive a 3 px cobalt outline plus 3 px clearance. The demo is
a compact ledger: changing policy immediately recomputes actions, headroom, and
status in an `aria-live` result. Touch targets are at least 44 px.

## Motion policy

One 220 ms transform/opacity entrance connects the hero copy to the allocation
ledger; control feedback is 120 ms. Nothing loops. Under
`prefers-reduced-motion: reduce`, transforms and smooth scrolling are removed
and state changes are instantaneous.

## Asset plan and provenance

The hero asset is an original raster illustration showing cobalt file blocks
being measured against a lime disk-capacity ruler, with one amber conflict block
held aside. It contains no text so it remains useful at narrow sizes. It will be
generated with the factory image deployment, converted locally to WebP, and
kept below 300 KB. The final prompt, deployment metadata, dimensions, and file
size are recorded here after generation. CSS icons are simple project-original
geometric marks; no third-party icon library is used.
