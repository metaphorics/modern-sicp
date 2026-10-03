# Notice

## Attribution

"Structure and Interpretation of Computer Programs", second edition, by Harold Abelson and Gerald Jay Sussman with Julie Sussman. The MIT Press, 1996. Licensed by The MIT Press under the Creative Commons Attribution-ShareAlike 4.0 International License, https://creativecommons.org/licenses/by-sa/4.0/.

## Lineage

1. The MIT Press HTML edition, https://mitpress.mit.edu/sicp.
2. The Unofficial Texinfo Format by Lytha Ayth and Neil Van Dyke, https://www.neilvandyke.org/sicp-texi/.
3. The HTML5 and EPUB3 edition by Andres Raba (`sarabander/sicp`), https://github.com/sarabander/sicp, with contributions by Gavrie Philipson, Li Xuanji, J. E. Johnson, Matt Iversen, and Eugene Sharygin.
4. This repository, `metaphorics/modern-sicp`.

## Changes made in this repository

- The prose is rewritten four times, once for each of Rust, OCaml, TypeScript, and Kotlin. Section and exercise numbers stay the same as in the book. Sections 3.2, 3.4, and 4.3 are re-cut for each language's own closure model, concurrency mechanisms, and search mechanism.
- Every program printed in the book is re-expressed in the four languages. The re-expressed programs live under `examples/`, `exercises/`, and `solutions/` in each edition.
- A Chapter 0 primer is added to each edition.
- Chapters 4 and 5 interpret and compile a checked subset of each edition's own language, specified under `spec/host-subsets/`, in place of the book's Scheme evaluator. The book's lessons, section numbers, and exercise numbers are unchanged.
- Shared figure labels use conceptual mathematics and descriptive operation names instead of source-language expressions. Their evaluation steps, values, connections, and legal metadata are preserved. PDF figures are regenerated from the adapted SVG sources.
- The build machinery of the 2014 edition is replaced by stock Texinfo 7.3.

## Licenses of the adapted work

- Prose, figures, and exercise statements: CC BY-SA 4.0, https://creativecommons.org/licenses/by-sa/4.0/legalcode.
- Programs: GPL-3.0-only, https://www.gnu.org/licenses/gpl-3.0.html, applied as the BY-SA Compatible License that CC BY-SA 4.0 Section 3(b)(1) permits.
- Original tooling and the site: MIT, https://opensource.org/license/mit.

The prose may be reused under CC BY-SA 4.0 and the code under GPL-3.0-only; the two never mix in one file. The scope map by path is in `LICENSE`.

## Warranty

The licensor offers the licensed material as-is and as-available. See CC BY-SA 4.0 Section 5 and GPL-3.0 Sections 15 and 16.
