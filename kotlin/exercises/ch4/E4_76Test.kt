// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.76

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_76Test :
    FunSpec({
        test("Exercise 4.76: merge-and").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            mergeAndDemos() shouldBe listOf("MEASURE")
        }
    })
