// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.29

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.29: Simpson's Rule is a more accurate method of numerical
 * integration than the method of section 1.3.1. Using Simpson's Rule,
 * the integral of a function `f` between `a` and `b` is approximated as
 * `(h / 3)(y0 + 4y1 + 2y2 + 4y3 + ... + 2y_(n-2) + 4y_(n-1) + yn)`, where
 * `h = (b - a) / n` for some even integer `n`, and `y_k = f(a + kh)`.
 * Write a procedure that takes `f`, `a`, `b`, and `n` and returns the
 * value of the integral, computed using Simpson's Rule. Use your
 * procedure to integrate `cube` between 0 and 1 (with `n = 100` and
 * `n = 1000`), and compare the results to those of the `integral`
 * procedure of section 1.3.1. The statement lives in the section 1.3
 * chapter text.
 *
 * The scaffold returns Simpson's-Rule integrals of `cube` from 0 to 1
 * at `n = 100` and `n = 1000`.
 */
public fun ex_1_29(): Pair<Double, Double> = throw PendingSolution()
