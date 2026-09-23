// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.18

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_18Test :
    FunSpec({
        test("Exercise 1.18: 3 times 7, iteratively").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(21L, ex_1_18(3L, 7L))
        }
    })
