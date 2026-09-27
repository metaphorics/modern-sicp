// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_65a

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_65aTest :
    FunSpec({
        test("Exercise 4.65a: the deduplicated wheel listing") {
            deduplicatedWheel() shouldBe
                listOf(
                    "(wheel (Bitdiddle Ben))",
                    "(wheel (Warbucks Oliver))",
                )
        }
    })
