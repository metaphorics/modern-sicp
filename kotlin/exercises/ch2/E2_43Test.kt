// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.43

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_43Test :
    FunSpec({
        test("Exercise 2.43: queensSlow(6) finds the same 4 solutions as queens(6)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_43() shouldBe 4
        }
    })
