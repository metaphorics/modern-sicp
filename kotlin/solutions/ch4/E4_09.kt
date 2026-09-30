// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.9: iteration constructs designed as derived expressions.
// `while` repeats its body while the test holds; `until` repeats until
// it holds. Both lower to a self-calling one-argument procedure bound
// around its first call: the test picks the body block or the done
// value, and the block ends by invoking the loop again. The kernel has
// no inequality but equality, so the `until` probe exits on equality.

/** The loop lowerings: test picks the body block or the done value. */
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

/** `while` sums 1..4; `until` doubles into the total until equality.
 * => "4\n10\n3\n12\n" */
public fun loopsTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + LOOPS_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("n" to GNumV(0L), "total" to GNumV(0L), "m" to GNumV(0L), "total2" to GNumV(0L)), null)
    val whileLoop = whileToCombination(GLt(GVar("n"), GNum(4L)), listOf(GAssignStmt("n", GAdd(GVar("n"), GNum(1L))), GAssignStmt("total", GAdd(GVar("total"), GVar("n")))))
    val whileValue = gEval(whileLoop, env)
    println(renderValue(gEval(GVar("n"), env)))
    println(renderValue(gEval(GVar("total"), env)))
    val untilLoop = untilToCombination(GEq(GVar("m"), GNum(3L)), listOf(GAssignStmt("m", GAdd(GVar("m"), GNum(1L))), GAssignStmt("total2", GAdd(GVar("total2"), GAdd(GVar("m"), GVar("m"))))))
    val untilValue = gEval(untilLoop, env)
    println(renderValue(gEval(GVar("m"), env)))
    println(renderValue(gEval(GVar("total2"), env)))
}
                """.trimIndent(),
        ),
    )
