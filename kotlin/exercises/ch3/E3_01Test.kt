// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.1

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_01Test :
    FunSpec({
        test("Exercise 3.1: makeAccumulator(5), then two calls with 10, give 15 and 25").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = makeAccumulator(5L)
            org.junit.jupiter.api.Assertions
                .assertEquals(15L, a(10L))
            org.junit.jupiter.api.Assertions
                .assertEquals(25L, a(10L))
        }
    })
