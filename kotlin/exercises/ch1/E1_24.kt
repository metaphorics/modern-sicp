// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.24

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.24, adapted for the shared seeded generator of decision
 * 0001 in place of an unspecified `random`: modify `timedPrimeTest` to
 * use `fastPrime` (the Fermat method) and classify the 12 primes found in
 * exercise 1.22. Since the Fermat test has Theta(log n) growth, the book
 * asks how the timing near 1,000,000 should compare with timing near
 * 1000; that comparison is a wall-clock measurement, discussed in the
 * rationale rather than asserted by a test. The statement lives in the
 * section 1.2 chapter text.
 *
 * The scaffold classifies the 12 primes found in exercise 1.22 with a
 * fixed seed, and returns whether every one of them passes.
 */
public fun ex_1_24(): List<Boolean> = throw PendingSolution()
