// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.14

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_14Test :
    FunSpec({
        test("Exercise 4.14: Louis's primitive map cannot call the compound procedure") {
            louisTranscript() shouldBe "Error: not a procedure: #[compound-procedure]\n"
        }

        test("Exercise 4.14: Eva's object-language map answers the same call") {
            evaTranscript() shouldBe "(1 4 9)\n"
        }
    })
