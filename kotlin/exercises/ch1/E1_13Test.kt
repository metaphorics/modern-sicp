// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.13

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_13Test :
    FunSpec({
        test("Exercise 1.13: the closed form gives Fib(10)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(55L, ex_1_13(10))
        }
    })
