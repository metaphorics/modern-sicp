// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.40

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_40Test :
    FunSpec({
        test("Exercise 4.40: 3125 assignments before distinctness, 120 after") {
            assignmentsBeforeDistinct() shouldBe 3125
            assignmentsAfterDistinct() shouldBe 120
        }

        test("Exercise 4.40: pruning cuts the backtracks from 1835 to 260") {
            // Measured guest backtracks (this search counts 1835/260 where the
            // book's evaluator counts 1470/210).
            naiveBacktracksToFirst() shouldBe 1835L
            prunedBacktracksToFirst() shouldBe 260L
        }

        test("Exercise 4.40: the pruned program answers the same assignment") {
            prunedAnswer() shouldBe "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
        }
    })
