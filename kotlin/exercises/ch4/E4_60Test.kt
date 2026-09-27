// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.60

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_60Test :
    FunSpec({
        test("Exercise 4.60: the lives-near queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            livesNearQueries() shouldBe listOf("MEASURE")
        }
    })
