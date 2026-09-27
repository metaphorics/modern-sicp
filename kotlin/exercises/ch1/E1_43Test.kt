// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_43Test :
    FunSpec({
        test("Exercise 1.43: repeated(square, 2)(5) is 625, the fourth power of 5").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_43() shouldBe 625.0
        }
    })
