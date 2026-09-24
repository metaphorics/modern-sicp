// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.63

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_63Test :
    FunSpec({
        test(
            "Exercise 2.63: treeToList1 and treeToList2 agree on the sorted set for every Figure 2.16 tree shape",
        ).config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_63().forEach { it shouldBe listOf(1L, 3L, 5L, 7L, 9L, 11L) }
        }
    })
