// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.59

package sicp.ch2.exercises

public fun unionSet(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> =
    when {
        set1.isEmpty() -> set2
        elementOfSet(set1[0], set2) -> unionSet(set1.drop(1), set2)
        else -> listOf(set1[0]) + unionSet(set1.drop(1), set2)
    }

public fun ex_2_59(): List<Long> = unionSet(listOf(1L, 2L, 3L), listOf(2L, 3L, 4L))
