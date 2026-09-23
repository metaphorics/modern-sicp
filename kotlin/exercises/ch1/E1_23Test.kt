// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.23

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_23Test :
    FunSpec({
        test("Exercise 1.23: the faster search still finds 19999's smallest divisor").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(7L, ex_1_23(19_999L))
        }
    })
