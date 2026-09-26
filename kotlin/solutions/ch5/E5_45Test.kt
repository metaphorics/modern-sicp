// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.45

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_45Test :
    FunSpec({
        test("measured factorial stack counts distinguish evaluator, compiler, and special machine") {
            stackRatioTable() shouldBe
                listOf(
                    "n=5: interpreted pushes=144 depth=28; compiled pushes=31 depth=14; special pushes=8 depth=8; " +
                        "compiled/interpreted=0.215/0.500; special/interpreted=0.056/0.286",
                    "n=10: interpreted pushes=304 depth=53; compiled pushes=61 depth=29; special pushes=18 depth=18; " +
                        "compiled/interpreted=0.201/0.547; special/interpreted=0.059/0.340",
                    "n=20: interpreted pushes=624 depth=103; compiled pushes=121 depth=59; special pushes=38 depth=38; " +
                        "compiled/interpreted=0.194/0.573; special/interpreted=0.061/0.369",
                    "Compiler improvements: propagate register needs more precisely and open-code the recursive " +
                        "call/return path, as the hand controller avoids evaluator dispatch and redundant saves.",
                )
        }
    })
