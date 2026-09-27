// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.41a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.yield

public class E3_41aTest :
    FunSpec({
        test("Exercise 3.41a: a read before the withdraw commits approves on a stale balance").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val account = makeSerializedAccount(100L)
                val purchase = Purchase(50L)
                coroutineScope {
                    launch {
                        authorizeRead(account, purchase)
                        kotlinx.coroutines.yield()
                    }
                    launch { spendBehindTheRead(account, 60L) }
                }
                purchase.observed shouldBe 100L
                purchase.decision shouldBe PurchaseDecision.Approved
                account.balance() shouldBe 40L
            }
        }

        test("Exercise 3.41a: the same decision after the withdraw sees the fresh balance and declines").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val account = makeSerializedAccount(100L)
                val purchase = Purchase(50L)
                coroutineScope {
                    launch { spendBehindTheRead(account, 60L) }
                    launch {
                        authorizeRead(account, purchase)
                        kotlinx.coroutines.yield()
                    }
                }
                purchase.observed shouldBe 40L
                purchase.decision shouldBe PurchaseDecision.Declined
                account.balance() shouldBe 40L
            }
        }
    })
