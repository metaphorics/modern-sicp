// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.69

package sicp.ch2.exercises

/**
 * Exercise 2.69: write `successiveMerge`, which uses `makeCodeTree` to
 * successively merge the smallest-weight elements of an ordered set of
 * trees until one element remains, the desired Huffman tree.
 * `generateHuffmanTree` (given below) builds the initial ordered leaf set
 * with `makeLeafSet` (`Huffman23.kt`) and reduces it with
 * `successiveMerge`.
 *
 * The scaffold returns the Huffman tree for the book's A-through-H
 * alphabet (`A 8, B 3, C 1, D 1, E 1, F 1, G 1, H 1`).
 */
public fun successiveMerge(set: List<HuffmanTree>): HuffmanTree = throw PendingExercise()

public fun generateHuffmanTree(pairs: List<Pair<String, Long>>): HuffmanTree = successiveMerge(makeLeafSet(pairs))

public fun ex_2_69(): HuffmanTree = throw PendingExercise()
