// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.54

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_54Test :
    FunSpec({
        test("Exercise 3.54: mul-streams multiplies elementwise").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            mulStreams(integers, ones).take(6) shouldBe listOf(1L, 2L, 3L, 4L, 5L, 6L)
            mulStreams(streamEnumerateInterval(1, 4), streamEnumerateInterval(2, 5)).take(10) shouldBe
                listOf(2L, 6L, 12L, 20L)
        }

        test("Exercise 3.54: factorials counts 1, 1, 2, 6, 24, ...").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            factorials.take(8) shouldBe listOf(1L, 1L, 2L, 6L, 24L, 120L, 720L, 5040L)
            streamRef(factorials, 20) shouldBe 2432902008176640000L
        }
    })
