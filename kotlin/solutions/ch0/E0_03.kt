// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

/**
 * The binary tree of exercise 0.3, as a sealed hierarchy: a [Tree] is a
 * [Leaf] or a [Node], and nothing else.
 */
public sealed interface Tree

public data class Leaf(
    val value: Long,
) : Tree

public data class Node(
    val left: Tree,
    val right: Tree,
) : Tree

/**
 * The exhaustive `when` has no `else`: adding a variant to [Tree] must
 * break this function at compile time, which is part two of the exercise.
 */
public fun depth(t: Tree): Int =
    when (t) {
        is Leaf -> 1
        is Node -> 1 + maxOf(depth(t.left), depth(t.right))
    }
