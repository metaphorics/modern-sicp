// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.30

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_30Test :
    FunSpec({
        test("Exercise 1.30: sum-cubes(1, 10) run iteratively still totals 3025").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_30() shouldBe 3025L
        }
    })
