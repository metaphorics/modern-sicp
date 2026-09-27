// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55 (replaced)

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.VSym

public class E2_55Test :
    FunSpec({
        test("Exercise 2.55: the car of the doubled-quote value is the symbol quote").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_55() shouldBe VSym("quote")
        }
    })
