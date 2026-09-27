// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.77

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

public class E3_77Test :
    FunSpec({
        test("Exercise 3.77: a finite integrand ends the integral") {
            val integrand: LStream<Double> =
                consStream(1.0) { consStream(2.0) { consStream(3.0) { LStream.Empty } } }
            integralAlt(lazy { integrand }, 10.0, 0.5).take(5) shouldBe
                listOf(10.0, 10.5, 11.5, 13.0)
        }

        test("Exercise 3.77: the delayed integral drives solve to e at t = 1") {
            streamRef(solveAlt({ y -> y }, 1.0, 0.001), 1000) shouldBe 2.716923932235896
        }

        test("Exercise 3.77: solveAlt agrees with the prelude solve element for element") {
            solveAlt({ y -> y }, 1.0, 0.001).take(1001) shouldBe
                solve({ y -> y }, 1.0, 0.001).take(1001)
        }
    })
