// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.10

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_10Test :
    FunSpec({
        test("the three named applications of the statement") {
            ex_1_10() shouldBe listOf(1024L, 65536L, 65536L)
        }
        test("f doubles, g is a power of two, h is a tower of powers of two") {
            (0L..5L).map { ackermannF(it) } shouldBe listOf(0L, 2L, 4L, 6L, 8L, 10L)
            (0L..5L).map { ackermannG(it) } shouldBe listOf(0L, 2L, 4L, 8L, 16L, 32L)
            (0L..4L).map { ackermannH(it) } shouldBe listOf(0L, 2L, 4L, 16L, 65536L)
        }
    })
