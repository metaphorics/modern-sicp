// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.3

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.3: data-directed dispatch. The kernel's `when` over the
// sealed family already dispatches on the expression tag; this exercise
// moves that dispatch into data -- a `Map` from tag names to handler
// functions -- so a new clause installs with one entry instead of an
// edit. The probe shows the additivity the book is after: the base table
// answers numbers but has no addition clause, and the late-installed
// `add` entry answers the sum through the same driver.

/** The expression tag: one name per sealed variant. */
internal val TAG_SOURCE: String =
    """
fun tagOf(expr: GExpr): String =
    when (expr) {
        is GNum -> "num"
        is GBool -> "bool"
        is GStr -> "str"
        is GNull -> "null"
        is GVar -> "var"
        is GLam -> "lam"
        is GApp -> "app"
        is GLet -> "let"
        is GLetRec -> "letrec"
        is GIf -> "if"
        is GAdd -> "add"
        is GMul -> "mul"
        is GLt -> "lt"
        is GSet -> "set"
        is GSub -> "sub"
        is GDiv -> "div"
        is GMod -> "mod"
        is GEq -> "eq"
        is GBlock -> "block"
        is GWhen -> "when"
        is GIs -> "is"
        is GMember -> "member"
        is GIndex -> "index"
        is GConstruct -> "construct"
    }
    """.trimIndent()

/** The installable handlers: numbers, and addition over kernel values. */
internal val HANDLERS_SOURCE: String =
    """
fun evalNum(expr: GExpr, env: GFrame): GValue? {
    if (expr is GNum) {
        return GNumV(expr.n)
    }
    return null
}

fun evalAdd(expr: GExpr, env: GFrame): GValue? {
    if (expr is GAdd) {
        val left = gEval(expr.left, env) ?: return null
        val right = gEval(expr.right, env) ?: return null
        if (left is GNumV && right is GNumV) {
            return GNumV(left.n + right.n)
        }
        return null
    }
    return null
}

fun evalTable(expr: GExpr, env: GFrame, table: Map<String, (GExpr, GFrame) -> GValue?>): GValue? {
    val handler = table[tagOf(expr)]
    if (handler == null) {
        return null
    }
    return handler(expr, env)
}
    """.trimIndent()

/** The base table answers numbers; addition installs late and answers.
 * => "42\nerror\n5\n" */
public fun tableDispatchTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + TAG_SOURCE + "\n" + HANDLERS_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val base: Map<String, (GExpr, GFrame) -> GValue?> = mapOf("num" to ::evalNum)
    println(renderValue(evalTable(GNum(42L), env, base)))
    println(renderValue(evalTable(GAdd(GNum(2L), GNum(3L)), env, base)))
    val extended = base + ("add" to ::evalAdd)
    println(renderValue(evalTable(GAdd(GNum(2L), GNum(3L)), env, extended)))
}
                """.trimIndent(),
        ),
    )
