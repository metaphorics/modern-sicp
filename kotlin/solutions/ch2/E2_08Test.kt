// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.8

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_08Test :
    FunSpec({
        test("subInterval takes the smallest possible and largest possible difference") {
            subInterval(makeInterval(6.0, 8.0), makeInterval(3.0, 5.0)) shouldBe Interval(1.0, 5.0)
        }
        test("subtracting an interval from itself is symmetric around zero") {
            val a = makeInterval(9.0, 11.0)
            subInterval(a, a) shouldBe Interval(-2.0, 2.0)
        }
        test("ex_2_08 matches [6, 8] minus [3, 5]") {
            ex_2_08() shouldBe Interval(1.0, 5.0)
        }
    })
