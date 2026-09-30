// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.3 solutions: the shared guest programs and the
// observation helpers the exercise solutions drive. Internal to the
// solutions source set; nothing here is an exercise answer by itself.

package sicp.ch4.solutions

import sicp.ch4.SearchModule
import sicp.ch4.SearchRun

/**
 * The section's base library as ED39 guest source for the `Search`
 * experiment: requirements, element and integer choices, primality,
 * membership, and distinctness. The choice-counting hook follows the
 * experiment's counting rule -- one choice per entered alternative --
 * and it fires inside each alternative (first and recursive-branch
 * entries alike, before a branch can fail), so the guest count stays
 * in lockstep with the engine's own counter. The write is permanent
 * so the cumulative count survives backtracking. The budget hook is this
 * edition's typed budget probe: once the cap is spent every further
 * choice fails, the run exhausts, and the budget line prints once.
 */
internal val AMB_BASE_PRELUDE: String =
    """
var choicesTaken: Long = 0L

var backtracks: Long = 0L

var budgetCap: Long = 1000000L

fun showLong(n: Long): String = "${'$'}{n}"

fun showPair(a: Long, b: Long): String = "(" + showLong(a) + " " + showLong(b) + ")"

fun dwellingLine(baker: Long, cooper: Long, fletcher: Long, miller: Long, smith: Long): String =
    "((baker " + showLong(baker) + ") (cooper " + showLong(cooper) + ") (fletcher " + showLong(fletcher) +
        ") (miller " + showLong(miller) + ") (smith " + showLong(smith) + "))"

/** The requirement of the section: a false condition fails the attempt,
 * and the failure is one backtrack of the search. */
fun requireThat(condition: Boolean): Unit {
    if (!condition) {
        setPermanent { backtracks = backtracks + 1L }
    }
    demand(condition)
}

/** A first `Long` alternative: entering it consumes one choice, so the
 * count fires inside the alternative, before the value reaches the
 * caller downstream. */
fun firstLong(value: Long): Long {
    countChoice()
    return value
}

/** A later alternative of `anElementOf`: entering it counts as well, so
 * the count fires at the recursive branch entry, before the remaining
 * items can reach the empty-list failure that discards them. */
fun restElementOf(rest: List<Long>): Long {
    countChoice()
    return anElementOf(rest)
}

/** A later alternative of `anIntegerBetween`, counted at entry. */
fun restIntegerBetween(low: Long, high: Long): Long {
    countChoice()
    return anIntegerBetween(low + 1L, high)
}

/** A later alternative of the unbounded generator, counted at entry. */
fun restIntegerFrom(n: Long): Long {
    countChoice()
    return anIntegerStartingFrom(n + 1L)
}

/** A first `String` alternative, counted like the `Long` one. */
fun firstString(value: String): String {
    countChoice()
    return value
}

/** A later alternative of the string enumeration, counted at entry. */
fun restElementOfString(rest: List<String>): String {
    countChoice()
    return anElementOfString(rest)
}

fun countChoice(): Unit {
    setPermanent { choicesTaken = choicesTaken + 1L }
    if (choicesTaken == budgetCap + 1L) {
        println("choice budget exhausted after " + showLong(budgetCap) + " choices")
    }
    demand(choicesTaken <= budgetCap)
}

fun anElementOf(items: List<Long>): Long {
    requireThat(items.size > 0)
    return choose(firstLong(items.get(0)), restElementOf(items.drop(1)))
}

fun anIntegerBetween(low: Long, high: Long): Long {
    requireThat(low <= high)
    return choose(firstLong(low), restIntegerBetween(low, high))
}

fun anIntegerStartingFrom(n: Long): Long {
    return choose(firstLong(n), restIntegerFrom(n))
}

fun isEven(n: Long): Boolean = n % 2L == 0L

fun absLong(n: Long): Long = if (n < 0L) 0L - n else n

fun isDivides(a: Long, b: Long): Boolean = b % a == 0L

fun findDivisor(n: Long, test: Long): Long {
    if (test * test > n) {
        return n
    }
    if (isDivides(test, n)) {
        return test
    }
    return findDivisor(n, test + 1L)
}

fun smallestDivisor(n: Long): Long = findDivisor(n, 2L)

fun isPrime(n: Long): Boolean = n == smallestDivisor(n)

fun contains(items: List<Long>, value: Long): Boolean {
    var index = 0
    while (index < items.size) {
        if (items.get(index) == value) {
            return true
        }
        index = index + 1
    }
    return false
}

fun containsText(items: List<String>, value: String): Boolean {
    var index = 0
    while (index < items.size) {
        if (items.get(index) == value) {
            return true
        }
        index = index + 1
    }
    return false
}

fun anElementOfString(items: List<String>): String {
    requireThat(items.size > 0)
    return choose(firstString(items.get(0)), restElementOfString(items.drop(1)))
}

fun isDistinctText(items: List<String>): Boolean {
    var index = 0
    while (index < items.size) {
        if (containsText(items.drop(index + 1), items.get(index))) {
            return false
        }
        index = index + 1
    }
    return true
}

fun isDistinct(items: List<Long>): Boolean {
    var index = 0
    while (index < items.size) {
        if (contains(items.drop(index + 1), items.get(index))) {
            return false
        }
        index = index + 1
    }
    return true
}
    """.trimIndent()

/** The observation of one search run: the answer lines in search order. */
internal fun searchLines(source: String): List<String> =
    SearchModule.run(source).fold(
        { e -> listOf("Error: " + e.category) },
        { run ->
            run.result.output
                .lines()
                .filter { line -> line.isNotEmpty() }
        },
    )

/** One search run as data, for the counting exercises. */
internal fun searchRun(source: String): SearchRun = SearchModule.run(source).fold({ e -> throw AssertionError(e.toString()) }, { it })
