// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.17: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_17Test :
    FunSpec({
        test("Exercise 4.17: the scan-out captures one frame more") {
            scannedFramesTranscript() shouldBe "21\n3\n1\n"
        }

        test("Exercise 4.17: sequential defines capture the call frame") {
            plainFramesTranscript() shouldBe "21\n2\n1\n"
        }
    })
