// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.69

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_69Test :
    FunSpec({
        test("ex_2_69's tree carries the total weight of all eight symbols") {
            ex_2_69().weight shouldBe 17L
        }
        test("ex_2_69's tree reaches every symbol of the alphabet exactly once") {
            symbols(ex_2_69()).toSet() shouldBe setOf("A", "B", "C", "D", "E", "F", "G", "H")
        }
        test("encoding then decoding a message built from the generated tree round-trips") {
            val tree = ex_2_69()
            val message = listOf("B", "A", "C")
            decode(encode(message, tree), tree) shouldBe message
        }
        test("successiveMerge of a single tree returns it unchanged") {
            val leaf = makeLeaf("X", 1L)
            successiveMerge(listOf(leaf)) shouldBe leaf
        }
    })
