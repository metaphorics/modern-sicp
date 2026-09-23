// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.44

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.44: the idea of smoothing a function is an important
 * concept in signal processing. If `f` is a function and `dx` is some
 * small number, then the smoothed version of `f` is the function whose
 * value at `x` is the average of `f(x - dx)`, `f(x)`, and `f(x + dx)`.
 * Write a procedure `smooth` that takes a procedure computing `f` and
 * returns a procedure computing the smoothed `f`. It is sometimes
 * valuable to repeatedly smooth a function (smooth the smoothed
 * function, and so on) to obtain the n-fold smoothed function. Show how
 * to generate the n-fold smoothed function of any given function using
 * `smooth` and `repeated` from exercise 1.43. The statement lives in the
 * section 1.3 chapter text.
 *
 * The scaffold returns `smooth(square)(2.0)` and the 5-fold smoothed
 * `square` applied to 2.0.
 */
public fun ex_1_44(): Pair<Double, Double> = throw PendingSolution()
