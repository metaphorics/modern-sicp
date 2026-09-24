// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.71

package sicp.ch2.exercises

/** The alphabet of `n` symbols with relative frequencies `1, 2, 4, ..., 2^(n-1)`. */
public fun skewedPairs(n: Int): List<Pair<String, Long>> = (0 until n).map { i -> "S$i" to (1L shl i) }

/**
 * For the skewed alphabet, the most frequent symbol (`S(n-1)`, weight
 * `2^(n-1)`) always merges last, landing at depth 1; the least frequent
 * (`S0`, weight 1) is nested `n - 1` levels deep, tied with `S1` at the
 * very bottom. `(mostFrequentBits, leastFrequentBits)` is therefore
 * `(1, n - 1)`, confirmed here by actually building the tree and
 * encoding both symbols rather than asserting the closed form directly.
 */
public fun ex_2_71(n: Int): Pair<Int, Int> {
    val tree = generateHuffmanTree(skewedPairs(n))
    val mostFrequentBits = encodeSymbol("S${n - 1}", tree).size
    val leastFrequentBits = encodeSymbol("S0", tree).size
    return mostFrequentBits to leastFrequentBits
}
