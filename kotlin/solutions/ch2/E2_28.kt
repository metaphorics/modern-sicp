// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.28

package sicp.ch2.exercises

/** Return each leaf of [tree], from left to right. */
public fun fringe(tree: Tree): List<Long> =
    when (tree) {
        is Tree.Leaf -> listOf(tree.value)
        is Tree.Node -> tree.subtrees.flatMap(::fringe)
    }

/** The left-to-right leaves of the shared two-branch sample tree. */
public fun ex_2_28(): List<Long> = fringe(twoBranchTree())
