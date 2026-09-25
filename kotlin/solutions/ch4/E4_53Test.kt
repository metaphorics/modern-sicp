// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.53

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_53Test :
    FunSpec({
        test("Exercise 4.53: the accumulated pairs survive the final failure") {
            pairsResult() shouldBe "((8 35) (3 110) (3 20))"
        }
    })
