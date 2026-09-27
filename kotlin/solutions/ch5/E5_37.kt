// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.37: preserving disabled. With `preservingOn`
// false, every register in every preserved set is saved and restored
// unconditionally. The factorial's compilation grows, and the monitored
// session at n = 5 pays for each blind save while the answer stays 120.
// The unnecessary operations are precisely the saves whose register the
// second sequence does not need: most visibly the `save continue`
// around every sequence link.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private val factorialSource: String =
    """
    (define (factorial n)
      (if (= n 1)
          1
          (* (factorial (- n 1)) n)))
    """.trimIndent()

/** The comparison: the statement and save/restore counts with the
 *  mechanism on and off, and the monitored sessions at n = 5. */
public fun preservingComparison(): List<String> {
    val plain = CompilerConfig()
    val blind = CompilerConfig(preservingOn = false)
    val (stmtsWith, savesWith) = compileCounts(plain, factorialSource)
    val (stmtsWithout, savesWithout) = compileCounts(blind, factorialSource)
    val statsWith = statLines(runCompiledMonitored(plain, factorialSource, "(factorial 5)"))
    val statsWithout = statLines(runCompiledMonitored(blind, factorialSource, "(factorial 5)"))
    val valuesWith = valuesOf(runCompiledMonitored(plain, factorialSource, "(factorial 5)")).filter { it != "ok" }
    return listOf(
        "with preserving: $stmtsWith statements, $savesWith saves/restores",
        "without: $stmtsWithout statements, $savesWithout saves/restores",
        "monitored with: ${statsWith.joinToString(" ")}",
        "monitored without: ${statsWithout.joinToString(" ")}",
        "the answer stays ${valuesWith.joinToString(" ")} either way",
    )
}
