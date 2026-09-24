// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.46

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest
import java.util.concurrent.CyclicBarrier
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicIntegerArray
import kotlin.concurrent.thread

public class E3_46Test :
    FunSpec({
        test("Exercise 3.46: two processes in the test-and-set window both acquire").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val cell = FlagCell(false)
                val acquired = intArrayOf(0, 0)
                coroutineScope {
                    launch { acquired[0] = if (testAndSetRacy(cell)) 0 else 1 }
                    launch { acquired[1] = if (testAndSetRacy(cell)) 0 else 1 }
                }
                (acquired[0] + acquired[1]) shouldBe 2
            }
        }

        test("Exercise 3.46: the atomic cell lets exactly one acquirer through, over 3000 real races").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val cell = AtomicBoolean(false)
            val rounds = 3000
            val wins = AtomicIntegerArray(rounds)
            val barrier = CyclicBarrier(2)
            val racer =
                thread {
                    repeat(rounds) { r ->
                        cell.set(false)
                        barrier.await()
                        if (cell.compareAndSet(false, true)) {
                            wins.incrementAndGet(r)
                        }
                        barrier.await()
                    }
                }
            val other =
                thread {
                    repeat(rounds) { r ->
                        cell.set(false)
                        barrier.await()
                        if (cell.compareAndSet(false, true)) {
                            wins.incrementAndGet(r)
                        }
                        barrier.await()
                    }
                }
            racer.join()
            other.join()
            for (r in 0 until rounds) {
                wins.get(r) shouldBe 1
            }
        }
    })
