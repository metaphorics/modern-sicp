// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.33: compare factorial with its multiplication
// operands reversed. The compiler's Kotlin binary operands evaluate
// left-to-right, so comparing their typed instruction sequences shows
// the consequences of which operand makes the recursive call.

package sicp.ch5.solutions

/** Factorial with the recursive call before the multiplication's `n`. */
public val factorialRecursiveProbe: String = recursiveFactorialSource

/** The alternative puts the recursive call after its other operand. */
public val factorialAlternativeProbe: String =
    """
    fun factorial(n: Long): Long {
        if (n == 1L) {
            return 1L
        }
        return n * factorial(n - 1L)
    }
    """.trimIndent()

/** The two operand orders beside each other: statements, save pairs,
 *  and the agreement of their compiled answers. */
public fun factorialComparison(): List<String> {
    val originalSource = factorialRecursiveProbe + "\nfun main() { println(factorial(5L)) }\n"
    val alternativeSource = factorialAlternativeProbe + "\nfun main() { println(factorial(5L)) }\n"
    val original = compiledStatements(originalSource)
    val alternative = compiledStatements(alternativeSource)
    val originalAnswer = outputLines(runCompiled(originalSource))
    val alternativeAnswer = outputLines(runCompiled(alternativeSource))
    val originalDepth = compiledStats(originalSource).second
    val alternativeDepth = compiledStats(alternativeSource).second
    return listOf(
        "recursive operand first: ${original.size} statements, ${savePairs(original).size} save pairs",
        "n operand first: ${alternative.size} statements, ${savePairs(alternative).size} save pairs",
        "the two runs answer alike: ${originalAnswer == listOf("120") && alternativeAnswer == originalAnswer}",
        "the alternative's pending operand uses deeper stack: ${alternativeDepth > originalDepth}",
    )
}
