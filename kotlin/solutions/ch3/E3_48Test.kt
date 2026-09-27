// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.48

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.withTimeout
import kotlinx.coroutines.yield

public class E3_48Test :
    FunSpec({
        test("Exercise 3.48: opposing ordered exchanges both complete") {
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

        test("Exercise 3.48: ordered exchanges run while the unordered pair deadlocks") {
            runTest {
                val a1 = makeNumberedAccount(1, 10L)
                val a2 = makeNumberedAccount(2, 20L)
                withTimeout(2_000) {
                    coroutineScope {
                        launch { orderedSerializedExchange(a1, a2) }
                        launch { orderedSerializedExchange(a2, a1) }
                    }
                }
                val b1 = makeNumberedAccount(1, 10L)
                val b2 = makeNumberedAccount(2, 20L)
                val deadlocked =
                    try {
                        withTimeout(2_000) {
                            coroutineScope {
                                // The book's unordered serialized-exchange twice
                                // over: Peter enters b1's serializer and then
                                // wants b2's; Paul enters b2's and then wants
                                // b1's. The yields mark the two switch points
                                // that put each process inside the other's head
                                // of the order.
                                launch {
                                    b1.serializer().serialized {
                                        yield()
                                        b2.serializer().serialized {
                                            exchange(b1, b2)
                                        }()
                                    }()
                                }
                                launch {
                                    b2.serializer().serialized {
                                        b1.serializer().serialized {
                                            exchange(b2, b1)
                                        }()
                                    }()
                                }
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
