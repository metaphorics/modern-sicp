// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, shared tree representation for the section 2.2 exercises

package sicp.ch2.exercises

/**
 * A tree as exercises 2.27 to 2.31 and 2.35 see it: either a leaf holding
 * one number, or a node holding the list of its subtrees. The book writes
 * such trees as nested lists such as `(1 (2 (3 4) 5) (6 7))`; this sealed
 * hierarchy makes the two cases explicit so the compiler checks every
 * traversal.
 */
public sealed interface Tree {
    /** A single number at a leaf position. */
    public data class Leaf(
        val value: Long,
    ) : Tree

    /** A node whose subtrees are themselves trees. */
    public data class Node(
        val subtrees: List<Tree>,
    ) : Tree
}

/** Shorthand for a leaf: `leaf(3L)`. */
public fun leaf(value: Long): Tree = Tree.Leaf(value)

/** Shorthand for a node: `tree(leaf(1L), leaf(2L))` is the book's `(1 2)`. */
public fun tree(vararg subtrees: Tree): Tree = Tree.Node(subtrees.toList())
