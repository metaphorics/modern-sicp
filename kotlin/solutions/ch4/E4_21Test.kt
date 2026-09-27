// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.21

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_21Test :
    FunSpec({
        test("Exercise 4.21: the book's self-application expression computes 10!") {
            selfApplicationFactTranscript() shouldBe "3628800\n"
        }

        test("Exercise 4.21: the same trick drives fib 10") {
            selfApplicationFibTranscript() shouldBe "55\n"
        }

        test("Exercise 4.21: the filled blanks answer (f 7) and (f 10)") {
            mutualEvenOddWithoutDefineTranscript() shouldBe "#f\n#t\n"
        }
    })
