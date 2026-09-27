// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.45

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.45: computing square roots by average-damped fixed-point
 * search works; a single average damp is not enough for fourth roots,
 * but damping twice makes the search for `y -> x / y^3` converge.
 * Experiment to find how many average damps a fixed-point search based
 * on repeated average damping of `y -> x / y^(n-1)` needs to compute
 * `n`-th roots, then implement a procedure for computing `n`-th roots
 * using `fixedPoint`, `averageDamp`, and `repeated` from exercise 1.43.
 * The statement lives in the section 1.3 chapter text.
 *
 * The scaffold returns the 4th root of 16 and the 8th root of 256, both
 * equal to 2.
 */
public fun ex_1_45(): Pair<Double, Double> = throw PendingSolution()
