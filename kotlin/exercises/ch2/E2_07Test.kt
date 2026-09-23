// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.7

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_07Test :
    FunSpec({
        test("Exercise 2.7: makeInterval(6.12, 7.48) has that lower and upper bound").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Interval(6.12, 7.48), ex_2_07())
        }
    })
