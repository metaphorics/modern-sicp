// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.35a

package sicp.ch4.solutions

// Exercise 4.35a (added by this edition): count the search's choices per
// triple. The counting rule is the experiment's own: one choice per
// entered alternative -- the first alternative on entering the choice
// point, and each later alternative a failure resumption enters, even
// when that branch then fails. The counter is a permanent write (it must
// survive backtracking to stay cumulative), and because the run streams
// its answers, the probe prints the cumulative count at each delivered
// triple: a triple's own share is the delta from the previous line. The
// engine's own `SearchRun.choices` reports the same total at the end of
// the run.

/** The triples program with the choice counter printed at each answer. */
internal val COUNTED_TRIPLES_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun aCountedTriple(low: Long, high: Long): Unit {
    val i = anIntegerBetween(low, high)
    val j = anIntegerBetween(i, high)
    val k = anIntegerBetween(j, high)
    requireThat(i * i + j * j == k * k)
    println(showLong(choicesTaken))
}

fun main() {
    budgetCap = 1000000L
    aCountedTriple(1L, 20L)
}
        """.trimIndent()

/** The same generator inside 1 and 9. */
internal val COUNTED_TRIPLES_9_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun aCountedTriple(low: Long, high: Long): Unit {
    val i = anIntegerBetween(low, high)
    val j = anIntegerBetween(i, high)
    val k = anIntegerBetween(j, high)
    requireThat(i * i + j * j == k * k)
    println(showLong(choicesTaken))
}

fun main() {
    budgetCap = 1000000L
    aCountedTriple(1L, 9L)
}
        """.trimIndent()

/** The cumulative choices taken by the time each triple arrives, between 1
 * and 20. => [925, 1805, 1979, 2583, 2713, 3181]
 *
 * Hand-derived, UNRUN: each `anIntegerBetween(low, H)` fully enumerated
 * costs 2*(H-low+1) entries (one delivery plus one branch entry per
 * value, including the final failing branch), so a completed i-row costs
 * (H-i)(H-i+3)+1 and answer (I,J,K) totals I + completed rows +
 * (J-I+1) j-deliveries + completed k-rows + (2(K-J)+1) k-entries.
 * Full-run engine totals reconcile: 3540 within 20, 438 within 9. */
public fun choicesTakenWithin20(): List<Long> = searchLines(COUNTED_TRIPLES_PROGRAM).map { it.toLong() }

/** The single triple between 1 and 9 arrives after 221 choices.
 * => [221] */
public fun choicesTakenWithin9(): List<Long> = searchLines(COUNTED_TRIPLES_9_PROGRAM).map { it.toLong() }
