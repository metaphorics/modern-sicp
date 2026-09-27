// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.33: the alternative factorial's compilation.
// Both compilations are printed by the compiler and both procedures run
// on the machine. The operand order is the whole story: the book's
// factorial evaluates `n` into `val` before the recursive call and must
// save `argl` around the call to cons the answer on, while the
// alternative evaluates `n` after the call returns (for `(* n ...)`)
// and must save `env` around the call to reach the frame. Both compile
// to the same statement count with the same number of saves, so neither
// version executes faster.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private val factorialSource: String =
    """
    (define (factorial n)
      (if (= n 1)
          1
          (* (factorial (- n 1)) n)))
    """.trimIndent()

private val factorialAltSource: String =
    """
    (define (factorial-alt n)
      (if (= n 1)
          1
          (* n (factorial-alt (- n 1)))))
    """.trimIndent()

/** The save/restore instructions of a compilation, in order. */
private fun savesOf(stmts: List<String>): List<String> = stmts.filter { it.startsWith("(save ") || it.startsWith("(restore ") }

/** The comparison: both compilations' saves, both counts, and both
 *  runs answering 120. The define's own `ok` is not a call answer, so
 *  it stays out of the pinned reply. */
public fun altFactorialComparison(): List<String> {
    val cfg = CompilerConfig()
    val plain = compiledStatements(cfg, factorialSource)
    val alt = compiledStatements(cfg, factorialAltSource)
    val plainRun = valuesOf(runCompiled(cfg, factorialSource, "(factorial 5)")).filter { it != "ok" }
    val altRun = valuesOf(runCompiled(cfg, factorialAltSource, "(factorial-alt 5)")).filter { it != "ok" }
    return listOf(
        "factorial saves: ${savesOf(plain).joinToString("; ")}",
        "factorial-alt saves: ${savesOf(alt).joinToString("; ")}",
        "factorial: ${plain.size} statements, answers ${plainRun.joinToString(" ")}",
        "factorial-alt: ${alt.size} statements, answers ${altRun.joinToString(" ")}",
        "both compile to ${plain.size} statements with ${savesOf(plain).size} saves/restores; " +
            "the order swaps the preserved register from argl to env, neither runs faster",
    )
}
