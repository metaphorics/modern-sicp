// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.11

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_11Test :
    FunSpec({
        test("Exercise 3.11: deposit 40 gives 90, withdraw 60 gives 30").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val acc = makeBankAccount(50L)
            org.junit.jupiter.api.Assertions
                .assertEquals(90L, acc.deposit(40L))
            org.junit.jupiter.api.Assertions
                .assertEquals(arrow.core.Either.Right(30L), acc.withdraw(60L))
        }
    })
