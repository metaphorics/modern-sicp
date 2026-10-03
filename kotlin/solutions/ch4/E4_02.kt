// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.2

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.2: Louis's applications-first order, and the `call` sugar.
// Louis checks the application clause before assignment and definition,
// so a definition evaluates as an application of the unbound operator
// `define`. The kernel's typed dispatch keeps the lesson sharp: the
// variant dispatcher below checks `GApp` first, and a use of the name
// `define` in operator position fails unbound -- while the kernel's own
// binding forms are distinct variants, so `GLet` and `GSet` route
// correctly whatever order the application clause takes. Part (b) is the
// `call` sugar as a guest rewrite: `call(operator, operand)` lowers to
// the kernel's single-argument `GApp`, and the square call answers 49.

/** Louis's dispatcher: the application clause runs before anything else. */
internal val APPLICATIONS_FIRST_SOURCE: String =
    """
fun evalApplicationsFirst(expr: GExpr, env: GFrame): GValue? {
    if (expr is GApp) {
        val fn = gEval(expr.fn, env) ?: return null
        if (fn is GClosV) {
            val arg = gEval(expr.arg, env) ?: return null
            return gApply(fn, arg)
        }
        return null
    }
    return gEval(expr, env)
}
    """.trimIndent()

/** The `call` sugar lowered to the kernel application. */
internal val CALL_SUGAR_SOURCE: String =
    """
fun callToApp(fn: GExpr, arg: GExpr): GExpr = GApp(fn, arg)
    """.trimIndent()

/** Louis's order: `define` in operator position is unbound, while the
 * kernel's binding forms still route by their own tags.
 * => "error\n3\ntrue\n" */
public fun louisDefineTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + APPLICATIONS_FIRST_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("x" to GNumV(0L)), null)
    println(renderValue(evalApplicationsFirst(GApp(GVar("define"), GNum(3L)), env)))
    println(renderValue(evalApplicationsFirst(GLet("y", GNum(3L), GVar("y")), env)))
    println(renderValue(evalApplicationsFirst(GSet("x", GNum(5L)), env)))
}
                """.trimIndent(),
        ),
    )

/** The `call` sugar: the square of 7. => "49\n" */
public fun callSugarTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + CALL_SUGAR_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val square = GLam("x", GMul(GVar("x"), GVar("x")))
    println(renderValue(gEval(callToApp(square, GNum(7L)), env)))
}
                """.trimIndent(),
        ),
    )
