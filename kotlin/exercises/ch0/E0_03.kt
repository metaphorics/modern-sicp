// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import sicp.runtime.PendingSolution

/**
 * The binary tree of exercise 0.3, as a sealed hierarchy: a [Tree] is a
 * [Leaf] or a [Node], and nothing else.
 *
 * Section 0.4.
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
 * Exercise 0.3, part one: the depth of [t] in levels. A [Leaf] has depth 1;
 * a [Node] has one more than its deeper child. Part two of the exercise is
 * a compile experiment: add a third variant and watch the non-exhaustive
 * `when` fail compilation. The statement lives in the section 0.4 text.
 */
public fun depth(t: Tree): Int = throw PendingSolution()
