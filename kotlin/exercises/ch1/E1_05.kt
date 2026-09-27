// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.5

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.5, re-cut by this edition as an eager-parameter versus
 * lambda-parameter demonstration: Kotlin always evaluates the arguments of
 * a call, so the eager `Long` parameter of Ben's `test` evaluates its
 * operand first, while a `() -> Long` parameter defers the operand until
 * the body invokes it. The statement lives in the section 1.1 chapter text.
 *
 * The scaffold runs the probe with a counting operand and returns the pair
 * (eager evaluations, deferred evaluations) for the two calls of the
 * statement.
 */
public fun ex_1_05(): Pair<Int, Int> = throw PendingSolution()
