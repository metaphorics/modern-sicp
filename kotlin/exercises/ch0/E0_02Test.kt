// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E0_02Test :
    FunSpec({
        test("Exercise 0.2: compose and repeated").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val inc: (Long) -> Long = { x -> x + 1L }
            val double: (Long) -> Long = { x -> x * 2L }
            org.junit.jupiter.api.Assertions
                .assertEquals(11L, compose(inc, double)(5L))
            org.junit.jupiter.api.Assertions
                .assertEquals(10L, repeated(inc, 10)(0L))
        }
    })
