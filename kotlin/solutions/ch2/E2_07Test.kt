// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.7

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_07Test :
    FunSpec({
        test("lowerBound and upperBound recover the two arguments makeInterval was given") {
            val i = makeInterval(6.12, 7.48)
            i.lowerBound shouldBe 6.12
            i.upperBound shouldBe 7.48
        }
        test("ex_2_07 matches makeInterval(6.12, 7.48)") {
            ex_2_07() shouldBe Interval(6.12, 7.48)
        }
    })
