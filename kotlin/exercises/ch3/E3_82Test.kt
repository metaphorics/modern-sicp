// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.82

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.comparables.shouldBeLessThan
import io.kotest.matchers.shouldBe
import sicp.runtime.take
import kotlin.math.PI
import kotlin.math.abs

public class E3_82Test :
    FunSpec({
        test("Exercise 3.82: pinned estimates after 100 and 1000 experiments").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val quarterCircle = estimateIntegral({ x, y -> x * x + y * y <= 1.0 }, 0.0, 1.0, 0.0, 1.0)
            streamRef(quarterCircle, 99) shouldBe 0.77
            streamRef(quarterCircle, 999) shouldBe 0.806
        }

        test("Exercise 3.82: the first estimate is one experiment's outcome").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val quarterCircle = estimateIntegral({ x, y -> x * x + y * y <= 1.0 }, 0.0, 1.0, 0.0, 1.0)
            streamRef(quarterCircle, 0) shouldBe 1.0
        }

        test("Exercise 3.82: four times the quarter-circle estimate approaches pi").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val quarterCircle = estimateIntegral({ x, y -> x * x + y * y <= 1.0 }, 0.0, 1.0, 0.0, 1.0)
            val pi = 4.0 * streamRef(quarterCircle, 99999)
            abs(pi - PI) shouldBeLessThan 0.05
        }

        test("Exercise 3.82: the area factor scales every estimate").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val rectangle = estimateIntegral({ _, _ -> true }, 0.0, 2.0, 0.0, 1.0)
            rectangle.take(3) shouldBe listOf(2.0, 2.0, 2.0)
        }
    })
