// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.10

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.10: new syntax by translation, never by parser. A `defun`
// surface form lowers to a defining statement for a procedure value;
// a `fun` surface form lowers to the procedure itself. Both rewrites
// are guest functions over kernel data, so no new evaluation rule and
// no foreign reader is involved: the lowering runs first, the kernel
// evaluates what it already knows.

/** Surface syntax lowered to kernel statements and procedures. */
internal val SYNTAX_SOURCE: String =
    """
fun defunToKernel(name: String, param: String, body: GExpr): GStmt = GVarStmt(name, GLam(param, body))

fun funToKernel(param: String, body: GExpr): GExpr = GLam(param, body)
    """.trimIndent()

/** The lowered `defun` defines; the lowered `fun` applies. => "49\n42\n" */
public fun newSyntaxTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SYNTAX_SOURCE + "\n" +
                """
fun main() {
    val square = defunToKernel("square", "x", GMul(GVar("x"), GVar("x")))
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val program = listOf(square, GExprStmt(GApp(GVar("square"), GNum(7L))))
    println(renderValue(runStatements(program, env)))
    val doubled = GApp(funToKernel("x", GAdd(GVar("x"), GVar("x"))), GNum(21L))
    println(renderValue(gEval(doubled, env)))
}
                """.trimIndent(),
        ),
    )
