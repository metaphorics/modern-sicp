#!/bin/bash
# Run texi2any with the MathML-capable tex4ht configuration, and fail when the
# TeX run dies.
#
# Every edition's `book` recipe calls this instead of texi2any directly, so the
# configuration lives in one committed file rather than in each developer's and
# each CI job's environment.
#
# Three things are not optional:
#
#   * `tools/tex4ht` goes first on PATH. texi2any's tex4ht extension runs the
#     converter with an empty option string (share/texi2any/ext/tex4ht.pm,
#     `my $options = '';`), and tex4ht renders math as PNG images unless it is
#     given `mathml` in argv $2. The wrappers there supply exactly that
#     argument and pass the generated document through untouched.
#   * The conversion runs through plain TeX. The extension emits a bare
#     `\documentclass{article}` preamble with no packages, so the LaTeX path
#     cannot resolve `\eqalign` or `\text`.
#   * The tex4ht logs are inspected afterwards. texi2any exits 0 when the TeX
#     run hits an emergency stop: it drops every equation from the output and
#     reports success, leaving the error only in `*_tex4ht_*.log`.
#
# Arguments are forwarded to texi2any unchanged.
set -euo pipefail
IFS=$'\n\t'

die() {
    printf '%s\n' "texi2any_html.sh: $1" >&2
    exit 1
}

tools_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd) || die "cannot resolve script directory"

PATH="$tools_dir/tex4ht:$PATH"
export PATH

T4H_MATH_CONVERSION="${T4H_MATH_CONVERSION:-tex}"
T4H_TEX_CONVERSION="${T4H_TEX_CONVERSION:-tex}"
export T4H_MATH_CONVERSION T4H_TEX_CONVERSION

for required in texi2any httex tex4ht t4ht; do
    command -v -- "$required" >/dev/null 2>&1 || die "$required is not on PATH; see CONTRIBUTING.md"
done

# tex4ht writes its logs beside whichever output directory texi2any chose, so
# the scan below is recursive. A marker file dates this run, keeping logs left
# by an earlier build from failing it.
marker=$(mktemp) || die "cannot create the run marker"
cleanup() {
    rm -f -- "$marker"
}
trap cleanup EXIT INT TERM

texi2any \
    -c HTML_MATH=t4h \
    -c "T4H_MATH_CONVERSION=$T4H_MATH_CONVERSION" \
    -c "T4H_TEX_CONVERSION=$T4H_TEX_CONVERSION" \
    "$@" || die "texi2any failed"

# texi2any's exit status does not cover the TeX subprocess; read its logs.
failed=0
while IFS= read -r log; do
    grep -q '^!' -- "$log" || continue
    printf '%s\n' "texi2any_html.sh: TeX errors in $log:" >&2
    grep -n '^!' -- "$log" >&2 || true
    failed=1
done < <(find . -name '*_tex4ht_*.log' -newer "$marker" -print)
[ "$failed" -eq 0 ] || die "the TeX math run failed; the output is missing math"
