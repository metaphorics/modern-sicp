// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.51

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_51Test :
    FunSpec({
        test("Exercise 4.51: a permanent write accumulates across the trials").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            permanentSetTranscript() shouldBe "[a, b, 2]\n[a, c, 3]\n[b, a, 4]\n"
        }

        test("Exercise 4.51: an ordinary write rolls back with its branch").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ordinarySetTranscript() shouldBe "[a, b, 1]\n[a, c, 1]\n[b, a, 1]\n"
        }
    })
