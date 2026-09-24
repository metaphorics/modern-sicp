// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.38

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.test.runTest

public class E3_38Test :
    FunSpec({
        test("Exercise 3.38: forced interleavings enumerate the possible balances").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val values = sortedSetOf<Long>()
                for (order in allInterleavings(listOf(2, 2, 2))) {
                    val balance = Cell(100L)
                    val peter = Process38()
                    val paul = Process38()
                    val mary = Process38()
                    val lanes: List<List<suspend () -> Unit>> =
                        listOf(
                            listOf({ peterDepositRead(balance, peter) }, { peterDepositWrite(balance, peter) }),
                            listOf({ paulWithdrawRead(balance, paul) }, { paulWithdrawWrite(balance, paul) }),
                            listOf({ maryHalfRead(balance, mary) }, { maryHalfWrite(balance, mary) }),
                        )
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
