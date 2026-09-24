// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.72

package sicp.ch2.exercises

/**
 * [encodeSymbol]'s exact algorithm, counting one step per element scanned
 * while testing `symbol in symbols(t.left)` at every branch visited.
 */
public fun encodeSymbolSteps(
    symbol: String,
    tree: HuffmanTree,
): Int {
    var steps = 0

    fun go(t: HuffmanTree) {
        if (t !is HuffmanTree.Branch) return
        val leftSymbols = symbols(t.left)
        steps += leftSymbols.size
        if (symbol in leftSymbols) go(t.left) else go(t.right)
    }
    go(tree)
    return steps
}

/**
 * On the skewed alphabet of exercise 2.71: the most frequent symbol sits
 * one level down, behind one membership check of size `n - 1`, so
 * encoding it costs `Θ(n)`. The least frequent symbol sits `n - 1` levels
 * down, behind membership checks of size `n - 1, n - 2, ..., 1`, so
 * encoding it costs `(n-1) + (n-2) + ... + 1 = n(n-1)/2`, `Θ(n²)`.
 */
public fun ex_2_72(n: Int): Pair<Int, Int> {
    val tree = generateHuffmanTree(skewedPairs(n))
    val mostFrequentSteps = encodeSymbolSteps("S${n - 1}", tree)
    val leastFrequentSteps = encodeSymbolSteps("S0", tree)
    return mostFrequentSteps to leastFrequentSteps
}
