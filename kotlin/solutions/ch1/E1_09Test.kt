// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.9

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_09Test :
    FunSpec({
        test("both procedures add 4 and 5 correctly") {
            ex_1_09() shouldBe (9L to 9L)
        }
        test("plusIterative stays a loop for an a no recursive call frame would survive") {
            plusIterative(1_000_000L, 0L) shouldBe 1_000_000L
        }
    })
