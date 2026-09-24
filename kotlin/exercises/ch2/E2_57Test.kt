// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.57

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_57Test :
    FunSpec({
        test(
            "Exercise 2.57: the derivative of (* x y (+ x 3)) with respect to x matches the hand-computed answer",
        ).config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_57() shouldBe "(+ (* x y) (* y (+ x 3)))"
        }
    })
