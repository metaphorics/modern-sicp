// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_20Test :
    FunSpec({
        test("Exercise 4.20: the mutual even?/odd? letrec runs under the reserved let").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letrecEvenOddTranscript() shouldBe "#t\n"
        }

        test("Exercise 4.20: the factorial letrec recurses to 3628800").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letrecFactTranscript() shouldBe "3628800\n"
        }

        test("Exercise 4.20: an initializer reading a later binding fails the typed premature read").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letrecPrematureReadTranscript() shouldBe "Error: type mismatch: y is read before it is assigned\n"
        }

        test("Exercise 4.20: part b -- the same recursion under a plain let fails unbound").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            plainLetUnboundTranscript() shouldBe "Error: unbound variable: fact\n"
        }
    })
