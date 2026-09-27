// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.23

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.23: after `smallestDivisor` finds that a number is not
 * divisible by 2, checking further even numbers wastes work. Define a
 * `next` step that returns 3 given 2, and otherwise the input plus 2, so
 * the search tries 2, 3, 5, 7, 9, ... instead of 2, 3, 4, 5, 6, .... The
 * book asks whether this halves the elapsed time; that comparison is a
 * wall-clock measurement, discussed in the rationale rather than
 * asserted by a test. The statement lives in the section 1.2 chapter
 * text.
 *
 * The scaffold returns the smallest divisor of [n] found by the faster
 * search.
 */
public fun ex_1_23(n: Long): Long = throw PendingSolution()
