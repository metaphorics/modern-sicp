// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.32

package sicp.ch2.exercises

/**
 * The book's `subsets` with the missing expressions filled in: the
 * subsets of the rest stay, and adjoining the head to each of them adds
 * every subset that contains it.
 */
public fun subsets(s: List<Long>): List<List<Long>> =
    if (s.isEmpty()) {
        listOf(emptyList())
    } else {
        val rest = subsets(s.drop(1))
        rest + rest.map { subset -> listOf(s.first()) + subset }
    }

/** `subsets` of `(1 2 3)`, the book's eight subsets in generation order. */
public fun ex_2_32(): List<List<Long>> = subsets(listOf(1L, 2L, 3L))
