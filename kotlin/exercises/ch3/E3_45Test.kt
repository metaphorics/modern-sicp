// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.45

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.withTimeout

public class E3_45Test :
    FunSpec({
        test("Exercise 3.45: Louis's plain deposits work").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val account = makeLouisAccount(100L)
                plainDeposit(account, 40L) shouldBe 140L
                account.balance() shouldBe 140L
            }
        }

        test("Exercise 3.45: serialized-exchange on Louis's account never returns").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val a1 = makeLouisAccount(10L)
                val a2 = makeLouisAccount(20L)
                val deadlocked =
                    try {
                        withTimeout(2_000) {
                            serializedExchange(a1, a2)
                        }
                        false
                    } catch (e: TimeoutCancellationException) {
                        true
                    }
                deadlocked shouldBe true
            }
        }

        test("Exercise 3.45: the same exchange on the section's account completes").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val a1 = makeAccountAndSerializer(10L)
                val a2 = makeAccountAndSerializer(20L)
                withTimeout(2_000) {
                    serializedExchange(a1, a2)
                }
                listOf(a1.balance(), a2.balance()).sorted() shouldBe listOf(10L, 20L)
            }
        }
    })
