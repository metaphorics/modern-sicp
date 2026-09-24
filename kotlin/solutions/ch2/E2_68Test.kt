// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.68

package sicp.ch2.exercises

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_68Test :
    FunSpec({
        test("encodeSymbol of each leaf reproduces its own path from the root") {
            encodeSymbol("A", sampleTree) shouldBe listOf(0)
            encodeSymbol("B", sampleTree) shouldBe listOf(1, 0)
            encodeSymbol("D", sampleTree) shouldBe listOf(1, 1, 0)
            encodeSymbol("C", sampleTree) shouldBe listOf(1, 1, 1)
        }
        test("encodeSymbol signals an error for a symbol absent from the tree") {
            shouldThrow<IllegalArgumentException> { encodeSymbol("Z", sampleTree) }
        }
        test("encode of the decoded sample message reproduces the original bits") {
            encode(listOf("A", "D", "A", "B", "B", "C", "A"), sampleTree) shouldBe sampleMessage
        }
        test("ex_2_68 round-trips exercise 2.67's decode back to sampleMessage") {
            ex_2_68() shouldBe sampleMessage
        }
    })
