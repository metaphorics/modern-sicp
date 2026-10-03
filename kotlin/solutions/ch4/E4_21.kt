// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.21

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.21: recursion without binding, by self-application. A
// procedure that takes itself as its first argument closes the loop
// every time it runs: applying the self-taker to itself answers a
// recursive procedure with no definition in force. Factorial and
// Fibonacci show the shape; even and odd share one self-taker, with
// oddness read off evenness.

// Exercise 4.21: self-application closes the loop with no definition.

/** Self-taking factorial, Fibonacci, and evenness. */
internal val SELF_APPLY_SOURCE: String =
    """
fun selfFact(): GExpr =
    GLam(
        "me",
        GLam(
            "n",
            GIf(
                GLt(GVar("n"), GNum(2L)),
                GNum(1L),
                GMul(GVar("n"), GApp(GApp(GVar("me"), GVar("me")), GSub(GVar("n"), GNum(1L)))),
            ),
        ),
    )

fun selfFib(): GExpr =
    GLam(
        "me",
        GLam(
            "n",
            GIf(
                GLt(GVar("n"), GNum(2L)),
                GVar("n"),
                GAdd(
                    GApp(GApp(GVar("me"), GVar("me")), GSub(GVar("n"), GNum(1L))),
                    GApp(GApp(GVar("me"), GVar("me")), GSub(GVar("n"), GNum(2L))),
                ),
            ),
        ),
    )

fun selfParity(): GExpr =
    GLam(
        "me",
        GLam(
            "tag",
            GLam(
                "n",
                GIf(
                    GEq(GVar("n"), GNum(0L)),
                    GEq(GVar("tag"), GNum(0L)),
                    GIf(
                        GEq(GVar("tag"), GNum(0L)),
                        GApp(GApp(GApp(GVar("me"), GVar("me")), GNum(1L)), GSub(GVar("n"), GNum(1L))),
                        GApp(GApp(GApp(GVar("me"), GVar("me")), GNum(0L)), GSub(GVar("n"), GNum(1L))),
                    ),
                ),
            ),
        ),
    )
    """.trimIndent()

/** Factorial by self-application: 10!. => "3628800\n" */
public fun selfApplicationFactTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SELF_APPLY_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    println(renderValue(gEval(GApp(GApp(selfFact(), selfFact()), GNum(10L)), env)))
}
                """.trimIndent(),
        ),
    )

/** Fibonacci by self-application: fib 10. => "55\n" */
public fun selfApplicationFibTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SELF_APPLY_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    println(renderValue(gEval(GApp(GApp(selfFib(), selfFib()), GNum(10L)), env)))
}
                """.trimIndent(),
        ),
    )

/** Even and odd close through one tag-dispatched self-taker. => "false\ntrue\n" */
public fun mutualEvenOddWithoutDefineTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SELF_APPLY_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val parity = GApp(selfParity(), selfParity())
    println(renderValue(gEval(GApp(GApp(parity, GNum(0L)), GNum(7L)), env)))
    println(renderValue(gEval(GApp(GApp(parity, GNum(0L)), GNum(10L)), env)))
}
                """.trimIndent(),
        ),
    )
