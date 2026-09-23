// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.3

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_03Test :
    FunSpec({
        test("both representations of a 4-by-3 rectangle agree on perimeter and area") {
            ex_2_03() shouldBe (14.0 to 12.0)
        }
        test("perimeter and area take only a width and a height, not a representation") {
            perimeter(4.0, 3.0) shouldBe 14.0
            area(4.0, 3.0) shouldBe 12.0
        }
    })
