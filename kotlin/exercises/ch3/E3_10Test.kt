// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.10

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_10Test :
    FunSpec({
        test("Exercise 3.10: makeWithdrawLet(100L), then w1(50L) is 50").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val w1 = makeWithdrawLet(100L)
            org.junit.jupiter.api.Assertions
                .assertEquals(arrow.core.Either.Right(50L), w1(50L))
        }
    })
