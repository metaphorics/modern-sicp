// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.66

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_66Test :
    FunSpec({
        test("Exercise 2.66: lookup finds key 7 in the sample database").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_66() shouldBe "dave"
        }
    })
