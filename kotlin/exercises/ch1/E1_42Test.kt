// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.42

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_42Test :
    FunSpec({
        test("Exercise 1.42: compose(square, inc)(6) is 49").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_42() shouldBe 49L
        }
    })
