// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.10

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_10Test :
    FunSpec({
        test("Exercise 1.10: the three named applications of the statement").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(listOf(1024L, 65536L, 65536L), ex_1_10())
        }
    })
