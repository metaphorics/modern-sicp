// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.4: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_04Test :
    FunSpec({
        test("Exercise 4.4: direct and/or short-circuit with effect pins") {
            specialAndOrTranscript() shouldBe "false\n1\n0\n0\ntrue\n1\n1\n0\n"
        }

        test("Exercise 4.4: the derived and keeps the short-circuit") {
            derivedAndTranscript() shouldBe "false\n0\n"
        }
    })
