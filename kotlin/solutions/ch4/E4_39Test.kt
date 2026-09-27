// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.39

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_39Test :
    FunSpec({
        test("Exercise 4.39: both orders answer the same assignment") {
            dwellingAnswer() shouldBe "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
        }

        test("Exercise 4.39: the book order costs 1470 backtracks to the first answer") {
            bookOrderBacktracks() shouldBe 1470L
        }

        test("Exercise 4.39: the Fletcher-first reorder costs the same 1470") {
            reorderedBacktracks() shouldBe 1470L
        }
    })
