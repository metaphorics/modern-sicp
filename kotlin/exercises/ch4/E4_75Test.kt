// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.75

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_75Test :
    FunSpec({
        test("Exercise 4.75: the unique special form").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            uniqueDemos() shouldBe listOf("MEASURE")
        }
    })
