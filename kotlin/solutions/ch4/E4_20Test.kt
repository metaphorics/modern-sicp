// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_20Test :
    FunSpec({
        test("Exercise 4.20: letrec binds the factorial recursion") {
            letrecFactTranscript() shouldBe "3628800\n"
        }

        test("Exercise 4.20: a premature read finds the reservation empty") {
            letrecPrematureReadTranscript() shouldBe "error\n"
        }

        test("Exercise 4.20: a plain let never installs the recursive name") {
            plainLetUnboundTranscript() shouldBe "error\n"
        }
    })
