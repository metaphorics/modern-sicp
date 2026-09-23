// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.1

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_01Test :
    FunSpec({
        test("Exercise 2.1: makeRat(-3, -9) normalizes to 1/3, both signs positive").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Rational(1L, 3L), ex_2_01())
        }
    })
