// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.70

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_70Test :
    FunSpec({
        test("the rock-song lyrics carry 36 symbols") {
            rockSongMessage.size shouldBe 36
        }
        test("decoding the Huffman-encoded lyrics reproduces the original message") {
            val tree = generateHuffmanTree(rockSongAlphabet)
            val bits = encode(rockSongMessage, tree)
            decode(bits, tree) shouldBe rockSongMessage
        }
        test("ex_2_70 needs 84 Huffman bits against 108 fixed-length bits, a real saving") {
            val (huffmanBits, fixedLengthBits) = ex_2_70()
            huffmanBits shouldBe 84
            fixedLengthBits shouldBe 108
            (huffmanBits < fixedLengthBits) shouldBe true
        }
    })
