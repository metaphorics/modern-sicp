// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.24

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Whole

public class E3_24Test :
    FunSpec({
        test("Exercise 3.24: a tolerance key test finds keys near the probe").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val near =
                makeTable { a, b -> a is Whole && b is Whole && kotlin.math.abs(a.value - b.value) <= 5 }
            near.insert(Whole(40), Whole(1))
            near.lookup(Whole(41)) shouldBe Whole(1)
        }
    })
