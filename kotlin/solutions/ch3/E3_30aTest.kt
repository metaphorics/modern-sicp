// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.random.Random

public class E3_30aTest :
    FunSpec({
        test("the boundary cases: zero, all ones, 8+7 carries, and a set carry in") {
            rippleAdd(0, 0, 0) shouldBe 0
            rippleAdd(15, 15, 0) shouldBe 30
            rippleAdd(15, 15, 1) shouldBe 31
            rippleAdd(8, 7, 0) shouldBe 15
            rippleAdd(8, 7, 1) shouldBe 16
            rippleAdd(0, 15, 1) shouldBe 16
            rippleAdd(1, 0, 1) shouldBe 2
            rippleAdd(15, 0, 1) shouldBe 16
        }

        test("a seeded sweep of 256 cases equals integer addition every time") {
            val rng = Random(20260924)
            repeat(256) {
                val a = rng.nextInt(16)
                val b = rng.nextInt(16)
                val cIn = rng.nextInt(2)
                rippleAdd(a, b, cIn) shouldBe a + b + cIn
            }
        }
    })
