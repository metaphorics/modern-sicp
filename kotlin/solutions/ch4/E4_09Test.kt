// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_09Test :
    FunSpec({
        test("Exercise 4.9: while sums the loop values") {
            whileSumTranscript() shouldBe "15\n"
        }

        test("Exercise 4.9: false while does not run its body") {
            whileNeverRunsTranscript() shouldBe "0\n"
        }

        test("Exercise 4.9: until computes a product and stops at its target") {
            untilProductTranscript() shouldBe "95040\n13\n"
        }

        test("Exercise 4.9: satisfied until does not run its body") {
            untilNeverRunsTranscript() shouldBe "0\n"
        }

        test("Exercise 4.9: nested loops keep their bindings local") {
            nestedLoopsTranscript() shouldBe "6\n"
        }
    })
