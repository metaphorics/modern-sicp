// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.33

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_33Test :
    FunSpec({
        test("Exercise 1.33: sum of squares of primes in [2, 20], and product of integers below 10 coprime to 10").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_33() shouldBe Pair(1027L, 189L)
        }
    })
