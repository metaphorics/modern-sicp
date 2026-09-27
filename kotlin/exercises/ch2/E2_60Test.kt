// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.60

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_60Test :
    FunSpec({
        test("Exercise 2.60: unionSetDup unions the book's dup-encoded {1, 2, 3} with {1, 3}").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_60() shouldBe listOf(2L, 3L, 2L, 1L, 3L, 2L, 2L, 1L, 3L)
        }
    })
