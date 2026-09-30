// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.36

package sicp.ch4.solutions

// Exercise 4.36: unbounded Pythagorean triples. The fair generator runs
// the hypotenuse as the outermost choice, so every finite k is searched
// out fully before the next k begins and the answers arrive in k order.
// The naive replacement makes the innermost choice unbounded: the search
// dives through k = 1, 2, 3, ... for i = j = 1 and never backtracks to j =
// 2, so no triple ever arrives. The run's horizon is the edition's typed
// budget probe: the naive generator exhausts its 600-choice budget, the
// budget line prints once, and every further choice fails, so the run
// terminates instead of hanging the suite.

/** The fair generator with a horizon-aware bound: the `n <= high` guard
 * runs BEFORE the next `choose` is constructed, so the k-spine is finite
 * (1..20) instead of an infinite `anIntegerStartingFrom` filtered after
 * the fact. */
internal val FAIR_TRIPLE_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun tripleLine(i: Long, j: Long, k: Long): String = "(" + showLong(i) + " " + showLong(j) + " " + showLong(k) + ")"

fun anIntegerStartingFromUpTo(n: Long, high: Long): Long {
    requireThat(n <= high)
    return choose(firstLong(n), restIntegerFromUpTo(n, high))
}

fun restIntegerFromUpTo(n: Long, high: Long): Long {
    countChoice()
    return anIntegerStartingFromUpTo(n + 1L, high)
}

fun aPythagoreanTriple(): Unit {
    val k = anIntegerStartingFromUpTo(1L, 20L)
    val i = anIntegerBetween(1L, k)
    val j = anIntegerBetween(i, k)
    requireThat(i * i + j * j == k * k)
    println(tripleLine(i, j, k))
}

fun main() {
    budgetCap = 1000000L
    aPythagoreanTriple()
}
        """.trimIndent()

/** The naive replacement: the innermost choice is unbounded. */
internal val NAIVE_TRIPLE_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun tripleLine(i: Long, j: Long, k: Long): String = "(" + showLong(i) + " " + showLong(j) + " " + showLong(k) + ")"

fun aPythagoreanTripleNaive(): Unit {
    val i = anIntegerStartingFrom(1L)
    val j = anIntegerStartingFrom(i)
    val k = anIntegerStartingFrom(j)
    requireThat(i * i + j * j == k * k)
    println(tripleLine(i, j, k))
}

fun main() {
    budgetCap = 600L
    aPythagoreanTripleNaive()
}
        """.trimIndent()

/** The fair generator's first six triples: the hypotenuse runs first and
 * each finite k is searched out fully.
 * => [(3 4 5), (6 8 10), (5 12 13), (9 12 15), (8 15 17), (12 16 20)] */
public fun fairTriplesFirstSix(): List<String> = searchLines(FAIR_TRIPLE_PROGRAM)

/** The naive replacement never leaves its first two choices: the innermost
 * unbounded choice never exhausts, so no triple ever arrives before the
 * 600-choice budget spends itself.
 * => "choice budget exhausted after 600 choices" */
public fun naiveBudgetFault(): String = searchLines(NAIVE_TRIPLE_PROGRAM).last()
