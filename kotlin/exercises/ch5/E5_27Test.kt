// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.27

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_27Test :
    FunSpec({
        test("Exercise 5.27: the recursive factorial's stack table and the two formulas").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            recursiveFactorialMeasurements() shouldBe emptyList()
        }
    })
