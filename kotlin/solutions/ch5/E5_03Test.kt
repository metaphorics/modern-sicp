// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_03

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_03Test :
    FunSpec({
        test("Exercise 5.3: the sqrt machine in two stages") {
            sqrtMachineTranscripts() shouldBe
                listOf(
                    "1.4142156862745097",
                    "1.4142156862745097",
                    "3.00009155413138",
                    "3.00009155413138",
                )
        }
        test("Exercise 5.3: the expanded machine computes the same fixed point") {
            sqrtStageOneTranscript(2.0) shouldBe sqrtStageTwoTranscript(2.0)
            sqrtStageOneTranscript(9.0) shouldBe sqrtStageTwoTranscript(9.0)
        }
    })
