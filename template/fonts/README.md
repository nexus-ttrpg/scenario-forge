# Bundled fonts

Both families are licensed under the SIL Open Font License 1.1, which permits
bundling and redistribution. Licence texts are alongside the files.

* **Saira Condensed** — display: titles, section heads, stat block bars,
  running heads. Copyright The Saira Project Authors.
* **Barlow Semi Condensed** — body text. Copyright The Barlow Project Authors.

Typst folds width variants into a stretch axis, so these are addressed in
`template/scenario.typ` as the families `"Saira"` (stretch 75%) and
`"Barlow"` (stretch 87.5%), not by their full names.

They stand in for the commercial faces the printed booklets use (Axion, Good OT,
Trade Gothic Condensed). Swap in anything you like via `fonts` in
`template/scenario.typ`.
