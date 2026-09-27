// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.3.4

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * A Huffman tree: a leaf holding one symbol and its weight, or a branch
 * holding its two subtrees plus the aggregate symbol list and weight of
 * everything below it, computed once at [makeCodeTree]. `symbols` and
 * `weight` are simple examples of a generic procedure: each dispatches on
 * which variant it was handed, one kind of data.
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

public fun makeLeaf(
    sym: String,
    weight: Long,
): HuffmanTree = HuffmanTree.Leaf(sym, weight)

public fun symbols(t: HuffmanTree): List<String> =
    when (t) {
        is HuffmanTree.Leaf -> listOf(t.sym)
        is HuffmanTree.Branch -> t.syms
    }

public fun makeCodeTree(
    left: HuffmanTree,
    right: HuffmanTree,
): HuffmanTree = HuffmanTree.Branch(left, right, left.weight + right.weight, symbols(left) + symbols(right))

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

/** Restarts at the root of [tree] every time it reaches a leaf. */
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

/** Adjoins a tree to a set of trees ordered by increasing weight. */
public fun adjoinHuffmanSet(
    x: HuffmanTree,
    set: List<HuffmanTree>,
): List<HuffmanTree> =
    when {
        set.isEmpty() -> listOf(x)
        x.weight < set[0].weight -> listOf(x) + set
        else -> listOf(set[0]) + adjoinHuffmanSet(x, set.drop(1))
    }

public fun makeLeafSet(pairs: List<Pair<String, Long>>): List<HuffmanTree> =
    pairs.fold(emptyList()) { set, (sym, weight) -> adjoinHuffmanSet(makeLeaf(sym, weight), set) }

public class S2_3_4HuffmanTreesTest :
    FunSpec({
        test("the Figure 2.18 tree decodes BAC from 10001010") {
            val cd = makeCodeTree(makeLeaf("C", 1L), makeLeaf("D", 1L))
            val bcd = makeCodeTree(makeLeaf("B", 3L), cd)
            val ef = makeCodeTree(makeLeaf("E", 1L), makeLeaf("F", 1L))
            val gh = makeCodeTree(makeLeaf("G", 1L), makeLeaf("H", 1L))
            val efgh = makeCodeTree(ef, gh)
            val bcdefgh = makeCodeTree(bcd, efgh)
            val tree = makeCodeTree(makeLeaf("A", 8L), bcdefgh)
            tree.weight shouldBe 17L
            decode(listOf(1, 0, 0, 0, 1, 0, 1, 0), tree) shouldBe listOf("B", "A", "C")
        }
        test("makeLeafSet keeps the set ordered by increasing weight") {
            val set = makeLeafSet(listOf("A" to 4L, "B" to 2L, "C" to 1L, "D" to 1L))
            set.map { it.weight } shouldBe listOf(1L, 1L, 2L, 4L)
        }
    })
