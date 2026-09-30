// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_49

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_49Test :
    FunSpec({
        test("Exercise 5.49: a compiled definition survives a second turn on the same machine") {
            readCompileExecutePrintRuns() shouldBe
                listOf(
                    "first turn: 144",
                    "second turn: 882",
                    "the two turns share one machine: true",
                )
        }
    })
