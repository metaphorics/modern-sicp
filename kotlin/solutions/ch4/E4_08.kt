// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
// Chapter 4, exercise 4.8

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.8: named `let`. The recursive name must be in scope while
// its procedure is being defined, so the rewrite uses `GLetRec` before
// making the initial `GApp`. The Fibonacci loop carries its three values
// in one nested pair because the kernel procedure has one parameter.

internal val NAMED_LET_SOURCE: String =
    """
fun namedLet(name: String, param: String, init: GExpr, body: GExpr): GExpr =
    GLetRec(name, GLam(param, body), GApp(GVar(name), init))

fun fibonacciBody(): GExpr {
    val state = GVar("p")
    val pair = GMember(state, "second")
    val a = GMember(pair, "first")
    val b = GMember(pair, "second")
    val count = GMember(state, "first")
    return GIf(
        GEq(count, GNum(0L)),
        a,
        GApp(
            GVar("loop"),
            GConstruct(
                "Pair",
                listOf(
                    GSub(count, GNum(1L)),
                    GConstruct("Pair", listOf(b, GAdd(a, b))),
                ),
            ),
        ),
    )
}

fun localLoopBody(): GExpr =
    GIf(
        GEq(GVar("n"), GNum(0L)),
        GNum(1L),
        GApp(GVar("loop"), GSub(GVar("n"), GNum(1L))),
    )
    """.trimIndent()

private fun runNamedLet(main: String): String = outcomeText(Direct.run(KERNEL_SOURCE + "\n" + NAMED_LET_SOURCE + "\n" + main.trimIndent()))

/** The named-let Fibonacci loop computes F(10). => "55\n" */
public fun namedLetFibonacciTranscript(): String =
    runNamedLet(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val initial = GConstruct("Pair", listOf(GNum(10L), GConstruct("Pair", listOf(GNum(0L), GNum(1L)))))
    val loop = namedLet("loop", "p", initial, fibonacciBody())
    println(renderValue(gEval(loop, env)))
}
        """,
    )

/** A local recursive name leaves an outer binding intact. => "1\n7\n" */
public fun loopNameLocalTranscript(): String =
    runNamedLet(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("loop" to GNumV(7L)), null)
    val loop = namedLet("loop", "n", GNum(1L), localLoopBody())
    println(renderValue(gEval(loop, env)))
    println(renderValue(gEval(GVar("loop"), env)))
}
        """,
    )

/** An ordinary `let` still evaluates. => "3\n" */
public fun plainLetTranscript(): String =
    runNamedLet(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    println(renderValue(gEval(GLet("x", GNum(3L), GVar("x")), env)))
}
        """,
    )
