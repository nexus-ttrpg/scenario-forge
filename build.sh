#!/usr/bin/env bash
# Build scenario PDFs:  Markdown -> (md2typ) -> Typst -> PDF
#
#   ./build.sh                          build every scenarios/*.md
#   ./build.sh scenarios/my-game.md     build one
#   ./build.sh --watch scenarios/x.md   rebuild whenever the source changes
#
# Output lands in build/ as <name>.typ and <name>.pdf.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$ROOT/build"
FONTS="$ROOT/template/fonts"
MD2TYP="$ROOT/md2typ/target/release/md2typ"

WATCH=0
SOURCES=()
for arg in "$@"; do
  case "$arg" in
    -w|--watch) WATCH=1 ;;
    -h|--help)  sed -n '2,10p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *)          SOURCES+=("$arg") ;;
  esac
done

if [ ${#SOURCES[@]} -eq 0 ]; then
  while IFS= read -r f; do SOURCES+=("$f"); done \
    < <(find "$ROOT/scenarios" -maxdepth 2 -name '*.md' ! -name '_*' | sort)
fi
if [ ${#SOURCES[@]} -eq 0 ]; then
  echo "build.sh: no scenarios found in $ROOT/scenarios" >&2
  exit 1
fi

# --- the converter -----------------------------------------------------------
if [ ! -x "$MD2TYP" ] || [ "$ROOT/md2typ/src/main.rs" -nt "$MD2TYP" ]; then
  echo "==> cargo build --release (md2typ)"
  (cd "$ROOT/md2typ" && cargo build --release --quiet)
fi

# --- the typesetter ----------------------------------------------------------
# Prefer the real Typst CLI; fall back to the `typst` PyPI package, which ships
# the same engine, so this works without a system install.
typeset_pdf() {
  local src="$1" pdf="$2"
  if command -v typst >/dev/null 2>&1; then
    typst compile --root "$ROOT" --font-path "$FONTS" "$src" "$pdf"
  elif python3 -c 'import typst' >/dev/null 2>&1; then
    python3 - "$src" "$pdf" "$ROOT" "$FONTS" <<'PY'
import sys, typst
src, pdf, root, fonts = sys.argv[1:5]
typst.compile(src, output=pdf, root=root, font_paths=[fonts])
PY
  else
    cat >&2 <<'MSG'
build.sh: no Typst found. Install one of:
  * the CLI      https://github.com/typst/typst/releases  (or: brew install typst)
  * the package  pip install typst
MSG
    exit 1
  fi
}

build_one() {
  local md="$1"
  local name; name="$(basename "${md%.md}")"
  mkdir -p "$OUT"
  "$MD2TYP" "$md" -o "$OUT/$name.typ"
  typeset_pdf "$OUT/$name.typ" "$OUT/$name.pdf"
  echo "==> $OUT/$name.pdf"
}

if [ "$WATCH" -eq 1 ]; then
  echo "==> watching ${SOURCES[*]} and template/ (ctrl-c to stop)"
  last=""
  while true; do
    # content checksum rather than mtimes: POSIX, and immune to touch(1)
    now="$(find "${SOURCES[@]}" "$ROOT/template" -type f -exec cksum {} + | cksum)"
    if [ "$now" != "$last" ]; then
      last="$now"
      for md in "${SOURCES[@]}"; do build_one "$md" || true; done
    fi
    sleep 1
  done
else
  for md in "${SOURCES[@]}"; do build_one "$md"; done
fi
