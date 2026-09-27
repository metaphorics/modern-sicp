// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.48

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_48Test :
    FunSpec({
        test("Exercise 2.48: the segment from (0,0) to (3,4) has length 5.0").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_48() shouldBe 5.0
        }
    })
