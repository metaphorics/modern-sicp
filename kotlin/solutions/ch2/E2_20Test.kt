// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.20

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_20Test :
    FunSpec({
        test("sameParity(1, 2, 3, 4, 5, 6, 7) is (1 3 5 7)") {
            ex_2_20() shouldBe listOf(1L, 3L, 5L, 7L)
        }
        test("sameParity(2, 3, 4, 5, 6, 7) is (2 4 6)") {
            sameParity(2L, 3L, 4L, 5L, 6L, 7L) shouldBe listOf(2L, 4L, 6L)
        }
        test("a lone argument returns just itself") {
            sameParity(5L) shouldBe listOf(5L)
        }
    })
