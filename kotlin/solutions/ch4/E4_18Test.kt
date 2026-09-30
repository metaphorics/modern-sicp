// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.18: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_18Test :
    FunSpec({
        test("Exercise 4.18: source-order assignment serves the fellow read") {
            textStrategyTranscript() shouldBe "3\n"
        }

        test("Exercise 4.18: initializers-first leaves the fellow read empty") {
            altStrategyTranscript() shouldBe "error\n"
        }
    })
