// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.64

package sicp.ch2.exercises

/**
 * Builds a balanced tree from the first `n` elements of `elts`, returning
 * that tree paired with the elements not included. Splits off `leftSize`
 * elements for the left subtree, takes the next element as this node's
 * entry, and gives everything remaining to the right subtree.
 */
public fun partialTree(
    elts: List<Long>,
    n: Int,
): Pair<SetTree, List<Long>> {
    if (n == 0) return SetTree.Empty to elts
    val leftSize = (n - 1) / 2
    val (leftTree, nonLeftElts) = partialTree(elts, leftSize)
    val rightSize = n - (leftSize + 1)
    val thisEntry = nonLeftElts[0]
    val (rightTree, remainingElts) = partialTree(nonLeftElts.drop(1), rightSize)
    return makeTree(thisEntry, leftTree, rightTree) to remainingElts
}

public fun listToTree(elements: List<Long>): SetTree = partialTree(elements, elements.size).first

public fun ex_2_64(): SetTree = listToTree(listOf(1L, 3L, 5L, 7L, 9L, 11L))
