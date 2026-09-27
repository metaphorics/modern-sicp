// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.66

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_66Test :
    FunSpec({
        test("Exercise 4.66: the salary sums").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            salarySums() shouldBe listOf("MEASURE")
        }
    })
