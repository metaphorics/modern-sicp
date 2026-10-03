// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.36: operand evaluation order in the compiler.
// The exercise turns the compiler's operand-order knob both ways over
// the nested-call probe and compares the two compilations; both runs
// must answer alike whatever the order.

package sicp.ch5.solutions

import sicp.ch5.Compiler
import sicp.ch5.CompilerOptions

/** The nested-call probe: one argument is itself a call. */
public val operandProbeSource: String =
    """
    fun g(value: String): String {
        return value
    }

    fun f(first: String, second: String): String {
        return first
    }

    val y: String = "y"

    fun probe(): String {
        return f(g("x"), y)
    }

    fun main() {
        println(probe())
    }
    """.trimIndent()

/** Both operand orders' compilations beside each other and the runs'
 *  agreement with direct execution. */
public fun operandOrderReport(): List<String> {
    val leftFirst = compiledStatements(operandProbeSource, CompilerOptions(leftToRightArguments = true))
    val rightFirst = compiledStatements(operandProbeSource, CompilerOptions(leftToRightArguments = false))
    val pairs = savePairs(leftFirst)
    val checked = admitProgram(operandProbeSource)
    val compiled = outputLines(Compiler.compileAndRun(checked))
    val direct = outputLines(sicp.ch4.Direct.run(checked))
    return listOf(
        "left to right: ${leftFirst.size} statements, ${savePairs(leftFirst).size} save pairs",
        "right to left: ${rightFirst.size} statements, ${savePairs(rightFirst).size} save pairs",
        "every save is paired with one restore: ${pairs.none { it.restoreIndex <= it.saveIndex }}",
        "compiled and direct runs agree: ${compiled == direct}",
    )
}
