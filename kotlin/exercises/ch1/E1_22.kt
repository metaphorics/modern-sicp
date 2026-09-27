// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.22

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.22, adapted for a host whose clock is `System.nanoTime`
 * rather than a Lisp `runtime` primitive: write a `searchForPrimes`
 * procedure that checks the primality of consecutive odd integers in a
 * range, and use it to find the three smallest primes larger than 1000,
 * 10,000, 100,000, and 1,000,000. (Since testing has order of growth
 * Theta(sqrt n), the book asks whether timing near 10,000 takes about
 * sqrt(10) times as long as timing near 1000, and similarly for the
 * larger ranges; that comparison is a wall-clock measurement, discussed
 * in the rationale rather than asserted by a test.) The statement lives
 * in the section 1.2 chapter text.
 *
 * The scaffold returns a map from each starting threshold to its three
 * smallest primes above it.
 */
public fun ex_1_22(): Map<Long, List<Long>> = throw PendingSolution()
