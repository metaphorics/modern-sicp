// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.17

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_17Test :
    FunSpec({
        test("Exercise 2.17 renders the final one-element suffix").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_17() shouldBe "PairCell(first=Whole(value=34), second=Empty)"
        }
    })
