// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.12

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_12Test :
    FunSpec({
        test("Exercise 1.12: row 4 of Pascal's triangle").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(listOf(1L, 4L, 6L, 4L, 1L), ex_1_12(4))
        }
    })
