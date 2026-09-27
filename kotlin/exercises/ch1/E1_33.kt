// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.33

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.33: an even more general version of `accumulate` (exercise
 * 1.32) introduces a `filter` on the terms to be combined: combine only
 * those terms derived from values in the range that satisfy a specified
 * predicate. `filteredAccumulate` takes the same arguments as
 * `accumulate`, together with an additional predicate of one argument.
 * Show how to express (a) the sum of the squares of the prime numbers in
 * the interval `a` to `b`, and (b) the product of all the positive
 * integers less than `n` that are relatively prime to `n` (all positive
 * integers `i < n` such that `gcd(i, n) = 1`). The statement lives in
 * the section 1.3 chapter text.
 *
 * The scaffold returns the sum of the squares of the primes from 2 to
 * 20, and the product of the positive integers below 10 relatively
 * prime to 10.
 */
public fun ex_1_33(): Pair<Long, Long> = throw PendingSolution()
