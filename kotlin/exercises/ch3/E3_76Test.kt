// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.76

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_76Test :
    FunSpec({
        test("Exercise 3.76: smooth averages successive elements").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            smooth(senseData).take(12) shouldBe
                listOf(1.5, 1.75, 1.25, 0.75, 0.2, -1.05, -2.5, -2.5, -1.25, -0.15, 1.6, 3.5)
        }

        test("Exercise 3.76: the smoothed signal crosses zero where the noise cancelled").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            smoothedZeroCrossings.take(12) shouldBe
                listOf(0L, 0L, 0L, 0L, 0L, -1L, 0L, 0L, 0L, 0L, 1L, 0L)
        }

        test("Exercise 3.76: the extractor component also reproduces the raw crossings").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            zeroCrossingsOf(senseData).take(20) shouldBe zeroCrossings.take(20)
        }
    })
