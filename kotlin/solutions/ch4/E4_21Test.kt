// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.21: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_21Test :
    FunSpec({
        test("Exercise 4.21: factorial by self-application") {
            selfApplicationFactTranscript() shouldBe "3628800\n"
        }

        test("Exercise 4.21: Fibonacci by self-application") {
            selfApplicationFibTranscript() shouldBe "55\n"
        }

        test("Exercise 4.21: even and odd with no definition in force") {
            mutualEvenOddWithoutDefineTranscript() shouldBe "false\ntrue\n"
        }
    })
