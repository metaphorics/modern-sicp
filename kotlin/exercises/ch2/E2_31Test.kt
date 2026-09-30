// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.31

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_31Test :
    FunSpec({
        test("Exercise 2.31 returns the square-mapped tree without changing its shape").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_31() shouldBe
                tree(leaf(1L), tree(leaf(4L), tree(leaf(9L), leaf(16L)), leaf(25L)), tree(leaf(36L), leaf(49L)))
        }
    })
