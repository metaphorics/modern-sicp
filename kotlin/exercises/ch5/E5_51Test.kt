// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.51

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_51Test :
    FunSpec({
        test("Exercise 5.51: the C translation builds and runs the factorial session").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            evaluatorInC() shouldBe
                listOf(
                    "120",
                    "the C evaluator agrees with the explicit-control run: true",
                )
        }
    })
