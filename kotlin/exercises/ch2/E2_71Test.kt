// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.71

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_71Test :
    FunSpec({
        test("Exercise 2.71: for n = 5 the most frequent symbol needs 1 bit and the least frequent needs 4").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_71(5) shouldBe (1 to 4)
        }
    })
