// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.15

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_15Test :
    FunSpec({
        test("p is applied 5 times for sine(12.15)") {
            ex_1_15() shouldBe 5
        }
    })
