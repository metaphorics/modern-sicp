// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.32

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_32Test :
    FunSpec({
        test("Exercise 2.32: subsets of (1 2 3) are the book's eight subsets in order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_32() shouldBe
                listOf(emptyList(), listOf(3L), listOf(2L), listOf(2L, 3L), listOf(1L), listOf(1L, 3L), listOf(1L, 2L), listOf(1L, 2L, 3L))
        }
    })
