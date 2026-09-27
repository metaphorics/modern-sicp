// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.7

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.abs

public class E1_07Test :
    FunSpec({
        test("the absolute test passes too early for a small radicand") {
            // True root 0.01; the absolute test stops at 0.0323...
            sqrtAbsolute(x = 0.0001) shouldBe 0.03230844833048122
        }
        test("the absolute test never passes for 3.14159e18") {
            val stalled = improveSteps(3.14159e18, 300)
            abs(stalled * stalled - 3.14159e18) shouldBe 512.0
        }
        test("the relative test converges at both extremes") {
            val small = ex_1_07(0.0001)
            (abs(small - 0.01) < 0.001 * 0.01) shouldBe true
            val large = ex_1_07(3.14159e18)
            (abs(large * large - 3.14159e18) < 0.002 * 3.14159e18) shouldBe true
        }
        test("the relative test still answers the session of the text") {
            ex_1_07(9.0) shouldBe 3.000000001396984
        }
    })
