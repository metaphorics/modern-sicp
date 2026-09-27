// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.15

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.15: the sine of an angle can be computed using the
 * approximation `sin(x) ~ x` for small `x`, and the identity
 * `sin(x) = 3 sin(x / 3) - 4 sin(x / 3)^3` otherwise, where "sufficiently
 * small" means a magnitude no greater than 0.1 radians. (a) How many
 * times is `p` applied when `sine(12.15)` is evaluated? (b) What is the
 * order of growth in space and number of steps, as a function of the
 * angle, used by the process `sine` generates? Part (b) is carried in the
 * rationale; the statement lives in the section 1.2 chapter text.
 *
 * The scaffold answers part (a): the count of `p` applications.
 */
public fun ex_1_15(): Int = throw PendingSolution()
