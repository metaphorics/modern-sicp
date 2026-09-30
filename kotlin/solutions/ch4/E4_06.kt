// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.6: `let` as a derived expression. A single binding lowers
// exactly to an application of a procedure to its init, and the kernel's
// curried application shapes that derivation: one name, one operand.
// Multi-binding `let` stays simultaneous -- every init evaluates in the
// outer frame before any name binds -- which the nested single derivation
// cannot promise under shadowing, so the multi probe evaluates the inits
// in the outer frame first and the shadowing pin answers 5, not 3. The
// nested derivation agrees wherever inits ignore the bound names.

/** One binding: `(let ((name init)) body)` as application. */
internal val LET_SOURCE: String =
    """
fun letRewrite(name: String, init: GExpr, body: GExpr): GExpr = GApp(GLam(name, body), init)

fun evalLet(names: List<String>, inits: List<GExpr>, body: GExpr, env: GFrame): GValue? {
    val frame = GFrame(mutableMapOf<String, GValue>(), env)
    var index = 0
    while (index < names.size) {
        val value = gEval(inits.get(index), env) ?: return null
        frame.cells[names.get(index)] = value
        index = index + 1
    }
    return gEval(body, frame)
}
    """.trimIndent()

/** The derivation, the simultaneous shadowing pin, and the nested
 * agreement. => "6\n5\n7\n" */
public fun letDerivedTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + LET_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("x" to GNumV(5L)), null)
    val single = letRewrite("x", GNum(3L), GAdd(GVar("x"), GVar("x")))
    println(renderValue(gEval(single, env)))
    val names = listOf("x", "y")
    val inits = listOf(GNum(3L), GVar("x"))
    println(renderValue(evalLet(names, inits, GVar("y"), env)))
    val nested = GApp(GLam("x", GApp(GLam("y", GAdd(GVar("x"), GVar("y"))), GNum(4L))), GNum(3L))
    println(renderValue(gEval(nested, env)))
}
                """.trimIndent(),
        ),
    )
