// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19a

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.19a is added by this edition and extends exercise 1.19;
 * SICP numbers stop at 1.19. Probe the overflow boundary of the
 * fixed-width host: reimplement the logarithmic Fibonacci transform over
 * checked `Long` arithmetic (`Math.addExact`/`Math.multiplyExact`)
 * instead of `BigInteger`, and find the exact `n` past which it throws,
 * even though `Fib(92)` itself still fits in a `Long`. The statement
 * lives in the section 1.2 chapter text.
 *
 * The scaffold returns `Fib(n)` computed by the checked-`Long` transform,
 * throwing `ArithmeticException` once the transform's own bookkeeping
 * overflows.
 */
public fun ex_1_19a(n: Long): Long = throw PendingSolution()
