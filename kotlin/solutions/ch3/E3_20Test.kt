// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Whole

public class E3_20Test :
    FunSpec({
        test("mutation through an alias reaches the shared closure slots") {
            val x = proceduralPair(Whole(1L), Whole(2L))
            val alias = x

            alias.setFirst(Whole(17L))

            x.first() shouldBe Whole(17L)
            x.second() shouldBe Whole(2L)
        }

        test("a fresh pair has its own captured slots") {
            val x = proceduralPair(Whole(1L), Whole(2L))
            x.setFirst(Whole(17L))

            val fresh = proceduralPair(Whole(1L), Whole(2L))

            fresh.first() shouldBe Whole(1L)
            fresh.second() shouldBe Whole(2L)
            (fresh === x) shouldBe false
        }
    })
