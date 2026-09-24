// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.68

package sicp.ch2.exercises

/**
 * At each branch, checks membership in the left branch's symbol set to
 * decide which way to go, matching the cost `encodeSymbolSteps` (exercise
 * 2.72) instruments. Signals with [IllegalArgumentException] if `symbol`
 * is not in the tree at all.
 */
public fun encodeSymbol(
    symbol: String,
    tree: HuffmanTree,
): List<Int> {
    require(symbol in symbols(tree)) { "symbol not found in tree: $symbol" }

    fun go(t: HuffmanTree): List<Int> =
        when (t) {
            is HuffmanTree.Leaf -> emptyList()
            is HuffmanTree.Branch -> if (symbol in symbols(t.left)) listOf(0) + go(t.left) else listOf(1) + go(t.right)
        }
    return go(tree)
}

public fun encode(
    message: List<String>,
    tree: HuffmanTree,
): List<Int> = message.flatMap { encodeSymbol(it, tree) }

/** `encode` of exercise 2.67's decoded message against its own sample tree: reproduces `sampleMessage`. */
public fun ex_2_68(): List<Int> = encode(ex_2_67(), sampleTree)
