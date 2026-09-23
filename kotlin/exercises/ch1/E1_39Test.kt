// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.39

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_39Test :
    FunSpec({
        test("Exercise 1.39: Lambert's continued fraction for tan(pi/4) is 1").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_39() shouldBe 1.0
        }
    })
