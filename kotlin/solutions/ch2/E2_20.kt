// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.20

package sicp.ch2.exercises

/**
 * The book's `same-parity`, over a vararg: [first] is the named first
 * parameter and [rest] collects every remaining argument, Kotlin's
 * replacement for Scheme's dotted-tail notation.
 */
public fun sameParity(
    first: Long,
    vararg rest: Long,
): List<Long> {
    val even = first % 2L == 0L
    return listOf(first) + rest.filter { (it % 2L == 0L) == even }
}

/** `sameParity(1, 2, 3, 4, 5, 6, 7)` keeps the odd ones. */
public fun ex_2_20(): List<Long> = sameParity(1L, 2L, 3L, 4L, 5L, 6L, 7L)
