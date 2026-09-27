// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.74

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_74Test :
    FunSpec({
        test("Exercise 4.74: the simple flatmap").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            simpleFlatmapDemo() shouldBe listOf("MEASURE")
        }
    })
