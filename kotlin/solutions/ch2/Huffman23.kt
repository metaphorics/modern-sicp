// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, shared Huffman-tree representation for the section 2.3.4 exercises

package sicp.ch2.exercises

/**
 * A Huffman tree: a leaf holding one symbol and its weight, or a branch
 * holding its two subtrees plus the aggregate symbol list and weight of
 * everything below it, computed once at [makeCodeTree] rather than walked
 * on every query. This closes the book's four-item list-of-lists shape
 * (`make-code-tree`, `left-branch`, `right-branch`, `symbols`, `weight`)
 * into one sealed hierarchy.
 */
public sealed interface HuffmanTree {
    public val weight: Long

    public data class Leaf(
        val sym: String,
        override val weight: Long,
    ) : HuffmanTree

    public data class Branch(
        val left: HuffmanTree,
        val right: HuffmanTree,
        override val weight: Long,
        val syms: List<String>,
    ) : HuffmanTree
}

/** The book's `make-leaf`. */
public fun makeLeaf(
    sym: String,
    weight: Long,
): HuffmanTree = HuffmanTree.Leaf(sym, weight)

/** The symbols reachable below [t]: a leaf contributes its one symbol; a branch its precomputed list. */
public fun symbols(t: HuffmanTree): List<String> =
    when (t) {
        is HuffmanTree.Leaf -> listOf(t.sym)
        is HuffmanTree.Branch -> t.syms
    }

/** The book's `make-code-tree`: merges two subtrees, computing the aggregate symbols and weight once. */
public fun makeCodeTree(
    left: HuffmanTree,
    right: HuffmanTree,
): HuffmanTree = HuffmanTree.Branch(left, right, left.weight + right.weight, symbols(left) + symbols(right))

/** The book's `choose-branch`: `0` takes the left branch, `1` the right. */
public fun chooseBranch(
    bit: Int,
    branch: HuffmanTree,
): HuffmanTree =
    when {
        branch !is HuffmanTree.Branch -> throw IllegalArgumentException("choose-branch of a leaf")
        bit == 0 -> branch.left
        bit == 1 -> branch.right
        else -> throw IllegalArgumentException("bad bit: $bit")
    }

/** The book's `decode`: walks [bits] down [tree], restarting at the root after every leaf. */
public fun decode(
    bits: List<Int>,
    tree: HuffmanTree,
): List<String> {
    val result = mutableListOf<String>()
    var current = tree
    for (bit in bits) {
        val next = chooseBranch(bit, current)
        if (next is HuffmanTree.Leaf) {
            result.add(next.sym)
            current = tree
        } else {
            current = next
        }
    }
    return result
}

/**
 * The book's `adjoin-set`, redefined for a set of trees ordered by
 * increasing weight, as the tree-generating algorithm of 2.3.4 needs.
 */
public fun adjoinHuffmanSet(
    x: HuffmanTree,
    set: List<HuffmanTree>,
): List<HuffmanTree> =
    when {
        set.isEmpty() -> listOf(x)
        x.weight < set[0].weight -> listOf(x) + set
        else -> listOf(set[0]) + adjoinHuffmanSet(x, set.drop(1))
    }

/** The book's `make-leaf-set`: builds an ordered set of leaves from symbol/weight pairs. */
public fun makeLeafSet(pairs: List<Pair<String, Long>>): List<HuffmanTree> =
    pairs.fold(emptyList()) { set, (sym, weight) -> adjoinHuffmanSet(makeLeaf(sym, weight), set) }
