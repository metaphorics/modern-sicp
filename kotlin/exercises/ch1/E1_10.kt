// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.10

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.10: Ackermann's function is `ackermann(x, y) = 0` if `y = 0`;
 * `2y` if `x = 0`; `2` if `y = 1`; otherwise
 * `ackermann(x - 1, ackermann(x, y - 1))`. What are the values of
 * `ackermann(1, 10)`, `ackermann(2, 4)`, and `ackermann(3, 3)`? Give
 * concise mathematical definitions for the functions computed by
 * `ackermannF(n) = ackermann(0, n)`, `ackermannG(n) = ackermann(1, n)`,
 * and `ackermannH(n) = ackermann(2, n)`, for positive integer `n`
 * (`ackermannK(n) = 5n^2` needs no call to `ackermann` to state). The
 * statement lives in the section 1.2 chapter text.
 *
 * The scaffold returns the three named values, in the order the
 * statement asks for them.
 */
public fun ex_1_10(): List<Long> = throw PendingSolution()
