// ===========================================================================
// scenario.typ — two-column layout for organised-play style RPG scenarios.
//
// No publisher trade dress lives in here: the colours are sampled for a
// family resemblance, the faces are OFL substitutes, there are no logos.
// Tune `palette`, `fonts` and `geometry` below to make the look your own.
//
// Font note: Typst folds width variants into a stretch axis, so the family
// "Saira Condensed" is addressed as "Saira" at stretch 75%, and
// "Barlow Semi Condensed" as "Barlow" at stretch 87.5%.
// ===========================================================================

#let palette = (
  blue:       rgb("#0F5CA7"),  // headings, rules, running head
  blue-dark:  rgb("#0A3C72"),
  blue-light: rgb("#34A1D4"),
  navy:       rgb("#163559"),  // sidebar title plate
  slate:      rgb("#5C728B"),  // sidebar body panel
  rust:       rgb("#B4502A"),  // read-aloud text
  ember:      rgb("#DF5325"),  // page badge, accents
  ember-dark: rgb("#BD3526"),
  ink:        rgb("#161616"),
  tint:       rgb("#E7EDF4"),  // scaling box, table stripes
  hairline:   rgb("#9AA8BA"),
  pale:       rgb("#BEDCF2"),
)

#let fonts = (
  display:         ("Saira", "Saira Condensed", "Oswald", "DejaVu Sans"),
  display-stretch: 75%,
  body:            ("Barlow", "Barlow Semi Condensed", "DejaVu Sans"),
  body-stretch:    87.5%,
)

#let geometry = (
  width:     603pt,   // 8.375 in trim
  height:    783pt,   // 10.875 in trim
  margin:    (top: 58pt, bottom: 52pt, inside: 38pt, outside: 38pt),
  gutter:    15pt,
  body-size: 8.8pt,
  body-lead: 4.2pt,
)

// --- typography helpers ----------------------------------------------------

#let dtext(
  size: 10pt, weight: "bold", fill: black, tracking: 0.03em, caps: true, body,
) = text(
  font: fonts.display, stretch: fonts.display-stretch,
  size: size, weight: weight, fill: fill, tracking: tracking,
  if caps { upper(body) } else { body },
)

// ---------------------------------------------------------------------------
// Headings
// ---------------------------------------------------------------------------

// `# ...`   major section head  (ADVENTURE SUMMARY, GETTING STARTED)
#let section(body) = block(above: 11pt, below: 3pt, breakable: false,
  dtext(size: 15pt, fill: palette.blue, tracking: 0.02em, body))

// `## ...`  encounter / area head  (A. THE BOARDING TUBE (CR 2))
#let subsection(body) = block(above: 9.5pt, below: 2.5pt, breakable: false,
  dtext(size: 12pt, fill: palette.blue, tracking: 0.02em, body))

// `### ...` subtier head  (SUBTIER 1–2 (CR 2))
#let subtier(body) = block(above: 8pt, below: 2pt, breakable: false,
  dtext(size: 10.5pt, weight: "semibold", fill: palette.blue, tracking: 0.05em, body))

// `#### ...` plain bold run-in head
#let minorhead(body) = block(above: 6pt, below: 1.5pt, breakable: false,
  text(weight: "bold", size: 8.8pt, fill: palette.ink, body))

// ---------------------------------------------------------------------------
// Read-aloud / boxed text  (`> ...`)
// ---------------------------------------------------------------------------

#let readaloud(body) = block(
  width: 100%, above: 7pt, below: 7pt, breakable: true,
  inset: (top: 4.5pt, bottom: 4.5pt),
  stroke: (top: 0.7pt + palette.blue, bottom: 0.7pt + palette.blue),
  {
    set par(first-line-indent: (amount: 9pt, all: false))
    text(fill: palette.rust, size: 8.4pt, body)
  },
)

// ---------------------------------------------------------------------------
// Sidebar  (```sidebar Title)
// ---------------------------------------------------------------------------

#let sidebar(title: none, float: true, body) = {
  let panel = block(
    width: 100%, above: 8pt, below: 9pt, breakable: false,
    radius: 4pt, fill: palette.slate, inset: 7pt,
    {
      if title != none {
        block(
          width: 100%, below: 7pt,
          radius: 3pt, fill: palette.navy, inset: (x: 8pt, y: 6pt),
          stroke: 1pt + palette.blue-light,
          align(center, dtext(size: 13pt, fill: white, tracking: 0.06em,
                              caps: false, title)),
        )
      }
      set text(fill: white, size: 8.2pt)
      set par(first-line-indent: 0pt)
      body
    },
  )
  if float { place(top, float: true, scope: "column", clearance: 9pt, panel) }
  else { panel }
}

// A black name-bar block, as used for the planet entry inside a sidebar
// (```planet DIM HARBOUR)
#let databar(title: none, body) = block(
  width: 100%, above: 7pt, below: 2pt, breakable: false,
  {
    if title != none {
      block(width: 100%, fill: black, inset: (x: 5pt, y: 2pt), below: 3pt,
            dtext(size: 12pt, fill: white, tracking: 0.03em, title))
    }
    set par(first-line-indent: 0pt, leading: 3.2pt, spacing: 3.2pt)
    body
  },
)

// ---------------------------------------------------------------------------
// Scaling box  (```scaling Scaling the Approach)
// ---------------------------------------------------------------------------

#let scalingbox(title: none, float: false, body) = {
  let panel = block(
    width: 100%, above: 8pt, below: 8pt, breakable: false,
    fill: palette.tint, radius: 3pt, inset: (x: 7pt, y: 6pt),
    {
      if title != none {
        block(below: 3.5pt, text(weight: "bold", style: "italic", size: 11.5pt,
                                 fill: palette.blue, title))
      }
      set text(size: 8.2pt)
      set par(first-line-indent: 0pt)
      body
    },
  )
  if float { place(top, float: true, scope: "column", clearance: 9pt, panel) }
  else { panel }
}

// ---------------------------------------------------------------------------
// Stat blocks  (```statblock)
// ---------------------------------------------------------------------------

// Black name bar: NAME on the left, CR on the right.
#let sb-title(name, cr: none) = block(
  width: 100%, fill: black, inset: (x: 4pt, y: 1.6pt), above: 8pt, below: 2.8pt,
  grid(
    columns: (1fr, auto), align: (left, right), column-gutter: 6pt,
    dtext(size: 12pt, fill: white, tracking: 0.02em, name),
    if cr != none { dtext(size: 12pt, fill: white, tracking: 0.02em, cr) },
  ),
)

// A ruled DEFENSE / OFFENSE / TACTICS / STATISTICS bar, optional right value.
#let sb-head(label, trailing: none) = block(
  width: 100%, above: 4.5pt, below: 2.2pt, breakable: false,
  inset: (bottom: 1.2pt), stroke: (bottom: 0.6pt + black),
  grid(
    columns: (auto, 1fr), align: (left + bottom, right + bottom),
    dtext(size: 9.5pt, tracking: 0.04em, label),
    if trailing != none { text(size: 8.3pt, trailing) },
  ),
)

// One line of a stat block, wrapped lines hang.
#let sb-line(body) = block(
  width: 100%, above: 1.4pt, below: 1.4pt,
  par(hanging-indent: 7pt, leading: 3.6pt, first-line-indent: 0pt, body),
)

#let sb-gap() = v(2.5pt, weak: true)

#let statblock(body) = block(
  width: 100%, above: 8pt, below: 8pt, breakable: true,
  { set text(size: 8.2pt); set par(first-line-indent: 0pt); body },
)

// ---------------------------------------------------------------------------
// Tables  (Markdown pipe tables)
// ---------------------------------------------------------------------------

#let datatable(columns: auto, header: (), rows: ()) = block(
  width: 100%, above: 8pt, below: 8pt, breakable: true,
  table(
    columns: columns,
    stroke: 0.5pt + palette.hairline,
    inset: (x: 4pt, y: 3pt),
    fill: (_, y) => if y == 0 { black }
                    else if calc.odd(y) { white }
                    else { palette.tint },
    ..header.map(c => dtext(size: 9pt, fill: white, tracking: 0.05em, c)),
    ..rows.flatten().map(c => {
        set par(first-line-indent: 0pt, leading: 3.2pt)
        text(size: 8pt, c)
      }),
  ),
)

// ---------------------------------------------------------------------------
// Map / art frame  (```map A. The Boarding Tube)
// ---------------------------------------------------------------------------

#let maparea(
  title: none, credit: none, scale: "1 square = 5 feet",
  src: none, height: 300pt, span: true,
) = {
  let frame = block(
    width: 100%, height: height, clip: true, radius: 2pt,
    fill: if src == none { palette.tint } else { white },
    stroke: 0.8pt + palette.navy,
    {
      if src != none {
        image(src, width: 100%, height: 100%, fit: "cover")
      } else {
        align(center + horizon,
              dtext(size: 13pt, fill: palette.hairline, tracking: 0.1em,
                    weight: "semibold", [map artwork]))
      }
      if title != none {
        place(top + left, dx: 6pt, dy: 6pt, block(
          fill: black.transparentize(12%), inset: (x: 6pt, y: 2.5pt), radius: 2pt,
          dtext(size: 12pt, fill: white, tracking: 0.04em, title),
        ))
      }
      place(bottom + left, block(
        width: 100%, fill: black.transparentize(22%), inset: (x: 6pt, y: 1.5pt),
        grid(
          columns: (1fr, auto), align: (left, right), column-gutter: 8pt,
          dtext(size: 8pt, fill: white, tracking: 0.06em, weight: "semibold",
                if credit == none { "" } else { credit }),
          dtext(size: 8pt, fill: white, tracking: 0.06em, weight: "semibold",
                if scale == none { "" } else { scale }),
        ),
      ))
    },
  )
  if span { place(top, float: true, scope: "parent", clearance: 11pt, frame) }
  else { frame }
}

// ---------------------------------------------------------------------------
// Page furniture: running head bar, accents, page badge
// ---------------------------------------------------------------------------

#let page-furniture(series: "", title: "", offset: 0, geo: geometry) = context {
  let n = counter(page).get().first() + offset
  let odd = calc.odd(n)
  let w = geo.width
  let head = if odd { title } else { series }

  // running head bar
  place(top + left, rect(
    width: w, height: 30pt, stroke: none,
    fill: gradient.linear(
      palette.blue-dark, palette.blue, palette.blue-light,
      palette.blue, palette.blue-dark, angle: 0deg),
  ))
  place(top + left, dy: 30pt, rect(
    width: w, height: 4pt, stroke: none,
    fill: gradient.linear(palette.ember-dark, palette.ember, palette.ember-dark,
                          angle: 0deg),
  ))
  place(top + center, dy: 6pt,
        dtext(size: 17pt, fill: white, tracking: 0.13em, head))

  // dashed accent under the bar, on the outer side
  let dashes = stack(dir: ltr, spacing: 3pt,
    ..range(8).map(_ => rect(width: 7pt, height: 7pt,
                             fill: palette.ember, stroke: none)))
  if odd { place(top + left, dx: 26pt, dy: 42pt, dashes) }
  else   { place(top + right, dx: -26pt, dy: 42pt, dashes) }

  // outer-edge chevron tab
  let tab = block(
    width: 13pt, height: 70pt, fill: palette.ember, inset: 3pt,
    radius: (left: 0pt, right: 2pt),
    stack(dir: ttb, spacing: 4pt,
      ..range(5).map(_ => rect(width: 100%, height: 7pt,
                               fill: white.transparentize(55%), stroke: none))),
  )
  if odd { place(top + right, dy: 46pt, tab) }
  else   { place(top + left,  dy: 46pt, tab) }

  // faint circuitry at the foot
  for i in range(8) {
    place(bottom + left, dx: 28pt + i * 72pt, dy: -6pt,
          circle(radius: 12pt,
                 stroke: 0.7pt + palette.blue-light.transparentize(82%)))
  }
  place(bottom + left, dy: -31pt,
        line(length: w, stroke: 0.7pt + palette.blue-light.transparentize(80%)))

  // page number badge
  let badge = block(
    width: 25pt, height: 25pt, radius: 5pt,
    fill: gradient.linear(palette.ember, palette.ember-dark, angle: 60deg),
    stroke: 1.2pt + white,
    align(center + horizon,
          dtext(size: 14pt, fill: white, caps: false, str(n))),
  )
  if odd { place(bottom + right, dx: -17pt, dy: -15pt, badge) }
  else   { place(bottom + left,  dx: 17pt,  dy: -15pt, badge) }
}

// ---------------------------------------------------------------------------
// Cover page
// ---------------------------------------------------------------------------

#let cover-page(
  series: "", number: none, tier: none, banner: none,
  title: "", author: "", art: none,
  credits: (), contents: (), resources: none, tags: (),
  how-to-play: none, legal: none, geo: geometry,
) = {
  let banner-h = 200pt
  page(
    width: geo.width, height: geo.height,
    margin: (top: banner-h + 26pt, bottom: 30pt, x: 34pt),
    columns: 1,
    header: none, footer: none,
    background: {
      // banner
      if art != none {
        place(top + left, block(width: geo.width, height: banner-h, clip: true,
              image(art, width: 100%, height: 100%, fit: "cover")))
        place(top + left, rect(width: geo.width, height: banner-h,
              stroke: none, fill: palette.blue-dark.transparentize(35%)))
      } else {
        place(top + left, rect(
          width: geo.width, height: banner-h, stroke: none,
          fill: gradient.linear(
            palette.blue-dark, palette.blue, palette.blue-light,
            palette.blue, palette.blue-dark, angle: 0deg),
        ))
      }
      // angled ember band, then a dark lip under it
      place(top + left, dy: banner-h, polygon(fill: palette.ember,
            (0pt, 0pt), (geo.width, 0pt), (geo.width, 15pt), (0pt, 6pt)))
      place(top + left, dy: banner-h + 6pt, polygon(fill: palette.blue-dark,
            (0pt, 0pt), (geo.width, 9pt), (geo.width, 14pt), (0pt, 7pt)))

      // banner text
      place(top + center, dy: 26pt,
            dtext(size: 12pt, fill: white, tracking: 0.3em,
                  if banner == none { "" } else { banner }))
      place(top + center, dy: 46pt, dtext(
        size: 9.5pt, fill: palette.pale, tracking: 0.22em,
        series
          + (if number != none { " " + number } else { "" })
          + (if tier != none { ", " + tier } else { "" })))
      place(top + center, dy: 86pt,
            dtext(size: 44pt, fill: white, tracking: 0.01em, title))
      place(top + center, dy: 146pt,
            dtext(size: 17pt, fill: palette.pale, tracking: 0.08em,
                  "by " + author))
    },
  )[
    #set text(font: fonts.body, stretch: fonts.body-stretch,
              size: 7.5pt, fill: palette.ink)
    #set par(justify: false, leading: 3pt, spacing: 3pt, first-line-indent: 0pt)
    #let minihead(t) = block(above: 9pt, below: 2.5pt,
      dtext(size: 12pt, fill: palette.blue, tracking: 0.04em, t))

    #grid(
      columns: (1.1fr, 1fr), gutter: 22pt,
      {
        set par(hanging-indent: 7pt)
        for c in credits { c; linebreak() }
      },
      {
        if contents.len() > 0 {
          minihead("Table of Contents")
          for row in contents {
            grid(columns: (1fr, auto), row.at(0),
                 text(weight: "bold", row.at(1)))
            v(1.5pt)
          }
        }
        if resources != none { minihead("GM Resources"); resources }
        if tags.len() > 0 {
          minihead("Scenario Tags")
          for t in tags {
            block(below: 2.5pt, box(fill: palette.tint, radius: 2pt,
                                    inset: (x: 4pt, y: 2pt), t))
          }
        }
        if how-to-play != none { minihead("How to Play"); how-to-play }
      },
    )

    #if legal != none {
      place(bottom + left, block(width: 100%, inset: (top: 5pt),
        stroke: (top: 0.5pt + palette.hairline),
        text(size: 5.6pt, fill: rgb("#555555"), legal)))
    }
  ]
}

// ---------------------------------------------------------------------------
// Document entry point
// ---------------------------------------------------------------------------

#let scenario(
  title: "Untitled Scenario",
  author: "",
  series: "Society Scenario",
  number: none,
  tier: none,
  banner: none,
  art: none,
  credits: (),
  contents: (),
  resources: none,
  tags: (),
  how-to-play: none,
  legal: none,
  cover: true,
  start-page: 3,
  geo: geometry,
  body,
) = {
  set document(title: title, author: author)
  set text(font: fonts.body, stretch: fonts.body-stretch,
           size: geo.body-size, fill: palette.ink, lang: "en",
           hyphenate: true)
  set par(justify: true, leading: geo.body-lead, spacing: geo.body-lead,
          first-line-indent: (amount: 9pt, all: false))
  set list(indent: 4pt, body-indent: 4pt, marker: text(fill: palette.ember)[•])
  set enum(indent: 4pt, body-indent: 4pt)
  show link: it => text(fill: palette.blue, it)
  show emph: it => text(style: "italic", it.body)

  if cover {
    cover-page(
      series: series, number: number, tier: tier, banner: banner,
      title: title, author: author, art: art, credits: credits,
      contents: contents, resources: resources, tags: tags,
      how-to-play: how-to-play, legal: legal, geo: geo,
    )
  }

  set page(
    width: geo.width, height: geo.height,
    margin: geo.margin,
    columns: 2,
    header: none, footer: none,
    background: page-furniture(
      series: series, title: title, geo: geo,
      // physical page of the first body page -> printed `start-page`
      offset: start-page - (if cover { 2 } else { 1 }),
    ),
  )
  set columns(gutter: geo.gutter)

  body
}
