// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.30

package sicp.ch2.exercises

/** The book's `square-tree`, direct: square every leaf, recurse through every node. */
public fun squareTree(t: Tree): Tree =
    when (t) {
        is Tree.Leaf -> Tree.Leaf(t.value * t.value)
        is Tree.Node -> Tree.Node(t.subtrees.map(::squareTree))
    }

/** The book's second `square-tree`: map over the subtrees, recursing on the nodes. */
public fun squareTreeViaMap(t: Tree): Tree =
    when (t) {
        is Tree.Leaf -> {
            Tree.Leaf(t.value * t.value)
        }

        is Tree.Node -> {
            Tree.Node(
                t.subtrees.map { subtree ->
                    when (subtree) {
                        is Tree.Leaf -> Tree.Leaf(subtree.value * subtree.value)
                        is Tree.Node -> squareTreeViaMap(subtree)
                    }
                },
            )
        }
    }

/** The book's tree: `(1 (2 (3 4) 5) (6 7))`. */
public fun nestedTree(): Tree = tree(leaf(1L), tree(leaf(2L), tree(leaf(3L), leaf(4L)), leaf(5L)), tree(leaf(6L), leaf(7L)))

/** The book-syntax rendering of the direct `squareTree` result. */
public fun ex_2_30(): String = toBookString(squareTree(nestedTree()))
