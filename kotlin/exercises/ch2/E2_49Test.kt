// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.49

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_49Test :
    FunSpec({
        test("Exercise 2.49: outline has 4 segments, X has 2, diamond has 4").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_49() shouldBe listOf(4, 2, 4)
        }
    })
