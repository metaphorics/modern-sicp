// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.16

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.16: internal definitions scan out. A recursive binding
// installs the name first as the unassigned marker and evaluates the
// value behind it, so self-reference closes but a premature read has
// no value to answer: the kernel's variable rule filters the marker
// to a null answer. Mutual recursion installs both closures in one
// shared frame so each can see the other; the marker's spelling is
// observable only through the frame.

/** Mutually recursive bodies installed in one shared tie-the-knot frame. */
internal val MUTUAL_SOURCE: String =
    """
fun evenBody(): GExpr =
    GLam("n", GIf(GEq(GVar("n"), GNum(0L)), GBool(true), GApp(GVar("odd"), GSub(GVar("n"), GNum(1L)))))

fun oddBody(): GExpr =
    GLam("n", GIf(GEq(GVar("n"), GNum(0L)), GBool(false), GApp(GVar("even"), GSub(GVar("n"), GNum(1L)))))

    """.trimIndent()

/** Mutual recursion over the scan-out answers. => "true\n" */
public fun mutualRecursionTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + MUTUAL_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val both = GFrame(mutableMapOf<String, GValue>("even" to GUnassigned, "odd" to GUnassigned), env)
    gEval(GSet("even", evenBody()), both)
    gEval(GSet("odd", oddBody()), both)
    println(renderValue(gEval(GApp(GVar("even"), GNum(10L)), both)))
}
                """.trimIndent(),
        ),
    )

/** A premature read and an unbound read both fail; the marker spells
 * itself through the frame. => "error\nerror\nunassigned\n" */
public fun prematureReadTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    println(renderValue(gEval(GLetRec("a", GVar("a"), GNum(0L)), env)))
    println(renderValue(gEval(GVar("missing"), env)))
    val marked = GFrame(mutableMapOf<String, GValue>("a" to GUnassigned), null)
    println(renderValue(marked.cells["a"]))
}
                """.trimIndent(),
        ),
    )
