// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_04Test :
    FunSpec({
        test("Exercise 3.4: the eighth consecutive wrong password calls the cops").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val acc = makeAccountWithLockout(100L, "secret-password")
            repeat(7) { acc.withdraw("wrong", 1L) }
            org.junit.jupiter.api.Assertions
                .assertEquals(
                    AccountError.CallTheCops,
                    acc.withdraw("wrong", 1L).leftOrNull(),
                )
        }
    })
