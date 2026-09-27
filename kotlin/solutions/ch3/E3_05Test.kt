// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.5

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.PI
import kotlin.math.abs

public class E3_05Test :
    FunSpec({
        test("randomInRange always stays inside its half-open bounds") {
            val rand = makeRand(42UL)
            val allInRange = (1..200).all { randomInRange(2.0, 8.0, rand) in 2.0..<8.0 }
            allInRange shouldBe true
        }

        test("estimateIntegral over the full rectangle with an always-true predicate is the rectangle's area") {
            val estimate = estimateIntegral(500, 0.0, 3.0, 0.0, 4.0, makeRand(5UL)) { _, _ -> true }
            estimate shouldBe 12.0
        }

        test("estimateIntegral over an empty predicate is zero") {
            val estimate = estimateIntegral(500, 0.0, 3.0, 0.0, 4.0, makeRand(5UL)) { _, _ -> false }
            estimate shouldBe 0.0
        }

        test("estimatePiViaIntegral with a large trial count lands close to pi") {
            val estimate = estimatePiViaIntegral(200_000, makeRand(1UL))
            (abs(estimate - PI) < 0.05) shouldBe true
        }
    })
