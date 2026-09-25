// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.43

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_43Test :
    FunSpec({
        test("Exercise 4.43: told Mary Ann is Moore's, Lorna's father is Downing").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            yachtToldAnswers() shouldBe listOf("((lornas-father downing))")
        }

        test("Exercise 4.43: untold, Parker and Downing both fit").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            yachtUntoldAnswers() shouldBe
                listOf("((lornas-father parker))", "((lornas-father downing))")
        }
    })
