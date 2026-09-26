// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.46

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_46Test :
    FunSpec({
        test("measured Fibonacci stack counts grow with the recursive tree") {
            fibStackRatioTable() shouldBe
                listOf(
                    "n=5: interpreted pushes=408 depth=28; compiled pushes=77 depth=14; special pushes=28 depth=8; " +
                        "compiled/interpreted=0.189/0.500; special/interpreted=0.069/0.286",
                    "n=8: interpreted pushes=1864 depth=43; compiled pushes=337 depth=23; special pushes=132 depth=14; " +
                        "compiled/interpreted=0.181/0.535; special/interpreted=0.071/0.326",
                    "n=10: interpreted pushes=4944 depth=53; compiled pushes=887 depth=29; special pushes=352 depth=18; " +
                        "compiled/interpreted=0.179/0.547; special/interpreted=0.071/0.340",
                )
        }
    })
