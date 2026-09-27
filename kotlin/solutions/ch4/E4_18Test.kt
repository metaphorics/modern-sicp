// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.18

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_18Test :
    FunSpec({
        test("Exercise 4.18: the text strategy assigns in source order and the forced read succeeds") {
            textStrategyTranscript() shouldBe "3\n"
        }

        test("Exercise 4.18: the alternative strategy reads the reserved name at initializer time") {
            altStrategyTranscript() shouldBe "Error: type mismatch: dy is read before it is assigned\n"
        }
    })
