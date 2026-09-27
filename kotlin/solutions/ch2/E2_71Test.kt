// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.71

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_71Test :
    FunSpec({
        test("n = 5: the most frequent symbol needs 1 bit, the least frequent needs 4") {
            ex_2_71(5) shouldBe (1 to 4)
        }
        test("n = 10: the most frequent symbol needs 1 bit, the least frequent needs 9") {
            ex_2_71(10) shouldBe (1 to 9)
        }
        test("the most frequent symbol always needs exactly 1 bit, for every n") {
            (3..12).forEach { n -> ex_2_71(n).first shouldBe 1 }
        }
        test("the least frequent symbol always needs exactly n - 1 bits, for every n") {
            (3..12).forEach { n -> ex_2_71(n).second shouldBe n - 1 }
        }
    })
