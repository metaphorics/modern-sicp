// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.28

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.28: the Miller-Rabin test starts from the alternate form of
 * Fermat's Little Theorem: if `n` is prime and `a` is any positive
 * integer less than `n`, then `a` raised to the `(n - 1)`-st power is
 * congruent to 1 modulo `n`. Modify `expmod` to signal (by returning 0)
 * when it discovers a nontrivial square root of 1 modulo `n` during a
 * squaring step, and use this to test primality; check the procedure on
 * known primes and on the Carmichael numbers of exercise 1.27, which the
 * plain Fermat test cannot tell apart from primes. The statement lives in
 * the section 1.2 chapter text.
 *
 * The scaffold classifies the six Carmichael numbers with a fixed seed
 * and 30 trials each, and returns whether each one passes.
 */
public fun ex_1_28(): Map<Long, Boolean> = throw PendingSolution()
