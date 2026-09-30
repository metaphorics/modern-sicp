// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_48

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_48Test :
    FunSpec({
        test("Exercise 5.48: the two-phase compile and run agrees with the direct run") {
            compileAndRunReport() shouldBe
                listOf(
                    "the compiled run answers: 120",
                    "the direct run answers: 120",
                    "the two runs agree: true",
                )
        }
    })
