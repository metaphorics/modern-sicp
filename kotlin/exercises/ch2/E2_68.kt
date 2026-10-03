// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.68

package sicp.ch2.exercises

/**
 * Exercise 2.68: write `encodeSymbol`, returning the bits that encode one
 * symbol according to a Huffman tree (`encode`, given below, calls it once
 * per message symbol). `encodeSymbol` must signal an error if the symbol
 * is not in the tree at all. Test it by encoding the result of exercise
 * 2.67 and checking it reproduces the original sample message.
 *
 * The scaffold returns `encode` of exercise 2.67's decoded message
 * against its own sample tree.
 */
public fun encodeSymbol(
    symbol: String,
    tree: HuffmanTree,
): List<Int> = throw PendingExercise()

public fun encode(
    message: List<String>,
    tree: HuffmanTree,
): List<Int> = message.flatMap { encodeSymbol(it, tree) }

public fun ex_2_68(): List<Int> = throw PendingExercise()
