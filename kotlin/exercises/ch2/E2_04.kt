// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4

package sicp.ch2.exercises

/**
 * Exercise 2.4: here is an alternative procedural representation of pairs.
 * For this representation, verify that `carFn(consFn(x, y))` yields `x` for
 * any objects `x` and `y`:
 *
 * ```kotlin
 * fun <T> consFn(x: T, y: T): ((T, T) -> T) -> T = { m -> m(x, y) }
 * fun <T> carFn(z: ((T, T) -> T) -> T): T = z { p, _ -> p }
 * ```
 *
 * What is the corresponding definition of `cdrFn`? (Hint: to verify that
 * this works, make use of the substitution model of section 1.1.5.) The
 * statement lives in the section 2.1 chapter text.
 *
 * The scaffold returns `cdrFn(consFn(3L, 4L))`.
 */
public fun ex_2_04(): Long = throw PendingExercise()
