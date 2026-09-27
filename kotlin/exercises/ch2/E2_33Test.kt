// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.33

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_33Test :
    FunSpec({
        test("Exercise 2.33: mapViaAccumulate squares (1 2 3 4) to (1 4 9 16)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_33() shouldBe listOf(1L, 4L, 9L, 16L)
        }
    })
