// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.47

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Semaphore
import kotlinx.coroutines.test.runTest

public class E3_47Test :
    FunSpec({
        test("Exercise 3.47: the mutex semaphore conserves its permits") {
            runTest {
                val s = MutexSemaphore(3)
                val probe = PermitProbe()
                probeOccupancy(6, { s.acquire() }, { s.release() }, probe)
                probe.completed shouldBe 6
                probe.maxInside shouldBe 3
            }
        }

        test("Exercise 3.47: the test-and-set semaphore conserves its permits") {
            runTest {
                val s = TestAndSetSemaphore(3)
                val probe = PermitProbe()
                probeOccupancy(6, { s.acquire() }, { s.release() }, probe)
                probe.completed shouldBe 6
                probe.maxInside shouldBe 3
            }
        }

        test("Exercise 3.47: the token semaphore conserves its permits") {
            runTest {
                val s = TokenSemaphore(3)
                val probe = PermitProbe()
                probeOccupancy(6, { s.acquire() }, { s.release() }, probe)
                probe.completed shouldBe 6
                probe.maxInside shouldBe 3
            }
        }

        test("Exercise 3.47: kotlinx's Semaphore shows the same conservation") {
            runTest {
                val s = Semaphore(3)
                val probe = PermitProbe()
                probeOccupancy(6, { s.acquire() }, { s.release() }, probe)
                probe.completed shouldBe 6
                probe.maxInside shouldBe 3
            }
        }

        test("Exercise 3.47: the token semaphore's use brackets the critical section") {
            runTest {
                val s = TokenSemaphore(2)
                val seen = mutableListOf<Int>()
                kotlinx.coroutines.coroutineScope {
                    repeat(4) { i ->
                        launch {
                            s.use {
                                seen.add(i)
                            }
                        }
                    }
                }
                seen.size shouldBe 4
            }
        }
    })
