// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.21

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_21Test :
    FunSpec({
        test("Exercise 1.21: the smallest divisors of 199, 1999, 19999").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Triple(199L, 1999L, 7L), ex_1_21())
        }
    })
