// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.13

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.int
import io.kotest.property.checkAll

public class E1_13Test :
    FunSpec({
        test("the closed form gives Fib(10)") {
            ex_1_13(10) shouldBe 55L
        }
        test("the closed form matches the direct definition for every n up to 40") {
            checkAll(Arb.int(0..40)) { n ->
                closedFormFib(n) shouldBe fibDirect(n)
            }
        }
    })
