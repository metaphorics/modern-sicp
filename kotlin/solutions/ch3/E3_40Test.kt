// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.40

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest

public class E3_40Test :
    FunSpec({
        test("Exercise 3.40: the 35 interleavings give exactly the fifth powers") {
            runTest {
                val values = sortedSetOf<Long>()
                for (order in allInterleavings(listOf(3, 4))) {
                    val x = Cell(10L)
                    val p1 = Square40()
                    val p2 = Cube40()
                    val lanes: List<List<suspend () -> Unit>> =
                        listOf(
                            listOf({ squareReadFirst(x, p1) }, { squareReadSecond(x, p1) }, { squareWrite(x, p1) }),
                            listOf({ cubeReadFirst(x, p2) }, { cubeReadSecond(x, p2) }, { cubeReadThird(x, p2) }, { cubeWrite(x, p2) }),
                        )
                    val progress = IntArray(2)
                    for (lane in order) {
                        lanes[lane][progress[lane]]()
                        progress[lane] += 1
                    }
                    values.add(x.value)
                }
                values shouldBe sortedSetOf(100L, 1000L, 10000L, 100000L, 1000000L)
            }
        }

        test("Exercise 3.40: serialized, both launch orders answer a million") {
            runTest {
                val x = Cell(10L)
                val s = Serializer()
                coroutineScope {
                    launch { serializedSquare(x, s) }
                    launch { serializedCube(x, s) }
                }
                x.value shouldBe 1000000L
                val y = Cell(10L)
                coroutineScope {
                    launch { serializedCube(y, s) }
                    launch { serializedSquare(y, s) }
                }
                y.value shouldBe 1000000L
            }
        }
    })
