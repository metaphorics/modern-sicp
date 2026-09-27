// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.10

package sicp.ch1.exercises

public fun ackermann(
    x: Long,
    y: Long,
): Long =
    when {
        y == 0L -> 0L
        x == 0L -> 2L * y
        y == 1L -> 2L
        else -> ackermann(x - 1L, ackermann(x, y - 1L))
    }

/** `ackermann(0, n) = 2n`. */
public fun ackermannF(n: Long): Long = ackermann(0L, n)

/** `ackermann(1, n) = 2^n` for `n >= 1`, with `ackermannG(0) = 0`. */
public fun ackermannG(n: Long): Long = ackermann(1L, n)

/** `ackermann(2, n)` is a tower of `n` 2's for `n >= 1`, with `ackermannH(0) = 0`. */
public fun ackermannH(n: Long): Long = ackermann(2L, n)

public fun ex_1_10(): List<Long> = listOf(ackermann(1L, 10L), ackermann(2L, 4L), ackermann(3L, 3L))
