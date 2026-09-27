// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.70

package sicp.ch2.exercises

import kotlin.math.ceil
import kotlin.math.ln

/** The book's eight-symbol rock-song alphabet and its relative frequencies. */
public val rockSongAlphabet: List<Pair<String, Long>> =
    listOf("A" to 2L, "BOOM" to 1L, "GET" to 2L, "JOB" to 2L, "NA" to 16L, "SHA" to 3L, "WAH" to 1L, "YIP" to 9L)

/** The song's lyrics, one symbol per word: two verses of "Get a job / Sha na (x8)", then "Wah yip (x9) / Sha boom". */
public val rockSongMessage: List<String> =
    (0 until 2).flatMap { listOf("GET", "A", "JOB", "SHA") + List(8) { "NA" } } +
        listOf("WAH") + List(9) { "YIP" } +
        listOf("SHA", "BOOM")

/** `(huffmanBits, fixedLengthBits)` for encoding [rockSongMessage] with [rockSongAlphabet]. */
public fun ex_2_70(): Pair<Int, Int> {
    val tree = generateHuffmanTree(rockSongAlphabet)
    val huffmanBits = encode(rockSongMessage, tree).size
    val fixedWidth = ceil(ln(rockSongAlphabet.size.toDouble()) / ln(2.0)).toInt()
    val fixedLengthBits = rockSongMessage.size * fixedWidth
    return huffmanBits to fixedLengthBits
}
