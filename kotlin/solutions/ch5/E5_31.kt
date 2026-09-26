// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.31: which of the evaluator's saves survive
// preserving for four combinations. The compiler answers by
// construction: each compilation's preserving emits exactly the saves
// the register analysis demands, so the answer is the compiler's own
// output, listed save by save. The combinations are bare top-level
// expressions, so they compile the way the book's `compile-and-go`
// compiles a top-level expression: target `val`, linkage `return`.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig
import sicp.ch5.Linkage

/** The four combinations of the exercise, in order. */
private val combinations: List<String> =
    listOf(
        "(f 'x 'y)",
        "((f) 'x 'y)",
        "(f (g 'x) y)",
        "(f (g 'x) 'y)",
    )

/** The saves and restores of each combination's compilation: (a) keeps
 *  no saves at all, because every evaluation is a lookup that cannot
 *  clobber a register; (b) preserves `continue` around the operator
 *  call `((f))`, whose call machinery sets `continue` while the
 *  top-level return still needs it; (c) preserves `continue` and saves
 *  `argl` and `proc` around `(g 'x)`, whose call machinery would
 *  clobber them; (d) is (c) with the constant last operand, which
 *  changes nothing the mechanism can see. */
public fun superfluousSaves(): List<String> {
    val cfg = CompilerConfig()
    return combinations.map { src ->
        val saves =
            compiledStatements(cfg, src, Linkage.Return).filter { it.startsWith("(save ") || it.startsWith("(restore ") }
        "$src: ${saves.joinToString("; ")}"
    }
}
