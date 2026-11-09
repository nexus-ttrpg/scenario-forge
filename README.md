# scenario-forge

Write a Starfinder scenario in Markdown, get back a PDF that looks like the
organised-play booklets: two columns, blue display heads, slate sidebars,
rust-coloured read-aloud text, black-barred stat blocks, page badges.

```
scenarios/*.md  ──md2typ──>  build/*.typ  ──typst──>  build/*.pdf
```

Three pieces, each replaceable on its own:

| | |
|---|---|
| `scenarios/` | your adventures, in [the Markdown dialect](DIALECT.md) |
| `md2typ/` | a small dependency-free Rust converter, Markdown → Typst |
| `template/scenario.typ` | the layout: page furniture, colours, every block type |

## Build

```sh
./build.sh                              # every scenarios/*.md
./build.sh scenarios/my-adventure.md    # just one
./build.sh --watch scenarios/my-adventure.md
```

You need Rust (for `md2typ`, built automatically on first run) and Typst. Either
the CLI from <https://github.com/typst/typst/releases> or `pip install typst`
will do — `build.sh` finds whichever you have.

## Start a new scenario

```sh
cp scenarios/_template.md scenarios/my-adventure.md
./build.sh --watch scenarios/my-adventure.md
```

`_template.md` is a working skeleton with one of everything — sidebar, planet
bar, scaling box, read-aloud, map frame, stat block, chase table, boons. Files
beginning with `_` are skipped by a bare `./build.sh`.

Read [DIALECT.md](DIALECT.md) for the full syntax. The short version:

* `#`/`##`/`###` are section, encounter, and subtier heads.
* `> …` is read-aloud text.
* Pipe tables are tables.
* Fenced blocks named `sidebar`, `planet`, `scaling`, `map`, `statblock`, and
  `typst` cover everything Markdown can't say.

Stat blocks are typed almost exactly as they are printed — the converter bolds
`Init`, `EAC`, `Melee`, `Str`, `During Combat` and the rest of the vocabulary
for you, and an ALL-CAPS line becomes a ruled `DEFENSE` / `OFFENSE` bar.

## Making it yours

* **Colours** — `palette` at the top of `template/scenario.typ`.
* **Fonts** — `fonts` in the same place. Shipped: Saira Condensed for display
  and Barlow Semi Condensed for body, both OFL, in `template/fonts/`. Note that
  Typst folds width variants into a stretch axis, so the families are addressed
  as `"Saira"` at 75% and `"Barlow"` at 87.5%.
* **Page size and margins** — `geometry`, also at the top. Currently 603 × 783 pt,
  the trim the printed booklets use.
* **A new kind of block** — add a function to `template/scenario.typ`, then a
  branch to `fenced()` in `md2typ/src/block.rs`. Until then, a ```` ```typst ````
  block will call it directly.

## Artwork

`map` blocks render a labelled placeholder frame until you give them
`src=art/whatever.png`. The cover takes `art:` in the front matter and falls
back to a blue gradient. Put image files anywhere under this directory; paths
in a scenario are resolved from the project root.

## Published material stays out of the repo

Everything committed here is either original or openly licensed. The template
carries no publisher trade dress — no logos, no licensed typefaces, colours
sampled only closely enough for a family resemblance — and every example in
the docs and the skeleton is invented for this project.

Converting a scenario you bought is a fine way to test the dialect, but the
result is someone else's text. `.gitignore` keeps such conversions out of the
repo two ways:

* `scenarios/private/` is ignored wholesale. Put conversions of published
  adventures there; `build.sh` still finds and builds them.
* `*.pdf` is ignored, so a source booklet dropped into the tree never gets
  staged by accident. Use `git add -f` for a PDF you actually mean to commit.

Starfinder is a trademark of Paizo Inc. Anything you make with this is an
unofficial fan work; check the Paizo Community Use Policy before it leaves
your table.
