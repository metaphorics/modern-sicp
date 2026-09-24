// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.21

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_21Test :
    FunSpec({
        test("Exercise 2.21: squareList of (1 2 3 4) is (1 4 9 16)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_21() shouldBe "(1 4 9 16)"
        }
    })
