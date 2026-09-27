// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.7

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_07Test :
    FunSpec({
        test("Exercise 1.7: the relative end test at both extremes").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(3.000000001396984, ex_1_07(9.0), 1e-12)
        }
    })
