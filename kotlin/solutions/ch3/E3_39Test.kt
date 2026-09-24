// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.39

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest

public class E3_39Test :
    FunSpec({
        test("Exercise 3.39: straight through, P1 then P2 answers 101") {
            runTest {
                val x = Cell(10L)
                val s = Serializer()
                coroutineScope {
                    launch { squareThenAssign(x, s) }
                    launch { serializedIncrement(x, s) }
                }
                x.value shouldBe 101L
            }
        }

        test("Exercise 3.39: P2 first answers 121") {
            runTest {
                val x = Cell(10L)
                val s = Serializer()
                coroutineScope {
                    launch { serializedIncrement(x, s) }
                    launch { squareThenAssign(x, s) }
                }
                x.value shouldBe 121L
            }
        }

        test("Exercise 3.39: a switch between square and bare assignment loses P2 and answers 100") {
            runTest {
                val x = Cell(10L)
                val s = Serializer()
                coroutineScope {
                    launch { squareThenAssignWithPause(x, s) }
                    launch { serializedIncrement(x, s) }
                }
                x.value shouldBe 100L
            }
        }

        test("Exercise 3.39: the three schedules give exactly 100, 101, 121") {
            runTest {
                val outcomes = mutableListOf<Long>()
                for (square in listOf(::squareThenAssign, ::squareThenAssignWithPause)) {
                    for (first in 0..1) {
                        val x = Cell(10L)
                        val s = Serializer()
                        val p1: suspend () -> Unit = { square(x, s) }
                        val p2: suspend () -> Unit = { serializedIncrement(x, s) }
                        coroutineScope {
                            if (first == 0) {
                                launch { p1() }
                                launch { p2() }
                            } else {
                                launch { p2() }
                                launch { p1() }
                            }
                        }
                        outcomes.add(x.value)
                    }
                }
                outcomes.toSortedSet() shouldBe sortedSetOf(100L, 101L, 121L)
            }
        }

        test("Exercise 3.39: contrast, the fully serialized pair answers only 101 or 121") {
            runTest {
                val x = Cell(10L)
                val s = Serializer()
                coroutineScope {
                    launch { fullySerializedSquare(x, s) }
                    launch { serializedIncrement(x, s) }
                }
                x.value shouldBe 101L
                val y = Cell(10L)
                coroutineScope {
                    launch { serializedIncrement(y, s) }
                    launch { fullySerializedSquare(y, s) }
                }
                y.value shouldBe 121L
            }
        }
    })
