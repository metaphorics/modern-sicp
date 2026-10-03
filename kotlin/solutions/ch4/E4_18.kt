// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.18

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.18: the alternative scan-out strategy. The text strategy
// assigns in source order, so each initializer sees the bindings before
// it; the alternative evaluates every initializer first and assigns
// after, so an initializer that reads a fellow definition finds the
// reservation still empty. The probe is the book's shape: `dy` defers
// its value behind a procedure, and `y`'s initializer applies it.

/** Source-order assignment against all-initializers-first assignment. */
internal val STRATEGIES_SOURCE: String =
    """
fun textEval(names: List<String>, inits: List<GExpr>, body: GExpr, env: GFrame): GValue? {
    val frame = GFrame(mutableMapOf<String, GValue>(), env)
    var index = 0
    while (index < names.size) {
        val value = gEval(inits.get(index), frame) ?: return null
        frame.cells[names.get(index)] = value
        index = index + 1
    }
    return gEval(body, frame)
}

fun altEval(names: List<String>, inits: List<GExpr>, body: GExpr, env: GFrame): GValue? {
    val frame = GFrame(mutableMapOf<String, GValue>(), env)
    var index = 0
    while (index < names.size) {
        val value = gEval(inits.get(index), env) ?: return null
        frame.cells[names.get(index)] = value
        index = index + 1
    }
    return gEval(body, frame)
}

fun fellowReads(): List<GExpr> = listOf(GLam("u", GNum(3L)), GApp(GVar("dy"), GNum(0L)))
    """.trimIndent()

/** The text strategy assigns in source order, so the applied read lands
 * after `dy` is bound. => "3\n" */
public fun textStrategyTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + STRATEGIES_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val names = listOf("dy", "y")
    println(renderValue(textEval(names, fellowReads(), GVar("y"), env)))
}
                """.trimIndent(),
        ),
    )

/** The alternative evaluates `y`'s initializer before any assignment,
 * and the read hits the empty reservation. => "error\n" */
public fun altStrategyTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + STRATEGIES_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val names = listOf("dy", "y")
    println(renderValue(altEval(names, fellowReads(), GVar("y"), env)))
}
                """.trimIndent(),
        ),
    )
