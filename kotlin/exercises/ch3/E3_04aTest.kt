// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_04aTest :
    FunSpec({
        test("Exercise 3.4a: a successful withdrawal is logged as ok").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val acc = makeAccountWithAuditLog(100L, "secret-password")
            acc.withdraw("secret-password", 40L)
            org.junit.jupiter.api.Assertions
                .assertEquals(1, acc.auditLog().size)
            org.junit.jupiter.api.Assertions
                .assertEquals("ok", acc.auditLog().first().outcome)
        }
    })
