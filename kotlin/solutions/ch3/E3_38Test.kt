// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.38

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.yield

/** The six steps of the three processes, grouped by lane. */
private fun lanes38(
    balance: Cell,
    peter: Process38,
    paul: Process38,
    mary: Process38,
): List<List<suspend () -> Unit>> =
    listOf(
        listOf({ peterDepositRead(balance, peter) }, { peterDepositWrite(balance, peter) }),
        listOf({ paulWithdrawRead(balance, paul) }, { paulWithdrawWrite(balance, paul) }),
        listOf({ maryHalfRead(balance, mary) }, { maryHalfWrite(balance, mary) }),
    )

/**
 * One forced run: the processes launch in `order`, and a process whose
 * flag is true yields between its read and its write, the point where
 * the scheduler may interleave the others.
 */
private suspend fun forced38(
    order: List<Int>,
    peterPauses: Boolean = false,
    paulPauses: Boolean = false,
    maryPauses: Boolean = false,
): Long {
    val balance = Cell(100L)
    val peter = Process38()
    val paul = Process38()
    val mary = Process38()
    val pauses = listOf(peterPauses, paulPauses, maryPauses)
    val lanes = lanes38(balance, peter, paul, mary)
    coroutineScope {
        for (lane in order) {
            launch {
                lanes[lane][0]()
                if (pauses[lane]) {
                    yield()
                }
                lanes[lane][1]()
            }
        }
    }
    return balance.value
}

public class E3_38Test :
    FunSpec({
        test("Exercise 3.38: the six sequential orders give exactly 35, 40, 45, 50") {
            runTest {
                val values = sortedSetOf<Long>()
                for (order in listOf(
                    listOf(0, 1, 2),
                    listOf(0, 2, 1),
                    listOf(1, 0, 2),
                    listOf(1, 2, 0),
                    listOf(2, 0, 1),
                    listOf(2, 1, 0),
                )) {
                    values.add(forced38(order))
                }
                values shouldBe sortedSetOf(35L, 40L, 45L, 50L)
            }
        }

        test("Exercise 3.38: forced interleavings land on the interleaved-only values") {
            runTest {
                // All three pause: reads before writes, writes 110, 90, 50,
                // Mary's write last on the 100 she read.
                forced38(listOf(0, 1, 2), peterPauses = true, paulPauses = true, maryPauses = true) shouldBe 50L
                // Peter and Paul run through first; Mary reads the 90
                // Paul left and halves it to 45.
                forced38(listOf(0, 1, 2), maryPauses = true) shouldBe 45L
                // Peter reads 100 and pauses while Paul and Mary write
                // 80 then 40; Peter's write of 110 lands last.
                forced38(listOf(0, 1, 2), peterPauses = true) shouldBe 110L
                // Peter writes 110 first; Paul reads 110 and waits while
                // Mary halves it, then restores 90.
                forced38(listOf(0, 1, 2), paulPauses = true) shouldBe 90L
                // Mary and Paul read 100, Peter runs 100 to 110, Mary
                // halves to 50, and Paul writes the 80 he saw.
                forced38(listOf(2, 1, 0), paulPauses = true, maryPauses = true) shouldBe 80L
                // Peter writes 110, Paul writes 90, then Mary halves the
                // 110 she saw before Paul's write.
                forced38(listOf(0, 2, 1), maryPauses = true) shouldBe 55L
                // Mary halves to 50, Peter reads 50 and waits out Paul's
                // write of 30, then writes 60.
                forced38(listOf(2, 0, 1), peterPauses = true) shouldBe 60L
            }
        }

        test("Exercise 3.38: all 90 interleavings give exactly the ten-value set") {
            runTest {
                val values = sortedSetOf<Long>()
                for (order in allInterleavings(listOf(2, 2, 2))) {
                    val balance = Cell(100L)
                    val peter = Process38()
                    val paul = Process38()
                    val mary = Process38()
                    val lanes = lanes38(balance, peter, paul, mary)
                    val progress = IntArray(3)
                    for (lane in order) {
                        lanes[lane][progress[lane]]()
                        progress[lane] += 1
                    }
                    values.add(balance.value)
                }
                values shouldBe sortedSetOf(30L, 35L, 40L, 45L, 50L, 55L, 60L, 80L, 90L, 110L)
            }
        }
    })
