// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_02

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_02Test :
    FunSpec({
        test("Exercise 5.2: the machine described in the register-machine language") {
            factorialAssemblyReport() shouldBe
                listOf(
                    "5 instructions, 2 labels; fact-loop=0 fact-done=5",
                    "120",
                    "720",
                )
        }
    })
