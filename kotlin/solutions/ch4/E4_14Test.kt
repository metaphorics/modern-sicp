// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.14: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_14Test :
    FunSpec({
        test("Exercise 4.14: the table map answers known names and fails the closure") {
            louisTranscript() shouldBe "[1, 4, 9]\nerror\n"
        }

        test("Exercise 4.14: the value map applies the closure") {
            evaTranscript() shouldBe "[1, 4, 9]\n"
        }
    })
