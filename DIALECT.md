# The scenario Markdown dialect

Ordinary Markdown for the prose, plus a handful of fenced blocks for the things
an adventure needs that Markdown has no idea about: stat blocks, sidebars,
scaling boxes, maps.

Everything here is handled by `md2typ`, which emits Typst that calls the
functions in `template/scenario.typ`. If you ever need something the dialect
doesn't cover, drop into Typst with a ```` ```typst ```` block.

All examples below come from the skeleton in `scenarios/_template.md`, so you
can build that file and see each of them rendered.

---

## Front matter

A YAML-ish block at the top of the file, between `---` lines. Three shapes are
supported — scalars, lists, and folded blocks — and nothing else:

```yaml
---
title: The Salvage at Dim Harbour   # scalar
author: Your Name Here
series: Home Campaign Scenario
number: "#HB-01"                    # quote anything starting with # or -
tier: Tier 1–2
banner: Season of the Long Dark     # the small line above the title
art: art/cover.jpg                  # optional cover image, else a blue gradient
cover: true                         # set false to skip the cover page entirely
start-page: 3                       # printed number of the first body page

credits:                            # list
  - "**Author** • Your Name Here"
  - "**Cartographer** • Your Name Here"

contents:                           # list of "label | page"
  - The Salvage at Dim Harbour | 3
  - Map | 8

resources: >                        # folded block; blank lines make paragraphs
  Makes use of the core rules and
  the first bestiary.

how-to-play: >
  Designed for 1st- through 2nd-level characters.

legal: >
  Small print along the bottom of the cover.
---
```

`|` instead of `>` keeps the line breaks instead of folding them.

Unknown keys are ignored, so you can keep notes to yourself up there.

---

## Prose

| Markdown | Renders as |
|---|---|
| `# Adventure Summary` | major section head, blue display caps |
| `## A. The Boarding Tube (CR 2)` | encounter head |
| `### Subtier 1–2 (CR 2)` | subtier head |
| `#### What They Can Salvage` | plain bold run-in head |
| `**Treasure:** The hounds have dragged…` | the usual bold run-in paragraph |
| `*text*`, `_text_`, `**text**`, `` `text` `` | italic, italic, bold, mono |
| `- item` | bullet list |
| `line ending in a backslash` | hard line break |
| `[label](https://…)` | link |

Paragraphs are separated by blank lines and justified, with the second and
later paragraphs of a run indented, as in the printed books.

### Read-aloud text

A blockquote becomes boxed read-aloud text — rust-coloured, between two blue
rules:

```markdown
> The boarding tube ends at a hatch that someone has already cut open.
> Beyond it the drydock turns slowly against the stars, every window dark.
>
> Something small skitters away across the deck plating and is gone.
```

### Tables

Pipe tables. The first row is the header; the last column takes the slack.

```markdown
| Zone | Leader Skill | Effects |
|---|---|---|
| Cargo Deck | Athletics | **Altered Movement:** drifting crates (–1 to Athletics) |
| Coolant Fog | Survival | **Altered Attacks:** vapour (concealment) |
```

---

## Fenced blocks

The info string is `name`, then any number of `key=value` attributes, and
anything left over is the title:

~~~markdown
```sidebar Where Are We?
```map title="A. The Boarding Tube" credit="map by you" height=320pt
~~~

Use `title="…"` explicitly if your title contains an `=`.

### `sidebar` — the slate panel with a navy title plate

Floats to the top of its column by default; `float=false` keeps it inline.
The body is ordinary Markdown, so it can contain anything, including a
`planet` block. **Nested fences need a longer outer fence** — four backticks
outside, three inside:

~~~markdown
````sidebar Where Are We?
Dim Harbour was a repair yard until the company that ran it stopped
answering, and nobody has filed a salvage claim since.

```planet Dim Harbour
**Derelict orbital drydock**\
**Diameter:** ×0; **Mass:** ×0; **Gravity:** 1/4
```
````
~~~

### `planet` — a black name bar over tight lines

Useful anywhere you want a labelled data strip, not just for planets.

### `scaling` — the tinted four-player adjustment box

Inline by default (`float=true` to send it to the top of the column):

~~~markdown
```scaling Scaling the Approach
With only four PCs, lower every skill check in this section by 2 and let the
first success count twice.
```
~~~

### `map` — a framed map or art plate

No body; everything is an attribute. Spans both columns and floats to the top
of a page unless you pass `span=false`.

~~~markdown
```map title="B. The Flooded Hold" src=art/hold.png credit="map by you" height=300pt scale="1 square = 5 feet"
```
~~~

Without `src` you get a labelled placeholder frame, which is what you want
while drafting.

### `typst` — raw escape hatch

The body is passed through untouched. Everything in `template/scenario.typ` is
in scope, so you can call `#sidebar`, `#datatable`, `#palette.blue` and so on
directly.

~~~markdown
```typst
#place(top, float: true, scope: "parent", image("art/splash.jpg", width: 100%))
```
~~~

---

## `statblock`

The body is close to what you would type from the printed page.

~~~markdown
```statblock
SCRAP HOUND (2) | CR 1/2
N Small animal
Init +3; Senses low-light vision; Perception +4

DEFENSE | HP 13 each
EAC 10; KAC 12
Fort +4; Ref +4; Will +0

OFFENSE
Speed 40 ft.
Melee bite +5 (1d4+2 P)

TACTICS
During Combat The hounds circle and harry whoever is slowest, breaking off
  to drag loose gear away rather than press a losing fight.
Morale The pack scatters when half its number are down.

STATISTICS
Str +2; Dex +3; Con +1; Int –4; Wis +1; Cha –2
Skills Acrobatics +8, Survival +4
Gear none

SPECIAL ABILITIES
Scent Salvage (Ex) A scrap hound can smell worked metal through a bulkhead,
  and always knows the direction of the nearest powered device within 60 feet.
```
~~~

The rules:

* **First line** is the black name bar. Text after `|` is set flush right —
  that is where the CR goes.
* **An ALL-CAPS line** is a ruled section bar. Text after `|` sits at the right
  end of the rule — that is where `HP` goes. Any all-caps line works, so
  `ECOLOGY` or `DESCRIPTION` need no special support.
* **Every other line** is a stat line with a hanging indent.
* **A line indented two or more spaces** continues the line above it, so long
  tactics entries can wrap in the source.
* **A blank line** inserts a small gap.

### Automatic bolding

Known keywords at the start of a line, and after each `; `, are bolded for
you — `Init`, `Senses`, `EAC`, `KAC`, `Fort`, `Ref`, `Will`, `DR`, `SR`,
`Speed`, `Melee`, `Ranged`, `Str` … `Cha`, `Skills`, `Feats`, `Gear`,
`Languages`, `During Combat`, `Morale`, `Type`, `Trigger`, `Effect`, and the
rest of the usual vocabulary (the full list is `KEYWORDS` in
`md2typ/src/statblock.rs` — add to it freely).

Inside a section whose name contains `ABILITIES`, the name up to and including
the `(Ex)` / `(Su)` / `(Sp)` tag is bolded instead.

Write `**…**` yourself and the automatic pass leaves that segment alone. Turn
it off for one block with ```` ```statblock autobold=off ````.
