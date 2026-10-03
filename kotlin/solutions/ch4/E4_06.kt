// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
// Chapter 4, exercise 4.6

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.6: a single `let` binding is an application of a lambda to
// its initializer. The application evaluates that initializer in the
// current frame, then runs the body in a frame with the new binding.

internal val LET_SOURCE: String =
    """
fun letRewrite(name: String, init: GExpr, body: GExpr): GExpr = GApp(GLam(name, body), init)
    """.trimIndent()

private fun runLet(main: String): String = outcomeText(Direct.run(KERNEL_SOURCE + "\n" + LET_SOURCE + "\n" + main.trimIndent()))

/** Ordinary and derived `let` both answer 7. => "7\n7\n" */
public fun letEquivalenceTranscript(): String =
    runLet(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val body = GAdd(GVar("x"), GNum(4L))
    println(renderValue(gEval(GLet("x", GNum(3L), body), env)))
    println(renderValue(gEval(letRewrite("x", GNum(3L), body), env)))
}
        """,
    )

/** The initializer reads the outer binding; the body reads the new one.
 * => "7\n" */
public fun letBodyTranscript(): String =
    runLet(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("x" to GNumV(5L)), null)
    val init = GAdd(GVar("x"), GNum(2L))
    println(renderValue(gEval(letRewrite("x", init, GVar("x")), env)))
}
        """,
    )

/** Nested derived bindings evaluate in their respective frames. => "3\n" */
public fun letNestedTranscript(): String =
    runLet(
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val nested = letRewrite("x", GNum(1L), letRewrite("y", GNum(2L), GAdd(GVar("x"), GVar("y"))))
    println(renderValue(gEval(nested, env)))
}
        """,
    )
