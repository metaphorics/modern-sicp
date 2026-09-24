// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.51

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_51Test :
    FunSpec({
        test("Exercise 2.51: belowViaTransform and belowViaRotation paint the same segments").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_51() shouldBe true
        }
    })
