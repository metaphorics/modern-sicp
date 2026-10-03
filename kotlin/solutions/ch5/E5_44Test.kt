// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_44

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_44Test :
    FunSpec({
        test("Exercise 5.44: the shadowing analysis reports the rebound name and the probe answers") {
            openCodingShadowingReport() shouldBe
                listOf(
                    "rebound names reported: plus",
                    "the probe answers: 3",
                )
        }
    })
