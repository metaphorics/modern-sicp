// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.61

package sicp.ch2.exercises

public fun adjoinSetOrdered(
    x: Long,
    set: List<Long>,
): List<Long> =
    when {
        set.isEmpty() -> listOf(x)
        x == set[0] -> set
        x < set[0] -> listOf(x) + set
        else -> listOf(set[0]) + adjoinSetOrdered(x, set.drop(1))
    }

public fun ex_2_61(): List<Long> = adjoinSetOrdered(4L, listOf(1L, 3L, 6L, 10L))
