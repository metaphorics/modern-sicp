// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.41

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest

public class E3_41Test :
    FunSpec({
        test("Exercise 3.41: an observer that straddles the transfer sums money that never existed").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val a1 = makeSerializedAccount(100L)
                val a2 = makeSerializedAccount(100L)
                sumStraddling(a1, a2, 60L) shouldBe 260L
                a1.balance() shouldBe 40L
                a2.balance() shouldBe 160L
            }
        }

        test("Exercise 3.41: an observer that reads before the transfer sees the true total").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val a1 = makeSerializedAccount(100L)
                val a2 = makeSerializedAccount(100L)
                sumUnstraddled(a1, a2, 60L) shouldBe 200L
                a1.balance() shouldBe 40L
                a2.balance() shouldBe 160L
            }
        }

        test("Exercise 3.41: Ben's serialized read answers only committed balances").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val ben = makeBenAccount(100L)
                coroutineScope {
                    launch { ben.withdraw(60L) }
                }
                ben.balance() shouldBe 40L
            }
        }
    })
