// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.6

package sicp.ch1.exercises

import kotlin.math.abs

/**
 * Alyssa's `if` as an ordinary function: the two clause expressions are
 * eager arguments, evaluated before [newIf] runs.
 */
public fun <A> newIf(
    predicate: Boolean,
    thenClause: A,
    elseClause: A,
): A =
    when {
        predicate -> thenClause
        else -> elseClause
    }

private fun goodEnough(
    guess: Double,
    x: Double,
): Boolean = abs(guess * guess - x) < 0.001

private fun improve(
    guess: Double,
    x: Double,
): Double = (guess + x / guess) / 2.0

/** Alyssa's rewritten `sqrtIter`: the recursive call is an eager argument. */
public fun sqrtIterNewIf(
    guess: Double,
    x: Double,
): Double = newIf(goodEnough(guess, x), guess, sqrtIterNewIf(improve(guess, x), x))

/**
 * The evidence: one `newIf` call with a true predicate still evaluates its
 * else-clause expression. Returns (then evaluations, else evaluations).
 */
public fun ex_1_06(): Pair<Int, Int> {
    var thenEvaluations = 0
    var elseEvaluations = 0

    fun thenArm(): Long {
        thenEvaluations += 1
        return 0L
    }

    fun elseArm(): Long {
        elseEvaluations += 1
        return 5L
    }
    newIf(1L == 1L, thenArm(), elseArm())
    return thenEvaluations to elseEvaluations
}
