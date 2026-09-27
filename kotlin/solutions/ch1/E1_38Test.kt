// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.38

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_38Test :
    FunSpec({
        test("20 terms of Euler's expansion matches kotlin.math.E to full double precision") {
            ex_1_38() shouldBe kotlin.math.E
        }
        test("eulerDenominator reproduces Euler's own sequence 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8") {
            (1L..11L).map(::eulerDenominator) shouldBe listOf(1.0, 2.0, 1.0, 1.0, 4.0, 1.0, 1.0, 6.0, 1.0, 1.0, 8.0)
        }
    })
