// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.5

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import kotlin.math.abs

public class E3_05Test :
    FunSpec({
        test("Exercise 3.5: estimatePiViaIntegral lands close to pi").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val estimate = estimatePiViaIntegral(50_000, makeRand(1UL))
            org.junit.jupiter.api.Assertions
                .assertTrue(abs(estimate - kotlin.math.PI) < 0.1)
        }
    })
