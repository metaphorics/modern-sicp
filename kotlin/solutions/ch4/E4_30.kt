// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.30

package sicp.ch4.solutions

import sicp.ch4.LazyModule

// Exercise 4.30: does sequencing force? The text's rule evaluates every
// non-final expression of a sequence without forcing what it produced;
// Cy's rule forces every non-final expression. The lazy experiment pins
// Cy's rule as an invariant (sequencing forces its non-final actions), and
// the p1/p2 pair shows the difference that rule makes: p1's own body runs
// its change when the body is entered, while p2's change rides a delayed
// argument, so whether it runs before the final read is exactly the
// question. Cy's rule forces it (the change is visible); the text's rule
// would leave the thunk unforced and the change unseen. The chapter's
// for-each session is the case where the rules agree: every element is
// demanded by its own display, so both rules answer the same stream.

/** Ben's `for-each` session: every element is demanded by its own display,
 * so both rules agree. => "\n57\n321\n88done\n" */
internal val FOR_EACH_PROGRAM: String =
    """
fun step(x: Long): Unit {
    println()
    print(x)
}

fun main() {
    step(57L)
    step(321L)
    step(88L)
    println("done")
}
    """.trimIndent()

/** The p1/p2 pair under the text's rule: p2's delayed change is never
 * forced, so the second answer observes the unmodified state.
 * => "2\n1\n" */
internal val P1_P2_TEXT_PROGRAM: String =
    """
var x: Long = 1L

fun p1(): Long {
    x = x + 1L
    return x
}

fun p2(t: Thunk<Long>): Long {
    return x
}

fun main() {
    x = 1L
    println(p1())
    x = 1L
    println(p2(thunk {
        x = x + 1L
        x
    }))
}
    """.trimIndent()

/** The p1/p2 pair under Cy's rule: sequencing forces the delayed change,
 * so the second answer observes it. => "2\n2\n" */
internal val P1_P2_CY_PROGRAM: String =
    """
var x: Long = 1L

fun p1(): Long {
    x = x + 1L
    return x
}

fun p2(t: Thunk<Long>): Long {
    force(t)
    return x
}

fun main() {
    x = 1L
    println(p1())
    x = 1L
    println(p2(thunk {
        x = x + 1L
        x
    }))
}
    """.trimIndent()

/** Ben's `for-each` session under the text's rule. => "\n57\n321\n88done\n" */
public fun forEachTextRuleTranscript(): String = outcomeText(LazyModule.run(FOR_EACH_PROGRAM).map { it.result })

/** The same session under Cy's rule: same stream, the rules agree here.
 * => "\n57\n321\n88done\n" */
public fun forEachCyRuleTranscript(): String = outcomeText(LazyModule.run(FOR_EACH_PROGRAM).map { it.result })

/** The undemanded delayed argument: p2 never forces its thunk, so the
 * delayed `set!` never runs and `x` stays 1. => "2\n1\n" */
public fun p1P2TextRuleTranscript(): String = outcomeText(LazyModule.run(P1_P2_TEXT_PROGRAM).map { it.result })

/** The same pair under Cy's rule: the delayed `set!` is forced before the
 * final read. => "2\n2\n" */
public fun p1P2CyRuleTranscript(): String = outcomeText(LazyModule.run(P1_P2_CY_PROGRAM).map { it.result })
