// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.39

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.39: a continued fraction representation of the tangent
 * function was published in 1770 by the German mathematician J.H.
 * Lambert: `tan(x) = x / (1 - x^2 / (3 - x^2 / (5 - ...)))`, where `x`
 * is in radians. Define a procedure `tanCf(x, k)` that computes an
 * approximation to the tangent function based on Lambert's formula.
 * `k` specifies the number of terms to compute, as in exercise 1.37.
 * The statement lives in the section 1.3 chapter text.
 *
 * The scaffold returns `tanCf(pi/4, 10)`, an approximation to
 * `tan(pi/4) = 1`.
 */
public fun ex_1_39(): Double = throw PendingSolution()
