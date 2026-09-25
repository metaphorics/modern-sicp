// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.72

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.72: in a similar way to 3.71, generate a stream of all
 * numbers that can be written as the sum of two squares in three
 * different ways, showing how they can be so written. Each element pairs
 * the number with the list of pairs `(i, j)` whose `i^2 + j^2` equals
 * it. The first is 325 = 1^2 + 18^2 = 6^2 + 17^2 = 10^2 + 15^2; the
 * next five are 425, 650, 725, 845, and 850.
 */
public fun threeSquareNumbers(): LStream<Pair<Long, List<Pair<Long, Long>>>> = throw PendingSolution()
