// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.42

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest

public class E3_42Test :
    FunSpec({
        test("Exercise 3.42: Ben's account behaves like the text's under the same scenario").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                runBenScenario(makeSerializedAccount(100L)) shouldBe 110L
                runBenScenario(bensAccount(100L)) shouldBe 110L
            }
        }

        test("Exercise 3.42: both versions keep the total under interleaved transfers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val a = makeSerializedAccount(100L)
                val b = bensAccount(100L)
                coroutineScope {
                    launch { a.deposit(30L) }
                    launch { a.withdraw(10L) }
                    launch { b.deposit(30L) }
                    launch { b.withdraw(10L) }
                }
                a.balance() shouldBe 120L
                b.balance() shouldBe 120L
            }
        }
    })
