// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.67

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_67Test :
    FunSpec({
        test("sampleTree's total weight is the sum of its four leaves") {
            sampleTree.weight shouldBe 8L
        }
        test("ex_2_67 decodes the sample message to A D A B B C A") {
            ex_2_67() shouldBe listOf("A", "D", "A", "B", "B", "C", "A")
        }
    })
