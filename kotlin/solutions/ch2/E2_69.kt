// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.69

package sicp.ch2.exercises

/** Repeatedly merges the two smallest-weight elements until one tree remains. */
public fun successiveMerge(set: List<HuffmanTree>): HuffmanTree =
    when (set.size) {
        1 -> set[0]
        else -> successiveMerge(adjoinHuffmanSet(makeCodeTree(set[0], set[1]), set.drop(2)))
    }

public fun generateHuffmanTree(pairs: List<Pair<String, Long>>): HuffmanTree = successiveMerge(makeLeafSet(pairs))

/** The Huffman tree for the book's A-through-H alphabet. */
public fun ex_2_69(): HuffmanTree =
    generateHuffmanTree(
        listOf("A" to 8L, "B" to 3L, "C" to 1L, "D" to 1L, "E" to 1L, "F" to 1L, "G" to 1L, "H" to 1L),
    )
