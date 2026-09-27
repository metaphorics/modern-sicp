// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import java.math.BigInteger

public class E1_19Test :
    FunSpec({
        test("Exercise 1.19: Fib(20) by successive squaring of the transform").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(BigInteger.valueOf(6765L), ex_1_19(20L))
        }
    })
