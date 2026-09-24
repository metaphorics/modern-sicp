// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.63

package sicp.ch2.exercises

/** `Θ(n log n)` on a balanced tree: `+` on `List` copies its receiver, mirroring the book's `append`. */
public fun treeToList1(tree: SetTree): List<Long> =
    when (tree) {
        is SetTree.Empty -> emptyList()
        is SetTree.Node -> treeToList1(tree.left) + listOf(tree.entry) + treeToList1(tree.right)
    }

/**
 * `Θ(n)` on any tree: threads one mutable [ArrayDeque] through the whole
 * walk, `addFirst` standing in for the book's O(1) `cons`, so no
 * intermediate list is ever copied.
 */
public fun treeToList2(tree: SetTree): List<Long> {
    fun copyToList(
        tree: SetTree,
        result: ArrayDeque<Long>,
    ): ArrayDeque<Long> {
        if (tree is SetTree.Empty) return result
        check(tree is SetTree.Node)
        val fromRight = copyToList(tree.right, result)
        fromRight.addFirst(tree.entry)
        return copyToList(tree.left, fromRight)
    }
    return copyToList(tree, ArrayDeque()).toList()
}

private val figureTree1: SetTree =
    makeTree(
        7L,
        makeTree(3L, makeTree(1L, SetTree.Empty, SetTree.Empty), makeTree(5L, SetTree.Empty, SetTree.Empty)),
        makeTree(9L, SetTree.Empty, makeTree(11L, SetTree.Empty, SetTree.Empty)),
    )

private val figureTree2: SetTree =
    makeTree(
        3L,
        makeTree(1L, SetTree.Empty, SetTree.Empty),
        makeTree(
            7L,
            makeTree(5L, SetTree.Empty, SetTree.Empty),
            makeTree(9L, SetTree.Empty, makeTree(11L, SetTree.Empty, SetTree.Empty)),
        ),
    )

private val figureTree3: SetTree =
    makeTree(
        5L,
        makeTree(3L, makeTree(1L, SetTree.Empty, SetTree.Empty), SetTree.Empty),
        makeTree(9L, makeTree(7L, SetTree.Empty, SetTree.Empty), makeTree(11L, SetTree.Empty, SetTree.Empty)),
    )

/** [treeToList1] and [treeToList2] of the three Figure 2.16 trees, six lists in total. */
public fun ex_2_63(): List<List<Long>> = listOf(figureTree1, figureTree2, figureTree3).flatMap { listOf(treeToList1(it), treeToList2(it)) }
