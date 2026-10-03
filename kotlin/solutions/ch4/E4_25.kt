// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.25

package sicp.ch4.solutions

import sicp.ch4.Direct
import sicp.ch4.LazyModule

// Exercise 4.25: `unless` under delayed arguments. In the lazy experiment a
// compound procedure's unannotated parameters are delayed, so `unless`'s
// three operands reach it as memoized thunks: the condition is forced only
// when the body's `if` demands it and the unchosen arm is never forced.
// The `unless`-based `factorial` therefore bottoms out at 1 and the product
// climbs back up: `factorial(5)` answers 120. In an applicative-order run
// every operand evaluates before `unless` is ever entered, which the armed
// call shows directly -- `1 / 0` raises before `42` is even reached -- and
// which dooms the recursion: `n * factorial(n - 1)` evaluates on every
// entry, so the descent never reaches the guard. The engine has no step
// budget, so the probe carries an honest entry budget of its own: under
// strict evaluation the descent exhausts it and the run reports the budget
// instead of hanging the suite.

/** The statement's definitions and recursion, carrying the entry budget.
 * The lazy run answers 120; the strict run exhausts the budget. */
internal val UNLESS_FACTORIAL_PROGRAM: String =
    """
var entries: Long = 0L

fun unless(condition: Boolean, usual: Long, exceptional: Long): Long = if (condition) exceptional else usual

fun factorial(n: Long): Long {
    entries = entries + 1L
    if (entries > 200L) {
        return -1L
    }
    return unless(n == 1L, n * factorial(n - 1L), 1L)
}

fun main() {
    val answer = factorial(5L)
    if (entries > 200L) {
        println("entry budget exhausted")
    } else {
        println(answer)
    }
}
    """.trimIndent()

/** The armed call: the operands evaluate before `unless` is entered, so the
 * exceptional arm's division raises first. */
internal val ARMED_UNLESS_PROGRAM: String =
    """
fun unless(condition: Boolean, usual: Long, exceptional: Long): Long = if (condition) exceptional else usual

fun main() {
    println(unless(1L == 1L, 1L / 0L, 42L))
}
    """.trimIndent()

/** The recursion under delayed arguments bottoms out and answers.
 * => "120\n" */
public fun lazyFactorialTranscript(): String = outcomeText(LazyModule.run(UNLESS_FACTORIAL_PROGRAM).map { it.result })

/** The armed call under applicative order: `1 / 0` raises before `42` is
 * reached. => "Error: DivisionByZero\n" */
public fun strictArmedUnlessTranscript(): String = outcomeText(Direct.run(ARMED_UNLESS_PROGRAM))

/** The same recursion under applicative order and the entry budget: the
 * arms evaluate on every entry, so the descent never reaches the guard.
 * => "entry budget exhausted\n" */
public fun strictFactorialTranscript(): String = outcomeText(Direct.run(UNLESS_FACTORIAL_PROGRAM))

/** The strict armed run as data: the typed fault category, or null. */
internal fun strictArmedCategory(): String? = Direct.run(ARMED_UNLESS_PROGRAM).fold({ null }, { it.error?.category })
