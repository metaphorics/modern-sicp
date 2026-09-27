// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.24

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_24Test :
    FunSpec({
        test("Exercise 1.24: fastPrime accepts all 12 primes found in exercise 1.22").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(List(12) { true }, ex_1_24())
        }
    })
