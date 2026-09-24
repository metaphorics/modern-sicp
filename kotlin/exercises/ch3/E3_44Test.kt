// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.44

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.test.runTest

public class E3_44Test :
    FunSpec({
        test("Exercise 3.44: transfers interleaved between the two steps conserve the total").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val (a, b) = concurrentTransfers(30L, 20L)
                a + b shouldBe 200L
            }
        }

        test("Exercise 3.44: the other launch order conserves it too").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            runTest {
                val (a, b) = concurrentTransfers(20L, 30L)
                a + b shouldBe 200L
            }
        }
    })
