// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43a

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_43aTest :
    FunSpec({
        test("Exercise 1.43a: repeatedLog(square, 2)(5) is 625, same as the linear repeated").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_43a() shouldBe 625.0
        }
    })
