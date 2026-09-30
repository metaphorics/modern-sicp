// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.32

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_32Test :
    FunSpec({
        test("Exercise 5.32: the fast path runs the session and the base factorial cost is reported").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            symbolOperatorRuns().last() shouldBe "the two runs answer alike: true"
        }
    })
