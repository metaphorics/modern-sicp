// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.37

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_37Test :
    FunSpec({
        test("Exercise 5.37: the counts and the monitored session show the blind saves").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            preservingComparison().last() shouldBe "every save pairs with one restore: true"
        }
    })
