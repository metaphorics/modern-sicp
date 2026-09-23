// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.5

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_05Test :
    FunSpec({
        test("the eager parameter evaluates, the lambda parameter does not") {
            ex_1_05() shouldBe (1 to 0)
        }
        test("the deferred call returns 0 with a diverging operand in the slot") {
            testDeferred(0L) { p() } shouldBe 0L
        }
    })
