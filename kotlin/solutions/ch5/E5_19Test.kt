// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_19

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_19Test :
    FunSpec({
        test("Exercise 5.19: the machine parks before each n-th execution, proceed resumes, cancel releases") {
            breakpointSession() shouldBe
                listOf(
                    "break at test-b: a = 40, b = 6",
                    "break at test-b: a = 4, b = 2",
                    "finished: gcd(206, 40) = 2",
                    "cancel and restart: gcd(206, 40) = 2",
                )
        }
    })
