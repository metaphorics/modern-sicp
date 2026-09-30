// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26

package sicp.ch4.solutions

import sicp.ch4.Direct
import sicp.ch4.LazyModule

// Exercise 4.26: two implementations of `unless`, and what each costs.
// Ben's side: `unless` as a derived expression -- [UNLESS_DERIVED_PROBE]
// lowers the three-arm application to `GIf(condition, exceptional, usual)`
// in guest code, so the untouched arm never evaluates. But the derivation
// is syntax: nothing is bound, so using `unless` as a value fails, and the
// kernel's application guard fires before any argument effect. Alyssa's
// side: under the lazy experiment `unless` stays an ordinary procedure
// whose operands are delayed thunks, so it composes -- a higher-order
// caller takes it as a value, and the unchosen arm's thunk is simply never
// forced.

/** The three-arm `unless` application shape of the object language. */
internal val UNLESS_CALL_SOURCE: String =
    """
fun unlessCall(condition: GExpr, usual: GExpr, exceptional: GExpr): GExpr =
    GApp(GApp(GApp(GVar("unless"), condition), usual), exceptional)

/** The derived-expression rewrite, or the input unchanged when it is not a
 * three-arm `unless` application. */
fun unlessToIf(g: GExpr): GExpr {
    if (g !is GApp) return g
    val middle = g.fn
    if (middle !is GApp) return g
    val inner = middle.fn
    if (inner !is GApp) return g
    val head = inner.fn
    if (head !is GVar) return g
    if (head.name != "unless") return g
    return GIf(inner.arg, g.arg, middle.arg)
}
    """.trimIndent()

/** Ben's derivation on the armed call: the chosen arm answers, the other
 * never evaluates. => "42\n" */
public fun unlessDerivedTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + UNLESS_CALL_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("armed" to GNumV(0L)), null)
    val derived = unlessToIf(unlessCall(GBool(true), GSet("armed", GBool(true)), GNum(42L)))
    println(renderValue(gEval(derived, env)))
}
                """.trimIndent(),
        ),
    )

/** Alyssa's objection, pinned: the derived `unless` is syntax, so its name
 * is not a value; the application guard fires before any argument effect.
 * => "error\n" */
public fun unlessDerivedValueUseTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + UNLESS_CALL_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("used" to GNumV(0L)), null)
    val call = unlessCall(GBool(false), GNum(1L), GSet("used", GBool(true)))
    println(renderValue(gEval(call, env)))
}
                """.trimIndent(),
        ),
    )

/** The full plane probe both sides share: the derived answer with its
 * unchosen arm's counter, then the name-as-value failure with its argument
 * counter. => "42\n0\nerror\n0\n" */
internal val UNLESS_PLANES_PROBE: String =
    KERNEL_SOURCE + "\n" + UNLESS_CALL_SOURCE + "\n" +
        """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("armed" to GNumV(0L), "used" to GNumV(0L)), null)
    val derived = unlessToIf(unlessCall(GBool(true), GSet("armed", GBool(true)), GNum(42L)))
    println(renderValue(gEval(derived, env)))
    println(renderValue(gEval(GVar("armed"), env)))
    val call = unlessCall(GBool(false), GNum(1L), GSet("used", GBool(true)))
    println(renderValue(gEval(call, env)))
    println(renderValue(gEval(GVar("used"), env)))
}
        """.trimIndent()

/** The statement's `unless`, as a lazy procedure. */
internal val UNLESS_LAZY_DEFINITION: String =
    """
fun unless(condition: Boolean, usual: Long, exceptional: Long): Long = if (condition) exceptional else usual

fun applyTriple(chooser: (Boolean, Long, Long) -> Long, condition: Boolean, usual: Long, exceptional: Long): Long =
    chooser(condition, usual, exceptional)

fun showLong(n: Long): String = "${'$'}{n}"
    """.trimIndent()

/** Alyssa's implementation: an ordinary procedure under delayed arguments.
 * The unchosen arm is a thunk that is never forced. => "42\n" */
public fun unlessLazyProcedureTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                UNLESS_LAZY_DEFINITION + "\n" +
                    """
fun main() {
    println(unless(false, 42L, 1L / 0L))
}
                    """.trimIndent(),
            ).map { it.result },
    )

/** The mapped composition's program: `unless` taken as a value twice. */
internal val UNLESS_LAZY_MAPPED_PROGRAM: String =
    UNLESS_LAZY_DEFINITION + "\n" +
        """
fun main() {
    val first = applyTriple(::unless, false, 42L, 1L / 0L)
    val second = applyTriple(::unless, false, 7L, 1L / 0L)
    println("[" + showLong(first) + ", " + showLong(second) + "]")
}
        """.trimIndent()

/** The higher-order composition the procedure keeps: a caller takes
 * `unless` as a value twice, the exceptional arm never forced.
 * => "[42, 7]\n" */
public fun unlessLazyMappedTranscript(): String = outcomeText(LazyModule.run(UNLESS_LAZY_MAPPED_PROGRAM).map { it.result })

/** The procedure value the special form could never name, applied through
 * the same higher-order caller; the delayed operand list stays lazy past
 * the call. => "7\n" */
public fun unlessLazyApplyTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                UNLESS_LAZY_DEFINITION + "\n" +
                    """
fun main() {
    println(applyTriple(::unless, false, 7L, 1L / 0L))
}
                    """.trimIndent(),
            ).map { it.result },
    )
