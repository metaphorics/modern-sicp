// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.14

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_14Test :
    FunSpec({
        test("Exercise 4.14: Louis's primitive map cannot call the compound procedure").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            louisTranscript() shouldBe "Error: not a procedure: #[compound-procedure]\n"
        }

        test("Exercise 4.14: Eva's object-language map answers the same call").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            evaTranscript() shouldBe "(1 4 9)\n"
        }
    })
