// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_58

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_58Test :
    FunSpec({
        test("Exercise 4.58: the big-shot query") {
            // Fair QOr round-robin order (book 4.4.3 interleaving);
            // same answers, branches alternate instead of running out.
            bigShotQuery() shouldBe
                listOf(
                    "?person = [Bitdiddle, Ben]",
                    "?division = computer",
                    "?person = [Warbucks, Oliver]",
                    "?division = administration",
                    "?person = [Scrooge, Eben]",
                    "?division = accounting",
                )
        }
    })
