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


# Prefer a prefix-installed Texinfo 7.3 (`just setup-books`, CONTRIBUTING
# step 6) over the distribution's older texi2any. MODERN_SICP_PREFIX wins;
# otherwise ~/.local and the documented cache fallback are tried in order.
prefix="${MODERN_SICP_PREFIX:-}"
if [ -n "$prefix" ]; then
    { [ -x "$prefix/bin/texi2any" ] \
        && "$prefix/bin/texi2any" --version 2>/dev/null | head -1 | grep -q '7\.3'; } \
        || die "MODERN_SICP_PREFIX=$prefix has no texi2any 7.3; run just setup-books"
fi
if [ -z "$prefix" ]; then
    for candidate in "$HOME/.local" "$HOME/.cache/modern-sicp/opt"; do
        if [ -x "$candidate/bin/texi2any" ] \
            && "$candidate/bin/texi2any" --version 2>/dev/null | head -1 | grep -q '7\.3'; then
            prefix="$candidate"
            break
        fi
    done
fi
if [ -n "$prefix" ] && [ -x "$prefix/bin/texi2any" ]; then
    PATH="$tools_dir/tex4ht:$prefix/bin:$PATH"
else
    PATH="$tools_dir/tex4ht:$PATH"
fi
export PATH

# The prefix Texinfo needs Archive::Zip for EPUB output; setup-books
# installs it under the same prefix.
if [ -n "$prefix" ] && [ -d "$prefix/lib/perl5" ]; then
    PERL5LIB="$prefix/lib/perl5${PERL5LIB:+:$PERL5LIB}"
    export PERL5LIB
fi
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

# The tex4ht extension leaves its working document beside the delivered pages
# (main_tex4ht_tex.html and friends). It is not UTF-8 and not book content, so
# downstream page tools must never see it.
find . -name '*_tex4ht_*.html' -delete || die "cannot remove the tex4ht work pages"
find . -name '*_tex4ht_tex.tex' -delete || die "cannot remove the tex4ht work TeX"
find . \( -name '*_tex4ht_*.4ct' -o -name '*_tex4ht_*.4tc' -o -name '*_tex4ht_*.dvi' \
    -o -name '*_tex4ht_*.idv' -o -name '*_tex4ht_*.lg' -o -name '*_tex4ht_*.log' \
    -o -name '*_tex4ht_*.tmp' -o -name '*_tex4ht_*.xref' \) -delete \
    || die "cannot remove the tex4ht work files"

# Delivered pages carry site-root-absolute asset URLs (the convention the
# prototype ticket records); texi2any writes the source tree's relative
# figure prefix, which resolves from the .texi file, not from a served page.
out=""
prev=""
for arg in "$@"; do
    if [ "$prev" = "-o" ]; then out="$arg"; fi
    prev="$arg"
done
if [ -n "$out" ] && printf '%s\n' "$@" | grep -q -- '--html'; then
    find "$out" -name '*.html' -type f -exec \
        sed -i 's|src="\(\.\./\)\+text/original/figures/|src="/text/original/figures/|g' {} + \
        || die "cannot rewrite asset URLs under $out"
fi
