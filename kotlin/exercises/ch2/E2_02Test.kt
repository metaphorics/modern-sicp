// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.2

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_02Test :
    FunSpec({
        test("Exercise 2.2: the midpoint of (2, 3) to (8, 11) is (5, 7)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Point(5.0, 7.0), ex_2_02())
        }
    })
