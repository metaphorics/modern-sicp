// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4a

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_04aTest :
    FunSpec({
        test("Exercise 2.4a: the identity law holds for the pair (3, 4)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_04a() shouldBe true
        }
    })
