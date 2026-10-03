// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.70

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_70Test :
    FunSpec({
        test("Exercise 4.70: the let-binding discipline").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letPurposeDemo() shouldBe
                listOf(
                    "?x = a",
                    "?x = b",
                    "?x = a",
                    "?x = b",
                    "?x = c",
                    "snapshot unchanged: true",
                )
        }
    })
