// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, shared tree representation for the section 2.2 exercises

package sicp.ch2.exercises

/**
 * A tree is either a leaf holding one number or a node holding an ordered
 * list of subtrees. The sealed hierarchy makes both cases explicit and lets
 * the compiler check every traversal.
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

/** Construct a node from its ordered child trees. */
public fun tree(vararg subtrees: Tree): Tree = Tree.Node(subtrees.toList())
