// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.15: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_15Test :
    FunSpec({
        test("Exercise 4.15: the budget halts the quick program, abstains on the diagonal") {
            haltingProbeTranscript() shouldBe "halts\nunknown after 200 steps\nunknown after 300 steps\n"
        }
    })
