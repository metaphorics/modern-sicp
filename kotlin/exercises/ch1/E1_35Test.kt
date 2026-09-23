// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.35

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_35Test :
    FunSpec({
        test("Exercise 1.35: the golden ratio as a fixed point of x -> 1 + 1/x").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_35() shouldBe 1.6180327868852458
        }
    })
