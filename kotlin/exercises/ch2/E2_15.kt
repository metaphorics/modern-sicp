// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.15

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.15: Eva Lu Ator, another user, has also noticed that the
 * different intervals computed by algebraically equivalent expressions may
 * be different. She says that a formula to compute with intervals using
 * Alyssa's system will produce tighter error bounds if it can be written in
 * such a form that no variable that represents an uncertain number is
 * repeated. Thus, she says, `par2` (exercise 2.14) is a "better" program
 * for parallel resistances than `par1`. Is she right? Why? `makeInterval`
 * is exercise 2.7's; `percent` is exercise 2.12's; `par1` and `par2` are
 * exercise 2.14's; all are reused here from the same package. The statement
 * lives in the section 2.1 chapter text.
 *
 * The scaffold returns the percentage tolerance of `par1(r1, r2)` paired
 * with `par2(r1, r2)`, for the resistors 6.12 to 7.48 ohms and 4.465 to
 * 4.935 ohms of section 2.1.4's own example.
 */
public fun ex_2_15(): Pair<Double, Double> = throw PendingSolution()
