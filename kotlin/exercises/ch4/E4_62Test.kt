// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.62

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_62Test :
    FunSpec({
        test("Exercise 4.62: the last-pair queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lastPairQueries() shouldBe listOf("MEASURE")
        }
    })
