// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.5

package sicp.ch1.exercises

/** Ben's procedure: applying it never returns. */
public fun p(): Nothing = p()

/** Ben's test, with the eager `Long` parameter the direct translation gives it. */
public fun test(
    x: Long,
    y: Long,
): Long = if (x == 0L) 0L else y

/** The deferred shape: the operand arrives as a lambda, run only if needed. */
public fun testDeferred(
    x: Long,
    y: () -> Long,
): Long = if (x == 0L) 0L else y()

/**
 * The probe: an eager operand is evaluated although the taken branch
 * ignores it; a lambda body is evaluated only when the body invokes it.
 * Returns (eager evaluations, deferred evaluations).
 */
public fun ex_1_05(): Pair<Int, Int> {
    var eagerEvaluations = 0

    fun operand(): Long {
        eagerEvaluations += 1
        return 7L
    }
    test(0L, operand())
    var deferredEvaluations = 0
    testDeferred(0L) {
        deferredEvaluations += 1
        7L
    }
    return eagerEvaluations to deferredEvaluations
}
