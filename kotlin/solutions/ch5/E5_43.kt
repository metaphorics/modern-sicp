// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.43: scan out the internal definitions of a
// procedure body. The exercise turns the scan-out knob both ways over
// the probe with internal declarations and compares the runs: the
// scanned-out body and the original answer alike.

package sicp.ch5.solutions

import sicp.ch5.Compiler
import sicp.ch5.CompilerOptions

/** The probe with internal declarations. */
private val internalDefinitionsSource: String =
    """
    fun compose(n: Long): Long {
        fun step(value: Long): Long {
            return value * 2L
        }
        val base: Long = n + 1L
        return step(base)
    }

    fun main() {
        println(compose(3L))
    }
    """.trimIndent()

/** The scan-out's verdict: both settings answer alike, and the scanned
 *  body's answer is the probe's answer. */
public fun scanOutInternalDefines(): List<String> {
    val original =
        outputLines(sicp.ch4.Direct.run(admitProgram(internalDefinitionsSource)))
    val scanned =
        outputLines(Compiler.compileAndRun(admitProgram(internalDefinitionsSource), CompilerOptions(scanOutDefines = true)))
    val unscanned =
        outputLines(Compiler.compileAndRun(admitProgram(internalDefinitionsSource), CompilerOptions(scanOutDefines = false)))
    return listOf(
        "the two programs answer alike: ${original == scanned && scanned == unscanned}",
        "the scanned-out body answers: ${scanned.joinToString(" ")}",
    )
}
