// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.72

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_72Test :
    FunSpec({
        test("Exercise 4.72: interleave versus append").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            interleaveVersusAppend() shouldBe listOf("MEASURE")
        }
    })
