// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.13

package sicp.ch2.exercises

/**
 * Exercise 2.13: show that, under the assumption of small percentage
 * tolerances, there is a simple formula for the approximate percentage
 * tolerance of the product of two intervals in terms of the tolerances of
 * the factors. You may simplify the problem by assuming that all numbers
 * are positive. `makeCenterPercent` and `percent` are exercise 2.12's
 * public declarations; `mulInterval` is exercise 2.7's; all are reused here
 * from the same package. The statement lives in the section 2.1 chapter
 * text.
 *
 * The scaffold returns the exact percentage tolerance of a product,
 * `makeCenterPercent(100.0, 0.01) * makeCenterPercent(50.0, 0.02)`, paired
 * with the approximate formula's prediction, `0.01 + 0.02`.
 */
public fun ex_2_13(): Pair<Double, Double> = throw PendingExercise()
