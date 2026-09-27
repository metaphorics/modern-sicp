// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.16

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe

public class E2_16Test :
    FunSpec({
        test("a - a is not the exact zero interval, even for the simplest repeated-variable expression") {
            val a = makeInterval(9.0, 11.0)
            subInterval(a, a) shouldNotBe makeInterval(0.0, 0.0)
        }
        test("ex_2_16 matches subInterval([9, 11], [9, 11]), which is [-2, 2]") {
            ex_2_16() shouldBe Interval(-2.0, 2.0)
        }
    })
