// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.42

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_42Test :
    FunSpec({
        test("Exercise 4.42: one placement survives the liars' statements").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            liarsSolutions() shouldBe
                listOf("((betty 3) (ethel 5) (joan 2) (kitty 1) (mary 4))")
        }
    })
