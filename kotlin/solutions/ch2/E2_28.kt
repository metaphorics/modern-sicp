// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.28

package sicp.ch2.exercises

/** The book's `fringe`: every leaf of [t], left to right. */
public fun fringe(t: Tree): List<Long> =
    when (t) {
        is Tree.Leaf -> listOf(t.value)
        is Tree.Node -> t.subtrees.flatMap(::fringe)
    }

/** `fringe` of the book's x, the four leaves `[1, 2, 3, 4]`. */
public fun ex_2_28(): List<Long> = fringe(bookPairTree())
