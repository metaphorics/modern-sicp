// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.7

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.7: `let*` as nested `let`s. Each init sees the earlier
// bindings, so the lowering folds right to left: every binding but the
// last wraps the lowering of the rest. The kernel's single-binding
// `GLet` is exactly the node the fold builds, and the sequential pins
// show each init reading its predecessors while shadowing still works.

internal val LET_STAR_SOURCE: String =
    """
fun letStar(names: List<String>, inits: List<GExpr>, body: GExpr): GExpr {
    if (names.isEmpty()) {
        return body
    }
    return GLet(names.get(0), inits.get(0), letStar(names.drop(1), inits.drop(1), body))
}
    """.trimIndent()

/** Each init reads its predecessors; shadowing still binds inward.
 * => "7\n2\n" */
public fun letStarTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + LET_STAR_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("x" to GNumV(5L)), null)
    val sequential = letStar(listOf("x", "y"), listOf(GNum(3L), GAdd(GVar("x"), GNum(1L))), GAdd(GVar("x"), GVar("y")))
    println(renderValue(gEval(sequential, env)))
    val shadowed = letStar(listOf("x", "x"), listOf(GNum(5L), GNum(2L)), GVar("x"))
    println(renderValue(gEval(shadowed, env)))
}
                """.trimIndent(),
        ),
    )
