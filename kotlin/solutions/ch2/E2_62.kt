// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.62

package sicp.ch2.exercises

public fun unionSetOrdered(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> {
    if (set1.isEmpty()) return set2
    if (set2.isEmpty()) return set1
    val x1 = set1[0]
    val x2 = set2[0]
    return when {
        x1 == x2 -> listOf(x1) + unionSetOrdered(set1.drop(1), set2.drop(1))
        x1 < x2 -> listOf(x1) + unionSetOrdered(set1.drop(1), set2)
        else -> listOf(x2) + unionSetOrdered(set1, set2.drop(1))
    }
}

public fun ex_2_62(): List<Long> = unionSetOrdered(listOf(1L, 3L, 6L, 10L), listOf(3L, 6L, 9L))
