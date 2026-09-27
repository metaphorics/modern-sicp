# Design review (designer agent) — raw output

{
  "adjudications": [
    {
      "advisor": "overcritic: Texinfo 7.3 lacks Pygments support",
      "ruling": "rejected",
      "basis": "The Texinfo 7.3 manual node 'Code Examples Syntax Highlighting in HTML' (gnu.org, fetched 2026-09-22, transcript line 38) documents -c HIGHLIGHT_SYNTAX=pygments with pygmentize and the @example language argument; the 7.3 release announcement was read the same session. The advisor reasons from pre-7.3 behavior (the machine has 7.2, where the feature was experimental). D21 stands; residual risk is absorbed by Amendment 10, which fails loudly if built HTML ever contains only user-<language> class markup and no Pygments token spans."
    },
    {
      "advisor": "default: amend text-pipeline steps 2 and 5 for floats and display math",
      "ruling": "already delivered",
      "basis": "Amendments 1, 2, and 10 say exactly this, grounded by repo verification: no @ifhtml anywhere in sicp-pocket.texi, floats carry only @ifinfo plus @iftex, and more than 80 @tex display blocks exist. One divergence: the advisor checks <math> in 1.2.2; the deliverable checks section 2.1, which carries both Figure 2.1 and display blocks :30:/:31:, so one section covers both assertions. Either anchor satisfies the acceptance."
    },
    {
      "advisor": "default: fix build commands, two stylesheets, real file names, jsMath GPL",
      "ruling": "already delivered",
      "basis": "Amendments 3 and 4 cover the missing EPUB --css-include, the split-depth breakage of relative font URLs, the style.css/prettify.css/fonts.css naming correction, and the jsMath GPL correction. No new action."
    },
    {
      "advisor": "default: pin interaction display and close Chapter 0 gaps",
      "ruling": "already delivered",
      "basis": "Amendments 5 and 7 cover the => marker, printed-output versus returned-value rules, string quoting, ok results, compound-procedure values, the Rust no-REPL shape, the PDF legibility argument, the 0.2 and 0.5 row fixes, and the wayfinder ticket .md to .texi fix. One divergence kept: the advisory writes '; => ok'; the deliverable pins per-language comment syntax (// in Rust, TypeScript, Kotlin; (* *) in OCaml), matching the Kotlin companion's existing convention."
    },
    {
      "advisor": "overcritic: explicit font and EPUB asset step, @import flattening",
      "ruling": "partially accepted, folded into Amendments 3 and 10",
      "basis": "Right: style.css line 1 is @import url(fonts/fonts.css) (verified by grep), so assembly must inline fonts.css and let no @import survive, and the site build must copy the font files and check their URLs; the deliverable previously scoped the asset check to img/object only. Wrong: PDF output typesets with TeX fonts and the OFL woff files play no part in texi2pdf, so no font step is owed to PDF."
    }
  ],
  "contract_part_1_decision_register": [
    {
      "principle": "clarity",
      "verdict": "amend",
      "reason": "The plan names every reader-facing convention but fixes none of them, so four implementers will invent four interaction shapes, four signpost wordings, and two stylesheet assemblies."
    },
    {
      "principle": "impact",
      "verdict": "amend",
      "reason": "Verified in the repo and the 7.3 manual: stock Texinfo 7.3 renders the source's figure floats and @tex display blocks as empty in HTML and EPUB, and the EPUB command ships no stylesheet, so figures, math, and styled pages all fail silently under the current V2 checks."
    },
    {
      "principle": "audience",
      "verdict": "amend",
      "reason": "The Chapter 0 skeleton omits what 1.1 actually uses, contradicts the TypeScript digest on when Effect is taught, and leaves the reader-facing wording of replacements and additions unfixed."
    },
    {
      "principle": "risk",
      "verdict": "amend",
      "reason": "book.css does not exist upstream, the moved fonts are mixed-license (jsMath is GPL, not OFL as the plan states), inlined CSS resolves font and image URLs against split pages at wrong depths, and texi2pdf has no upstream evidence for this source; each is cheap to fix in Phase A and expensive after four editions exist."
    },
    {
      "principle": "sequencing",
      "verdict": "accept",
      "reason": "Prototyping 1.1 through the full pipeline before the primers, lockstep authoring by section, and publication last prove every convention on real content before scaling, and every amendment slots into the existing phases without reordering."
    },
    {
      "principle": "reversibility",
      "verdict": "accept",
      "reason": "Every reader-facing artifact is a checked-in file rebuilt by just books, so a convention change replays across all four editions, and the only irreversible step (Pages deployment) is per edition and runs last."
    }
  ],
  "contract_part_2_amendments": [
    {
      "n": 1,
      "section": "Text pipeline and demolition, step 2 (split_texi.py)",
      "text": "2a. Figure floats. Each float is rewritten to: @float / @anchor{Figure N.M} / @ifinfo: @strong{Figure N.M:} <caption>, blank line, <ASCII art, unchanged> / @end ifinfo / @ifhtml: @image{<rewritten path>,,<width>,Figure N.M: <caption>,.std.svg} / @end ifhtml / @iftex: @image{<rewritten path>,,<width>,,.pdf} / @end iftex / @caption{@strong{Figure N.M:} <caption>} / @end float. The @caption moves out of @iftex so HTML, EPUB, and PDF all emit a real caption; the fourth @image argument is set to 'Figure N.M: <caption>' as alt text. tools/parity_check.py pins 91 floats, 91 @ifhtml @image lines, 91 non-empty alt arguments, and 91 @caption lines. Grounding: the source's 91 floats carry only @ifinfo and @iftex and no @ifhtml exists anywhere in sicp-pocket.texi, so stock Texinfo 7.3 (manual: @ifinfo is Info and plain text only, HTML and EPUB read @ifhtml) renders no figures today."
    },
    {
      "n": 2,
      "section": "Text pipeline and demolition, step 2 (split_texi.py)",
      "text": "2b. Displayed math. Each @tex \\[ … \\] @end tex block becomes @displaymath … @end displaymath with the \\[ \\] delimiters removed, placed before the existing @ifinfo ASCII fallback, which is kept. Blocks using \\begin{eqnarray} or \\begin{array} are rewritten into plain-TeX \\matrix or \\eqalign alignments by the tool's REWRITES table, because texi2pdf typesets with texinfo.tex, which has no LaTeX environments, and the upstream Makefile never built PDF. tools/parity_check.py pins the displaymath count equal to the source @tex \\[ count and fails on any remaining @tex \\[. Grounding: the source has more than 80 numbered display blocks and HTML_MATH=t4h processes @math and @displaymath only."
    },
    {
      "n": 3,
      "section": "Repository layout (text/assets/css/ line) and Text pipeline and demolition, step 5",
      "text": "Replace the layout line 'book.css and the OFL fonts (from html/css)' with: style.css, prettify.css, fonts.css, fonts/, and the font license files OFL-1.1.txt, GPL.txt, LICENCE.txt (from html/css, moved verbatim; Libertine, Biolinum, Inconsolata, DejaVu, and STIX are OFL, jsMath is GPL). Add to step 5: a tools/build_css.py assembles two stylesheets per edition. book.css (HTML) is fonts.css with url() rewritten to the deployed font directory, plus the stock-7.3-ported rules from style.css, plus highlight.css (Amendment 4); the assembly inlines fonts.css directly and lets no @import survive, because style.css line 1 is @import url(fonts/fonts.css), which would resolve against each split page. book-epub.css (EPUB) drops the @font-face rules and the .jump fixed-position navigation and sets html { font-size: 100% }, matching the upstream comment in style.css. The EPUB command gains --css-include=book-epub.css. The jsMath font is GPL and is dropped because MathML replaces its only use. EPUB ships without embedded fonts and uses reader defaults. The site build copies the font files into the deployed assets directory; inlined CSS resolves font and image URLs against each split page, and section-split pages live in chapter subdirectories, so the prototype ticket records the one chosen convention (root-relative URLs for the site, or a flat node split) and tools/check_html_assets.py fails the build on any unresolvable src or font URL. The OFL woff files play no role in PDF output, which typesets with TeX fonts."
    },
    {
      "n": 4,
      "section": "Edition design, new bullet after 'Interaction display'",
      "text": "Highlighting theme. text/assets/css/highlight.css is the single Pygments theme for all four languages, assembled into both stylesheets. Result lines are comments, so Pygments renders them in the comment style in HTML and EPUB; that is intentional. Class map: keyword .k .kd .kr #5a3696; type and class .kt .nc #7a3e00; string .s .s1 .s2 .sc #2f6b2f; number .m .mi .mf .mh #8a4b08; builtin .nb #0b5fa5; function name .nf .fm #383838 bold; comment and results .c .c1 .cm .cp #6a737d italic; operator and punctuation .o .p #383838; lexer error .err #b00020. All colors meet WCAG AA contrast against the page background. Code keeps the book faces (Inconsolata LGC at the existing size)."
    },
    {
      "n": 5,
      "section": "Edition design, 'Interaction display' bullet (replace)",
      "text": "Interaction display: the source prints results as @i{…} lines after the expression. Each edition shows the definition, the call, and the result in one @example <language> block. The result is a comment line carrying a => marker: '// => 441' in Rust, TypeScript, and Kotlin; '(* => 441 *)' in OCaml. The marker keeps results valid comments in every format and distinguishable in the unhighlighted PDF. Shapes: printed output lines (for-each, probes, display-stream) are comment lines without the marker, in order, then the returned value with the marker; a call with no useful value ends '// => ok'; strings print double-quoted; from 4.1 on, compound procedures print '// => compound-procedure' and errors print '// => Error: <message>'; wrapped results end with a comment line containing only the comment opener and an ellipsis. Each companion fixes its call-line shape (OCaml keeps utop form and inferred types); Rust has no REPL, so its call line is a comment naming the example that printed the result. Section 0.8 states the convention and shows one worked example per format. The same value is asserted in the examples/ test, so printed outputs are true by construction."
    },
    {
      "n": 6,
      "section": "Exercise policy, after the class definitions",
      "text": "Reader-facing signposts are fixed sentences. A replacement (R) opens its statement block with: '@emph{SICP exercise N.M is about Scheme itself, so this edition replaces it; the exercise below keeps the number and teaches the same section material.}' A tailored addition opens with: '@emph{Exercise N.Ma is added by this edition and extends exercise N.M; SICP numbers stop at N.M.}' Anchors follow the numbers (@anchor{Exercise N.Ma}); tools/texi_indexes.py matches @anchor{Exercise ([0-9]+\\.[0-9]+[a-z]?)} and lists an addition immediately after its base exercise in the List of Exercises; tools/exercise_map_check.py expects 356 base anchors plus each edition's additions."
    },
    {
      "n": 7,
      "section": "Chapter 0 primer, skeleton table; wayfinder companion",
      "text": "Replace the 0.2 row with: '0.2 Values, names, and functions | Literals, bindings, functions, closures, higher-order functions; booleans, comparison operators, and conditionals; operator precedence in infix languages; integers, floats, and the division rule used in this book'. Replace the 0.5 row with: '0.5 Mutation and ownership | The language's mutable cell, aliasing, and identity; Rust ownership and borrowing; OCaml refs and mutable fields; TypeScript assignment and Ref (Effect enters at 3.1, not here); Kotlin captured var'. Add after the table: '0.1 states that running the code is optional and the book reads standalone in every format. The interaction convention taught in 0.8 is the one fixed under Edition design, applied from the first listing in 0.2. Each companion lists its 0.x exercises; the exercises use only constructs the primer has taught.' In docs/plan/wayfinder-map.md, replace the Chapter 0 ticket's 'book/ch0/*.md exists with the eight sections' with 'book/ch0/0.1.texi to book/ch0/0.8.texi exist with the eight sections'."
    },
    {
      "n": 8,
      "section": "Edition design, 'Chapter appendix per edition' bullet (replace)",
      "text": "Chapter appendix per edition: sections 2.4, 2.5, 3.3, and 4.1 close with an 'In this language' note set as a quotation block whose first line is '@strong{In this language: <topic>.}', where <topic> names the host mechanism. The note runs at most fifteen lines: one paragraph naming the host mechanism, one short typed sketch, one sentence saying when the book's table mechanism still wins. It sits after the section's last prose paragraph and before its first exercise. The OCaml edition closes every chapter with the Base and Core appendix in this fixed six-subsection format: Base translation; Core translation; Errors; Collections; Data interchange; Tests."
    },
    {
      "n": 9,
      "section": "Work breakdown, Phase H row",
      "text": "Add the first unit: H0 landing page and cross-edition links (one unit, before per-edition publication). A root site/ holds the landing source; just site builds it and deploys it to GitHub Pages beside the four editions at /rust/, /ocaml/, /typescript/, /kotlin/. The page contains, in order: the title and the subtitle 'Four editions: Rust, OCaml, TypeScript, Kotlin'; a lineage and license paragraph pointing to NOTICE.md; a four-row table (edition, toolchain line, HTML, EPUB 3, PDF links); a 'How the editions differ' section naming the three re-cuts (3.2, 3.4, 4.3) and linking to the exercise map; a 'How to read' section pointing at 0.8 of each edition; a footer with the attribution notice. tools/edition_switcher.py runs after each HTML build and injects a nav class='editions' into every page linking to the same section file in the other three editions; the mapping is mechanical because section numbers and node names are 1:1. book.css styles the nav as a small bar above the page header. EPUB and PDF carry no nav."
    },
    {
      "n": 10,
      "section": "Verification, V2",
      "text": "Add: (1) the built HTML for section 2.1, which carries Figure 2.1 and displayed math in every edition, contains at least one figure image and at least one <math> element produced by a @displaymath block; (2) the built HTML contains 91 figure elements, and every img or object src and every CSS-referenced font URL resolves to an existing file (tools/check_html_assets.py); (3) section 1.1 HTML contains at least one span with a Pygments token class (k, s, mi, c1) inside a pre.example, which proves -c HIGHLIGHT_SYNTAX=pygments ran and fails loudly if the converter emits only its default user-<language> class markup; (4) the EPUB, unzipped, contains <math> markup and 91 image files; epubcheck runs when installed."
    },
    {
      "n": 11,
      "section": "Work breakdown, unit contract, step 3",
      "text": "Add: preserve every @newterm and @cindex of the source paragraph in the adapted prose, and add @newterm for each host-language concept the section introduces, so the Term Index stays complete per edition."
    }
  ],
  "contract_part_3_tasks": [
    {
      "n": 1,
      "phase": "A3",
      "item": "Extend tools/split_texi.py with the figure-float normalization (Amendment 1) and the display-math conversion with its REWRITES table (Amendment 2)",
      "acceptance": "tools/parity_check.py pins the 91 floats, 91 displaymath conversions, and zero remaining @tex \\[ blocks, and texi2any --html on the assembled tree renders 91 figure images"
    },
    {
      "n": 2,
      "phase": "A3",
      "item": "Write tools/build_css.py and text/assets/css/highlight.css to assemble book.css and book-epub.css (Amendments 3 and 4), flattening the style.css @import, and add --css-include=book-epub.css to the EPUB command",
      "acceptance": "both stylesheets build; the books CI job passes; no built HTML references a missing font file"
    },
    {
      "n": 3,
      "phase": "B (prototype ticket)",
      "item": "Record the chosen font-URL and image-path convention on the prototype ticket and implement tools/check_html_assets.py covering image srcs and font URLs",
      "acceptance": "the section 1.1 build in each edition passes the src-resolution check from a clean checkout"
    },
    {
      "n": 4,
      "phase": "B",
      "item": "Write the four Chapter 0 primers from the amended skeleton, with 0.8 teaching the fixed interaction convention",
      "acceptance": "the four wayfinder Chapter 0 tickets' Done-when lists pass, and each 0.2 covers booleans, conditionals, precedence, and the number model"
    },
    {
      "n": 5,
      "phase": "B (prototype)",
      "item": "Apply the fixed R and N.Ma signpost sentences, the @anchor{Exercise N.Ma} pattern, and the texi_indexes.py regex change (Amendment 6)",
      "acceptance": "tools/exercise_map_check.py expects 356 base anchors plus additions, and a built List of Exercises shows an addition after its base exercise"
    },
    {
      "n": 6,
      "phase": "C and D",
      "item": "Write the 'In this language' closing notes in 2.4, 2.5, 3.3, and 4.1 in the fixed quotation format, and the OCaml chapter appendixes in the six-subsection format (Amendment 8)",
      "acceptance": "just book fails for an edition when a required section lacks its closing note, checked by a small lint in the books gate"
    },
    {
      "n": 7,
      "phase": "H0",
      "item": "Build the site/ landing page and tools/edition_switcher.py (Amendment 9)",
      "acceptance": "every section page in each edition links to the same section in the other three editions, and all links resolve on the deployed site"
    },
    {
      "n": 8,
      "phase": "H",
      "item": "Run the per-edition render verification of Amendment 10 (figures, display math, Pygments spans, EPUB math and image count, font URLs, epubcheck when installed)",
      "acceptance": "the books CI job runs these checks on every push and fails on any missing element"
    }
  ],
  "contract_part_4_open_questions": "none",
  "critical_files": [
    "tools/split_texi.py — to be created in Phase A3; owns the float normalization and display-math conversion that the current build silently drops",
    "text/original/sicp-pocket.texi — the 91 float shapes and 80-plus @tex \\[ blocks the tool must transform; verified anchors are in the plan's Repository facts",
    "html/css/style.css and html/css/fonts.css — the real upstream stylesheets (there is no book.css); sources for the ported rules, the @import to flatten, and the rewritten font URLs",
    "tools/texi_indexes.py — to be created in Phase A3; its anchor regex decides whether additions appear in the List of Exercises",
    "justfile (root) — the books and new site recipes wire the CSS assembly, asset checks, switcher injection, and render verification into one gate"
  ]
}