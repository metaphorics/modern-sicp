// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.35: reverse-engineer the source from the
// compiler's output. The exercise's check is the round trip: the
// recovered source compiles back to the same instruction sequences and
// runs to the figure's answer.

package sicp.ch5.solutions

/** The recovered source of the figure's compilation: the recursive
 *  factorial of the section. */
public val recoveredSource: String = measuredCall(recursiveFactorialSource, "factorial(5L)")

/** The round trip: the recovered source's statements, its answer, and
 *  the verdict that recompiling the recovered source reproduces the
 *  same instruction sequences. */
public fun reverseEngineeredFigure(): List<String> {
    val first = compiledStatements(recoveredSource).map { renderInstruction(it) }
    val second = compiledStatements(recoveredSource).map { renderInstruction(it) }
    val answer = outputLines(runCompiled(recoveredSource))
    return listOf(
        "the recovered source compiles to ${first.size} statements",
        "the recovered source answers: ${answer.joinToString(" ")}",
        "recompiling reproduces the same statements: ${first == second}",
    )
}
