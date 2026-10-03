// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20a

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.20a (added by this edition): nested shadowing under the
// reservation. While an inner scope's initializers run, the names it
// binds are already in scope with no values yet: an inner reservation
// of `f` shadows the outer recursive `f` from its first initializer
// onward, and the initializer's call reads the empty inner binding.
// Read with the outer binding in scope instead, the same call finds
// the recursive procedure and answers.

/** The outer recursive countdown both probes share. */
internal val SHADOW_OUTER_SOURCE: String =
    """
fun countdownLam(): GExpr =
    GLam("n", GIf(GEq(GVar("n"), GNum(0L)), GNum(1L), GApp(GVar("f"), GSub(GVar("n"), GNum(1L)))))

fun countdownValue(outer: GFrame): GValue? = gEval(GLetRec("f", countdownLam(), GVar("f")), outer)
    """.trimIndent()

/** The inner reservation shadows `f` during its own initializer.
 * => "error\n" */
public fun shadowedPrematureReadTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SHADOW_OUTER_SOURCE + "\n" +
                """
fun main() {
    val outer = GFrame(mutableMapOf<String, GValue>(), null)
    val fValue = countdownValue(outer)
    if (fValue == null) {
        println("error")
    } else {
        val scope = GFrame(mutableMapOf<String, GValue>("f" to fValue, "n" to GNumV(2L)), outer)
        val reserved = GFrame(mutableMapOf<String, GValue>("k" to GUnassigned, "f" to GUnassigned), scope)
        println(renderValue(gEval(GApp(GVar("f"), GSub(GVar("n"), GNum(1L))), reserved)))
    }
}
                """.trimIndent(),
        ),
    )

/** With the outer binding in scope the same call recurses. => "1\n" */
public fun outerScopeRecursionTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SHADOW_OUTER_SOURCE + "\n" +
                """
fun main() {
    val outer = GFrame(mutableMapOf<String, GValue>(), null)
    val fValue = countdownValue(outer)
    if (fValue == null) {
        println("error")
    } else {
        val scope = GFrame(mutableMapOf<String, GValue>("f" to fValue, "n" to GNumV(2L)), outer)
        val inner = GLet("k", GApp(GVar("f"), GSub(GVar("n"), GNum(1L))), GVar("k"))
        println(renderValue(gEval(inner, scope)))
    }
}
                """.trimIndent(),
        ),
    )
