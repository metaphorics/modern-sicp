// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.35a

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_35aTest :
    FunSpec({
        test("Exercise 4.35a: the choice totals at the six triples between 1 and 20") {
            choicesTakenWithin20() shouldBe listOf(925L, 1805L, 1979L, 2583L, 2713L, 3181L)
        }

        test("Exercise 4.35a: the single triple between 1 and 9 takes 221 choices") {
            choicesTakenWithin9() shouldBe listOf(221L)
        }
    })
