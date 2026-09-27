// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.29

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_29Test :
    FunSpec({
        test("Exercise 1.29: Simpson's Rule integrates cube from 0 to 1 far more accurately than section 1.3.1's integral").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_29() shouldBe Pair(0.25000000000000006, 0.25000000000000006)
        }
    })
