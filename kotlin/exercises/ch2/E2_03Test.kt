// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.3

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_03Test :
    FunSpec({
        test("Exercise 2.3: a 4-by-3 rectangle has perimeter 14 and area 12").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(14.0 to 12.0, ex_2_03())
        }
    })
