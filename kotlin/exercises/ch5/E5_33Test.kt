// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.33

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_33Test :
    FunSpec({
        test("Exercise 5.33: both compilations are shown with their save/restore pairs and both answers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            altFactorialComparison().last() shouldBe "the two runs answer alike: true"
        }
    })
