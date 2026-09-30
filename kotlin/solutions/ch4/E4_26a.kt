// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26a

package sicp.ch4.solutions

import sicp.ch4.Direct
import sicp.ch4.LazyModule

// Exercise 4.26a (added by this edition): `when` as a derived expression,
// the mirror of 4.26's `unless`. The derivation is macro-style: one rewrite
// to forms the evaluator already has -- `(when condition body ...)` lowers
// to `(if condition (begin body ...) false)`, the `false` standing where
// the missing alternative stands. [whenToDerived] is that rewrite in guest
// code over the sealed family, so it composes with the kernel's own
// dispatch; a before/after trace pins that the name is unbound before the
// derivation and answers after it.

/** The derived-expression rewrite `(when c body ...)` ->
 * `(if c (begin body ...) false)` in kernel data. */
internal val WHEN_DERIVE_SOURCE: String =
    """
fun whenToDerived(g: GWhen): GExpr {
    val body = mutableListOf<GStmt>()
    var index = 0
    while (index < g.cases.size) {
        body.add(GExprStmt(g.cases.get(index).body))
        index = index + 1
    }
    val test = g.subject
    return GIf(test ?: GBool(true), GBlock(body), GBool(false))
}
    """.trimIndent()

/** The before trace: without the derivation, `when` is an application of
 * an unbound name. => "error\n" */
public fun whenBeforeTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    println(renderValue(gEval(GApp(GVar("when"), GNum(1L)), env)))
}
                """.trimIndent(),
        ),
    )

/** The after trace: the derived `when` evaluates. => "yes\n" */
public fun whenAfterTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + WHEN_DERIVE_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val derived = whenToDerived(GWhen(GLt(GNum(2L), GNum(3L)), listOf(GCase(null, GStr("yes"))), null))
    println(renderValue(gEval(derived, env)))
}
                """.trimIndent(),
        ),
    )

/** A false condition with no else arm answers the derived `false`.
 * => "false\n" */
public fun whenNoElseTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + WHEN_DERIVE_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val derived = whenToDerived(GWhen(GEq(GNum(1L), GNum(2L)), listOf(GCase(null, GStr("yes")), GCase(null, GStr("no"))), null))
    println(renderValue(gEval(derived, env)))
}
                """.trimIndent(),
        ),
    )

/** A multi-expression body runs as one sequence, answering the last.
 * => "3\n" */
public fun whenBodySequenceTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + WHEN_DERIVE_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val bodies = listOf(GCase(null, GNum(1L)), GCase(null, GNum(2L)), GCase(null, GNum(3L)))
    val derived = whenToDerived(GWhen(GBool(true), bodies, null))
    println(renderValue(gEval(derived, env)))
}
                """.trimIndent(),
        ),
    )

/** The same derivation over the lazy experiment's delayed discipline: the
 * unchosen body stays unforced, so its division never fires. => "42\n" */
internal val WHEN_LAZY_PROBE: String =
    """
fun whenLazy(condition: Boolean, body: Long, otherwise: Long): Long = if (condition) body else otherwise

fun main() {
    println(whenLazy(true, 42L, 1L / 0L))
}
    """.trimIndent()

/** The lazy side of the derivation. => "42\n" */
public fun whenLazyTranscript(): String = outcomeText(LazyModule.run(WHEN_LAZY_PROBE).map { it.result })
