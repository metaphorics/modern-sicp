// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.68

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_68Test :
    FunSpec({
        test("Exercise 2.68: encode round-trips exercise 2.67's decoded message back to the sample bits").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_68() shouldBe listOf(0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0)
        }
    })
