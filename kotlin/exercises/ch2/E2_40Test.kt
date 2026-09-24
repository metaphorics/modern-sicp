// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.40

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_40Test :
    FunSpec({
        test("Exercise 2.40: uniquePairs(6) enumerates the 15 ordered pairs").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_40().size shouldBe 15
        }
    })
