// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.02

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_02Test :
    FunSpec({
        test("Exercise 5.02: the machine written in the register-machine language").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            factorialAssemblyReport() shouldBe
                listOf(
                    "5 instructions, 2 labels; fact-loop=0 fact-done=5",
                    "120",
                    "720",
                )
        }
    })
