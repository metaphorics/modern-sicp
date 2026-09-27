// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_13

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_13Test :
    FunSpec({
        test("Exercise 5.13: the register list is derived from the controller text") {
            derivedRegisterRuns() shouldBe
                listOf(
                    "derived registers: a b t",
                    "gcd(206, 40) = 2",
                    "allocated registers: a b t",
                )
        }
    })
