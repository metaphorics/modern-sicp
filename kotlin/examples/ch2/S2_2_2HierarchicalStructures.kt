// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.2.2

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * A tree: either a leaf holding a number, or a node holding the list of
 * its subtrees. The book writes trees as nested lists, such as `(1 (2 3)
 * 4)`, where an element that is itself a list is a subtree and any other
 * element is a leaf; this sealed hierarchy makes that case split
 * explicit for the compiler.
 */
public sealed interface Tree {
    public data class Leaf(
        val value: Long,
    ) : Tree

    public data class Node(
        val subtrees: List<Tree>,
    ) : Tree
}

public fun leaf(value: Long): Tree = Tree.Leaf(value)

public fun tree(vararg subtrees: Tree): Tree = Tree.Node(subtrees.toList())

/**
 * The book's `count-leaves`: as `length` reduces over the pairs of a
 * flat list, `count-leaves` reduces over the branches of a tree. A
 * [Tree.Leaf] contributes 1; a [Tree.Node] contributes the sum over its
 * subtrees.
 */
public fun countLeaves(t: Tree): Long =
    when (t) {
        is Tree.Leaf -> 1L
        is Tree.Node -> t.subtrees.sumOf(::countLeaves)
    }

/**
 * The book's `scale-tree`: multiplies every leaf by `factor`, recursing
 * through every node exactly as [countLeaves] does.
 */
public fun scaleTree(
    t: Tree,
    factor: Long,
): Tree =
    when (t) {
        is Tree.Leaf -> Tree.Leaf(t.value * factor)
        is Tree.Node -> Tree.Node(t.subtrees.map { scaleTree(it, factor) })
    }

/** `scale-tree` redefined with `map`, the book's next step: `tree.subtrees.map` in place of hand recursion. */
public fun scaleTreeViaMap(
    t: Tree,
    factor: Long,
): Tree =
    when (t) {
        is Tree.Leaf -> Tree.Leaf(t.value * factor)
        is Tree.Node -> Tree.Node(t.subtrees.map { scaleTreeViaMap(it, factor) })
    }

public class S2_2_2HierarchicalStructuresTest :
    FunSpec({
        test("countLeaves counts every leaf, the book's tree of 1 (2 (3 4)) x has 4 leaves") {
            val x = tree(leaf(1L), tree(leaf(2L), leaf(3L), leaf(4L)))
            countLeaves(x) shouldBe 4L
        }
        test("count-leaves of (cons x x) is twice count-leaves of x") {
            val x = tree(leaf(1L), tree(leaf(2L), leaf(3L), leaf(4L)))
            countLeaves(tree(x, x)) shouldBe 2L * countLeaves(x)
        }
        test("scaleTree multiplies every leaf, the book's (1 (2 (3 4) 5) (6 7)) scaled by 10") {
            val input = tree(leaf(1L), tree(leaf(2L), tree(leaf(3L), leaf(4L)), leaf(5L)), tree(leaf(6L), leaf(7L)))
            val expected = tree(leaf(10L), tree(leaf(20L), tree(leaf(30L), leaf(40L)), leaf(50L)), tree(leaf(60L), leaf(70L)))
            scaleTree(input, 10L) shouldBe expected
        }
        test("scaleTreeViaMap agrees with the hand-recursive scaleTree") {
            val input = tree(leaf(1L), tree(leaf(2L), tree(leaf(3L), leaf(4L)), leaf(5L)), tree(leaf(6L), leaf(7L)))
            scaleTreeViaMap(input, 10L) shouldBe scaleTree(input, 10L)
        }
    })
