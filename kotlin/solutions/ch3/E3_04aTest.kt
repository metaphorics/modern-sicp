// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_04aTest :
    FunSpec({
        test("a successful withdrawal is logged as ok with sequence 1") {
            val acc = makeAccountWithAuditLog(100L, "secret-password")
            acc.withdraw("secret-password", 40L)
            acc.auditLog() shouldBe listOf(AuditEntry(1, "withdraw", "ok"))
        }

        test("wrong password, insufficient funds, and call-the-cops each log their own outcome") {
            val acc = makeAccountWithAuditLog(100L, "secret-password")
            acc.deposit("wrong", 1L)
            acc.withdraw("secret-password", 1_000L)
            repeat(8) { acc.withdraw("wrong", 1L) }
            val log = acc.auditLog()
            log.map { it.outcome } shouldBe
                listOf(
                    "wrong-password",
                    "insufficient-funds",
                    "wrong-password",
                    "wrong-password",
                    "wrong-password",
                    "wrong-password",
                    "wrong-password",
                    "wrong-password",
                    "wrong-password",
                    "call-the-cops",
                )
            log.map { it.seq } shouldBe (1..10).toList()
        }

        test("every entry names its operation") {
            val acc = makeAccountWithAuditLog(50L, "pw")
            acc.deposit("pw", 10L)
            acc.withdraw("pw", 5L)
            acc.auditLog().map { it.operation } shouldBe listOf("deposit", "withdraw")
        }
    })
