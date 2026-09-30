// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4a

package sicp.ch2.exercises

/**
 * Exercise 2.4a is added by this edition and extends exercise 2.4; SICP
 * numbers stop at 2.4. Exercise 2.4 checks the identity law
 * `carFn(consFn(x, y)) == x` and `cdrFn(consFn(x, y)) == y` for one pair,
 * `(3, 4)`, by direct substitution. Write a Kotest property test that
 * checks the same law for every generated pair of `Long` values, not just
 * one, using `checkAll`.
 *
 * The scaffold returns whether the law holds for the fixed pair `(3, 4)`;
 * the real property test lives in the solution's test file.
 */
public fun ex_2_04a(): Boolean = throw PendingExercise()
