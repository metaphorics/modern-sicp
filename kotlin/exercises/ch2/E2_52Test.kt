// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.52

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_52Test :
    FunSpec({
        test(
            "Exercise 2.52: squareLimitModified(waveWithSmile, 1) is 4 square-of-four corners of 4 cornerSplitModified copies each",
        ).config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_52() shouldBe 256
        }
    })
