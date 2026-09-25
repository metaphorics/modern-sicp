// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.57

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_57Test :
    FunSpec({
        test("Exercise 3.57: the memoized fibs spend one addition per element") {
            fibAdditions(0, memoized = true) shouldBe 0
            fibAdditions(1, memoized = true) shouldBe 0
            (2..15).forEach { i -> fibAdditions(i, memoized = true) shouldBe i - 1 }
            fibAdditions(15, memoized = true) shouldBe 14
            streamRef(fibs, 15) shouldBe 610L
        }

        test("Exercise 3.57: the unmemoized construction pays the whole re-derivation tree") {
            (2..15).map { fibAdditions(it, memoized = false) } shouldBe
                listOf(1, 3, 7, 14, 26, 46, 79, 133, 221, 364, 596, 972, 1581, 2567)
            fibAdditions(15, memoized = false) shouldBe 2567
            (fibAdditions(15, memoized = false) > 1000) shouldBe true
        }
    })
