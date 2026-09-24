// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_09Test :
    FunSpec({
        test("Exercise 3.9: factorial(6) is 720 in both versions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(720L, factorial(6L))
            org.junit.jupiter.api.Assertions
                .assertEquals(720L, factorialIter(6L))
        }
    })
