// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.41

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_41Test :
    FunSpec({
        test("Exercise 1.41: double(double(double))(inc)(5) applies inc sixteen times").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_41() shouldBe 21L
        }
    })
