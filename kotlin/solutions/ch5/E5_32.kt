// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.32: the compiler could recognize a call whose
// operator is a symbol naming a primitive and open-code it, or keep the
// generic application path. This exercise compares the two compilations
// of the same computation -- a call of the named operator beside a call
// through a variable -- and records the design opinion in the rationale.
// The comparison rides the shipped compiler's own instruction sequences;
// both compilations must answer alike whatever the dispatch.

package sicp.ch5.solutions

/** The named-operator probe. */
public val namedOperatorSource: String =
    """
    fun probe(): Long {
        return 2L + 3L
    }

    fun main() {
        println(probe())
    }
    """.trimIndent()

/** The variable-operator probe: the same computation through a binding. */
public val variableOperatorSource: String =
    """
    val add: (Long, Long) -> Long = { first: Long, second: Long -> first + second }

    fun probe(): Long {
        return add(2L, 3L)
    }

    fun main() {
        println(probe())
    }
    """.trimIndent()

/** The two compilations beside each other: statement counts, save
 *  counts, and the agreement of the two runs. */
public fun symbolOperatorComparison(): List<String> {
    val named = compiledStatements(namedOperatorSource)
    val variable = compiledStatements(variableOperatorSource)
    val namedAnswer = outputLines(runCompiled(namedOperatorSource))
    val variableAnswer = outputLines(runCompiled(variableOperatorSource))
    return listOf(
        "named operator: ${named.size} statements, ${savePairs(named).size} save pairs",
        "variable operator: ${variable.size} statements, ${savePairs(variable).size} save pairs",
        "the two runs answer alike: ${namedAnswer == variableAnswer}",
    )
}
