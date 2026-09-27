// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.16

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.16: explain, in general, why equivalent algebraic expressions
 * may lead to different answers. Can you devise an interval-arithmetic
 * package that does not have this shortcoming, or is this task impossible?
 * (Warning: this problem is very difficult.) `makeInterval` and
 * `subInterval` are exercise 2.7's and exercise 2.8's public declarations,
 * reused here from the same package. The statement lives in the section 2.1
 * chapter text.
 *
 * The scaffold subtracts the interval `[9, 11]` from itself with
 * `subInterval`, the simplest possible repeated-variable expression: `a -
 * a`, which is exactly zero for every real number `a` could be.
 */
public fun ex_2_16(): Interval = throw PendingSolution()
