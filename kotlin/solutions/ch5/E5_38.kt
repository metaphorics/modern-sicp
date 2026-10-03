// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.38: open-coding the primitive arithmetic. The
// exercise turns the open-coded primitives knob both ways over the
// arithmetic probe: with open coding the arithmetic compiles beside the
// operands, without it the generic call path carries it. The observable
// kept here is behavioral: both compilations answer alike.

package sicp.ch5.solutions

import sicp.ch5.Compiler
import sicp.ch5.CompilerOptions

/** The arithmetic probe. */
public val arithmeticProbeSource: String =
    """
    fun probe(): Long {
        return (2L + 3L) * 4L - 5L
    }

    fun main() {
        println(probe())
    }
    """.trimIndent()

/** Both dispatch routes' compilations beside each other and the runs'
 *  agreement with direct execution. */
public fun openCodingReport(): List<String> {
    val open = compiledStatements(arithmeticProbeSource, CompilerOptions(openCodedPrimitives = setOf("+", "-", "*")))
    val generic = compiledStatements(arithmeticProbeSource, CompilerOptions(openCodedPrimitives = emptySet()))
    val checked = admitProgram(arithmeticProbeSource)
    val compiled = outputLines(Compiler.compileAndRun(checked))
    val direct = outputLines(sicp.ch4.Direct.run(checked))
    return listOf(
        "open-coded: ${open.size} statements and ${savePairs(open).size} save pairs",
        "generic calls: ${generic.size} statements and ${savePairs(generic).size} save pairs",
        "compiled and direct runs agree: ${compiled == direct}",
    )
}
