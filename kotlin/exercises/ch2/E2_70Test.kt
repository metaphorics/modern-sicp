// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.70

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_70Test :
    FunSpec({
        test("Exercise 2.70: the rock-song lyrics need 84 Huffman bits against 108 fixed-length bits").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_70() shouldBe (84 to 108)
        }
    })
