// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.20

package sicp.ch2.exercises

/** Use the named first value and remaining vararg values to select one parity. */
public fun sameParity(
    first: Long,
    vararg rest: Long,
): List<Long> {
    val even = first % 2L == 0L
    return listOf(first) + rest.filter { (it % 2L == 0L) == even }
}

/** `sameParity(1, 2, 3, 4, 5, 6, 7)` keeps the odd ones. */
public fun ex_2_20(): List<Long> = sameParity(1L, 2L, 3L, 4L, 5L, 6L, 7L)
