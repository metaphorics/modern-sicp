// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.68

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_68Test :
    FunSpec({
        test("Exercise 4.68: the reverse queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            reverseQueries() shouldBe listOf("MEASURE")
        }
    })
