// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt

public class E3_20Test :
    FunSpec({
        test("the replaced session: x.car() answers 17 after alias.setCar") {
            val x = proceduralCons(VInt(1L), VInt(2L))
            val alias = x

            alias.setCar(VInt(17L))

            x.car() shouldBe VInt(17L)
            x.cdr() shouldBe VInt(2L)
        }

        test("a fresh pair has its own captured slots, untouched by x's mutation") {
            val x = proceduralCons(VInt(1L), VInt(2L))
            x.setCar(VInt(17L))

            val fresh = proceduralCons(VInt(1L), VInt(2L))

            fresh.car() shouldBe VInt(1L)
            fresh.cdr() shouldBe VInt(2L)
            (fresh === x) shouldBe false
        }
    })
