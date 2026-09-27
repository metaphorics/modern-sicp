// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.50

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_50Test :
    FunSpec({
        test("Exercise 2.50: rotate180(wave) still paints 14 segments").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_50() shouldBe 14
        }
    })
