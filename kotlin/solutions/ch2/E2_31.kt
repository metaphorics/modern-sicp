// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.31

package sicp.ch2.exercises

/** The book's `tree-map`: applies [f] to every leaf, keeping the shape. */
public fun treeMap(
    f: (Long) -> Long,
    t: Tree,
): Tree =
    when (t) {
        is Tree.Leaf -> Tree.Leaf(f(t.value))
        is Tree.Node -> Tree.Node(t.subtrees.map { treeMap(f, it) })
    }

/** `squareTree` defined as `treeMap` with a squaring function. */
public fun squareTreeViaTreeMap(t: Tree): Tree = treeMap({ x -> x * x }, t)

/** The book-syntax rendering of the `treeMap`-based `squareTree` result. */
public fun ex_2_31(): String = toBookString(squareTreeViaTreeMap(nestedTree()))
