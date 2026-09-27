// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.3.3

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** Sets as unordered lists: no duplicates, in no particular order. */
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

/** Sets as ordered lists: elements listed in increasing order. */
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
 * Sets as binary trees: [SetTree.Node]'s own fields `entry`, `left`, and
 * `right` are the book's `entry`/`left-branch`/`right-branch` selectors.
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

public class S2_3_3RepresentingSetsTest :
    FunSpec({
        test("unordered elementOfSet/adjoinSet/intersectionSet") {
            val set = listOf(3L, 1L, 4L)
            elementOfSet(4L, set) shouldBe true
            elementOfSet(9L, set) shouldBe false
            adjoinSet(9L, set) shouldBe listOf(9L, 3L, 1L, 4L)
            adjoinSet(3L, set) shouldBe set
            intersectionSet(listOf(1L, 2L, 3L), listOf(2L, 3L, 4L)) shouldBe listOf(2L, 3L)
        }
        test("ordered elementOfSetOrdered/intersectionSetOrdered stop early past the target") {
            val ordered = listOf(1L, 3L, 6L, 10L)
            elementOfSetOrdered(6L, ordered) shouldBe true
            elementOfSetOrdered(5L, ordered) shouldBe false
            intersectionSetOrdered(listOf(1L, 3L, 6L, 10L), listOf(3L, 6L, 9L)) shouldBe listOf(3L, 6L)
        }
        test("tree elementOfSetTree/adjoinSetTree over a small balanced tree") {
            val tree = makeTree(7L, makeTree(3L, SetTree.Empty, SetTree.Empty), makeTree(9L, SetTree.Empty, SetTree.Empty))
            elementOfSetTree(3L, tree) shouldBe true
            elementOfSetTree(5L, tree) shouldBe false
            val grown = adjoinSetTree(5L, tree)
            elementOfSetTree(5L, grown) shouldBe true
            elementOfSetTree(3L, grown) shouldBe true
        }
    })
