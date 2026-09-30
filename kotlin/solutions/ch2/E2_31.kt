// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.31

package sicp.ch2.exercises

/** Map [f] over every leaf while preserving the tree's shape. */
public fun treeMap(
    f: (Long) -> Long,
    t: Tree,
): Tree =
    when (t) {
        is Tree.Leaf -> Tree.Leaf(f(t.value))
        is Tree.Node -> Tree.Node(t.subtrees.map { treeMap(f, it) })
    }

/** Square every leaf by applying [treeMap] recursively. */
public fun squareTreeViaTreeMap(t: Tree): Tree = treeMap({ value -> value * value }, t)

/** Return the sample tree after each leaf value is squared. */
public fun ex_2_31(): Tree = squareTreeViaTreeMap(nestedTree())
