// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.64

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.64 (parts a and b): `listToTree` converts an ordered list to
 * a balanced binary tree via the helper `partialTree`, which takes a list
 * of at least `n` elements and returns a pair of the tree built from the
 * first `n` elements and the elements not included. Part a) asks for a
 * paragraph explaining `partialTree` and the tree it draws for
 * `[1, 3, 5, 7, 9, 11]`; part b) asks for the order of growth. The
 * rationale answers both parts.
 *
 * The scaffold returns `listToTree` of `[1, 3, 5, 7, 9, 11]`.
 */
public fun partialTree(
    elts: List<Long>,
    n: Int,
): Pair<SetTree, List<Long>> = throw PendingSolution()

public fun listToTree(elements: List<Long>): SetTree = throw PendingSolution()

public fun ex_2_64(): SetTree = throw PendingSolution()
