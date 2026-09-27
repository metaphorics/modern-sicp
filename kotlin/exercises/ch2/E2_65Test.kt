// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_65Test :
    FunSpec({
        test(
            "Exercise 2.65: the union and intersection of the tree sets built from {1, 3, 5, 7, 9} and {3, 5, 7, 9, 11}",
        ).config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_65() shouldBe (listOf(1L, 3L, 5L, 7L, 9L, 11L) to listOf(3L, 5L, 7L, 9L))
        }
    })
