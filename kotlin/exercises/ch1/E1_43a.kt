// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43a

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.43a is added by this edition and extends exercise 1.43;
 * SICP numbers stop at 1.43. Exercise 1.43's `repeated(f, n)` composes
 * `f` with itself `n - 1` times, one `compose` call per step: `Theta(n)`
 * calls for `n`-fold repetition. Write `repeatedLog(f, n)` that
 * computes the same function by composing by squaring: halve `n` and
 * square the result whenever `n` is even, the same doubling trick
 * `fastExpt` used for exponentiation in section 1.2.4, so the number of
 * `compose` calls grows with `log2(n)` instead of `n`. Prove the two
 * procedures compute the same function with a Kotest property test.
 *
 * The scaffold returns `repeatedLog(square, 2)(5)`, section 1.43's own
 * example.
 */
public fun ex_1_43a(): Double = throw PendingSolution()
