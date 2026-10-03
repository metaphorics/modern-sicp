// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.54

package sicp.ch4.solutions

// Exercise 4.54: `require` as a special form. The experiment's `demand`
// is intrinsic syntax, not a procedure call: its condition is evaluated
// in the current attempt and a false condition fails that attempt back to
// the most recent untried alternative. As a procedure it would run in its
// own application frame and could not fail the caller's attempt at all.
// The probes pin the two observable ends: a requirement filters the
// choices that reach the answer, and a satisfied requirement lets the
// attempt through.

/** The filtering requirement. */
internal val REQUIRE_FILTER_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    val x = anElementOf(listOf(1L, 2L, 3L, 4L))
    requireThat(isEven(x))
    println(showLong(x))
}
        """.trimIndent()

/** The satisfied requirement. */
internal val REQUIRE_SATISFIED_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    requireThat(1L < 2L)
    println("ok")
}
        """.trimIndent()

/** The special form prunes the odd choices. => [2, 4] */
public fun requireFilteredEvens(): List<String> = searchLines(REQUIRE_FILTER_PROGRAM)

/** A satisfied requirement lets the attempt through. => "ok" */
public fun requireSatisfied(): String = searchLines(REQUIRE_SATISFIED_PROGRAM).first()
