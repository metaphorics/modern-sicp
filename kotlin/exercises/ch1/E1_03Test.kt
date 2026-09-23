// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.3

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_03Test :
    FunSpec({
        test("Exercise 1.3: sum of squares of the two larger numbers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(13L, ex_1_03(1L, 2L, 3L))
        }
    })
