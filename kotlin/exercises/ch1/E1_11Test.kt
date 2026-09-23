// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.11

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_11Test :
    FunSpec({
        test("Exercise 1.11: both processes compute f(10)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(1892L to 1892L, ex_1_11(10L))
        }
    })
