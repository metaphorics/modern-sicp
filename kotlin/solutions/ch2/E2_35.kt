// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.35

package sicp.ch2.exercises

/**
 * The book's `count-leaves` as an accumulation: `fringe` of Exercise
 * 2.28 plays the role of `enumerate-tree`, so the answer is one per leaf,
 * summed from the right.
 */
public fun countLeavesAccumulate(t: Tree): Long = accumulateList({ _, acc -> 1L + acc }, 0L, fringe(t))

/** `countLeavesAccumulate` of the book's `(1 (2 (3 4)))`, which has four leaves. */
public fun ex_2_35(): Long = countLeavesAccumulate(tree(leaf(1L), tree(leaf(2L), leaf(3L), leaf(4L))))
