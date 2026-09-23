// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.int
import io.kotest.property.checkAll

public class E1_43Test :
    FunSpec({
        test("repeated(square, 2)(5) is 5 to the fourth power, 625") {
            ex_1_43() shouldBe 625.0
        }
        test("repeated(inc, n)(0) is n, for every generated positive n") {
            checkAll(Arb.int(1..30)) { n ->
                repeated({ x: Double -> x + 1.0 }, n.toLong())(0.0) shouldBe n.toDouble()
            }
        }
    })
