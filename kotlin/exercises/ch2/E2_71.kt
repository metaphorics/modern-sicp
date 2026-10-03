// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.71

package sicp.ch2.exercises

/**
 * Exercise 2.71: for an alphabet of `n` symbols with relative frequencies
 * `1, 2, 4, ..., 2^(n-1)`, sketch the Huffman tree for `n = 5` and
 * `n = 10`, and state how many bits the most and the least frequent
 * symbol need.
 *
 * The scaffold returns `(mostFrequentBits, leastFrequentBits)` for the
 * given `n`.
 */
public fun skewedPairs(n: Int): List<Pair<String, Long>> = (0 until n).map { i -> "S$i" to (1L shl i) }

public fun ex_2_71(n: Int): Pair<Int, Int> = throw PendingExercise()
