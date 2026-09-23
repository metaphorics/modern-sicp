// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.15

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_15Test :
    FunSpec({
        test("Exercise 1.15: p is applied 5 times for sine(12.15)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(5, ex_1_15())
        }
    })
