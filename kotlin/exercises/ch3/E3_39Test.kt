// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.39

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest

public class E3_39Test :
    FunSpec({
        test("Exercise 3.39: the three schedules give exactly 100, 101, 121").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val outcomes = mutableListOf<Long>()
                val s = Serializer()
                for (first in 0..1) {
                    val x = Cell(10L)
                    coroutineScope {
                        if (first == 0) {
                            launch { squareThenAssign(x, s) }
                            launch { serializedIncrement(x, s) }
                        } else {
                            launch { serializedIncrement(x, s) }
                            launch { squareThenAssign(x, s) }
                        }
                    }
                    outcomes.add(x.value)
                }
                val y = Cell(10L)
                coroutineScope {
                    launch { squareThenAssignWithPause(y, s) }
                    launch { serializedIncrement(y, s) }
                }
                outcomes.add(y.value)
                outcomes.toSortedSet() shouldBe sortedSetOf(100L, 101L, 121L)
            }
        }
    })
