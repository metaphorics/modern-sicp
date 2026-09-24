// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.67

package sicp.ch2.exercises

/** The book's sample encoding tree. */
public val sampleTree: HuffmanTree =
    makeCodeTree(
        makeLeaf("A", 4L),
        makeCodeTree(makeLeaf("B", 2L), makeCodeTree(makeLeaf("D", 1L), makeLeaf("C", 1L))),
    )

/** The book's sample message, encoded against [sampleTree]. */
public val sampleMessage: List<Int> = listOf(0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0)

/** `decode` of [sampleMessage] against [sampleTree]: `[A, D, A, B, B, C, A]`. */
public fun ex_2_67(): List<String> = decode(sampleMessage, sampleTree)
