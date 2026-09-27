// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.16

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_16Test :
    FunSpec({
        test("Exercise 2.16: a - a over [9, 11] reports [-2, 2], not the exact zero").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Interval(-2.0, 2.0), ex_2_16())
        }
    })
