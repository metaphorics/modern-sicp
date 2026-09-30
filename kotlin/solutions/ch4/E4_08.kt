// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.8: named `let`. A named loop is a recursive procedure
// bound around its first call, which is exactly the kernel's
// tie-the-knot binder: the loop name binds the procedure and the body
// calls it. The kernel binds one parameter, so the countdown carries
// its state -- remaining count and running total -- in one pair, and
// the five-step sum answers 15.

/** `(loop init body...)` as a recursive procedure bound around its call. */
internal val NAMED_LET_SOURCE: String =
    """
fun namedLet(name: String, param: String, init: GExpr, body: GExpr): GExpr =
    GLetRec(name, GLam(param, body), GApp(GVar(name), init))

fun countdownBody(): GExpr =
    GIf(
        GEq(GMember(GVar("p"), "first"), GNum(0L)),
        GMember(GVar("p"), "second"),
        GApp(
            GVar("loop"),
            GConstruct(
                "Pair",
                listOf(
                    GSub(GMember(GVar("p"), "first"), GNum(1L)),
                    GAdd(GMember(GVar("p"), "second"), GMember(GVar("p"), "first")),
                ),
            ),
        ),
    )
    """.trimIndent()

/** The five-step countdown sums to 15. => "15\n" */
public fun namedLetTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + NAMED_LET_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val loop = namedLet("loop", "p", GConstruct("Pair", listOf(GNum(5L), GNum(0L))), countdownBody())
    println(renderValue(gEval(loop, env)))
}
                """.trimIndent(),
        ),
    )
