// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.30

package sicp.ch2.exercises

/** Square each leaf value by recursively visiting every child tree. */
public fun squareTree(t: Tree): Tree =
    when (t) {
        is Tree.Leaf -> Tree.Leaf(t.value * t.value)
        is Tree.Node -> Tree.Node(t.subtrees.map(::squareTree))
    }

/** Apply the same leaf transformation through a map at each internal node. */
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

/** A nested sample tree with two internal branches. */
public fun nestedTree(): Tree = tree(leaf(1L), tree(leaf(2L), tree(leaf(3L), leaf(4L)), leaf(5L)), tree(leaf(6L), leaf(7L)))

/** Return the tree after squaring every leaf value. */
public fun ex_2_30(): Tree = squareTree(nestedTree())
