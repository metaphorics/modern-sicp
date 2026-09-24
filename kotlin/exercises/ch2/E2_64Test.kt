// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.64

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_64Test :
    FunSpec({
        test("Exercise 2.64: listToTree of [1, 3, 5, 7, 9, 11] round-trips to the original six elements").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            treeToList1(ex_2_64()) shouldBe listOf(1L, 3L, 5L, 7L, 9L, 11L)
        }
    })
