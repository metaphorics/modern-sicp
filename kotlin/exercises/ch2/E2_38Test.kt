// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.38

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_38Test :
    FunSpec({
        test("Exercise 2.38: foldRight divides to 3/2, foldLeft to 1/6").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_38() shouldBe listOf(1.5, 1.0 / 6.0)
        }
    })
