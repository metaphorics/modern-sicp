// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.36

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.36: modify `fixedPoint` so that it records each
 * approximation it generates, instead of returning only the final one.
 * Then find a solution to `x^x = 1000` by finding a fixed point of
 * `x -> log(1000) / log(x)` (use Kotlin's primitive `ln`). Compare the
 * number of steps this takes with and without average damping. (Note
 * that you cannot start `fixedPoint` with a guess of 1, as this would
 * cause division by `log(1) = 0`.) The statement lives in the section
 * 1.3 chapter text.
 *
 * The scaffold returns the number of steps `fixedPoint`, starting from
 * a guess of 2.0, takes without and with average damping.
 */
public fun ex_1_36(): Pair<Int, Int> = throw PendingSolution()
