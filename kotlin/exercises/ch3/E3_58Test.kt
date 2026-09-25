// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.58

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_58Test :
    FunSpec({
        test("Exercise 3.58: expanding 1 by 7 at radix 10 gives the digits of 1/7").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            expand(1, 7, 10).take(8) shouldBe listOf(1L, 4L, 2L, 8L, 5L, 7L, 1L, 4L)
        }

        test("Exercise 3.58: expanding 3 by 8 comes out exact and runs on with zeros").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            expand(3, 8, 10).take(6) shouldBe listOf(3L, 7L, 5L, 0L, 0L, 0L)
        }

        test("Exercise 3.58: other radices and pairs give other digit strings").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            expand(1, 3, 2).take(6) shouldBe listOf(0L, 1L, 0L, 1L, 0L, 1L)
            expand(1, 2, 10).take(3) shouldBe listOf(5L, 0L, 0L)
        }
    })
