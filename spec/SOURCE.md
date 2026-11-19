# Vendored specification

Source: https://github.com/Intralogistics-2X-LIF/Layout-Interchange-Format
Branch: `dev/V2`
Commit: `d6898083356c23f735ff1aaa6ba82b5927bab79f` (2026-10-27)
Version: LIF 2.0.0, September 2026 (draft in public comment; re-check when published)

Acknowledgement (required by the publisher): LIF - Layout Interchange Format,
Version 2.0.0, © VDMA e. V., Lyoner Strasse 18, 60528 Frankfurt am Main.
"Reprinting and any other form of reproduction is permitted only if the source is
acknowledged." The source is the repository above.

Files:
- `LIF.schema` — copied from `json_schemas/LIF.schema` (not an official part of the standard, LIF.md §9.2)
- `LIF.md` — specification text (leading if it differs from the schema)
- `examples/*.md` — the 32 official examples
- `LICENSE-VDMA` — upstream licence (MIT, © VDMA e. V.)

Do not edit these files. To update: copy again from a newer commit and note it here.

Known upstream inconsistency (in the pinned commit): `LIF.md` §8.3.11 defines `loadRestriction.restrictionType`
(enum `UNLOADED`/`LOADED`/`BOTH`, upstream commit 33f4425), but `LIF.schema` and all official examples still use
the booleans `unloaded` and `loaded`. This crate follows the schema and examples (decision Q251).

Test fixtures: `tests/examples/NN-*.json` are the JSON blocks of `examples/01-…28-*.md`, unmodified. The seven
blocks in `examples/28.1–28.4` are VDA 5050 order fragments, not LIF files, and are not used.
