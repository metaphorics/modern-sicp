// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.60

package sicp.ch2.exercises

/** Θ(n): unchanged from the non-duplicate representation. */
public fun elementOfSetDup(
    x: Long,
    set: List<Long>,
): Boolean =
    when {
        set.isEmpty() -> false
        x == set[0] -> true
        else -> elementOfSetDup(x, set.drop(1))
    }

/** Θ(1): no scan is needed, since duplicates are allowed. */
public fun adjoinSetDup(
    x: Long,
    set: List<Long>,
): List<Long> = listOf(x) + set

/** Θ(n): a plain concatenation, since neither set needs filtering to keep the result valid. */
public fun unionSetDup(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> = set1 + set2

/** Θ(n²): the same recursive shape as the non-duplicate version; duplicates in `set1` simply recur into the result. */
public fun intersectionSetDup(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> =
    when {
        set1.isEmpty() || set2.isEmpty() -> emptyList()
        elementOfSetDup(set1[0], set2) -> listOf(set1[0]) + intersectionSetDup(set1.drop(1), set2)
        else -> intersectionSetDup(set1.drop(1), set2)
    }

/** `unionSetDup` of `[2, 3, 2, 1, 3, 2, 2]` (the book's `{1, 2, 3}`) and `[1, 3]`. */
public fun ex_2_60(): List<Long> = unionSetDup(listOf(2L, 3L, 2L, 1L, 3L, 2L, 2L), listOf(1L, 3L))
