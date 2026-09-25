// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_79

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_79Test :
    FunSpec({
        test("Exercise 4.79: scoped versus renaming") {
            scopedVersusRenaming() shouldBe
                listOf(
                    "query: (outranked-by (Bitdiddle Ben) ?who)",
                    "renaming answers=1 scoped answers=1 equal=true",
                    "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))",
                    "query: (outranked-by ?staff-person ?boss)",
                    "renaming answers=14 scoped answers=14 equal=true",
                    "query: (and (salary ?staff-person ?amount) (outranked-by ?staff-person ?boss))",
                    "renaming answers=14 scoped answers=14 equal=true",
                )
        }
    })
