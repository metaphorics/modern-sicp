// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.20

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_20Test :
    FunSpec({
        test("Exercise 5.20: the memory-vector drawing of the three conses").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            memoryVectorDrawing() shouldBe emptyList()
        }
    })
