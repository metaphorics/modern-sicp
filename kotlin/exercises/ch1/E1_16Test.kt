// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.16

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_16Test :
    FunSpec({
        test("Exercise 1.16: the iterative process computes 2^10").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(1024L, ex_1_16(2L, 10L))
        }
    })
