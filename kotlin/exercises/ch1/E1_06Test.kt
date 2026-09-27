// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.6

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_06Test :
    FunSpec({
        test("Exercise 1.6: both clauses of newIf evaluate").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(1 to 1, ex_1_06())
        }
    })
