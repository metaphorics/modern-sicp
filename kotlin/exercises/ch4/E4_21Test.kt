// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.21

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_21Test :
    FunSpec({
        test("Exercise 4.21: the book's self-application expression computes 10!").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            selfApplicationFactTranscript() shouldBe "3628800\n"
        }

        test("Exercise 4.21: the same trick drives fib 10").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            selfApplicationFibTranscript() shouldBe "55\n"
        }

        test("Exercise 4.21: the filled blanks answer (f 7) and (f 10)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            mutualEvenOddWithoutDefineTranscript() shouldBe "false\ntrue\n"
        }
    })
