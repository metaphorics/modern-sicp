// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.40

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_40Test :
    FunSpec({
        test("Exercise 4.40: 3125 assignments before distinctness, 120 after").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            assignmentsBeforeDistinct() shouldBe 3125
            assignmentsAfterDistinct() shouldBe 120
        }

        test("Exercise 4.40: pruning cuts the backtracks from 1470 to 210").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            naiveBacktracksToFirst() shouldBe 1470L
            prunedBacktracksToFirst() shouldBe 210L
        }

        test("Exercise 4.40: the pruned program answers the same assignment").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            prunedAnswer() shouldBe "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
        }
    })
