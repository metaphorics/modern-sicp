// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.34

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_34Test :
    FunSpec({
        test("Exercise 5.34: the tail call's direct transfer and the constant depths are pinned").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            iterativeFactorialCompilation().last() shouldBe "compiled maximum depth independent of n: true"
        }
    })
