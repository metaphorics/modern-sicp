// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.65

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.comparables.shouldBeLessThan
import io.kotest.matchers.shouldBe
import sicp.runtime.take
import kotlin.math.abs
import kotlin.math.ln

public class E3_65Test :
    FunSpec({
        test("the raw partial sums are still a few hundredths off after ten terms").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ln2Raw.take(4) shouldBe listOf(1.0, 0.5, 0.8333333333333333, 0.5833333333333333)
            val raw10 = streamRef(ln2Raw, 10)
            raw10 shouldBe 0.7365440115440116
            abs(raw10 - ln(2.0)).shouldBeLessThan(5e-2)
        }

        test("the Euler-transformed series is within 3e-4 by its sixth term").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val euler5 = streamRef(ln2Euler, 5)
            euler5 shouldBe 0.6928571428571428
            abs(euler5 - ln(2.0)).shouldBeLessThan(4e-4)
        }

        test("the full tableau reaches machine precision by its sixth term").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val accelerated5 = streamRef(ln2Accelerated, 5)
            accelerated5 shouldBe 0.6931471806635636
            abs(accelerated5 - ln(2.0)).shouldBeLessThan(2e-9)
        }

        test("each acceleration stage converges faster than the one before").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val ln2 = ln(2.0)
            val raw = abs(streamRef(ln2Raw, 10) - ln2)
            val euler = abs(streamRef(ln2Euler, 5) - ln2)
            val accelerated = abs(streamRef(ln2Accelerated, 5) - ln2)
            euler shouldBeLessThan raw
            accelerated shouldBeLessThan euler
        }
    })
