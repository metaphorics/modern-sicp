// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.4

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.4: `and` and `or` as special forms. Operands must re-enter
// the evaluator one at a time, because short-circuit is the
// specification: the first false operand stops `and`, the first true
// operand stops `or`, and whatever follows never evaluates. The probe
// operands record their own evaluation in counter cells, so the printed
// counters pin exactly which operands ran. The derived side lowers the
// same forms to nested `GIf` in guest code, where the kernel's own
// conditional keeps the short-circuit.

/** The direct evaluators: first false stops `and`, first true stops `or`. */
internal val AND_OR_SOURCE: String =
    """
fun evalAnd(operands: List<GExpr>, env: GFrame): GValue? {
    var index = 0
    var current: GValue = GBoolV(true)
    while (index < operands.size) {
        val value = gEval(operands.get(index), env) ?: return null
        if (value is GBoolV && !value.b) {
            return GBoolV(false)
        }
        current = value
        index = index + 1
    }
    return current
}

fun evalOr(operands: List<GExpr>, env: GFrame): GValue? {
    var index = 0
    while (index < operands.size) {
        val value = gEval(operands.get(index), env) ?: return null
        if (value is GBoolV && value.b) {
            return value
        }
        index = index + 1
    }
    return GBoolV(false)
}

fun probe(name: String, value: Boolean): GExpr =
    GLet("u", GSet(name, GAdd(GVar(name), GNum(1L))), GBool(value))

fun counters(): GFrame =
    GFrame(mutableMapOf<String, GValue>("ran1" to GNumV(0L), "ran2" to GNumV(0L), "ran3" to GNumV(0L)), null)

fun showCounter(env: GFrame, name: String): GValue? = gEval(GVar(name), env)
    """.trimIndent()

/** The derived lowering: `(and a b ...)` nests conditionals. */
internal val DERIVED_AND_SOURCE: String =
    """
fun derivedAnd(operands: List<GExpr>): GExpr {
    if (operands.isEmpty()) {
        return GBool(true)
    }
    if (operands.size == 1) {
        return operands.get(0)
    }
    return GIf(operands.get(0), derivedAnd(operands.drop(1)), GBool(false))
}
    """.trimIndent()

/** Direct `and` stops at the first false; direct `or` stops at the first
 * true; the counters pin which operands ran.
 * => "false\n1\n0\n0\ntrue\n1\n1\n0\n" */
public fun specialAndOrTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + AND_OR_SOURCE + "\n" +
                """
fun main() {
    val andEnv = counters()
    val andOperands = listOf(probe("ran1", false), probe("ran2", true), probe("ran3", true))
    println(renderValue(evalAnd(andOperands, andEnv)))
    println(renderValue(showCounter(andEnv, "ran1")))
    println(renderValue(showCounter(andEnv, "ran2")))
    println(renderValue(showCounter(andEnv, "ran3")))
    val orEnv = counters()
    val orOperands = listOf(probe("ran1", false), probe("ran2", true), probe("ran3", true))
    println(renderValue(evalOr(orOperands, orEnv)))
    println(renderValue(showCounter(orEnv, "ran1")))
    println(renderValue(showCounter(orEnv, "ran2")))
    println(renderValue(showCounter(orEnv, "ran3")))
}
                """.trimIndent(),
        ),
    )

/** The derived `and` keeps the short-circuit through the kernel `GIf`.
 * => "false\n0\n" */
public fun derivedAndTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + AND_OR_SOURCE + "\n" + DERIVED_AND_SOURCE + "\n" +
                """
fun main() {
    val env = counters()
    val lowered = derivedAnd(listOf(GBool(false), GSet("ran1", GNum(1L))))
    println(renderValue(gEval(lowered, env)))
    println(renderValue(showCounter(env, "ran1")))
}
                """.trimIndent(),
        ),
    )
