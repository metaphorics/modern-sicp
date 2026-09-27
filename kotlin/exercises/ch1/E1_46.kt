// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.46

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.46: several of the numerical methods of this chapter are
 * instances of iterative improvement: start with a guess, test if it is
 * good enough, and otherwise improve the guess and continue. Write a
 * procedure `iterativeImprove` that takes two procedures: a method for
 * telling whether a guess is good enough, and a method for improving a
 * guess. `iterativeImprove` returns a procedure that takes a guess and
 * keeps improving it until it is good enough. Rewrite the `sqrt`
 * procedure of section 1.1.7 and the `fixedPoint` procedure of section
 * 1.3.3 in terms of `iterativeImprove`. The statement lives in the
 * section 1.3 chapter text.
 *
 * The scaffold returns the square root of 9 via the rewritten `sqrt`,
 * and a fixed point of cosine via the rewritten `fixedPoint`.
 */
public fun ex_1_46(): Pair<Double, Double> = throw PendingSolution()
