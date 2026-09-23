// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.12

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_12Test :
    FunSpec({
        test("Exercise 2.12: a center of 50 with ten percent tolerance is [45, 55]").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Interval(45.0, 55.0), ex_2_12())
        }
    })
