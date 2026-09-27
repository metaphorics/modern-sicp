// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.8

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_08Test :
    FunSpec({
        test("Exercise 1.8: cube root by Newton's method").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(3.0000005410641766, ex_1_08(27.0), 1e-9)
        }
    })
