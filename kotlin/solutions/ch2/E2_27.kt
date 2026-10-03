// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.27

package sicp.ch2.exercises

/** Reverse only the direct children of each node. */
public fun reverseTree(tree: Tree): Tree =
    when (tree) {
        is Tree.Leaf -> tree
        is Tree.Node -> Tree.Node(tree.subtrees.asReversed())
    }

/** Reverse child order recursively at every node. */
public fun deepReverse(tree: Tree): Tree =
    when (tree) {
        is Tree.Leaf -> tree
        is Tree.Node -> Tree.Node(tree.subtrees.asReversed().map(::deepReverse))
    }

/** The deep-reversed result for the two-branch sample tree. */
public fun ex_2_27(): Tree = deepReverse(twoBranchTree())

/** A root with two two-leaf subtrees. */
public fun twoBranchTree(): Tree = tree(tree(leaf(1L), leaf(2L)), tree(leaf(3L), leaf(4L)))
