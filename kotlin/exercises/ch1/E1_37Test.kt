// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.37

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_37Test :
    FunSpec({
        test("Exercise 1.37: k=10 is the smallest k accurate to 4 decimal places, both processes agree").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_37() shouldBe Triple(10L, 0.6179775280898876, 0.6179775280898876)
        }
    })
