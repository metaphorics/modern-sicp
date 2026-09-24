// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.39

package sicp.ch2.exercises

/** The book's first definition: `fold-right`, appending each head after the reversed rest. */
public fun reverseRight(sequence: List<Long>): List<Long> = foldRight({ x, acc -> acc + listOf(x) }, emptyList(), sequence)

/** The book's second definition: `fold-left`, consing each head onto the accumulated answer. */
public fun reverseLeft(sequence: List<Long>): List<Long> = foldLeft({ acc, x -> listOf(x) + acc }, emptyList(), sequence)

/** `reverseLeft` of `(1 4 9 16 25)`, the book's `(25 16 9 4 1)`. */
public fun ex_2_39(): List<Long> = reverseLeft(listOf(1L, 4L, 9L, 16L, 25L))
