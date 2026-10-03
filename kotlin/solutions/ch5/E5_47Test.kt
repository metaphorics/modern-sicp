// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_47

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_47Test :
    FunSpec({
        test("Exercise 5.47: compiled caller invokes an interpreted closure") {
            mixedCallsRun() shouldBe
                listOf(
                    "the compiled declaration answers: 141",
                    "the interpreted binding answers: 42",
                    "the mixed call reached the interpreted closure: true",
                )
        }
    })
