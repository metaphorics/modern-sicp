// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.1

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.1: An accumulator is a function that is called repeatedly
 * with a single numeric argument and accumulates its arguments into a
 * sum. Each time it is called, it returns the currently accumulated sum.
 * Write a function `makeAccumulator` that generates accumulators, each
 * maintaining an independent sum. The input to `makeAccumulator`
 * specifies the initial value of the sum; for example, if `a` is
 * `makeAccumulator(5)`, then `a(10)` is 15 and a further `a(10)` is 25.
 *
 * The scaffold always returns the start value, ignoring every call.
 */
public fun makeAccumulator(start: Long): (Long) -> Long = throw PendingSolution()
