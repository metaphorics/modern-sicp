// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.11

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_11Test :
    FunSpec({
        test("Exercise 2.11: [-2, 3] times [-1, 2], both spanning zero, is [-4, 6]").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Interval(-4.0, 6.0), ex_2_11())
        }
    })
