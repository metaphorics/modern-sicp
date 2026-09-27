// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.61

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_61Test :
    FunSpec({
        test("Exercise 4.61: the next-to queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            nextToQueries() shouldBe listOf("MEASURE")
        }
    })
