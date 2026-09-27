// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.8

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_08Test :
    FunSpec({
        test("Exercise 2.8: [6, 8] minus [3, 5] is [1, 5]").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Interval(1.0, 5.0), ex_2_08())
        }
    })
