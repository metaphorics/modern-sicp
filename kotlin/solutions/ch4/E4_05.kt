// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.5

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.5: `cond` clauses of the shape `(test => recipient)`. The
// rewrite evaluates the test exactly once: it binds the test value and
// applies the recipient to it when true, falling through to the rest
// otherwise. The probe test counts its own evaluations, so the printed
// counter pins the single-evaluation contract both when the test holds
// and when it fails.

/** `(test => recipient)` with `rest` after it, as kernel data. */
internal val ARROW_SOURCE: String =
    """
fun arrowToApp(test: GExpr, recipient: GExpr, rest: GExpr): GExpr =
    GApp(GLam("arrowValue", GIf(GVar("arrowValue"), GApp(recipient, GVar("arrowValue")), rest)), test)

fun countedTest(value: Boolean): GExpr =
    GLet("u", GSet("effects", GAdd(GVar("effects"), GNum(1L))), GBool(value))

fun effectFrame(): GFrame = GFrame(mutableMapOf<String, GValue>("effects" to GNumV(0L)), null)
    """.trimIndent()

/** True test answers through the recipient; false test answers the rest;
 * both evaluate the test exactly once. => "42\n1\n7\n1\n" */
public fun arrowClauseTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ARROW_SOURCE + "\n" +
                """
fun main() {
    val firstEnv = effectFrame()
    val first = arrowToApp(countedTest(true), GLam("v", GNum(42L)), GNum(0L))
    println(renderValue(gEval(first, firstEnv)))
    println(renderValue(gEval(GVar("effects"), firstEnv)))
    val secondEnv = effectFrame()
    val second = arrowToApp(countedTest(false), GLam("v", GNum(42L)), GNum(7L))
    println(renderValue(gEval(second, secondEnv)))
    println(renderValue(gEval(GVar("effects"), secondEnv)))
}
                """.trimIndent(),
        ),
    )
