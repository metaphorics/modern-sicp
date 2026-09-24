// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.60

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.60: design `elementOfSetDup`, `adjoinSetDup`, `unionSetDup`,
 * and `intersectionSetDup` for sets represented as lists that may contain
 * duplicates, such as `[2, 3, 2, 1, 3, 2, 2]` for `{1, 2, 3}`. The
 * rationale compares the order of growth of each against the
 * non-duplicate representation of `Sets23.kt`.
 *
 * The scaffold returns `unionSetDup` of `[2, 3, 2, 1, 3, 2, 2]` and `[1, 3]`.
 */
public fun elementOfSetDup(
    x: Long,
    set: List<Long>,
): Boolean = throw PendingSolution()

public fun adjoinSetDup(
    x: Long,
    set: List<Long>,
): List<Long> = throw PendingSolution()

public fun unionSetDup(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> = throw PendingSolution()

public fun intersectionSetDup(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> = throw PendingSolution()

public fun ex_2_60(): List<Long> = throw PendingSolution()
