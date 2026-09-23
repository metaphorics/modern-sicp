// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.32

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_32Test :
    FunSpec({
        test("Exercise 1.32: sum-cubes and factorial, both defined as calls to accumulate").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_32() shouldBe Pair(3025L, 720L)
        }
    })
