// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.27

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_27Test :
    FunSpec({
        test("Exercise 2.27: deepReverse of ((1 2) (3 4)) is ((4 3) (2 1))").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_27() shouldBe "((4 3) (2 1))"
        }
    })
