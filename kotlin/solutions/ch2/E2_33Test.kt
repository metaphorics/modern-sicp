// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.33

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_33Test :
    FunSpec({
        test("mapViaAccumulate squares (1 2 3 4) to (1 4 9 16)") {
            ex_2_33() shouldBe listOf(1L, 4L, 9L, 16L)
        }
        test("filterViaAccumulate keeps the odd elements") {
            filterViaAccumulate({ x -> x % 2L != 0L }, listOf(1L, 2L, 3L, 4L, 5L)) shouldBe listOf(1L, 3L, 5L)
        }
        test("appendViaAccumulate glues squares onto odds") {
            appendViaAccumulate(listOf(1L, 4L, 9L, 16L, 25L), listOf(1L, 3L, 5L, 7L)) shouldBe
                listOf(1L, 4L, 9L, 16L, 25L, 1L, 3L, 5L, 7L)
        }
        test("lengthViaAccumulate counts (1 2 3 4 5)") {
            lengthViaAccumulate(listOf(1L, 2L, 3L, 4L, 5L)) shouldBe 5L
        }
    })
