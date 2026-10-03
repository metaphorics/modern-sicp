// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_08Test :
    FunSpec({
        test("Exercise 4.8: the book's named-let fibonacci runs").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            namedLetFibonacciTranscript() shouldBe "55\n"
        }

        test("Exercise 4.8: the loop name stays local to the wrapper frame").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            loopNameLocalTranscript() shouldBe "1\n7\n"
        }

        test("Exercise 4.8: plain let still evaluates").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            plainLetTranscript() shouldBe "3\n"
        }
    })
