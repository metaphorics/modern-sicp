// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.20: `letrec` as the recursive binder. The binding installs
// the name as the unassigned marker and evaluates the value behind it,
// so the factorial procedure closes over its own name and the call
// answers 3628800. Reading the name during its own initialization
// finds the marker empty, and a plain `let` never installs the name at
// all: both fail, through the reservation and through absence.

// Exercise 4.20: letrec ties the knot; premature and absent reads fail.

/** Factorial as one `letrec` over the kernel's tie-the-knot binder. */
internal val LETREC_SOURCE: String =
    """
fun letrecFact(): GExpr =
    GLetRec(
        "fact",
        GLam(
            "n",
            GIf(
                GLt(GVar("n"), GNum(2L)),
                GNum(1L),
                GMul(GVar("n"), GApp(GVar("fact"), GSub(GVar("n"), GNum(1L)))),
            ),
        ),
        GApp(GVar("fact"), GNum(10L)),
    )
    """.trimIndent()

/** The factorial recursion bound by one `letrec`. => "3628800\n" */
public fun letrecFactTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + LETREC_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    println(renderValue(gEval(letrecFact(), env)))
}
                """.trimIndent(),
        ),
    )

/** A premature read finds the reservation empty. => "error\n" */
public fun letrecPrematureReadTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    println(renderValue(gEval(GLetRec("y", GVar("y"), GNum(0L)), env)))
}
                """.trimIndent(),
        ),
    )

/** A plain `let` never installs the recursive name. => "error\n" */
public fun plainLetUnboundTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + LETREC_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val fact = GLam(
        "n",
        GIf(
            GLt(GVar("n"), GNum(2L)),
            GNum(1L),
            GMul(GVar("n"), GApp(GVar("fact"), GSub(GVar("n"), GNum(1L)))),
        ),
    )
    println(renderValue(gEval(GLet("fact", fact, GApp(GVar("fact"), GNum(10L))), env)))
}
                """.trimIndent(),
        ),
    )
