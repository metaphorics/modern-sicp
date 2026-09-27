// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_01

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_01Test :
    FunSpec({
        test("Exercise 5.1: the iterative factorial machine") {
            factorialMachineRuns() shouldBe
                listOf(
                    "1",
                    "1",
                    "120",
                    "3628800",
                )
        }
    })
