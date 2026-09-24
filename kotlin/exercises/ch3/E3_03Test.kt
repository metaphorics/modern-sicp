// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.3

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_03Test :
    FunSpec({
        test("Exercise 3.3: the correct password withdraws, the wrong one is refused").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val acc = makeAccount(100L, "secret-password")
            org.junit.jupiter.api.Assertions
                .assertEquals(60L, acc.withdraw("secret-password", 40L).getOrNull())
            org.junit.jupiter.api.Assertions
                .assertEquals(
                    AccountError.WrongPassword,
                    acc.deposit("some-other-password", 50L).leftOrNull(),
                )
        }
    })
