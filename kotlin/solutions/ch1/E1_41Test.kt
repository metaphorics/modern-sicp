// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.41

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

public class E1_41Test :
    FunSpec({
        test("double(double(double))(inc)(5) is 21: inc applied sixteen times") {
            ex_1_41() shouldBe 21L
        }
        test("double applies its argument exactly twice, for every generated function and input") {
            checkAll(Arb.long(-100L, 100L), Arb.long(1L, 20L)) { x, n ->
                double<Long> { y -> y + n }(x) shouldBe x + 2L * n
            }
        }
    })
