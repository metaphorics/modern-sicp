// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.17

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_17Test :
    FunSpec({
        test("Exercise 4.17: the scanned body answers identically and its closure carries one extra frame") {
            scannedFramesTranscript() shouldBe "21\n3\n1\n"
        }

        test("Exercise 4.17: the plain sequential body runs in the parameter frame alone") {
            plainFramesTranscript() shouldBe "21\n2\n1\n"
        }
    })
