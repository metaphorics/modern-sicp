// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
// Chapter 4, exercise 4.9

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.9: `while` and `until` lower to recursive procedures. Each
// generated loop is a `GLetRec` binding whose `GIf` selects the body or the
// done value; a body block ends with the recursive `GApp`.

/** Both derived loops bind their recursive procedure in guest code. */
internal val LOOPS_SOURCE: String =
    """
fun whileToCombination(test: GExpr, body: List<GStmt>): GExpr {
    val again: List<GStmt> = listOf(GExprStmt(GApp(GVar("loop"), GNum(0L))))
    val round = GIf(test, GBlock(body + again), GNum(0L))
    return GLetRec("loop", GLam("u", round), GApp(GVar("loop"), GNum(0L)))
}

fun untilToCombination(test: GExpr, body: List<GStmt>): GExpr {
    val again: List<GStmt> = listOf(GExprStmt(GApp(GVar("loop"), GNum(0L))))
    val round = GIf(test, GNum(0L), GBlock(body + again))
    return GLetRec("loop", GLam("u", round), GApp(GVar("loop"), GNum(0L)))
}
    """.trimIndent()

private fun runLoop(main: String): String = outcomeText(Direct.run(KERNEL_SOURCE + "\n" + LOOPS_SOURCE + "\n" + main.trimIndent()))

/** A while loop sums the integers from 1 through 5. => "15\n" */
public fun whileSumTranscript(): String =
    runLoop(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("n" to GNumV(0L), "total" to GNumV(0L)), null)
    val body = listOf(
        GAssignStmt("n", GAdd(GVar("n"), GNum(1L))),
        GAssignStmt("total", GAdd(GVar("total"), GVar("n"))),
    )
    gEval(whileToCombination(GLt(GVar("n"), GNum(5L)), body), env)
    println(renderValue(gEval(GVar("total"), env)))
}
        """,
    )

/** A false while test skips its body. => "0\n" */
public fun whileNeverRunsTranscript(): String =
    runLoop(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("n" to GNumV(0L)), null)
    gEval(whileToCombination(GLt(GVar("n"), GNum(0L)), listOf(GAssignStmt("n", GNum(1L)))), env)
    println(renderValue(gEval(GVar("n"), env)))
}
        """,
    )

/** Until multiplies 8 through 12 and stops at 13. => "95040\n13\n" */
public fun untilProductTranscript(): String =
    runLoop(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("n" to GNumV(8L), "product" to GNumV(1L)), null)
    val body = listOf(
        GAssignStmt("product", GMul(GVar("product"), GVar("n"))),
        GAssignStmt("n", GAdd(GVar("n"), GNum(1L))),
    )
    gEval(untilToCombination(GEq(GVar("n"), GNum(13L)), body), env)
    println(renderValue(gEval(GVar("product"), env)))
    println(renderValue(gEval(GVar("n"), env)))
}
        """,
    )

/** An already-satisfied until test skips its body. => "0\n" */
public fun untilNeverRunsTranscript(): String =
    runLoop(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("n" to GNumV(0L)), null)
    gEval(untilToCombination(GEq(GVar("n"), GNum(0L)), listOf(GAssignStmt("n", GNum(1L)))), env)
    println(renderValue(gEval(GVar("n"), env)))
}
        """,
    )

/** Nested derived loops keep their loop bindings local. => "6\n" */
public fun nestedLoopsTranscript(): String =
    runLoop(
        """
fun main() {
    val env = GFrame(
        mutableMapOf<String, GValue>("outer" to GNumV(0L), "inner" to GNumV(0L), "total" to GNumV(0L)),
        null,
    )
    val innerBody = listOf(
        GAssignStmt("inner", GAdd(GVar("inner"), GNum(1L))),
        GAssignStmt("total", GAdd(GVar("total"), GNum(1L))),
    )
    val innerLoop = whileToCombination(GLt(GVar("inner"), GNum(3L)), innerBody)
    val outerBody = listOf(
        GAssignStmt("outer", GAdd(GVar("outer"), GNum(1L))),
        GAssignStmt("inner", GNum(0L)),
        GExprStmt(innerLoop),
    )
    gEval(whileToCombination(GLt(GVar("outer"), GNum(2L)), outerBody), env)
    println(renderValue(gEval(GVar("total"), env)))
}
        """,
    )
