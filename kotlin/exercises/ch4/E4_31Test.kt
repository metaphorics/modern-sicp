// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.31

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_31Test :
    FunSpec({
        test("Exercise 4.31: the declared mix counts 5").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            annotatedMixedTranscript() shouldBe "(1 5 5 4 30 30)\n5\n"
        }

        test("Exercise 4.31: all lazy-memo counts 4").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            annotatedAllMemoTranscript() shouldBe "(1 5 5 4 30 30)\n4\n"
        }

        test("Exercise 4.31: a lazy parameter skips its dangerous argument").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lazyParamSkipsTranscript() shouldBe "7\n"
        }

        test("Exercise 4.31: a strict parameter evaluates at the call").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            strictParamEagerTranscript() shouldBe "Error: division by zero\n"
        }
    })
