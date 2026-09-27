// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.1

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_01Test :
    FunSpec({
        test("Exercise 1.1: results of the expression sequence").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(
                    listOf(10L, 12L, 8L, 3L, 6L, 19L, 4L, 16L, 6L, 16L),
                    ex_1_01(),
                )
        }
    })
