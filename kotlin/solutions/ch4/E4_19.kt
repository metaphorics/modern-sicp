// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.19

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.19: whose rule for fellow definitions. Ben evaluates the
// initializers in source order, so `b` reads the outer `a` and the pair
// sums to 16. Alyssa reserves every name first, so `b`'s initializer
// reads the empty reservation and fails. Eva reserves and then assigns
// in source order: `a` is set before `b` initializes, `b` sees the
// final 5, and the pair sums to 20. The debate program binds the outer
// `a` to 1 and calls with `x` at 10 throughout.

// Exercise 4.19: source order reads outward; reservation rejects; Eva sets first.

/** The debate frame: outer `a` at 1, the call's `x` at 10. */
internal val DEBATE_SOURCE: String =
    """
fun debateOuter(): GFrame = GFrame(mutableMapOf<String, GValue>("a" to GNumV(1L), "x" to GNumV(10L)), null)

fun debateNames(): List<String> = listOf("b", "a")

fun debateInits(): List<GExpr> = listOf(GAdd(GVar("a"), GVar("x")), GNum(5L))
    """.trimIndent()

/** Ben's source order: `b` reads the outer `a`. => "16\n" */
public fun benRuleTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + STRATEGIES_SOURCE + "\n" + DEBATE_SOURCE + "\n" +
                """
fun main() {
    println(renderValue(textEval(debateNames(), debateInits(), GAdd(GVar("a"), GVar("b")), debateOuter())))
}
                """.trimIndent(),
        ),
    )

/** Alyssa's reservation: the fellow read finds the marker empty.
 * => "error\n" */
public fun alyssaRuleTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + DEBATE_SOURCE + "\n" +
                """
fun main() {
    val reserved = GFrame(mutableMapOf<String, GValue>("b" to GUnassigned, "a" to GUnassigned), debateOuter())
    println(renderValue(gEval(GAdd(GVar("a"), GVar("x")), reserved)))
}
                """.trimIndent(),
        ),
    )

/** Eva's reserve-then-assign: `a` is set before `b` initializes, so `b`
 * sees the final 5. => "20\n" */
public fun evaRuleTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + DEBATE_SOURCE + "\n" +
                """
fun main() {
    val frame = GFrame(mutableMapOf<String, GValue>("b" to GUnassigned, "a" to GUnassigned), debateOuter())
    val setA = gEval(GSet("a", GNum(5L)), frame)
    val setB = gEval(GSet("b", GAdd(GVar("a"), GVar("x"))), frame)
    println(renderValue(gEval(GAdd(GVar("a"), GVar("b")), frame)))
}
                """.trimIndent(),
        ),
    )
