// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.48

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.withTimeout

public class E3_48Test :
    FunSpec({
        test("Exercise 3.48: opposing ordered exchanges both complete").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val a1 = makeNumberedAccount(1, 10L)
                val a2 = makeNumberedAccount(2, 20L)
                coroutineScope {
                    launch { orderedSerializedExchange(a1, a2) }
                    launch { orderedSerializedExchange(a2, a1) }
                }
                listOf(a1.balance(), a2.balance()).sorted() shouldBe listOf(10L, 20L)
            }
        }

        test("Exercise 3.48: the unordered exchange deadlocks on the same run").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val a1 = makeNumberedAccount(1, 10L)
                val a2 = makeNumberedAccount(2, 20L)
                val deadlocked =
                    try {
                        withTimeout(2_000) {
                            kotlinx.coroutines.coroutineScope {
                                launch { kotlinx.coroutines.withTimeout(1_000) { serializedExchange(a1, a2) } }
                                launch { kotlinx.coroutines.withTimeout(1_000) { serializedExchange(a2, a1) } }
                            }
                        }
                        false
                    } catch (e: TimeoutCancellationException) {
                        true
                    }
                deadlocked shouldBe true
            }
        }
    })
