// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.45

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_45Test :
    FunSpec({
        test("Exercise 2.45: split(::beside, ::below)(wave, 1) matches rightSplit(wave, 1)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_45() shouldBe 3 * 14
        }
    })
