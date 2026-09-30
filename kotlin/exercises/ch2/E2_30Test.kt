// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.30

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_30Test :
    FunSpec({
        test("Exercise 2.30 squares leaves while preserving the shared tree shape").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_30() shouldBe
                tree(leaf(1L), tree(leaf(4L), tree(leaf(9L), leaf(16L)), leaf(25L)), tree(leaf(36L), leaf(49L)))
        }
    })
