// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.27

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_27Test :
    FunSpec({
        test("Exercise 2.27 reverses children at every node").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_27() shouldBe tree(tree(leaf(4L), leaf(3L)), tree(leaf(2L), leaf(1L)))
        }
    })
