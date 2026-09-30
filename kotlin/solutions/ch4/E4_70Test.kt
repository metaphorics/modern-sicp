// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_70

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_70Test :
    FunSpec({
        test("Exercise 4.70: the let-binding discipline") {
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
