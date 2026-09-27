// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43a

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.int
import io.kotest.property.checkAll

public class E1_43aTest :
    FunSpec({
        test("repeatedLog(square, 2)(5) is 625, section 1.43's own example") {
            ex_1_43a() shouldBe 625.0
        }
        test("repeatedLog(inc, n)(0) is n, for every generated positive n, odd or even") {
            checkAll(Arb.int(1..50)) { n ->
                repeatedLog({ x: Double -> x + 1.0 }, n.toLong())(0.0) shouldBe n.toDouble()
            }
        }
        test("repeatedLog agrees with the linear repeated of exercise 1.43 for every generated case") {
            checkAll(Arb.int(1..40), Arb.int(-20..20)) { n, x ->
                repeatedLog({ y: Long -> y + 3L }, n.toLong())(x.toLong()) shouldBe
                    repeated({ y: Long -> y + 3L }, n.toLong())(x.toLong())
            }
        }
    })
