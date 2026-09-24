// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, shared set representations for the section 2.3.3 exercises

package sicp.ch2.exercises

/**
 * Sets as unordered lists (2.3.3): a `List<Long>` with no duplicates. The
 * empty set is the empty list. `elementOfSet`/`adjoinSet`/`intersectionSet`
 * are the book's given procedures; `unionSet` (2.59) and the
 * duplicate-allowed variants (2.60) are exercises.
 */
public fun elementOfSet(
    x: Long,
    set: List<Long>,
): Boolean =
    when {
        set.isEmpty() -> false
        x == set[0] -> true
        else -> elementOfSet(x, set.drop(1))
    }

public fun adjoinSet(
    x: Long,
    set: List<Long>,
): List<Long> = if (elementOfSet(x, set)) set else listOf(x) + set

public fun intersectionSet(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> =
    when {
        set1.isEmpty() || set2.isEmpty() -> emptyList()
        elementOfSet(set1[0], set2) -> listOf(set1[0]) + intersectionSet(set1.drop(1), set2)
        else -> intersectionSet(set1.drop(1), set2)
    }

/**
 * Sets as ordered lists: elements listed in increasing order.
 * `elementOfSetOrdered` and `intersectionSetOrdered` are the book's given
 * procedures; `adjoinSetOrdered` (2.61) and `unionSetOrdered` (2.62) are
 * exercises.
 */
public fun elementOfSetOrdered(
    x: Long,
    set: List<Long>,
): Boolean =
    when {
        set.isEmpty() -> false
        x == set[0] -> true
        x < set[0] -> false
        else -> elementOfSetOrdered(x, set.drop(1))
    }

public fun intersectionSetOrdered(
    set1: List<Long>,
    set2: List<Long>,
): List<Long> {
    if (set1.isEmpty() || set2.isEmpty()) return emptyList()
    val x1 = set1[0]
    val x2 = set2[0]
    return when {
        x1 == x2 -> listOf(x1) + intersectionSetOrdered(set1.drop(1), set2.drop(1))
        x1 < x2 -> intersectionSetOrdered(set1.drop(1), set2)
        else -> intersectionSetOrdered(set1, set2.drop(1))
    }
}

/**
 * Sets as binary trees: each node holds one entry, a left branch of
 * smaller entries, and a right branch of larger ones. `entry`, `left`,
 * and `right` are [SetTree.Node]'s own fields, the book's `entry`,
 * `left-branch`, and `right-branch` selectors; `makeTree` is the book's
 * `make-tree`. `elementOfSetTree` and `adjoinSetTree` are given;
 * `treeToList1`/`treeToList2` (2.63), `listToTree` (2.64), and
 * `unionSetTree`/`intersectionSetTree` (2.65) are exercises.
 */
public sealed interface SetTree {
    public data object Empty : SetTree

    public data class Node(
        val entry: Long,
        val left: SetTree,
        val right: SetTree,
    ) : SetTree
}

public fun makeTree(
    entry: Long,
    left: SetTree,
    right: SetTree,
): SetTree = SetTree.Node(entry, left, right)

public fun elementOfSetTree(
    x: Long,
    tree: SetTree,
): Boolean =
    when (tree) {
        is SetTree.Empty -> {
            false
        }

        is SetTree.Node -> {
            when {
                x == tree.entry -> true
                x < tree.entry -> elementOfSetTree(x, tree.left)
                else -> elementOfSetTree(x, tree.right)
            }
        }
    }

public fun adjoinSetTree(
    x: Long,
    tree: SetTree,
): SetTree =
    when (tree) {
        is SetTree.Empty -> {
            makeTree(x, SetTree.Empty, SetTree.Empty)
        }

        is SetTree.Node -> {
            when {
                x == tree.entry -> tree
                x < tree.entry -> makeTree(tree.entry, adjoinSetTree(x, tree.left), tree.right)
                else -> makeTree(tree.entry, tree.left, adjoinSetTree(x, tree.right))
            }
        }
    }
